use crate::test_case::{test_names_duplicated, TestCase};
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
        let tests_ref = &self.tests;

        // Test each name in the test_suite against all the others to make
        // sure they aren't the same.
        let mut error_occur = false;
        let count = tests_ref.len();
        for index in 0..count {
            for inner_index in 0..count {
                if index != inner_index {
                    let result = test_names_duplicated(&tests_ref[index], &tests_ref[inner_index]);
                    if result.is_err() {
                        tokens.extend(result.err().unwrap().to_compile_error());
                        error_occur = true;
                    }
                }
            }
        }

        if error_occur {
            return;
        }

        let expanded = quote! {
            #(
                #tests_ref
            )*
        };

        tokens.extend(expanded);
    }
}
