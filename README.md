# tree-sitter-vlang

Rust bindings for the V language Tree-sitter grammar from
[vlang/v-analyzer](https://github.com/vlang/v-analyzer).
This package wraps a pinned upstream grammar; it is not a separate grammar
implementation or an official upstream release.

## Usage

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
tree-sitter-vlang = "0.1.0"
tree-sitter = "0.26"
```

The Rust crate name is `tree_sitter_vlang`. Load the grammar as follows:

```rust
let mut parser = tree_sitter::Parser::new();
parser
    .set_language(&tree_sitter_vlang::LANGUAGE.into())
    .expect("V grammar should load");

let source = "@[heap]\npub struct Point {\n    value int\n}\n";
let tree = parser.parse(source, None).expect("V source should parse");
assert!(!tree.root_node().has_error());
```

The grammar supports modern V attributes such as `@[heap]` and `@[inline]`,
and public unions. Parsing checks syntax, not V type correctness or semantics.
Consumers should inspect `has_error()` before relying on a syntax tree.

## Build requirements

Building requires Rust with edition 2024 support and a C compiler supported by
`cc`. Edition 2024 requires Rust 1.85 or newer; a minimum supported Rust version
for the complete dependency graph has not yet been established.

The generated parser is included, so Node.js, the Tree-sitter CLI, and parser
regeneration are not needed to build this crate. Tests use Tree-sitter 0.26.

## Upstream provenance

- Repository: [vlang/v-analyzer](https://github.com/vlang/v-analyzer)
- Revision: `925d4570d1668746762a2cdf0ecb9a25be704a67`
- Grammar directory: `tree_sitter_v`
- Parser ABI: 15
- External scanner: none

`src/parser.c`, `src/tree_sitter/parser.h`, and `LICENSE` are preserved from the
pinned source. Their SHA-256 checksums are recorded in
`Cargo.toml` under `package.metadata.upstream.sha256`.

Grammar fixes should be proposed upstream. Rust binding, build, and packaging
issues belong in this package. When updating the grammar, update the pinned
revision and checksums together, retain upstream license notices, and rerun all
validation below. Do not hand-edit the generated parser.

## Development

```sh
cargo check --tests
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Tests cover language loading, ABI compatibility, modern attributes on structs
and functions, and a real `pub union`. The README usage example is also compiled
and run as a Rust doctest.

GitHub Actions configuration checks builds and tests on Linux, macOS, and
Windows, plus formatting, Clippy, and local package verification on Linux.
There is no automatic publishing workflow.

With `just` installed, run `just` to list tasks or `just verify` to run the
checks above. `just fmt` applies formatting; `just lint` checks formatting and
runs Clippy. Cargo commands remain usable directly without `just`.

## Release process

Releases are maintained manually. Publishing to crates.io, pushing a Git tag,
and creating a GitHub release are separate operations. Do not release from a
dirty working tree or publish automatically as part of routine validation.

### Prepare

1. Update `version` in `Cargo.toml` and the dependency example in this README.
   Choose a new, unused version; a published version cannot be overwritten.
2. If updating the grammar, update the upstream revision, generated files,
   checksums, and any changed license notices together. Add regression tests
   for relevant grammar changes.
3. Run `just verify` and inspect `cargo package --list`. The package should
   contain the binding, generated parser and header, build script, README,
   license, and Cargo metadata, but not repository-only automation.
4. Review and commit the intended changes, push `main`, and wait for all CI
   jobs on that exact commit to pass. Confirm `git status --short` is empty.

### Publish

A crates.io account with a verified email and permission to publish this crate
is required. Authenticate locally with `cargo login`; never put tokens in
source files, command arguments, or issue/PR discussions. Prefer a short-lived,
crate-scoped token. Subsequent releases require `publish-update`; the initial
release used `publish-new`.

From the clean, CI-verified release commit, run:

```sh
just publish-dry-run
cargo publish
```

The dry run does not upload or reserve a version. `cargo publish` uploads the
archive and normally waits for registry availability. Confirm the intended
version is available on crates.io and check that its docs.rs build succeeds.
Published versions are immutable. Yanking a version is not deletion; fixes
normally require a new version.

### Tag and announce

After successful publication, create an annotated tag on the exact release
commit and push only that tag. Replace `VERSION` below with the version in
`Cargo.toml`, and `RELEASE_COMMIT` with the full CI-verified commit SHA:

```sh
git tag -a vVERSION RELEASE_COMMIT -m "Release tree-sitter-vlang VERSION"
git push origin refs/tags/vVERSION
```

Verify the remote tag points to the release commit. Optionally create a GitHub
release for that tag with a summary of changes and the pinned upstream grammar
revision. Revoke a one-off publishing token when finished.

### If a step fails

- If publishing fails, fix the reported problem and check registry state before
  retrying, especially after a timeout: the upload may already have succeeded.
- If publication succeeded but tagging or pushing failed, finish those steps
  using the same release commit; do not publish again or bump the version just
  to retry a Git operation.
- Do not move an existing release tag or try to overwrite a published version.
  Investigate mismatches before proceeding.

## License

MIT. See [LICENSE](LICENSE) for the retained upstream copyright and license
notices.
