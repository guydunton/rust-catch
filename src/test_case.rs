use crate::section::{kw::section, IndexSection, Section};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Ident, LitStr, Stmt};

mod kw {
    custom_keyword!(test_case);
}

#[derive(Clone)]
enum Lines {
    Statement(Box<Stmt>),
    Section(IndexSection),
}

impl ToTokens for Lines {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Lines::Statement(stmt) => stmt.to_tokens(tokens),
            Lines::Section(section) => section.to_tokens(tokens),
        }
    }
}

fn is_section(line: &Lines) -> bool {
    match line {
        Lines::Section(_) => true,
        _ => false,
    }
}

pub struct TestCase {
    name: LitStr,
    code: Vec<Lines>,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::test_case>()?;

        // Parse the brackets to get the name
        let paren_content;
        parenthesized!(paren_content in input);
        let name = paren_content.parse::<LitStr>()?;

        // Parse the block
        let brace_content;
        braced!(brace_content in input);

        let mut code = Vec::new();

        let mut section_count = 0;

        loop {
            if brace_content.is_empty() {
                break;
            }
            let lookahead = brace_content.lookahead1();
            if lookahead.peek(section) {
                let next_section = brace_content.parse::<Section>()?;
                code.push(Lines::Section(IndexSection::new(
                    section_count,
                    next_section,
                )));

                section_count += 1;
            } else {
                let next_line = brace_content.parse::<Stmt>()?;
                code.push(Lines::Statement(Box::new(next_line)));
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

        if code.iter().any(|l| is_section(l)) {
            // A stream to put the sections into
            let mut section_stream = proc_macro2::TokenStream::new();

            let sections = code
                .iter()
                .filter_map(|line| match line {
                    Lines::Section(section) => Some(section),
                    _ => None,
                })
                .collect::<Vec<&IndexSection>>();

            for section in sections {
                let name = section.name();
                let index = section.index();
                let trans = &code;

                section_stream.extend(quote! {
                    #[test]
                    fn #name() {
                        let __rust_catch_section = #index;

                        #( #trans )*
                    }
                });
            }

            tokens.extend(quote! {
                mod #final_name {
                    use super::*;
                    #section_stream
                }
            });
        } else {
            let result = quote! {
                #[test]
                fn #final_name() {
                    #( #code )*;
                }
            };
            tokens.extend(result);
        }
    }
}
