extern crate numliterals;

use numliterals::replace_numeric_literals;
use std::ops::Div;
use std::convert::TryFrom;
use std::fmt::Debug;

#[replace_numeric_literals(T::from(literal))]
fn test<T>() -> T where T: From<i8> + Div<Output=T> {
    3 / 2 / 2
}

#[replace_numeric_literals(T::try_from(literal).expect("Must fit"))]
fn test2<T>() -> T
    where T: TryFrom<i8>+ Div<Output=T>,
          <T as TryFrom<i8>>::Error: Debug
{
    3 / 2 / 2
}

fn main() {
    println!("{}", test::<f64>());
    println!("{}", test2::<f64>());
}