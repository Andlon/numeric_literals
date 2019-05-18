numeric_literals
================

**numeric_literals** is a Rust library that provides  procedural attribute-macro for replacing numeric literals
with arbitrary expressions.

While Rust's explicitness is generally a boon, it is a major pain when writing numeric
code that is intended to be generic over its scalar type. As an example, consider
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
# use num::Float;
use numeric_literals::replace_numeric_literals;

#[replace_numeric_literals(T::from(literal).unwrap())]
fn golden_ratio<T: Float>() -> T {
   (1 + T::sqrt(5)) / 2
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

That said, the macro should be used with care. It is recommended to keep the macro close to
the region in which the literals are being used, as to avoid confusion for readers of the code.
The Rust code that is usually not valid Rust (because of the lack of explicit conversion),
but without the context of the attribute, it is simply not clear why this code still compiles.