use std::sync::LazyLock;
use crate::rules::Rule;

macro_rules! rule {
    ($variant:ident, $fn:ident) => {
        Rule::new(
            Case::$variant,
            crate::rules::case_rule_functions::check::$fn,
            crate::rules::case_rule_functions::produce::$fn
        )
    }
}

static RULES: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        rule!(Ada, ada),
        rule!(Camel, camel),
        rule!(Dot, dot),
        rule!(Kebab, kebab),
        rule!(Pascal, pascal),
        rule!(Path, path),
        rule!(ScreamingSnake, screaming_snake),
        rule!(Snake, snake),
        rule!(Space, space),
        rule!(TitleDash, title_dash),
    ]
});

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

pub(crate) fn get_case(str: &str) -> Option<Case> {
    for rule in RULES.iter() {
        if let Some(case) = rule.check(str) {
            return Some(case);
        }
    }
    None
}

pub(crate) fn get_rule(c: Case) -> &'static Rule {
    for rule in RULES.iter() {
        if rule.ident == c {
            return rule;
        }
    }
    unreachable!("added a new Case without adding corresponding rule object")
}

pub(crate) fn permutations(parts: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(RULES.len());
    for rule in RULES.iter() {
        if rule.ident == Case::Space {
            continue;
        }
        result.push(rule.produce(parts));
    }
    result
}

pub(crate) fn to_vim_regex_find(parts: &[String]) -> String {
    let permutations = permutations(parts);
    format!(r#"\v\C({})"#, permutations.join("|"))
}

pub(crate) fn transform(parts: &[String], target: Case) -> String {
    let rule = get_rule(target);
    rule.produce(parts)
}
