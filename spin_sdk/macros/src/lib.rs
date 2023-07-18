use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn generate_payload(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let fn_args = function.sig.inputs;

    let struct_name = {
        let mut name = fn_name.to_string();
        if let Some(first_char) = name.chars().next() {
            name.replace_range(..1, &first_char.to_uppercase().to_string());
        }
        name += "Payload";
        syn::Ident::new(&name, fn_name.span())
    };

    let struct_fields = fn_args.iter().map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                let field_name = &ident.ident;
                let field_type = &pat_type.ty;
                quote! { pub #field_name: #field_type }
            } else {
                panic!("Invalid function argument pattern");
            }
        } else {
            panic!("Invalid function argument");
        }
    });

    let struct_tokens = quote! {
        struct #struct_name {
            #( #struct_fields ),*
        }
    };

    TokenStream::from(struct_tokens)
}
