use proc_macro2::{Span, TokenStream};
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
    let bitfield_type = &bitfield.underlying_type;
    let bitfield_type_bits = bitfield.underlying_type_bits();

    let mut data_mask: u64 = 0;
    let mut fields = vec![];
    for field in bitfield.fields {
        if field.range.lo >= field.range.hi || field.range.hi > bitfield_type_bits {
            return Err(Error::new(field.range.span, "Bitfield invalid range"));
        }

        let visibility = &field.visibility;
        let field_type = &field.underlying_type;
        let ident = &field.ident;
        let set_ident = format_ident!("set_{ident}");
        let return_type = &field.return_type();

        let mask = field.range.mask();
        let shift = field.range.shift();

        data_mask |= mask << shift;

        let modifier = if let Some(ref modifier) = field.modifier {
            quote! {
                let value = (#modifier)(value);
            }
        } else {
            TokenStream::new()
        };

        fields.push(quote! {
            #visibility fn #ident(&self) -> #return_type {
                let mask = #mask as #bitfield_type;
                let shift = #shift as #bitfield_type;
                let value = ((self.data >> shift) & mask) as #field_type;
                #modifier
                value
            }

            #visibility fn #set_ident(&mut self, value: #field_type) {
                let mask = #mask as #bitfield_type;
                let shift = #shift as #bitfield_type;
                self.data = (self.data & !(mask << shift)) | ((value & mask) << shift);
            }
        });
    }

    let visibility = &bitfield.visibility;
    let ident = &bitfield.ident;

    Ok(quote! {
        #[derive(Default, Clone, Copy)]
        #visibility struct #ident {
            pub data: #bitfield_type,
        }

        impl #ident {
            pub fn new(data: #bitfield_type) -> Self {
                Self { data }
            }

            #(#fields)*
        }
    })
}

struct Range {
    pub span: Span,
    pub lo: usize,
    pub hi: usize,
}

impl Parse for Range {
    fn parse(input: ParseStream) -> Result<Self> {
        let lo = input.parse::<LitInt>()?.base10_parse()?;
        let _: Token![..] = input.parse()?;
        let hi = input.parse::<LitInt>()?.base10_parse()?;
        Ok(Range {
            span: input.span(),
            lo,
            hi,
        })
    }
}

impl Range {
    pub fn mask(&self) -> u64 {
        u64::MAX >> (u64::BITS as usize - (self.hi - self.lo))
    }

    pub fn shift(&self) -> u64 {
        self.lo as u64
    }
}

struct Field {
    pub visibility: Visibility,
    pub ident: Ident,
    pub underlying_type: Type,
    pub range: Range,
    pub modifier: Option<ExprClosure>,
}

impl Field {
    pub fn return_type(&self) -> &Type {
        if let Some(ref modifier) = self.modifier {
            match &modifier.output {
                ReturnType::Default => &self.underlying_type,
                ReturnType::Type(_, return_type) => return_type,
            }
        } else {
            &self.underlying_type
        }
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        let ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let underlying_type = input.parse()?;
        let _: Token![@] = input.parse()?;
        let range = input.parse()?;

        let mut modifier = None;
        if input.parse::<Token![=>]>().is_ok() {
            modifier = Some(input.parse::<ExprClosure>()?);
        }

        Ok(Field {
            visibility,
            ident,
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

impl Bitfield {
    pub fn underlying_type_bits(&self) -> usize {
        let bits = match self.underlying_type.to_token_stream().to_string().as_str() {
            "u8" => u8::BITS,
            "u16" => u16::BITS,
            "u32" => u32::BITS,
            "u64" => u64::BITS,
            "usize" => usize::BITS,
            _ => unreachable!(),
        };
        bits as usize
    }
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
            "u8" | "u16" | "u32" | "u64" | "usize" => (),
            _ => {
                return Err(Error::new(
                    underlying_type.span(),
                    "Bitfield underlying type must be an unsigned int",
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
