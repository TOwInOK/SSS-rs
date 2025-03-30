use sss_core::LayoutLimitations;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct LayoutsInput {
    pub template_dir_path: LitStr,
    pub default_layout: LitStr,
}

impl Parse for LayoutsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let mut iter = content.into_iter();
        let template_dir_path = iter.next().expect("not found template dir");
        let default_layout = iter.next().expect("not found default layouts dir");
        Ok(LayoutsInput {
            template_dir_path,
            default_layout,
        })
    }
}

pub struct Templates {
    /// Lower name
    pub layout_name: LitStr,
    /// Upper name
    pub layout_ident: Ident,
    /// Name of const value
    pub const_limitations_name: Ident,
    // Tera
    pub template: &'static str,
    // Ron
    pub limitations: LayoutLimitations,
}

pub struct DefaultLayout {
    pub layout_name: LitStr,
    pub layout_ident: Ident,
    // Tera
    pub template: &'static str,
}
