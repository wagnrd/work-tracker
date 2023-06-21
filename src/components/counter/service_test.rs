#[cfg(test)]
mod test_filter_integers_from_text {
    use crate::components::counter::service::filter_integers_from_text;
    use std::collections::HashMap;

    #[test]
    fn only_integer_string() {
        let mut pairs = HashMap::<String, i32>::new();
        pairs.insert("0".to_string(), 0);
        pairs.insert("-0".to_string(), 0);
        pairs.insert("-2".to_string(), -2);
        pairs.insert("45".to_string(), 45);

        for (text, expected) in pairs {
            let actual = filter_integers_from_text(text);
            assert_eq!(Some(expected), actual);
        }
    }

    #[test]
    fn empty_string() {
        let text = "".to_string();
        let expected = None;
        let actual = filter_integers_from_text(text);
        assert_eq!(expected, actual);
    }

    #[test]
    fn remove_characters() {
        let mut pairs = HashMap::<String, Option<i32>>::new();
        pairs.insert("a3".to_string(), Some(3));
        pairs.insert("4four".to_string(), Some(4));
        pairs.insert("12ab34".to_string(), Some(1234));
        pairs.insert("5.6".to_string(), Some(56));
        pairs.insert("7,8".to_string(), Some(78));
        pairs.insert("-abcd9".to_string(), Some(-9));
        pairs.insert("ab-1cd".to_string(), Some(-1));
        pairs.insert("1abcd-".to_string(), Some(1));
        pairs.insert("1-1".to_string(), Some(11));
        pairs.insert("abcd".to_string(), None);

        for (text, expected) in pairs {
            let actual = filter_integers_from_text(text);
            assert_eq!(expected, actual);
        }
    }
}
