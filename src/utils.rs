use crate::rules::case_rule_functions::produce as p;
use crate::case::CaseBuf;

type CResult = Result<CaseBuf, ()>;

pub(crate) fn capitalize_first(str: &str) -> String {
    let mut owned = str.to_owned();
    capitalize_first_mut(&mut owned);
    owned
}

pub(crate) fn capitalize_first_mut(str: &mut str) {
    if let Some(f) = str.get_mut(0..1) {
        f.make_ascii_uppercase();
    }
}

pub(crate) fn permutations(parts: &[String]) -> Vec<String> {
    let producers = [
        p::ada,
        p::camel,
        p::dot,
        p::kebab,
        p::pascal,
        p::path,
        p::screaming_snake,
        p::snake,
        p::space,
        p::title_dash,
    ];
    let mut result = Vec::with_capacity(producers.len());
    for producer in producers {
        result.push(producer(parts))
    }
    result
}

pub(crate) fn to_vim_regex_find(parts: &[String]) -> String {
    let permutations = permutations(parts);
    format!(r#"\v\C({})"#, permutations.join("|"))
}

pub(crate) fn get_regex_lua(str: String) -> String {
    let case: CResult = str.as_str().try_into();
    let result = match case {
        Ok(c) => to_vim_regex_find(c.parts().as_slice()),
        Err(_) => str
    };
    result
}

pub(crate) fn replace_lua((from, replacement): (String, String)) -> String {
    let from_case: CResult = from.as_str().try_into();
    let replacement_case: CResult = replacement.as_str().try_into();
    match (from_case, replacement_case) {
        (Ok(f), Ok(r)) => {
            f.like_me(r.parts().as_slice())
        },
        (Ok(f), Err(())) if replacement.chars().all(|c| c.is_lowercase()) => {
            let x: Vec<_> = replacement.split(" ").map(String::from).collect();
            f.like_me(x.as_slice())
        },
        _ => replacement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            String::from("some word"),
            String::from("Some-Word"),
        ];

        let result = crate::utils::permutations(&parts);

        for x in 0..perms.len() {
            assert_eq!(&perms[x], &result[x])
        }
    }

    #[test]
    fn as_vim_regex() {
        let parts = [String::from("some"), String::from("word")];
        let vim_regex = format!(
            r#"\v\C({}|{}|{}|{}|{}|{}|{}|{}|{}|{})"#,
            "Some_Word",
            "someWord",
            "some.word",
            "some-word",
            "SomeWord",
            "some/word",
            "SOME_WORD",
            "some_word",
            "some word",
            "Some-Word"
        );

        assert_eq!(dbg!(crate::utils::to_vim_regex_find(&parts)), vim_regex);
    }

    #[test]
    fn replace_test_long() {
        let from = String::from("SomePascalCaseWord");
        let replacement = String::from("this will still be pascal case");
        assert_eq!("ThisWillStillBePascalCase", replace_lua((from, replacement)));
    }

    #[test]
    fn replace_short_lower() {
        let from = String::from("word");
        let replacement = String::from("another");
        assert_eq!("another", replace_lua((from, replacement)));
    }

    #[test]
    fn replace_short_upper() {
        let from = String::from("Word");
        let replacement = String::from("another");
        assert_eq!("Another", replace_lua((from, replacement)));
    }

    #[test]
    fn replace_short_upper_full() {
        let from = String::from("WORD");
        let replacement = String::from("another");
        assert_eq!("ANOTHER", replace_lua((from, replacement)));
    }

    #[test]
    fn capitalize_first_mut_test() {
        let mut some_str = String::from("word");
        capitalize_first_mut(&mut some_str);
        assert_eq!(some_str, "Word")
    }

    #[test]
    fn capitalize_first_test() {
        let some_str = String::from("word");
        assert_eq!(capitalize_first(&some_str), "Word")
    }
}
