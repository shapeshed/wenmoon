# Contributing

## Development

The toolchain is pinned in `rust-toolchain.toml` (currently Rust 1.88.0).

Run the standard checks before opening a pull request:

```sh
cargo fmt -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
```

Additional repository checks:

```sh
cargo deny check
typos
shellcheck scripts/*.sh
dprint check
```

## Commit style

Use short, imperative commit messages, for example:

```text
Fix handling of nullable provider API fields
Add CoinGecko market data provider
```

Keep unrelated changes in separate commits.

## Release process

The release script prepares a branch with the version bump, lock file update,
and regenerated changelog:

```sh
./scripts/release.sh X.Y.Z
git push -u origin release/X.Y.Z
gh pr create --base main --head release/X.Y.Z --title "Release X.Y.Z"
```

Once the PR is merged and CI is green, tag the release:

```sh
git switch main && git pull
git tag -s -a X.Y.Z -m "Release X.Y.Z"
git push origin X.Y.Z
```

Pushing the tag triggers the release workflow.

The workflow publishes the crate to crates.io using
[Trusted Publishing](https://crates.io/docs/trusted-publishing). Before the
first automated release, configure a trusted publisher for this repository on
the crate's crates.io settings page:

- repository: `shapeshed/wenmoon`
- workflow: `release.yml`

lib.rs indexes crates.io automatically, so no separate lib.rs publish step is
required.

## API keys

Never commit real API keys or a personal `config.toml`. Use a temporary config
when testing provider changes.
