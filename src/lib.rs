#![doc = include_str!("../README.md")]

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_v() -> *const ();
}

/// The V grammar, convertible into a `tree_sitter::Language` with `.into()`.
// SAFETY: the vendored parser exports a tree-sitter language with static lifetime.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_v) };

#[cfg(test)]
mod tests {
    use super::LANGUAGE;

    fn parser() -> tree_sitter::Parser {
        let language = LANGUAGE.into();
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .expect("V language should load with Tree-sitter 0.26");
        parser
    }

    fn assert_parses(source: &str, declaration_kind: &str) {
        let tree = parser().parse(source, None).expect("V source should parse");
        let root = tree.root_node();
        assert!(!root.has_error(), "Parse errors: {}", root.to_sexp());
        assert_eq!(root.start_byte(), 0);
        assert_eq!(root.end_byte(), source.len());
        let mut cursor = root.walk();
        assert!(
            root.named_children(&mut cursor)
                .any(|node| node.kind() == declaration_kind),
            "Expected {declaration_kind}: {}",
            root.to_sexp()
        );
    }

    #[test]
    fn language_loads() {
        let language: tree_sitter::Language = LANGUAGE.into();
        assert_eq!(language.abi_version(), 15);
        parser();
    }

    #[test]
    fn heap_attribute_on_struct() {
        assert_parses(
            "@[heap]\npub struct Point {\n    value int\n}\n",
            "struct_declaration",
        );
    }

    #[test]
    fn inline_attribute_on_function() {
        assert_parses(
            "@[inline]\npub fn add_one(value int) int {\n    return value + 1\n}\n",
            "function_declaration",
        );
    }

    #[test]
    fn public_union() {
        assert_parses(
            "pub union Value {\n    integer int\n    decimal f64\n}\n",
            "struct_declaration",
        );
    }
}
