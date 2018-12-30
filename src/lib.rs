extern crate proc_macro;
use proc_macro2::TokenStream;

#[macro_use]
extern crate syn;

use syn::{ Block, Token, token, Stmt, LitStr, Ident };
use syn::parse::{Parse, ParseStream, Result};

#[macro_use]
extern crate quote;

use quote::{ ToTokens };

mod kw {
    custom_keyword!(test_case);
}

struct TestCase {
    case_token: kw::test_case,
    bang_token: token::Bang,
    name: LitStr,
    brace_token: token::Brace,
    code: Vec<Stmt>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let case_token = input.parse::<kw::test_case>()?;
        let bang_token = input.parse::<Token![!]>()?;
        
        // Parse the brackets to get the name
        let paren_content;
        let _paren_token = parenthesized!(paren_content in input);
        let name = paren_content.parse::<LitStr>()?;
        
        // Parse the block
        let brace_content;
        let brace_token = braced!(brace_content in input);
        let code = brace_content.call(Block::parse_within)?;

        Ok(TestCase{
            case_token,
            bang_token,
            name,
            brace_token,
            code,
        })
    }
}

fn name_as_ident(name: &LitStr) -> Ident {
    let text = name.value().replace(' ', "_");
    Ident::new(
        &text[..],
        name.span(),
    )
}

impl ToTokens for TestCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let final_name = name_as_ident(&self.name);
        let code = self.code.clone();
        let result = quote! {
            #[test]
            fn #final_name() {
                #( #code )*;
            }
        };

        tokens.extend(result);
    }
}

struct TestSuite {
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

        Ok(TestSuite{
            tests,
        })
    }
}

#[proc_macro]
pub fn test_suite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let suite = parse_macro_input!(input as TestSuite);
    let test = &suite.tests;

    let expanded = quote! {
        #(
            #test
        )*
    };

    proc_macro::TokenStream::from(expanded)
}