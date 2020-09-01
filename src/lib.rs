extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Pat, PatIdent, Expr, Token, Ident, parse_macro_input};
use syn::parse::{Parse, ParseStream, Result};
use syn::visit::{self, Visit};

struct MustLet {
    pattern: Pat,
    rhs: Expr,
}

impl Parse for MustLet {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![let]>()?;
        let pattern: Pat = input.parse()?;
        input.parse::<Token![=]>()?;
        let rhs: Expr = input.parse()?;
        Ok(Self { pattern, rhs })
    }
}

struct IdentVisitor {
    idents: Vec<Ident>,
}

impl<'ast> Visit<'ast> for IdentVisitor {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        self.idents.push(node.ident.clone());
        if let Some((_, ref subpat)) = node.subpat {
            visit::visit_pat(self, subpat);
        }
    }
}

#[proc_macro]
pub fn must_let(input: TokenStream) -> TokenStream {
    let MustLet { pattern, rhs } = parse_macro_input!(input as MustLet);

    let ident_tuple = {
        let mut v = IdentVisitor { idents: vec![] };
        v.visit_pat(&pattern);
        let idents = v.idents;
        quote! { (#(#idents),*) }
    };
    let out = quote! {
        let #ident_tuple = match #rhs {
            #pattern => #ident_tuple,
            r => panic!("Expected {}, received {:?}", stringify!(#pattern), r),
        };
    };
    TokenStream::from(out)
}
