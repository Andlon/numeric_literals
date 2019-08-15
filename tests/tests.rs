extern crate numeric_literals;

use numeric_literals::{replace_float_literals, replace_int_literals, replace_numeric_literals};
use std::ops::Add;

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

#[test]
fn leaves_float_alone_when_integers_transformed() {
    #[replace_int_literals(())]
    fn gen() -> ((), f32) {
        (3, 3.0)
    }

    assert_eq!(gen(), ((), 3.0f32));
}

#[test]
fn leaves_integers_alone_when_floats_transformed() {
    #[replace_float_literals(())]
    fn gen() -> (i32, ()) {
        (3, 3.0)
    }

    assert_eq!(gen(), (3i32, ()));
}

#[test]
fn converts_integers_to_f32() {
    #[replace_numeric_literals(literal as f32)]
    fn gen_f32() -> f32 {
        3
    }

    assert_eq!(gen_f32(), 3.0);
}

#[test]
fn converts_floats_to_i32() {
    #[replace_numeric_literals(literal as i32)]
    fn gen_i32() -> i32 {
        3.2
    }

    assert_eq!(gen_i32(), 3);
}

#[test]
fn converts_vec_floats_to_i32() {
    #[replace_float_literals(literal as i32)]
    fn gen_i32_vec_1() -> Vec<i32> {
        vec![3.2, 5.7, 10.1]
    }

    assert_eq!(gen_i32_vec_1(), vec![3, 5, 10]);

    #[replace_float_literals(literal as i32)]
    fn gen_i32_vec_2() -> Vec<i32> {
        vec![2.99; 4]
    }

    assert_eq!(gen_i32_vec_2(), vec![2, 2, 2, 2]);
}

#[test]
fn converts_vec_integers_to_f64() {
    #[replace_int_literals(literal as f64)]
    fn gen_f64_vec() -> Vec<f64> {
        vec![3 / 2, 1 / 2, 9 / 2]
    }

    assert_eq!(gen_f64_vec(), vec![1.5, 0.5, 4.5]);
}

#[test]
fn converts_vec_numeric() {
    #[replace_numeric_literals(literal as i32)]
    fn gen_i32_vec() -> Vec<i32> {
        vec![3.2, 5.7, 10.1]
    }

    assert_eq!(gen_i32_vec(), vec![3, 5, 10]);

    #[replace_numeric_literals(literal as f64)]
    fn gen_f64_vec() -> Vec<f64> {
        vec![3 / 2, 1 / 2, 9 / 2]
    }

    assert_eq!(gen_f64_vec(), vec![1.5, 0.5, 4.5]);
}

#[test]
fn converts_assert_eq_floats_to_i32() {
    #[replace_float_literals(literal as i32)]
    fn assert_eq_test() -> bool {
        assert_eq!(1.1, 1);
        assert_eq!(2.6, 2);
        assert_eq!(10.99, 10);
        true
    }

    assert!(assert_eq_test());
}

#[test]
fn converts_assert_floats_to_i32() {
    #[replace_float_literals(literal as i32)]
    fn assert_test() -> bool {
        assert!(1.1 == 1);
        assert!(2.6 == 2);
        assert!(10.99 == 10);
        true
    }

    assert!(assert_test());
}

#[test]
fn converts_generic_with_from() {
    #[replace_numeric_literals(T::from(literal))]
    fn gen<T: From<i8>>() -> T {
        3
    }

    assert_eq!(gen::<f32>(), 3.0);
    assert_eq!(gen::<f64>(), 3.0);
    assert_eq!(gen::<i8>(), 3);
    assert_eq!(gen::<i16>(), 3);
    assert_eq!(gen::<i32>(), 3);
    assert_eq!(gen::<i64>(), 3);
    assert_eq!(gen::<i128>(), 3);
}

#[test]
fn converts_generic_arithmetic_with_from() {
    #[replace_numeric_literals(T::from(literal))]
    fn gen<T>() -> T
    where
        T: From<i8> + Add<T, Output = T>,
    {
        3 + 2
    }

    assert_eq!(gen::<f32>(), 5.0);
    assert_eq!(gen::<f64>(), 5.0);
    assert_eq!(gen::<i8>(), 5);
    assert_eq!(gen::<i16>(), 5);
    assert_eq!(gen::<i32>(), 5);
    assert_eq!(gen::<i64>(), 5);
    assert_eq!(gen::<i128>(), 5);
}
