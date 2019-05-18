extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Expr, Item};
use syn::visit_mut::{VisitMut, visit_item_mut, visit_expr_mut};

use quote::quote;
use quote::ToTokens;

struct NumericLiteralReplacer {
    pub typename: String
}

impl VisitMut for NumericLiteralReplacer {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i  {
            Expr::Lit(expr) => {
                let replaced = format!("{}::from({})", &self.typename, expr.into_token_stream().to_string());
                *i = syn::parse_str(&replaced).unwrap();
            },
            _ => visit_expr_mut(self, i)
        }
    }
}

#[proc_macro_attribute]
pub fn numeric_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    let numeric_type = attr.to_string().trim().to_string();

    let mut input = parse_macro_input!(item as Item);

    let mut replacer = NumericLiteralReplacer { typename: numeric_type };

    visit_item_mut(&mut replacer, &mut input);

    let expanded = quote!{ #input };

    TokenStream::from(expanded)
}
