use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ItemFn, Lit, MetaNameValue, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct ProblemIOSpec {
    input: String,
    output: String,
}

mod io_kw {
    syn::custom_keyword!(input);
    syn::custom_keyword!(output);
}

impl ProblemIOSpec {
    fn get_meta_value(meta: &MetaNameValue) -> String {
        let mut value = "".to_string();
        if let Expr::Lit(expr) = &meta.value {
            if let Lit::Str(lit_str) = &expr.lit {
                value = lit_str.value();
            }
        }
        value
    }
}

impl Parse for ProblemIOSpec {
    fn parse(tokens: ParseStream) -> Result<Self> {
        let attr_pairs = tokens.call(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)?;
        let mut input_source = "stdin".to_string();
        let mut output_source = "stdout".to_string();

        for attr in attr_pairs {
            if attr.path.is_ident("input") && Self::get_meta_value(&attr) != "stdin" {
                input_source = String::from("cases/") + &Self::get_meta_value(&attr);
            }
            if attr.path.is_ident("output") && Self::get_meta_value(&attr) != "stdout" {
                output_source = String::from("cases/") + &Self::get_meta_value(&attr);
            }
        }

        Ok(ProblemIOSpec {
            input: input_source,
            output: output_source,
        })
    }
}

/*
 * This macro will parse things that look like:
 * #[competitive_problem(input = "inputs/cows3.txt", output = "stdout")]
*/
#[proc_macro_attribute]
pub fn competitive_problem(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let io_spec = parse_macro_input!(args as ProblemIOSpec);

    // Declaring names for the IO strings as referencing
    // `io_spec` within the quoted block below requires
    // me to implement `ToTokens` trait on it.
    let input_source = io_spec.input;
    let output_source = io_spec.output;
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        pub fn run_problem() {
            use std::{
                fs::File,
                io::{self, BufRead, BufReader, Write},
            };

            let input_source = #input_source;
            let output_source = #output_source;

            let mut reader: Box<dyn BufRead> = Box::new(io::stdin().lock());
            if input_source != "stdin" {
                let f = File::open(input_source).unwrap();
                reader = Box::new(BufReader::new(f));
            }

            let mut writer: Box<dyn Write> = Box::new(io::stdout());
            if output_source != "stdout" {
                let f = File::create(output_source).unwrap();
                writer = Box::new(f);
            }

            #fn_name(reader, writer).unwrap();
        }
    };
    TokenStream::from(expanded)
}
