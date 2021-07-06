numeric_literals
================

[![On crates.io](https://img.shields.io/crates/v/numeric_literals)](https://crates.io/crates/numeric_literals)
[![On docs.rs](https://docs.rs/numeric_literals/badge.svg)](https://docs.rs/numeric_literals)
[![Build Status](https://github.com/Andlon/numeric_literals/workflows/build%20and%20run%20tests/badge.svg)](https://github.com/Andlon/numeric_literals/actions)

**numeric_literals** is a Rust library that provides procedural attribute macros for replacing
numeric literals with arbitrary expressions.

While Rust's explicitness is generally a boon, it is a major pain when writing numeric
code that is intended to be generic over a scalar type. As an example, consider
writing a function that returns the golden ratio for any type that implements `T: num::Float`.
An implementation might look like the following.

```rust
extern crate num;
use num::Float;

fn golden_ratio<T: Float>() -> T {
    ( T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap()
}
```

This is arguably very messy for such a simple task. With `numeric_literals`, we may
instead write:

```rust
use num::Float;
use numeric_literals::replace_numeric_literals;
#[replace_numeric_literals(T::from(literal).unwrap())]
fn golden_ratio<T: Float>() -> T {
   (1 + 5.sqrt()) / 2
}
```

The above two code segments do essentially the same thing
(apart from using `T::from(1)` instead of `T::one()`). However, in the latter example,
the `replace_numeric_literals` attribute replaces any numeric literal with the expression
`T::from(literal).unwrap()`, where `literal` is a placeholder for each individual literal.

There is no magic involved: the code is still explict about what it does to numeric literals.
The difference is that we can declare this behavior once for all numeric literals. Moreover,
we move the conversion behavior away from where the literals are needed, enhancing readability
by reducing the noise imposed by being explicit about the exact types involved.

Float and integer literal replacement
-------------------------------------

An issue with the replacement of numeric literals is that there is no way to distinguish
literals that are used for e.g. indexing from those that are part of a numerical computation.
In the example above, if you would additionally need to index into an array with a constant index
such as `array[0]`, the macro will try to convert the index `0` to a float type, which
would clearly fail. Thankfully, in most cases these examples will outright fail to compile
because of type mismatch. One possible resolution to this problem is to use the separate
macros `replace_float_literals` and `replace_int_literals`, which work in the exact same way,
but only trigger on float or integer literals, respectively. Below is an example from
Finite Element code that uses float literal replacement to improve readability of numerical
constants in generic code.

```rust
#[replace_float_literals(T::from_f64(literal).expect("Literal must fit in T"))]
pub fn assemble_element_mass<T>(quad: &Quad2d<T>) -> MatrixN<T, U8>
where
   T: RealField
{
    let phi = |alpha, beta, xi: &Vector2<T>| -(1.0 + alpha * xi[0]) * (1.0 + beta * xi[1]) / 4.0;
    let phi_grad = |alpha, beta, xi: &Vector2<T>| {
        Vector2::new(
            alpha * (1.0 + beta * xi[1]) / 4.0,
            beta * (1.0 + alpha * xi[0]) / 4.0,
        )
    };
    let alphas = [-1.0, 1.0, 1.0, -1.0];
    let betas = [-1.0, -1.0, 1.0, 1.0];

    // And so on...
}
```

In general, **the macros should be used with caution**. It is recommended to keep the macro close to
the region in which the literals are being used, as to avoid confusion for readers of the code.
The Rust code before macro expansion is usually not valid Rust (because of the lack of explicit
type conversion), but without the context of the attribute, it is simply not clear why this
code still compiles.
An option for the future would be to apply the attribute only to very local blocks of code that
are heavy on numerical constants. However, at present, Rust does not allow attribute macros
to apply to blocks or single expressions.

Replacement in macro invocations
--------------------------------
By default, the macros of this crate will also replace literals inside of macro invocations.
This allows code such as the following to compile:

```rust
use num::Float;
use numeric_literals::replace_numeric_literals;

#[replace_numeric_literals(T::from(literal).unwrap())]
fn zeros<T: Float>(n: usize) -> Vec<T> {
    vec![0.0; n]
}
```
If this behavior is unwanted, it is possible to disable replacement inside of macros with a
parameter:
```rust
#[replace_numeric_literals(T::from(literal).unwrap()), visit_macros = false]
```

Literals with suffixes
----------------------
In rust, literal suffixes can be used to disambiguate the type of a literal. For example, the suffix `_f64`
in the expression `1_f64.sqrt()` makes it clear that the value `1` is of type `f64`. This is also supported
by the macros of this crate for all floating point and integer suffixes. For example:

```rust
use num::Float;
use numeric_literals::replace_numeric_literals;

#[replace_numeric_literals(T::from(literal).unwrap())]
fn golden_ratio<T: Float>() -> T {
   (1.0_f64 + 5f32.sqrt()) / 2.0
}
```

License
=======

This crate is licensed under the MIT license. See LICENSE for details.
