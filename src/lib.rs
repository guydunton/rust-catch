#[macro_use] // For some reason needed for custom_keyword
extern crate syn;
extern crate proc_macro;

use quote::quote;
use syn::parse_macro_input;

mod names;
mod section;
mod test_case;
mod test_suite;
use crate::test_suite::TestSuite;

#[proc_macro]
#[deprecated(since = "0.2.2", note = "Use `tests` instead")]
pub fn test_suite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tests(input)
}

#[proc_macro]
pub fn tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let suite = parse_macro_input!(input as TestSuite);

    let expanded = quote! {
        #suite
    };

    proc_macro::TokenStream::from(expanded)
}
