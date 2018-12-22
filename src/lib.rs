#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use colored::Colorize;
use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::parse_macro_input;

struct ToDo {
    _description: String,
}

impl Parse for ToDo {
    fn parse(input: ParseStream) -> Result<Self> {
        let description: syn::LitStr = input.parse()?;
        Ok(ToDo {
            _description: description.value(),
        })
    }
}

#[proc_macro]
pub fn todo(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let to_do = parse_macro_input!(input_clone as ToDo);
    println!("{} {}", "warning:".red().bold(),to_do._description);
    TokenStream::new()
}
