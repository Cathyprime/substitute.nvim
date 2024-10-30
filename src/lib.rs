mod internals;
mod rules;
mod utils;

#[cfg(test)]
mod tests {

    macro_rules! simp_case {
        ($name:ident, $str:literal, $var:ident) => {
            #[test]
            fn $name() {
                let value = $str;
                assert_eq!(crate::internals::get_case(value).unwrap(), crate::internals::Case::$var);
            }
        };
    }

    simp_case!(is_ada_case, "Some_Ada_Case", Ada);
    simp_case!(is_camel_case, "someCamelCase", Camel);
    simp_case!(is_dot_case, "some.dot.case", Dot);
    simp_case!(is_kebab_case, "some-kebab-case", Kebab);
    simp_case!(is_pascal_case, "SomePascalCase", Pascal);
    simp_case!(is_path_case, "some/path/case", Path);
    simp_case!(
        is_screaming_snake_case,
        "SOME_SCREAMING_SNAKE_CASE",
        ScreamingSnake
    );
    simp_case!(is_snake_case, "some_snake_case", Snake);
    simp_case!(is_space_case, "some space case", Space);
    simp_case!(is_title_dash_case, "Some-Title-Dash-Case", TitleDash);

    #[test]
    fn permutations_test() {
        let parts = [String::from("some"), String::from("word")];
        let perms = vec![
            String::from("Some_Word"),
            String::from("someWord"),
            String::from("some.word"),
            String::from("some-word"),
            String::from("SomeWord"),
            String::from("some/word"),
            String::from("SOME_WORD"),
            String::from("some_word"),
            String::from("Some-Word"),
        ];

        let result = crate::internals::permutations(&parts);

        for x in 0..perms.len() {
            assert_eq!(&perms[x], &result[x])
        }
    }

    #[test]
    fn as_vim_regex() {
        let parts = [String::from("some"), String::from("word")];
        let vim_regex = format!(r#"\v\C({}|{}|{}|{}|{}|{}|{}|{}|{})"#,
            "Some_Word", "someWord", "some.word",
            "some-word", "SomeWord", "some/word",
            "SOME_WORD", "some_word", "Some-Word");

        assert_eq!(dbg!(crate::internals::to_vim_regex_find(&parts)), vim_regex);
    }

    #[test]
    fn transform_test() {
        let parts = [
            String::from("this"),
            String::from("is"),
            String::from("some"),
            String::from("identifier")
        ];
        assert_eq!(crate::internals::transform(&parts, crate::internals::Case::Ada), "This_Is_Some_Identifier");
        assert_eq!(crate::internals::transform(&parts, crate::internals::Case::Snake), "this_is_some_identifier");
        assert_eq!(crate::internals::transform(&parts, crate::internals::Case::Camel), "thisIsSomeIdentifier");
    }
}
