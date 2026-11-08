#!/bin/sh

set -eu

check_command() {
	if ! command -v "$1" >/dev/null 2>&1; then
		echo "$1 is not installed. $2"
		exit 1
	fi
}

usage() {
	echo "Usage: ./scripts/release.sh X.Y.Z"
}

check_command git "Please install git before running this script."
check_command git-cliff "Run 'cargo install git-cliff' to install it."
check_command cargo "Please install Rust before running this script."

if [ "$#" -ne 1 ]; then
	usage
	exit 1
fi

# The release workflow matches X.Y.Z tags, so drop an optional leading "v".
version=${1#v}
if ! printf '%s\n' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$'; then
	echo "Invalid version '$1'. Expected X.Y.Z."
	exit 1
fi

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

if [ -n "$(git status --porcelain)" ]; then
	echo "Working tree is not clean. Commit or stash your changes first."
	git status --short
	exit 1
fi

if git rev-parse -q --verify "refs/tags/$version" >/dev/null; then
	echo "Tag $version already exists."
	exit 1
fi

branch=$(git branch --show-current)
if [ -z "$branch" ]; then
	echo "Not on a branch. Check out a branch before releasing."
	exit 1
fi

if [ "$branch" = "main" ]; then
	branch="release/$version"
	if git show-ref --verify --quiet "refs/heads/$branch"; then
		echo "Branch $branch already exists. Check it out or delete it first."
		exit 1
	fi
	git switch -c "$branch"
	echo "Created branch $branch."
fi

current_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
if [ -z "$current_version" ]; then
	echo "Could not read the current version from Cargo.toml."
	exit 1
fi
if [ "$current_version" = "$version" ]; then
	echo "Cargo.toml is already at version $version."
	exit 1
fi

echo "Preparing $version on $branch..."

tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

# Update the version without sed -i, which is not portable between GNU and BSD.
sed -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"$version\"/" Cargo.toml >"$tmp_dir/Cargo.toml"
if cmp -s Cargo.toml "$tmp_dir/Cargo.toml"; then
	echo "Failed to update the version in Cargo.toml."
	exit 1
fi
mv "$tmp_dir/Cargo.toml" Cargo.toml

# Refresh Cargo.lock and verify it works with --locked, as used by the
# release workflow.
cargo metadata --format-version 1 >/dev/null
cargo metadata --format-version 1 --locked >/dev/null

# Generate to a temporary file so a failure cannot truncate CHANGELOG.md.
git cliff --config ./cliff.toml --tag "$version" --output "$tmp_dir/CHANGELOG.md"
if [ ! -s "$tmp_dir/CHANGELOG.md" ]; then
	echo "git-cliff produced an empty changelog."
	exit 1
fi
if ! grep -q "^## \[$version\]" "$tmp_dir/CHANGELOG.md"; then
	echo "The generated changelog has no entry for $version."
	exit 1
fi
mv "$tmp_dir/CHANGELOG.md" CHANGELOG.md

# Sanity-check that the version is consistent before committing.
if ! grep -q "^version = \"$version\"" Cargo.toml; then
	echo "Cargo.toml does not contain version $version."
	exit 1
fi
if ! awk -v version="$version" '
  $0 == "name = \"wenmoon\"" { found = 1; next }
  found && $0 == "version = \"" version "\"" { ok = 1; exit }
  END { exit ok ? 0 : 1 }
' Cargo.lock; then
	echo "Cargo.lock does not contain version $version for wenmoon."
	exit 1
fi

git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "Prepare for release $version"

echo
echo "Prepared $version on $branch."
echo
echo "Next steps:"
echo "  git push -u origin $branch"
echo "  gh pr create --base main --head $branch --title \"Release $version\""
echo
echo "After the PR is merged and CI is green:"
echo "  git switch main && git pull"
echo "  git tag -s -a $version -m \"Release $version\""
echo "  git push origin $version"
