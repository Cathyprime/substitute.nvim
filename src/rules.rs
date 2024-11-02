pub(crate) mod case_rule_functions {
    use crate::case::CaseBuf;

    pub(crate) fn split(case: &CaseBuf) -> Vec<String> {
        match case {
            CaseBuf::Ada(s) => splits::ada(s.as_str()),
            CaseBuf::Camel(s) => splits::camel(s.as_str()),
            CaseBuf::Dot(s) => splits::dot(s.as_str()),
            CaseBuf::Kebab(s) => splits::kebab(s.as_str()),
            CaseBuf::Pascal(s) => splits::pascal(s.as_str()),
            CaseBuf::Path(s) => splits::path(s.as_str()),
            CaseBuf::ScreamingSnake(s) => splits::screaming_snake(s.as_str()),
            CaseBuf::Snake(s) => splits::snake(s.as_str()),
            CaseBuf::Space(s) => splits::space(s.as_str()),
            CaseBuf::TitleDash(s) => splits::title_dash(s.as_str()),
        }
    }

    mod splits {
        pub(super) fn ada(str: &str) -> Vec<String> {
            snake(str.to_lowercase().as_str())
        }

        pub(super) fn camel(str: &str) -> Vec<String> {
            let mut peekable = str.chars().peekable();
            let mut temp = String::with_capacity(12);
            let mut results: Vec<String> = Vec::new();
            while let Some(ch) = peekable.next() {
                let mut c = ch;
                if c.is_uppercase() {
                    c.make_ascii_lowercase();
                }
                temp.push(c);

                if peekable.peek().map_or(true, |next| next.is_uppercase()) {
                    results.push(temp.clone());
                    temp.clear();
                }
            }
            results
        }

        pub(super) fn dot(str: &str) -> Vec<String> {
            str.split(".").map(String::from).collect()
        }

        pub(super) fn kebab(str: &str) -> Vec<String> {
            str.split("-").map(String::from).collect()
        }

        pub(super) fn pascal(str: &str) -> Vec<String> {
            if str.chars().filter(|c| c.is_uppercase()).count() == 1 {
                vec![str.to_lowercase()]
            } else {
                camel(str)
            }
        }

        pub(super) fn path(str: &str) -> Vec<String> {
            str.split("/").map(String::from).collect()
        }

        pub(super) fn screaming_snake(str: &str) -> Vec<String> {
            ada(str)
        }

        pub(super) fn snake(str: &str) -> Vec<String> {
            str.split("_").map(String::from).collect()
        }

        pub(super) fn space(str: &str) -> Vec<String> {
            str.split(" ").map(String::from).collect()
        }

        pub(super) fn title_dash(str: &str) -> Vec<String> {
            kebab(str.to_lowercase().as_str())
        }

        #[cfg(test)]
        mod tests {
            #[test]
            fn ada_test() {
                let ada = "Some_Ada_Case_Word";
                let should = ["some", "ada", "case", "word"];
                let result = super::ada(ada);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn camel_test() {
                let camel = "someCamelCaseWord";
                let should = ["some", "camel", "case", "word"];
                let result = super::camel(camel);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn dot_test() {
                let dot = "some.dot.case.word";
                let should = ["some", "dot", "case", "word"];
                let result = super::dot(dot);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn kebab_test() {
                let kebab = "some-kebab-case-word";
                let should = ["some", "kebab", "case", "word"];
                let result = super::kebab(kebab);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn pascal_test() {
                let pascal = "SomePascalCaseWord";
                let should = ["some", "pascal", "case", "word"];
                let result = super::pascal(pascal);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn pascal_test_single_word() {
                let pascal = "Pascal";
                let should = ["pascal"];
                let result = super::pascal(pascal);
                assert_eq!(should[0], result[0]);
            }

            #[test]
            fn path_test() {
                let path = "some/path/case/word";
                let should = ["some", "path", "case", "word"];
                let result = super::path(path);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn screaming_snake_test() {
                let screaming_snake = "SOME_SCREAMING_SNAKE_CASE_WORD";
                let should = ["some", "screaming", "snake", "case", "word"];
                let result = super::screaming_snake(screaming_snake);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn snake_test() {
                let snake = "some_snake_case_word";
                let should = ["some", "snake", "case", "word"];
                let result = super::snake(snake);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn space_test() {
                let space = "some space case word";
                let should = ["some", "space", "case", "word"];
                let result = super::space(space);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }

            #[test]
            fn title_dash_test() {
                let title_dash = "Some-Title-Dash-Case-Word";
                let should = ["some", "title", "dash", "case", "word"];
                let result = super::title_dash(title_dash);
                for x in 0..should.len() {
                    assert_eq!(should[x], result[x]);
                }
            }
        }
    }

    pub(crate) mod check {
        use crate::case::Case;
        use regex::Regex;

        macro_rules! check {
            ($name:ident, $variant:ident, $regex:literal) => {
                pub(crate) fn $name(str: &str) -> Option<Case> {
                    let regex = Regex::new($regex).expect("bad regex");
                    if regex.is_match(str) {
                        Some(Case::$variant)
                    } else {
                        None
                    }
                }
            };
        }

        check!(
            ada,
            Ada,
            r"^(?:[[:upper:]][[:lower:]]+_)+[[:upper:]][[:lower:]]+$"
        );
        check!(camel, Camel, r"^[[:lower:]]+(?:[[:upper:]][[:lower:]]+)+$");
        check!(dot, Dot, r"^(?:[[:lower:]]+\.)+[[:lower:]]+$");
        check!(kebab, Kebab, r"^(?:[[:lower:]]+-)+[[:lower:]]+$");
        check!(pascal, Pascal, r"^(?:[[:upper:]][[:lower:]]+)+$");
        check!(
            path,
            Path,
            r"^(?:(?:[[:upper:]]|[[:lower:]])+/)+(?:[[:upper:]]|[[:lower:]])+$"
        );
        check!(
            screaming_snake,
            ScreamingSnake,
            r"^[[:upper:]]+(?:_[[:upper:]]+)*$"
        );
        check!(snake, Snake, r"^(?:[[:lower:]]+_)+[[:lower:]]+$");
        check!(space, Space, r"^(?:[[:lower:]]+ )+[[:lower:]]+$");
        check!(
            title_dash,
            TitleDash,
            r"^(?:[[:upper:]][[:lower:]]+-)+[[:upper:]][[:lower:]]+$"
        );
    }

    pub(crate) mod produce {
        use crate::utils;
        const EMPTY_ITER: &str = "empty iterator in produce";

        pub(crate) fn ada(parts: &[String]) -> String {
            parts
                .iter()
                .map(|part| utils::capitalize_first(part))
                .reduce(|acc, el| acc + "_" + &el)
                .expect(EMPTY_ITER)
        }

        pub(crate) fn camel(parts: &[String]) -> String {
            let mut result = String::new();
            let mut iter = parts.iter().cloned();
            if let Some(first) = iter.next() {
                result.push_str(first.as_str());
            }
            iter.map(|mut part| {
                utils::capitalize_first_mut(part.as_mut());
                part
            })
            .for_each(|part| {
                result.push_str(part.as_str());
            });
            result
        }

        pub(crate) fn dot(parts: &[String]) -> String {
            parts.join(".")
        }

        pub(crate) fn kebab(parts: &[String]) -> String {
            parts.join("-")
        }

        pub(crate) fn pascal(parts: &[String]) -> String {
            parts
                .iter()
                .map(|part| utils::capitalize_first(part))
                .collect()
        }

        pub(crate) fn path(parts: &[String]) -> String {
            parts.join("/")
        }

        pub(crate) fn screaming_snake(parts: &[String]) -> String {
            let parts = parts.to_vec();
            parts
                .into_iter()
                .map(|mut part| {
                    part.make_ascii_uppercase();
                    part
                })
                .reduce(|acc, el| acc + "_" + &el)
                .expect(EMPTY_ITER)
        }

        pub(crate) fn snake(parts: &[String]) -> String {
            parts.join("_")
        }

        pub(crate) fn space(parts: &[String]) -> String {
            let parts = parts.to_vec();
            parts
                .into_iter()
                .map(|mut part| {
                    part.make_ascii_lowercase();
                    part
                })
                .reduce(|acc, el| acc + " " + &el)
                .expect(EMPTY_ITER)
        }

        pub(crate) fn title_dash(parts: &[String]) -> String {
            parts
                .iter()
                .map(|part| utils::capitalize_first(part))
                .fold(String::new(), |acc, el| {
                    if acc.is_empty() {
                        acc + &el
                    } else {
                        acc + "-" + &el
                    }
                })
        }
    }
}
