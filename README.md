# tree-sitter-vlang

Rust bindings for the V language Tree-sitter grammar from
[vlang/v-analyzer](https://github.com/vlang/v-analyzer).
This package wraps a pinned upstream grammar; it is not a separate grammar
implementation or an official upstream release.

Version 0.1.0 is being prepared locally and has not been published to crates.io.

## Usage

The Rust crate name is `tree_sitter_vlang`. With this package available as a
dependency and `tree-sitter = "0.26"`, load the grammar as follows:

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

## License

MIT. See [LICENSE](LICENSE) for the retained upstream copyright and license
notices.
