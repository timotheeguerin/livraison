extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};
// use syn::parse_quote;
// use syn::spanned::Spanned;

// #[derive(Debug)]
// pub struct RequireCommandArgs {
//     args: Punctuated<Path, syn::Token![,]>,
// }

/// Conditionally ignore tests based on the presence of a command.
#[proc_macro_attribute]
pub fn require_command(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let command = parse_macro_input!(args as LitStr);
    let clone = input.clone();
    let input_fn = parse_macro_input!(clone as ItemFn);

    // Extract the command string
    let command_str = command.value();

    // Get the function name and body
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    let command_exists = which::which(&command_str).is_ok();
    eprintln!(
        "Skipping test '{}', required command '{}' found: {}",
        fn_name, command_str, command_exists
    );

    if command_exists {
        input
    } else {
        let output = quote! {
            #[test]
            #[ignore]
            fn #fn_name() {
                #fn_body
            }
        };

        output.into()
    }
}
