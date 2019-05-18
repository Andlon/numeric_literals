extern crate numliterals;

use numliterals::numeric_literals;
use std::ops::Div;

#[numeric_literals(T)]
fn test<T>() -> T where T: From<i8> + Div<Output=T> {
    3 / 2 / 2
}

fn main() {
    println!("{}", test::<f64>());
}