use crate::test_case::TestCase;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

pub struct TestSuite {
    tests: Vec<TestCase>,
}

impl Parse for TestSuite {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tests = Vec::new();

        loop {
            if input.is_empty() {
                break;
            }
            let case = input.parse::<TestCase>()?;
            tests.push(case);
        }

        Ok(TestSuite { tests })
    }
}

impl ToTokens for TestSuite {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let test = &self.tests;

        let expanded = quote! {
            #(
                #test
            )*
        };

        tokens.extend(expanded);
    }
}
