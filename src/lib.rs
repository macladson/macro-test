use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemStruct};

#[proc_macro_attribute]
pub fn generate_feature_with_dependencies(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;

    let args: Vec<Expr> = parse_macro_input!(args as syn::ExprArray)
        .elems
        .into_iter()
        .collect();

    // Ensure vector elements are boxed.
    let boxed_exprs: Vec<proc_macro2::TokenStream> = args
        .into_iter()
        .map(|expr| quote! { Box::new(#expr) })
        .collect();

    let vec_expr = quote! {
        vec![#(#boxed_exprs),*]
    };

    // Generate the struct and implement Feature.
    let expanded = quote! {
        #input_struct

        impl Feature for #struct_name {
            fn dependencies(&self) -> Features {
                #vec_expr
            }
        }
    };

    expanded.into()
}
