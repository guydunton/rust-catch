use crate::section::{kw::section, Section};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Block, Ident, LitStr, Stmt, Token};

mod kw {
    custom_keyword!(test_case);
}

#[derive(Clone)]
enum CaseLines {
    Statement(Stmt),
    Section(Section),
}

impl ToTokens for CaseLines {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CaseLines::Statement(stmt) => {
                stmt.to_tokens(tokens);
            }
            CaseLines::Section(sec) => {
                sec.to_tokens(tokens);
            }
        }
    }
}

fn is_section(line: &CaseLines) -> bool {
    match line {
        CaseLines::Section(_) => true,
        _ => false,
    }
}

pub struct TestCase {
    name: LitStr,
    code: Vec<CaseLines>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::test_case>()?;
        input.parse::<Token![!]>()?;

        // Parse the brackets to get the name
        let paren_content;
        parenthesized!(paren_content in input);
        let name = paren_content.parse::<LitStr>()?;

        // Parse the block
        let brace_content;
        braced!(brace_content in input);

        let mut code = Vec::new();

        loop {
            if brace_content.is_empty() {
                break;
            }
            let lookahead = brace_content.lookahead1();
            if lookahead.peek(section) {
                let next_section = brace_content.parse::<Section>()?;
                code.push(CaseLines::Section(next_section));
            } else {
                let next_line = brace_content.parse::<Stmt>()?;
                code.push(CaseLines::Statement(next_line));
            }
        }

        Ok(TestCase { name, code })
    }
}

fn name_as_ident(name: &LitStr) -> Ident {
    let text = name.value().replace(' ', "_");
    Ident::new(&text[..], name.span())
}

impl ToTokens for TestCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let final_name = name_as_ident(&self.name);
        let code = self.code.clone();

        let result = if code.iter().any(|l| is_section(l)) {
            // Print each section with the current lines
            quote! {}
        } else {
            quote! {
                #[test]
                fn #final_name() {
                    #( #code )*;
                }
            }
        };

        tokens.extend(result);
    }
}
