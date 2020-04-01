use rustfix;
use std::collections::HashSet;
use std::fs;

macro_rules! expect_empty_json_test {
    ($name:ident, $file:expr) => {
        #[test]
        fn $name() {
            let json = fs::read_to_string(concat!("./tests/edge-cases/", $file)).unwrap();
            let expected_suggestions = rustfix::get_suggestions_from_json(
                &json,
                &HashSet::new(),
                rustfix::Filter::Everything,
            )
            .unwrap();
            assert!(expected_suggestions.is_empty());
        }
    };
}

expect_empty_json_test! {multiple_fix_options_yield_no_suggestions, "skip-multi-option-lints.json"}
expect_empty_json_test! {out_of_bounds_test, "out_of_bounds.recorded.json"}
expect_empty_json_test! {utf8_identifiers_test, "utf8_idents.recorded.json"}
