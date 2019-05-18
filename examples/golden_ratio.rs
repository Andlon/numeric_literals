extern crate num;
extern crate numeric_literals;

use num::Float;
use numeric_literals::replace_numeric_literals;

fn golden_ratio_vanilla<T: Float>() -> T {
    (T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap()
}

#[replace_numeric_literals(T::from(literal).expect("Literal must fit in T"))]
fn golden_ratio<T: Float>() -> T {
    (1 + T::sqrt(5)) / 2
}

fn main() {
    println!("{}", golden_ratio_vanilla::<f32>());
    println!("{}", golden_ratio::<f32>());
}
