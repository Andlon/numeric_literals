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

#[replace_numeric_literals(T::try_from(literal).map_err(|_| ())?)]
fn test3<T>() -> Result<T, ()>
    where T: TryFrom<i8>+ Div<Output=T>
{
    let res = 3 / 2 / 2;
    Ok(res)
}

#[replace_numeric_literals(f32::from(literal))]
pub trait A {}

#[allow(dead_code)]
struct B;

#[replace_numeric_literals(literal)]
impl B {

}

fn main() {
    println!("{}", test::<f64>());
    println!("{}", test2::<f64>());
    println!("{}", test3::<f64>().unwrap());
}