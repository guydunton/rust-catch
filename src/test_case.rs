use crate::names::name_as_ident2;
use crate::section::{kw::section, IndexSection, Section};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, LitStr, Stmt};

#[allow(non_camel_case_types)]
mod kw {
    custom_keyword!(test_case);
    custom_keyword!(test);
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

pub fn test_names_duplicated(lhs: &TestCase, rhs: &TestCase) -> Result<()> {
    if lhs.name.value() == rhs.name.value() {
        let error_message = format!("name `{}` is a duplicate", lhs.name.value());
        return Err(syn::Error::new(lhs.name.span(), error_message));
    }
    Ok(())
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::test_case) {
            // Would emit a warning here but that has to wait until
            // Procedural Macro Diagnostics (RFC 1566) is in the language
            input.parse::<kw::test_case>()?;
        } else {
            input.parse::<kw::test>()?;
        }

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

impl ToTokens for TestCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let final_name = name_as_ident2("test_case", &self.name);
        let code = self.code.clone();

        match final_name {
            Err(error) => {
                tokens.extend(error.to_compile_error());
            }
            Ok(name) => {
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
                        let section_name = section.name();
                        let index = section.index();
                        let trans = &code;

                        match section_name {
                            Err(error) => {
                                section_stream.extend(error.to_compile_error());
                            }
                            Ok(name) => {
                                section_stream.extend(quote! {
                                    #[test]
                                    #[allow(non_snake_case)]
                                    fn #name() {
                                        let __rust_catch_section = #index;

                                        #( #trans )*
                                    }
                                });
                            }
                        }
                    }

                    tokens.extend(quote! {
                        mod #name {
                            use super::*;
                            #section_stream
                        }
                    });
                } else {
                    let result = quote! {
                        #[test]
                        #[allow(non_snake_case)]
                        fn #name() {
                            #( #code )*;
                        }
                    };
                    tokens.extend(result);
                }
            }
        }
    }
}

#[test]
fn get_error_with_duplicate_named_test_cases() {
    let lit_str = LitStr::new("Duplicated", proc_macro2::Span::call_site());

    let tc1 = TestCase {
        name: lit_str.clone(),
        code: vec![],
    };

    let tc2 = TestCase {
        name: lit_str.clone(),
        code: vec![],
    };

    assert!(test_names_duplicated(&tc1, &tc2).is_err());
}

#[test]
fn get_no_error_with_distinctly_named_test_cases() {
    let tc1 = TestCase {
        name: LitStr::new("test1", proc_macro2::Span::call_site()),
        code: vec![],
    };

    let tc2 = TestCase {
        name: LitStr::new("test2", proc_macro2::Span::call_site()),
        code: vec![],
    };

    assert!(test_names_duplicated(&tc1, &tc2).is_ok());
}
