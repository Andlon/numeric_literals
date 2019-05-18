//!
//! **numeric_literals** is a Rust library that provides  procedural attribute-macro for replacing numeric literals
//! with arbitrary expressions.
//!
//! While Rust's explicitness is generally a boon, it is a major pain when writing numeric
//! code that is intended to be generic over its scalar type. As an example, consider
//! writing a function that returns the golden ratio for any type that implements `T: num::Float`.
//! An implementation might look like the following.
//!
//! ```rust
//! extern crate num;
//! use num::Float;
//!
//! fn golden_ratio<T: Float>() -> T {
//!     ( T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap()
//! }
//! ```
//!
//! This is arguably very messy for such a simple task. With `numeric_literals`, we may
//! instead write:
//!
//! ```rust
//! # use num::Float;
//! use numeric_literals::replace_numeric_literals;
//!
//! #[replace_numeric_literals(T::from(literal).unwrap())]
//! fn golden_ratio<T: Float>() -> T {
//!    (1 + T::sqrt(5)) / 2
//! }
//! ```
//!
//! The above two code segments do essentially the same thing
//! (apart from using `T::from(1)` instead of `T::one()`). However, in the latter example,
//! the `replace_numeric_literals` attribute replaces any numeric literal with the expression
//! `T::from(literal).unwrap()`, where `literal` is a placeholder for each individual literal.
//!
//! There is no magic involved: the code is still explict about what it does to numeric literals.
//! The difference is that we can declare this behavior once for all numeric literals. Moreover,
//! we move the conversion behavior away from where the literals are needed, enhancing readability
//! by reducing the noise imposed by being explicit about the exact types involved.
//!
//! That said, the macro should be used with care. It is recommended to keep the macro close to
//! the region in which the literals are being used, as to avoid confusion for readers of the code.
//! The Rust code that is usually not valid Rust (because of the lack of explicit conversion),
//! but without the context of the attribute, it is simply not clear why this code still compiles.

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::visit_mut::{visit_expr_mut, VisitMut};
use syn::{parse_macro_input, Expr, ExprLit, Item, Lit};

use quote::quote;
use syn::punctuated::Pair;

/// Visit an expression and replaces any numeric literal
/// with the replacement expression, in which a placeholder identifier
/// is replaced with the numeric literal.
struct NumericLiteralVisitor<'a> {
    pub placeholder: &'a str,
    pub replacement: &'a Expr,
}

impl<'a> VisitMut for NumericLiteralVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Lit(lit_expr) = expr {
            match lit_expr.lit {
                // TODO: Currently we cannot correctly treat integers that don't fit in 64
                // bits. For this we'd have to deal with verbatim literals and manually
                // parse the string
                Lit::Int(_) | Lit::Float(_) => {
                    let mut adapted_replacement = self.replacement.clone();
                    let mut replacer = ReplacementExpressionVisitor {
                        placeholder: self.placeholder,
                        literal: lit_expr,
                    };

                    replacer.visit_expr_mut(&mut adapted_replacement);
                    *expr = adapted_replacement;
                    return;
                }
                _ => {}
            }
        }
        visit_expr_mut(self, expr)
    }
}

/// Visits the "replacement expression", which replaces a placeholder identifier
/// with the given literal.
struct ReplacementExpressionVisitor<'a> {
    pub placeholder: &'a str,
    pub literal: &'a ExprLit,
}

impl<'a> VisitMut for ReplacementExpressionVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Path(path_expr) = expr {
            if let Some(last_pair) = path_expr.path.segments.last() {
                if let Pair::End(segment) = last_pair {
                    if segment.ident == self.placeholder {
                        *expr = Expr::Lit(self.literal.clone());
                        return;
                    }
                }
            }
        }
        visit_expr_mut(self, expr)
    }
}

/// Replace any numeric literal with custom transformation code.
///
/// Refer to the documentation at the root of the crate for usage instructions.
#[proc_macro_attribute]
pub fn replace_numeric_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);
    let attributes_tree = parse_macro_input!(attr as Expr);

    let mut replacer = NumericLiteralVisitor {
        placeholder: "literal",
        replacement: &attributes_tree,
    };
    replacer.visit_item_mut(&mut input);

    let expanded = quote! { #input };

    TokenStream::from(expanded)
}
