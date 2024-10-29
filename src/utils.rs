pub(crate) fn capitalize_first(str: &mut str) {
    if let Some(f) = str.get_mut(0..1) {
        f.make_ascii_uppercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_first_test() {
        let mut some_str = String::from("word");
        capitalize_first(&mut some_str);
        assert!(some_str == "Word")
    }
}
