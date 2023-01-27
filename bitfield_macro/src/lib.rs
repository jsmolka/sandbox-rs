use proc_macro2::TokenStream;
use quote::*;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

#[proc_macro]
pub fn bitfield(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_tokens(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn parse_tokens(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let bitfield = syn::parse::<Bitfield>(input)?;
    let ident = &bitfield.ident;
    let underlying_type = &bitfield.underlying_type;

    Ok(quote! {
        struct #ident {
            pub data: #underlying_type,
        }

        impl #ident {
            pub fn new() -> Self {
                Self { data: 0 }
            }
        }
    })
}

struct Range {
    pub lo: u8,
    pub hi: u8,
}

impl Parse for Range {
    fn parse(input: ParseStream) -> Result<Self> {
        let lo = input.parse::<LitInt>()?.base10_parse()?;
        let _: Token![..] = input.parse()?;
        let hi = input.parse::<LitInt>()?.base10_parse()?;

        // Todo: check range

        Ok(Range { lo, hi })
    }
}

struct Field {
    pub name: Ident,
    pub underlying_type: Type,
    pub range: Range,
    pub modifier: Option<ExprClosure>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _: Token![:] = input.parse()?;
        let underlying_type = input.parse()?;
        let _: Token![@] = input.parse()?;
        let range = input.parse()?;

        let mut modifier = None;
        if input.parse::<Token![=>]>().is_ok() {
            modifier = Some(input.parse::<ExprClosure>()?);
        }

        Ok(Field {
            name,
            underlying_type,
            range,
            modifier,
        })
    }
}

struct Bitfield {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub underlying_type: Type,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for Bitfield {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let underlying_type: Type = input.parse()?;
        match underlying_type.to_token_stream().to_string().as_str() {
            "bool" | "u8" | "u16" | "u32" | "u64" => (),
            _ => {
                return Err(Error::new(
                    underlying_type.span(),
                    "Bitfield underlying type must be a bool or unsigned int",
                ));
            }
        }

        let content;
        braced!(content in input);
        let fields = content.parse_terminated(Field::parse)?;

        Ok(Bitfield {
            attributes,
            visibility,
            ident,
            underlying_type,
            fields,
        })
    }
}
