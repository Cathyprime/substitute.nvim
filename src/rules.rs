use crate::{
    internals::Case,
    utils::{self, capitalize_first},
};
use regex::Regex;

const EMPTY_ITER: &str = "empty iterator in produce";

pub(crate) trait Rule: Sync + Send {
    fn check(&self, str: &str) -> Option<Case>;
    fn produce(&self, parts: Vec<String>) -> String;
}

macro_rules! check {
    ($regex:literal, $var:ident) => {
        fn check(&self, str: &str) -> Option<Case> {
            let regex = Regex::new($regex).expect("bad regex");
            if regex.is_match(str) {
                Some(Case::$var)
            } else {
                None
            }
        }
    };
}

pub(crate) struct Ada;
impl Rule for Ada {
    check!(
        r"^(?:[[:upper:]][[:lower:]]+_)+[[:upper:]][[:lower:]]+$",
        Ada
    );

    fn produce(&self, parts: Vec<String>) -> String {
        parts
            .into_iter()
            .map(|mut part| {
                capitalize_first(&mut part);
                part
            })
            .reduce(|acc, el| acc + "_" + &el)
            .expect(EMPTY_ITER)
    }
}

pub(crate) struct Camel;
impl Rule for Camel {
    check!(r"^[[:lower:]]+(?:[[:upper:]][[:lower:]]+)+$", Camel);

    fn produce(&self, parts: Vec<String>) -> String {
        let mut result = String::with_capacity(parts.len());
        let mut iter = parts.into_iter();
        result.push_str(&iter.next().expect(EMPTY_ITER));
        iter.fold(result, |mut acc, mut el| {
            capitalize_first(&mut el);
            acc.push_str(&el);
            acc
        })
    }
}

pub(crate) struct Dot;
impl Rule for Dot {
    check!(r"^(?:[[:lower:]]+\.)+[[:lower:]]+$", Dot);

    fn produce(&self, parts: Vec<String>) -> String {
        parts.join(".")
    }
}

pub(crate) struct Kebab;
impl Rule for Kebab {
    check!(r"^(?:[[:lower:]]+-)+[[:lower:]]+$", Kebab);

    fn produce(&self, parts: Vec<String>) -> String {
        parts.join("-")
    }
}

pub(crate) struct Pascal;
impl Rule for Pascal {
    check!(r"^(?:[[:upper:]][[:lower:]]+)+$", Pascal);

    fn produce(&self, parts: Vec<String>) -> String {
        parts
            .into_iter()
            .map(|mut part| {
                utils::capitalize_first(&mut part);
                part
            })
            .collect()
    }
}

pub(crate) struct Path;
impl Rule for Path {
    check!(
        r"^(?:(?:[[:upper:]]|[[:lower:]])+/)+(?:[[:upper:]]|[[:lower:]])+$",
        Path
    );

    fn produce(&self, parts: Vec<String>) -> String {
        parts.join("/")
    }
}

pub(crate) struct ScreamingSnake;
impl Rule for ScreamingSnake {
    check!(r"^(?:[[:upper:]]+_)+[[:upper:]]+$", ScreamingSnake);

    fn produce(&self, parts: Vec<String>) -> String {
        parts
            .into_iter()
            .map(|mut part| {
                part.make_ascii_uppercase();
                part
            })
            .reduce(|acc, el| acc + "_" + &el)
            .expect(EMPTY_ITER)
    }
}

pub(crate) struct Snake;
impl Rule for Snake {
    check!(r"^(?:[[:lower:]]+_)+[[:lower:]]+$", Snake);

    fn produce(&self, parts: Vec<String>) -> String {
        parts.join("_")
    }
}

pub(crate) struct Space;
impl Rule for Space {
    check!(r"^(?:[[:lower:]]+ )+[[:lower:]]+$", Space);

    fn produce(&self, parts: Vec<String>) -> String {
        parts
            .into_iter()
            .map(|mut part| {
                part.make_ascii_lowercase();
                part
            })
            .reduce(|acc, el| acc + " " + &el)
            .expect(EMPTY_ITER)
    }
}

pub(crate) struct TitleDash;
impl Rule for TitleDash {
    check!(
        r"^(?:[[:upper:]][[:lower:]]+-)+[[:upper:]][[:lower:]]+$",
        TitleDash
    );

    fn produce(&self, parts: Vec<String>) -> String {
        parts
            .into_iter()
            .map(|mut part| {
                utils::capitalize_first(&mut part);
                part
            })
            .reduce(|acc, el| acc + "-" + &el)
            .expect(EMPTY_ITER)
    }
}
