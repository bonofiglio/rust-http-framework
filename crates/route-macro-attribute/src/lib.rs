use http_types::method::HTTPMethod;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod arguments;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = syn::parse::<ItemFn>(input).unwrap();

    // Try to parse the list of args as the Arguments type
    let arguments = parse_macro_input!(args as arguments::Arguments);

    // Get the function name
    let fn_name = input_fn.sig.ident.clone();

    let Some(method) = HTTPMethod::from(&arguments.method) else {
        panic!("{} is not a valid HTTP method", &arguments.method);
    };

    let method_name = syn::LitStr::new(&method.as_str(), fn_name.span());
    let path = syn::LitStr::new(&arguments.path, fn_name.span());

    TokenStream::from(quote!(
        #input_fn

        #[allow(non_camel_case_types)]
        struct #fn_name {}

        impl #fn_name {
            pub fn route() -> Route {
                Route {
                    handler: #fn_name,
                    method: HTTPMethod::from(#method_name).unwrap(),
                    path: String::from(#path)
                }
            }
        }
    ))
}
