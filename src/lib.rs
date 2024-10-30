mod rules;
mod utils;
mod case;

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
        let vim_regex = format!(r#"\v\C({}|{}|{}|{}|{}|{}|{}|{}|{})"#,
            "Some_Word", "someWord", "some.word",
            "some-word", "SomeWord", "some/word",
            "SOME_WORD", "some_word", "Some-Word");

        assert_eq!(dbg!(crate::utils::to_vim_regex_find(&parts)), vim_regex);
    }
}
