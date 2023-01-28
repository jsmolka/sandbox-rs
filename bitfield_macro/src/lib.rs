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
    let bitfield_type = &bitfield.type_;
    let bitfield_type_bytes = bitfield.data_type_bytes();

    let mut data_mask: u64 = 0;
    let mut fields = vec![];
    for field in bitfield.fields {
        let visibility = &field.visibility;
        let field_type = &field.value_type;
        let ident = &field.ident;
        let ident_set = format_ident!("set_{ident}");
        let return_type = &field.return_type();

        let set_value_type = match field_type.to_token_stream().to_string().as_str() {
            "bool" => quote! {
                let value = value != 0;
            },
            "u8" | "u16" | "u32" | "u64" | "usize" => quote! {
                let value = value as #field_type;
            },
            _ => {
                return Err(Error::new(
                    field_type.span(),
                    "Bitfield field type must be a bool or unsigned int",
                ))
            }
        };

        let set_value_modifier = if let Some(ref modifier) = field.modifier {
            quote! {
                let value = (#modifier)(value);
            }
        } else {
            quote! {}
        };

        let range = field.open_range(8 * bitfield_type_bytes)?;
        let mask = u64::MAX >> (u64::BITS as usize - range.len());
        let shift = range.start;

        data_mask |= mask << shift;

        fields.push(quote! {
            #visibility fn #ident(&self) -> #return_type {
                let mask = #mask as #bitfield_type;
                let shift = #shift as #bitfield_type;
                let value = (self.data >> shift) & mask;
                #set_value_type
                #set_value_modifier
                value
            }

            #visibility fn #ident_set(&mut self, value: #field_type) {
                let value = value as #bitfield_type;
                let mask = #mask as #bitfield_type;
                let shift = #shift as #bitfield_type;
                self.data = (self.data & !(mask << shift)) | ((value & mask) << shift);
            }
        });
    }

    let attributes = &bitfield.attributes;
    let visibility = &bitfield.visibility;
    let ident = &bitfield.ident;

    Ok(quote! {
        #(#attributes)*
        #[derive(Default, Clone, Copy, Debug)]
        #visibility struct #ident {
            data: #bitfield_type,
        }

        impl #ident {
            pub fn new(data: #bitfield_type) -> Self {
                Self { data: data & Self::data_mask() }
            }

            pub const fn data_mask() -> #bitfield_type {
                #data_mask as #bitfield_type
            }

            pub fn data(&self) -> #bitfield_type {
                self.data
            }

            pub fn set_data(&mut self, data: #bitfield_type) {
                self.data = data & Self::data_mask();
            }

            pub fn byte(&self, index: usize) -> u8 {
                assert!(index < #bitfield_type_bytes);
                (self.data >> (8 * index)) as u8
            }

            pub fn set_byte(&mut self, index: usize, byte: u8) {
                assert!(index < #bitfield_type_bytes);
                let mask = (Self::data_mask() >> (8 * index)) as u8;
                let data = ((byte & mask) as #bitfield_type) << (8 * index);
                self.data = (self.data & !(0xFF << (8 * index))) | data;
            }

            #(#fields)*
        }

        impl From<#bitfield_type> for #ident {
            fn from(value: #bitfield_type) -> Self {
                #ident::new(value)
            }
        }

        impl From<#ident> for #bitfield_type {
            fn from(value: #ident) -> Self {
                value.data()
            }
        }
    })
}

struct Field {
    pub visibility: Visibility,
    pub ident: Ident,
    pub value_type: Type,
    pub range: ExprRange,
    pub modifier: Option<ExprClosure>,
}

impl Field {
    pub fn return_type(&self) -> &Type {
        if let Some(modifier) = &self.modifier {
            if let ReturnType::Type(_, ty) = &modifier.output {
                return ty;
            }
        }
        &self.value_type
    }

    pub fn open_range(&self, max_end: usize) -> Result<Range<usize>> {
        let parse = |expr: &Expr| {
            if let Expr::Lit(expr) = expr {
                if let Lit::Int(literal) = &expr.lit {
                    return literal.base10_parse();
                }
            }
            Err(Error::new(expr.span(), "Bitfield expected integer literal"))
        };

        let start = self
            .range
            .from
            .as_ref()
            .map_or_else(|| Ok(0), |expr| parse(expr))?;

        let mut end = self
            .range
            .to
            .as_ref()
            .map_or_else(|| Ok(max_end), |expr| parse(expr))?;

        if let RangeLimits::Closed(_) = self.range.limits {
            end += 1
        }

        if start >= end || end > max_end {
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
        let value_type = input.parse()?;
        let _: Token![@] = input.parse()?;
        let range = input.parse()?;
        let modifier = if input.parse::<Token![=>]>().is_ok() {
            Some(input.parse::<ExprClosure>()?)
        } else {
            None
        };

        Ok(Field {
            visibility,
            ident,
            value_type,
            range,
            modifier,
        })
    }
}

struct Bitfield {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub type_: Type,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Bitfield {
    pub fn data_type_bytes(&self) -> usize {
        use std::mem::size_of;
        match self.type_.to_token_stream().to_string().as_str() {
            "u8" => size_of::<u8>(),
            "u16" => size_of::<u16>(),
            "u32" => size_of::<u32>(),
            "u64" => size_of::<u64>(),
            "usize" => size_of::<usize>(),
            _ => unreachable!(),
        }
    }
}

impl Parse for Bitfield {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let type_: Type = input.parse()?;
        match type_.to_token_stream().to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" | "usize" => (),
            _ => {
                return Err(Error::new(
                    type_.span(),
                    "Bitfield type must be an unsigned int",
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
            type_,
            fields,
        })
    }
}
