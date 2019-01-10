use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Block, Ident, LitStr, Stmt, Token};

pub mod kw {
    custom_keyword!(section);
}

#[derive(Clone)]
pub struct Section {
    name: LitStr,
    code: Vec<Stmt>,
}

impl Parse for Section {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::section>()?;
        input.parse::<Token![!]>()?;

        // Parse the name
        let paren_content;
        parenthesized!(paren_content in input);
        let name = paren_content.parse::<LitStr>()?;

        // Parse the block
        let brace_content;
        braced!(brace_content in input);
        let code = brace_content.call(Block::parse_within)?;

        Ok(Section { name, code })
    }
}

fn name_as_ident(name: &LitStr) -> Ident {
    let text = name.value().replace(' ', "_");
    Ident::new(&text[..], name.span())
}

impl ToTokens for Section {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = self.code.clone();
        tokens.extend(quote! {
            #( #code )*;
        });
    }
}

#[derive(Clone)]
pub struct IndexSection {
    index: u32,
    section: Section,
}

impl IndexSection {
    pub fn new(index: u32, section: Section) -> IndexSection {
        IndexSection { index, section }
    }

    pub fn name(&self) -> Ident {
        name_as_ident(&self.section.name)
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl ToTokens for IndexSection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let section = &self.section;
        let index = self.index;

        tokens.extend(quote! {
            if __rust_catch_section == #index {
                #section
            }
        });
    }
}
