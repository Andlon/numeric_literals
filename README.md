numeric_literals
================

**numeric_literals** is a Rust library that provides  procedural attribute-macro for replacing numeric literals
with arbitrary expressions.

While Rust's explicitness is generally a boon, it is a major pain when writing numeric
code that is intended to be generic over its scalar type. As an example, consider
writing a function that returns the golden ratio for any type that implements `T: num::Float`.
An implementation might look like the following.

```rust
fn golden_ratio<T: Float>() -> T {
    ( T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap()
}
```

This is arguably very messy for such a simple task. With `numeric_literals`, we may
instead write:

```rust
#[replace_numeric_literals(T::from(literal).unwrap())]
fn golden_ratio<T: Float>() -> T {
    (1 + T::sqrt(5)) / 2
}
```

The above two code segments do essentially the same thing
(apart from using `T::from(1)` instead of `T::one()`). However, in the latter example,
the `replace_numeric_literals` attribute replaces any numeric literal with the expression
`T::from(literal).unwrap()`, where `literal` is a placeholder for each individual literal.