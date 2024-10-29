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
                assert!(dbg!(crate::internals::get_case(value)) == crate::internals::Case::$var);
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
}
