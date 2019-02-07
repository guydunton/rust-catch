use syn::{Error, Ident, LitStr};

pub fn name_as_ident2(construct_name: &str, name: &LitStr) -> syn::Result<Ident> {
    if name.value().is_empty() {
        let text = format!("{} names cannot be empty. \n\n Hint: Try to use a descriptive name such as \"Vec can be resized\".\n", construct_name);
        return Err(Error::new(name.span(), text));
    }
    let text = name.value().replace(' ', "_");
    Ok(Ident::new(&text[..], name.span()))
}
