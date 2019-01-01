use crate::section::{kw::section, Section};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Ident, LitStr, Stmt, Token};

mod kw {
    custom_keyword!(test_case);
}

#[derive(Clone)]
enum CaseLines {
    Statement(Box<Stmt>),
    Section(Section),
}

impl ToTokens for CaseLines {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CaseLines::Statement(stmt) => tokens.extend(quote! {
                #stmt
            }),
            CaseLines::Section(sec) => tokens.extend(quote! {
                #sec
            }),
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
                code.push(CaseLines::Statement(Box::new(next_line)));
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
            // Print each section with the current lines

            let section_indices: Vec<usize> = code
                .iter()
                .enumerate()
                .filter(|(_, l)| is_section(l))
                .map(|(i, _)| i)
                .collect();

            // A stream to put the sections into
            let mut section_stream = proc_macro2::TokenStream::new();

            for index in section_indices {
                match &code[index] {
                    CaseLines::Section(sec) => {
                        let before_code = code[..index].iter().filter(|l| !is_section(&l));
                        let after_code = code[index + 1..].iter().filter(|l| !is_section(&l));
                        let fn_name = sec.name();

                        section_stream.extend(quote! {
                            #[test]
                            fn #fn_name() {
                                #( #before_code )*;

                                #sec

                                #( #after_code )*;
                            }
                        });
                    }
                    _ => panic!("Found a line that has incorrectly been identified as a section"),
                }
            }

            tokens.extend(quote! {
                mod #final_name {
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
