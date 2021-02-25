#![feature(proc_macro_quote)]

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;

#[proc_macro_derive(ApiHelpers)]
pub fn derive_api_helpers(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);

    let field_count = input.fields.iter().count();    

    let name = &input.ident;

    let simple_type = if field_count == 1 {
        let Some(f)= input.fields.iter().next();
        f.ty.Name  
    } else {
        "Self" // name
    };

    let output = quote! {
        impl esc_api_helpers::ApiHelpers for #name {
            Simplified = #simple_type;

            fn field_count() -> usize {
                #field_count
            }
        }
    };

    // Return output tokenstream
    TokenStream::from(output)
}
