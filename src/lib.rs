//!
//! **numeric_literals** is a Rust library that provides procedural attribute macros for replacing
//! numeric literals with arbitrary expressions.
//!
//! While Rust's explicitness is generally a boon, it is a major pain when writing numeric
//! code that is intended to be generic over a scalar type. As an example, consider
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
//! Float and integer literal replacement
//! -------------------------------------
//!
//! An issue with the replacement of numeric literals is that there is no way to dinstinguish
//! literals that are used for e.g. indexing from those that are part of a numerical computation.
//! In the example above, if you would additionally need to index into an array with a constant index
//! such as `array[0]`, the macro will try to convert the index `0` to a float type, which
//! would clearly fail. Thankfully, in most cases these examples will outright fail to compile
//! because of type mismatch. One possible resolution to this problem is to use the separate
//! macros `replace_float_literals` and `replace_int_literals`, which work in the exact same way,
//! but only trigger on float or integer literals, respectively. Below is an example from
//! Finite Element code that uses float literal replacement to improve readability of numerical
//! constants in generic code.
//!
//! ```ignore
//! #[replace_float_literals(T::from_f64(literal).expect("Literal must fit in T"))]
//! pub fn assemble_element_mass<T>(quad: &Quad2d<T>) -> MatrixN<T, U8>
//! where
//!    T: RealField
//! {
//!     let phi = |alpha, beta, xi: &Vector2<T>| -(1.0 + alpha * xi[0]) * (1.0 + beta * xi[1]) / 4.0;
//!     let phi_grad = |alpha, beta, xi: &Vector2<T>| {
//!         Vector2::new(
//!             alpha * (1.0 + beta * xi[1]) / 4.0,
//!             beta * (1.0 + alpha * xi[0]) / 4.0,
//!         )
//!     };
//!     let alphas = [-1.0, 1.0, 1.0, -1.0];
//!     let betas = [-1.0, -1.0, 1.0, 1.0];
//!
//!     // And so on...
//! ```
//!
//! In general, **the macros should be used with caution**. It is recommended to keep the macro close to
//! the region in which the literals are being used, as to avoid confusion for readers of the code.
//! The Rust code before macro expansion is usually not valid Rust (because of the lack of explicit
//! type conversion), but without the context of the attribute, it is simply not clear why this
//! code still compiles.
//!
//! An option for the future would be to apply the attribute only to very local blocks of code that
//! are heavy on numerical constants. However, at present, Rust does not allow attribute macros
//! to apply to blocks or single expressions.

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::visit_mut::{visit_expr_mut, VisitMut};
use syn::{
    parse_macro_input, Expr, ExprAssign, ExprLit, ExprPath, Item, Lit, LitBool, Macro, Token,
};

use quote::{quote, ToTokens};

/// Visit an expression and replaces any numeric literal
/// with the replacement expression, in which a placeholder identifier
/// is replaced with the numeric literal.
struct NumericLiteralVisitor<'a> {
    pub parameters: MacroParameters,
    pub placeholder: &'a str,
    pub float_replacement: &'a Expr,
    pub int_replacement: &'a Expr,
}

struct FloatLiteralVisitor<'a> {
    pub parameters: MacroParameters,
    pub placeholder: &'a str,
    pub replacement: &'a Expr,
}

struct IntLiteralVisitor<'a> {
    pub parameters: MacroParameters,
    pub placeholder: &'a str,
    pub replacement: &'a Expr,
}

fn replace_literal(expr: &mut Expr, placeholder: &str, literal: &ExprLit) {
    let mut replacer = ReplacementExpressionVisitor {
        placeholder,
        literal,
    };
    replacer.visit_expr_mut(expr);
}

fn try_parse_punctuated_macro<P: ToTokens, V: VisitMut, F: Parser<Output = Punctuated<Expr, P>>>(
    visitor: &mut V,
    mac: &mut Macro,
    parser: F,
) -> bool {
    if let Ok(mut exprs) = mac.parse_body_with(parser) {
        exprs
            .iter_mut()
            .for_each(|expr| visitor.visit_expr_mut(expr));
        mac.tokens = exprs.into_token_stream();
        return true;
    }
    return false;
}

fn visit_macros_mut<V: VisitMut>(visitor: &mut V, mac: &mut Macro) {
    // Handle expression based macros (e.g. assert)
    if let Ok(mut expr) = mac.parse_body::<Expr>() {
        visitor.visit_expr_mut(&mut expr);
        mac.tokens = expr.into_token_stream();
        return;
    }

    // Handle , punctuation based macros (e.g. vec with list, assert_eq)
    let parser_comma = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    if try_parse_punctuated_macro(visitor, mac, parser_comma) {
        return;
    }

    // Handle ; punctuation based macros (e.g. vec with repeat)
    let parser_semicolon = Punctuated::<Expr, Token![;]>::parse_separated_nonempty;
    if try_parse_punctuated_macro(visitor, mac, parser_semicolon) {
        return;
    }
}

impl<'a> VisitMut for FloatLiteralVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Lit(lit_expr) = expr {
            if let Lit::Float(_) = lit_expr.lit {
                let mut adapted_replacement = self.replacement.clone();
                replace_literal(&mut adapted_replacement, self.placeholder, lit_expr);
                *expr = adapted_replacement;
                return;
            }
        }
        visit_expr_mut(self, expr)
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if self.parameters.visit_macros {
            visit_macros_mut(self, mac);
        }
    }
}

impl<'a> VisitMut for IntLiteralVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Lit(lit_expr) = expr {
            if let Lit::Int(_) = lit_expr.lit {
                let mut adapted_replacement = self.replacement.clone();
                replace_literal(&mut adapted_replacement, self.placeholder, lit_expr);
                *expr = adapted_replacement;
                return;
            }
        }
        visit_expr_mut(self, expr)
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if self.parameters.visit_macros {
            visit_macros_mut(self, mac);
        }
    }
}

