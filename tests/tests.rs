extern crate numeric_literals;

use numeric_literals::replace_numeric_literals;

#[test]
fn leaves_string_untouched() {
    #[replace_numeric_literals(())]
    fn gen_str() -> &'static str {
        "test"
    }

    assert_eq!(gen_str(), "test");
}

#[test]
fn leaves_byte_string_untouched() {
    #[replace_numeric_literals(())]
    fn gen_byte_str() -> &'static [u8] {
        b"test"
    }

    assert_eq!(gen_byte_str(), b"test");
}

#[test]
fn leaves_byte_untouched() {
    #[replace_numeric_literals(())]
    fn gen_byte() -> u8 {
        b'a'
    }

    assert_eq!(gen_byte(), b'a');
}

#[test]
fn leaves_char_untouched() {
    #[replace_numeric_literals(())]
    fn gen_char() -> char {
        'a'
    }

    assert_eq!(gen_char(), 'a');
}

#[test]
fn leaves_raw_untouched() {
    #[replace_numeric_literals(())]
    fn gen_raw() -> &'static str {
        r#"test"#
    }

    assert_eq!(gen_raw(), r#"test"#);
}