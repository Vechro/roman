#[cfg(test)]
mod roman {
    use crate::roman_to_arabic;

    #[test]
    fn ix() {
        assert_eq!(roman_to_arabic("ix"), Some(String::from("9")));
    }

    #[test]
    fn dccc() {
        assert_eq!(roman_to_arabic("dccc"), Some(String::from("800")));
    }

    #[test]
    fn xviii() {
        assert_eq!(roman_to_arabic("xviii"), Some(String::from("18")));
    }
}

mod arabic {
    use crate::arabic_to_roman;

    #[test]
    fn nine() {
        assert_eq!(arabic_to_roman("9"), Some("ix".to_owned()));
    }

    #[test]
    fn eight_hundred() {
        assert_eq!(arabic_to_roman("800"), Some("dccc".to_owned()));
    }

    #[test]
    fn eightteen() {
        assert_eq!(arabic_to_roman("18"), Some("xviii".to_owned()));
    }
}