impl<'a> VisitMut for NumericLiteralVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Lit(lit_expr) = expr {
            match lit_expr.lit {
                // TODO: Currently we cannot correctly treat integers that don't fit in 64
                // bits. For this we'd have to deal with verbatim literals and manually
                // parse the string
                Lit::Int(_) => {
                    let mut visitor = IntLiteralVisitor {
                        parameters: self.parameters,
                        placeholder: self.placeholder,
                        replacement: self.int_replacement,
                    };
                    visitor.visit_expr_mut(expr);
                    return;
                }
                Lit::Float(_) => {
                    let mut visitor = FloatLiteralVisitor {
                        parameters: self.parameters,
                        placeholder: self.placeholder,
                        replacement: self.float_replacement,
                    };
                    visitor.visit_expr_mut(expr);
                    return;
                }
                _ => {}
            }
        }
        visit_expr_mut(self, expr)
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if self.parameters.visit_macros {
            visit_macros_mut(self, mac);
        }
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
            if let Some(last_segment) = path_expr.path.segments.last() {
                if last_segment.ident == self.placeholder {
                    *expr = Expr::Lit(self.literal.clone());
                    return;
                }
            }
        }
        visit_expr_mut(self, expr)
    }
}

struct MacroParameterVisitor {
    pub name: Option<String>,
    pub value: Option<ParameterValue>,
}

impl MacroParameterVisitor {
    fn parse_flag(expr: &Expr) -> Option<(String, ParameterValue)> {
        let mut visitor = MacroParameterVisitor {
            name: None,
            value: None,
        };
        visitor.visit_expr(expr);
        let name = visitor.name.take();
        let value = visitor.value.take();
        name.and_then(|n| value.and_then(|v| Some((n, v))))
    }
}

impl<'ast> Visit<'ast> for MacroParameterVisitor {
    fn visit_expr_assign(&mut self, expr: &'ast ExprAssign) {
        self.visit_expr(&expr.left);
        self.visit_expr(&expr.right);
    }

    fn visit_expr_path(&mut self, expr: &'ast ExprPath) {
        let mut name = Vec::new();
        expr.path
            .leading_colon
            .map(|_| name.push(String::from("::")));
        for p in expr.path.segments.pairs() {
            match p {
                syn::punctuated::Pair::Punctuated(ps, _sep) => {
                    name.push(ps.ident.to_string());
                    name.push(String::from("::"));
                }
                syn::punctuated::Pair::End(ps) => {
                    name.push(ps.ident.to_string());
                }
            }
        }
        self.name = Some(name.concat());
    }

    fn visit_lit_bool(&mut self, expr: &'ast LitBool) {
        self.value = Some(ParameterValue::Bool(expr.value));
    }
}

enum ParameterValue {
    Bool(bool),
}

#[derive(Copy, Clone)]
struct MacroParameters {
    pub visit_macros: bool,
}

impl Default for MacroParameters {
    fn default() -> Self {
        Self { visit_macros: true }
    }
}

impl MacroParameters {
    fn set(&mut self, name: &str, value: ParameterValue) {
        match name {
            "visit_macros" => match value {
                ParameterValue::Bool(v) => self.visit_macros = v,
            },
            _ => {}
        }
    }
}

/// Obtain the replacement expression and parameters from the macro attr token stream.
fn parse_macro_attribute(attr: TokenStream) -> Result<(Expr, MacroParameters), syn::Error> {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let attributes = parser.parse(attr)?;

    let mut attr_iter = attributes.into_iter();
    let replacement = attr_iter.next().expect("No replacement provided");

    let user_parameters: Vec<_> = attr_iter
        .filter_map(|expr| MacroParameterVisitor::parse_flag(&expr))
        .collect();
    let mut parameters = MacroParameters::default();
    for (name, value) in user_parameters {
        parameters.set(&name, value);
    }

    Ok((replacement, parameters))
}

/// Replace any numeric literal with custom transformation code.
///
/// Refer to the documentation at the root of the crate for usage instructions.
#[proc_macro_attribute]
pub fn replace_numeric_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);
    let (replacement, parameters) = match parse_macro_attribute(attr) {
        Ok(res) => res,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let mut replacer = NumericLiteralVisitor {
        parameters,
        placeholder: "literal",
        int_replacement: &replacement,
        float_replacement: &replacement,
    };
    replacer.visit_item_mut(&mut input);

    let expanded = quote! { #input };

    TokenStream::from(expanded)
}

/// Replace any float literal with custom transformation code.
///
/// Refer to the documentation at the root of the crate for usage instructions.
#[proc_macro_attribute]
pub fn replace_float_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);
    let (replacement, parameters) = match parse_macro_attribute(attr) {
        Ok(res) => res,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let mut replacer = FloatLiteralVisitor {
        parameters,
        placeholder: "literal",
        replacement: &replacement,
    };
    replacer.visit_item_mut(&mut input);

    let expanded = quote! { #input };

    TokenStream::from(expanded)
}

/// Replace any integer literal with custom transformation code.
///
/// Refer to the documentation at the root of the crate for usage instructions.
#[proc_macro_attribute]
pub fn replace_int_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);
    let (replacement, parameters) = match parse_macro_attribute(attr) {
        Ok(res) => res,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let mut replacer = IntLiteralVisitor {
        parameters,
        placeholder: "literal",
        replacement: &replacement,
    };
    replacer.visit_item_mut(&mut input);

    let expanded = quote! { #input };

    TokenStream::from(expanded)
}
