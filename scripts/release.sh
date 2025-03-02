#!/bin/sh

check_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "$1 is not installed. $2"
    exit 1
  fi
}

check_command git "Please install git before running this script."
check_command git-cliff "Run 'cargo install git-cliff' to install it."
check_command typos "Run 'cargo install typos-cli' to install it, otherwise the typos won't be fixed."

if [ -z "$1" ]; then
  echo "Please provide a tag."
  echo "Usage: ./release.sh [X.Y.Z]"
  exit 1
fi

set -- "$1"

echo "Preparing $1..."

# update the version
sed -E -i "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/version = \"${1#v}\"/" wenmoon*/Cargo.toml
git cliff --config ./cliff.toml --tag "$1" >CHANGELOG.md
git add -A && git commit -m "Prepare for Release $1"
git show

# generate a changelog for the tag message
generate_changelog() {
  export GIT_CLIFF_TEMPLATE="
  {% for group, commits in commits | group_by(attribute='group') %}
  {{ group | upper_first }}
  {% for commit in commits %}
    - {% if commit.breaking %}(breaking) {% endif %}{{ commit.message | upper_first }} ({{ commit.id | truncate(length=7, end='') }})
  {% endfor %}
  {% endfor %}"
  git cliff --config detailed --unreleased --strip all
}

CHANGELOG_FILE=$(mktemp)
echo "Release $1" > "$CHANGELOG_FILE"
generate_changelog >> "$CHANGELOG_FILE"
git tag -s -a "$1" -F "$CHANGELOG_FILE"
rm -f "$CHANGELOG_FILE"


echo "Done!"
echo "Now push the commit (git push) and the tag (git push --tags)."
