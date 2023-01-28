use proc_macro2::TokenStream;
use quote::*;
use std::ops::Range;
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
        let visibility = &field.visibility;
        let field_type = &field.underlying_type;
        let ident = &field.ident;
        let ident_set = format_ident!("set_{ident}");
        let return_type = &field.return_type();
        let range = field.to_open_range(bitfield_type_bits)?;

        let mask = u64::MAX >> (u64::BITS as usize - (range.end - range.start));
        let shift = range.start;

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

            #visibility fn #ident_set(&mut self, value: #field_type) {
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

struct Field {
    pub visibility: Visibility,
    pub ident: Ident,
    pub underlying_type: Type,
    pub range: ExprRange,
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

impl Field {
    pub fn to_open_range(&self, max_open_end: usize) -> Result<Range<usize>> {
        let parse = |expr: Box<Expr>| {
            if let Expr::Lit(ref expr) = *expr {
                if let Lit::Int(ref literal) = expr.lit {
                    return literal.base10_parse();
                }
            }
            Err(Error::new(expr.span(), "Bitfield expected integer literal"))
        };

        let start = self.range.from.clone().map_or_else(|| Ok(0), parse)?;
        let mut end = self
            .range
            .to
            .clone()
            .map_or_else(|| Ok(max_open_end), parse)?;

        if let RangeLimits::Closed(_) = self.range.limits {
            end += 1
        }

        if start >= end || end > max_open_end {
            return Err(Error::new(self.range.span(), "Bitfield invalid range"));
        }

        Ok(Range { start, end })
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
