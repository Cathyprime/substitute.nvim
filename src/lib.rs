mod case;
mod rules;
mod utils;

use crate::case::CaseBuf;
use nvim_oxi::{Dictionary, Function};

type CResult = Result<CaseBuf, ()>;

#[nvim_oxi::plugin]
fn main() -> nvim_oxi::Result<Dictionary> {
    let get_regex = Function::from_fn(|str: String| -> String {
        let case: CResult = str.as_str().try_into();
        let result = match case {
            Ok(c) => utils::to_vim_regex_find(c.parts().as_slice()),
            Err(_) => str
        };
        result
    });

    let replace = Function::from_fn(|(from, replacement): (String, String)| -> String {
        let from_case: CResult = from.as_str().try_into();
        let replacement_case: CResult = replacement.as_str().try_into();
        match (from_case, replacement_case) {
            (Ok(f), Ok(r)) => {
                f.like_me(r.parts().as_slice())
            },
            _ => replacement
        }
    });

    let mut api = Dictionary::new();
    api.insert("find_regex", get_regex);
    api.insert("replace", replace);
    Ok(api)
}

#[cfg(test)]
mod tests {

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
}
