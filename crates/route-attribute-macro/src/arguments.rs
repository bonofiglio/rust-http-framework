use proc_macro::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, LitStr, Token,
};

#[derive(Debug)]
pub struct Arguments {
    pub method: String,
    pub path: String,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let vars = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let vars_vec: Vec<LitStr> = vars.into_iter().collect();

        if vars_vec.len() != 2 {
            Err(Error::new(
                Span::call_site().into(),
                format!("expected 2 arguments, found {}", vars_vec.len()),
            ))
        } else {
            let method = vars_vec[0].value();
            let path = vars_vec[1].value();

            Ok(Arguments { method, path })
        }
    }
}

impl Arguments {}
