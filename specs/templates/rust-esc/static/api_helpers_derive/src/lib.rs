#![feature(proc_macro_quote)]

use proc_macro::quote;
use proc_macro::TokenStream;
use syn;
use syn::parse_macro_input;

#[proc_macro_derive(ApiHelpers)]
pub fn derive_api_helpers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);

    let field_count = input.fields.iter().count();

    let name = &input.ident;

    let output = quote! {
        impl #name {
            pub fn field_count() -> usize {
                #field_count
            }
        }
    };

    // Return output tokenstream
    TokenStream::from(output)
}
