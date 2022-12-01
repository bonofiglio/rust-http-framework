use http_types::{HTTPMethod, UriParser};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, Pat, PatType};

mod arguments;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    let Ok(mut input_fn) = syn::parse::<ItemFn>(input) else {
        panic!("Could not parse function");
    };

    // Try to parse the list of args as the Arguments type
    let arguments = parse_macro_input!(args as arguments::Arguments);

    // Get the function name
    let fn_name = input_fn.sig.ident.clone();

    let Some(method) = HTTPMethod::from(&arguments.method) else {
        panic!("{} is not a valid HTTP method", &arguments.method);
    };

    let method_name = syn::LitStr::new(&method.as_str(), fn_name.span());
    let path = syn::LitStr::new(&arguments.path, fn_name.span());

    let uri_parser = UriParser::from(&arguments.path);
    let dynamic_params = uri_parser.dynamic_params.iter().map(|x| x.1);

    let mut dynamic_params_vec = dynamic_params.clone().collect::<Vec<&String>>();

    // Check for duplicate param names
    dynamic_params_vec.sort_unstable();

    for idx in 1..dynamic_params_vec.len() {
        if dynamic_params_vec[idx - 1] == dynamic_params_vec[idx] {
            panic!(
                "Parameter identifier \"{}\" used more than once.",
                &dynamic_params_vec[idx]
            );
        }
    }

    let Some(request_arg) = input_fn.sig.inputs.first() else {
        panic!("expected 1 argument, got 0");
    };
    let request_arg_ident = match request_arg {
        FnArg::Typed(PatType { pat, .. }) => match &**pat {
            Pat::Ident(ident) => ident,
            _ => panic!("expected ident"),
        },
        FnArg::Receiver(_) => panic!("expected request argument"),
    };

    input_fn.block.stmts.insert(
        0,
        parse_quote!(
            let __parser = UriParser::from(#path);
        ),
    );

    input_fn.block.stmts.insert(
        1,
        parse_quote!(
            let __params = __parser.parse(&#request_arg_ident.uri).unwrap();
        ),
    );

    let mut idx = 0;
    for param_name in dynamic_params {
        let param_name_ident = syn::Ident::new(param_name, fn_name.span());
        let idx_lit = syn::LitInt::new(&format!("{}", idx), fn_name.span());
        input_fn.block.stmts.insert(
            2,
            parse_quote!(
                let #param_name_ident: String = (&__params)[#idx_lit].1.to_owned();
            ),
        );
        idx += 1;
    }

    input_fn.block.stmts.insert(
        idx + 2,
        parse_quote!(
           std::mem::drop(__parser);
        ),
    );

    input_fn.block.stmts.insert(
        idx + 3,
        parse_quote!(
           std::mem::drop(__params);
        ),
    );

    TokenStream::from(quote!(
        #input_fn

        #[allow(non_camel_case_types)]
        struct #fn_name {}

        impl #fn_name {
            pub fn route() -> Route {
                Route {
                    handler: #fn_name,
                    method: HTTPMethod::from(#method_name).unwrap(),
                    uri_parser: UriParser::from(#path)
                }
            }
        }
    ))
}
