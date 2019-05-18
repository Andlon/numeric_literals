extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Expr, Item, ExprLit, Lit};
use syn::visit_mut::{VisitMut, visit_item_mut, visit_expr_mut};

use quote::quote;
use syn::punctuated::Pair;

/// Visit an expression and replaces any numeric literal
/// with the replacement expression, in which a placeholder identifier
/// is replaced with the numeric literal.
struct NumericLiteralVisitor {
    pub placeholder: String,
    pub replacement: Expr
}

impl VisitMut for NumericLiteralVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr  {
            Expr::Lit(lit_expr) => {
                match lit_expr.lit {
                    // TODO: Currently we cannot correctly treat integers that don't fit in 64
                    // bits. For this we'd have to deal with verbatim literals and manually
                    // parse the string
                    Lit::Int(_) | Lit::Float(_) => {
                        let mut adapted_replacement = self.replacement.clone();
                        let mut replacer = ReplacementExpressionVisitor {
                            placeholder: self.placeholder.clone(),
                            literal: lit_expr.clone()
                        };

                        replacer.visit_expr_mut(&mut adapted_replacement);
                        *expr = adapted_replacement;
                        return;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        visit_expr_mut(self, expr)
    }
}

/// Visits the "replacement expression", which replaces a placeholder identifier
/// with the given literal.
struct ReplacementExpressionVisitor {
    pub placeholder: String,
    pub literal: ExprLit
}

impl VisitMut for ReplacementExpressionVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Path(path_expr) => {
                if let Some(last_pair) = path_expr.path.segments.last() {
                    if let Pair::End(segment) = last_pair {
                        if segment.ident.to_string() == self.placeholder {
                            *expr = Expr::Lit(self.literal.clone());
                            return;
                        }
                    }
                }
            },
            _ => {}
        }
        visit_expr_mut(self, expr)
    }
}

#[proc_macro_attribute]
pub fn replace_numeric_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);
    let attributes_tree = parse_macro_input!(attr as Expr);

    let mut replacer = NumericLiteralVisitor { placeholder: "literal".to_string(), replacement: attributes_tree };

    visit_item_mut(&mut replacer, &mut input);

    let expanded = quote!{ #input };

    TokenStream::from(expanded)
}
