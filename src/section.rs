use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Block, LitStr, Stmt, Token};

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

impl ToTokens for Section {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}
