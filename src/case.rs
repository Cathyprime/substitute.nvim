use crate::rules::case_rule_functions::check as c;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CaseBuf {
    Ada(String),            // [[:upper:]] + _
    Camel(String),          // [[:upper:]] + !starts_with [[:upper]]
    Dot(String),            // .
    Kebab(String),          // -
    Pascal(String),         // [[:upper:]] + starts_with [[:upper]]
    Path(String),           // /
    ScreamingSnake(String), // all [[:upper:]] + _
    Snake(String),          // ![[:upper:]] + _
    Space(String),          // ' '
    TitleDash(String),      // [[:upper:]] + -
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Case {
    Ada,            // [[:upper:]] + _
    Camel,          // [[:upper:]] + !starts_with [[:upper]]
    Dot,            // .
    Kebab,          // -
    Pascal,         // [[:upper:]] + starts_with [[:upper]]
    Path,           // /
    ScreamingSnake, // all [[:upper:]] + _
    Snake,          // ![[:upper:]] + _
    Space,          // ' '
    TitleDash,      // [[:upper:]] + -
}

impl Case {
    pub(crate) fn into_buf_case(self, str: String) -> CaseBuf {
        match self {
            Case::Ada => CaseBuf::Ada(str),
            Case::Camel => CaseBuf::Camel(str),
            Case::Dot => CaseBuf::Dot(str),
            Case::Kebab => CaseBuf::Kebab(str),
            Case::Pascal => CaseBuf::Pascal(str),
            Case::Path => CaseBuf::Path(str),
            Case::ScreamingSnake => CaseBuf::ScreamingSnake(str),
            Case::Snake => CaseBuf::Snake(str),
            Case::Space => CaseBuf::Space(str),
            Case::TitleDash => CaseBuf::TitleDash(str),
        }
    }
}

impl TryFrom<&str> for CaseBuf {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let predicates = [
            c::ada,
            c::camel,
            c::dot,
            c::kebab,
            c::pascal,
            c::path,
            c::screaming_snake,
            c::snake,
            c::space,
            c::title_dash,
        ];
        for pred in predicates {
            if let Some(case) = pred(value) {
                return Ok(case.into_buf_case(value.to_owned()));
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! simp_case {
        ($name:ident, $var:ident, $str:literal) => {
            #[test]
            fn $name() {
                let value = $str;
                let case = CaseBuf::try_from(value);
                assert_eq!(case, Ok(CaseBuf::$var($str.to_string())))
            }
        };
    }

    simp_case!(get_ada_test, Ada, "Some_Ada_Case");
    simp_case!(get_camel_test, Camel, "someCamelCase");
    simp_case!(get_dot_test, Dot, "some.dot.case");
    simp_case!(get_kebab_test, Kebab, "some-kebab-case");
    simp_case!(get_pascal_test, Pascal, "SomeAdaCase");
    simp_case!(get_path_test, Path, "some/path/case");
    simp_case!(
        get_screaming_snake_test,
        ScreamingSnake,
        "SOME_SCREAMING_SNAKE_CASE"
    );
    simp_case!(get_snake_test, Snake, "some_snake_case");
    simp_case!(get_space_test, Space, "some ada case");
    simp_case!(get_title_dash_test, TitleDash, "Some-Title-Dash-Case");
}
