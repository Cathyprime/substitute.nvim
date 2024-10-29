use std::sync::LazyLock;

use crate::rules::*;

static RULES: LazyLock<Vec<Box<dyn Rule>>> = LazyLock::new(|| vec![
    Box::new(Ada),
    Box::new(Camel),
    Box::new(Dot),
    Box::new(Kebab),
    Box::new(Pascal),
    Box::new(Path),
    Box::new(ScreamingSnake),
    Box::new(Snake),
    Box::new(Space),
    Box::new(TitleDash),
]);

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

    Invalid, // default
}

pub(crate) fn get_case(str: &str) -> Case {
    for rule in RULES.iter() {
        if let Some(case) = rule.check(str) {
            return case;
        }
    }
    Case::Invalid
}
