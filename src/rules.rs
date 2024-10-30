use crate::internals::Case;

type Check = fn(&str) -> Option<Case>;
type Produce = fn(&[String]) -> String;

pub(crate) struct Rule {
    pub(crate) ident: Case,
    check_fn: Check,
    produce_fn: Produce,
}

impl Rule {
    pub(crate) fn check(&self, str: &str) -> Option<Case> {
        (self.check_fn)(str)
    }

    pub(crate) fn produce(&self, parts: &[String]) -> String {
        (self.produce_fn)(parts)
    }

    pub(crate) fn new(ident: Case, check_fn: Check, produce_fn: Produce) -> Self {
        Self {
            ident,
            check_fn,
            produce_fn,
        }
    }
}

pub(crate) mod case_rule_functions {

    pub(crate) mod check {
        use crate::internals::Case;
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

        check!(ada, Ada,
            r"^(?:[[:upper:]][[:lower:]]+_)+[[:upper:]][[:lower:]]+$");
        check!(camel, Camel, r"^[[:lower:]]+(?:[[:upper:]][[:lower:]]+)+$");
        check!(dot, Dot, r"^(?:[[:lower:]]+\.)+[[:lower:]]+$");
        check!(kebab, Kebab, r"^(?:[[:lower:]]+-)+[[:lower:]]+$");
        check!(pascal, Pascal, r"^(?:[[:upper:]][[:lower:]]+)+$");
        check!(path, Path,
            r"^(?:(?:[[:upper:]]|[[:lower:]])+/)+(?:[[:upper:]]|[[:lower:]])+$");
        check!(screaming_snake, ScreamingSnake,
            r"^(?:[[:upper:]]+_)+[[:upper:]]+$");
        check!(snake, Snake, r"^(?:[[:lower:]]+_)+[[:lower:]]+$");
        check!(space, Space, r"^(?:[[:lower:]]+ )+[[:lower:]]+$");
        check!(title_dash, TitleDash,
            r"^(?:[[:upper:]][[:lower:]]+-)+[[:upper:]][[:lower:]]+$");
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
