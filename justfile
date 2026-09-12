default:
    @just --list

check:
    cargo check --tests

test:
    cargo test

fmt:
    cargo fmt

lint:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

package:
    cargo package

verify: check test lint
    cargo package --allow-dirty

publish-dry-run:
    cargo publish --dry-run
