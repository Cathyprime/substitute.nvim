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

#[cfg(test)]
mod tests {
    use super::*;

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
