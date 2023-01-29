use proc_macro2::TokenStream;
use quote::*;
use std::ops::Range;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

#[proc_macro]
pub fn bitfield(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    make_bitfield(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn make_bitfield(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let bitfield = parse::<Bitfield>(input)?;
    let data_type = &bitfield.ty;
    let data_type_size = type_size(data_type);

    let mut functions = vec![];
    for field in bitfield.fields.iter() {
        let field_type = &field.ty;

        let cast = match field_type.to_token_stream().to_string().as_str() {
            "bool" => quote! {
                let value = value != 0;
            },
            "u8" | "u16" | "u32" | "u64" | "usize" => quote! {
                let value = value as #field_type;
            },
            _ => unreachable!(),
        };

        let pipe = if let Some(pipe) = &field.pipe {
            quote! {
                let value = (#pipe)(value);
            }
        } else {
            quote! {}
        };

        let range = &field.range;
        if !(range.start < range.end
            && range.end <= type_bits(data_type)
            && range.len() <= type_bits(field_type))
        {
            return Err(Error::new(
                field.range_expr.span(),
                "Bitfield range is invalid",
            ));
        }

        let mask = {
            let value = field.mask();
            quote! {
                (#value as #data_type)
            }
        };

        let shift = {
            let value = field.shift();
            quote! {
                (#value as #data_type)
            }
        };

        let visibility = &field.visibility;
        let ident = &field.ident;
        let ident_set = format_ident!("set_{ident}");
        let return_type = field.return_type();

        functions.push(quote! {
            #visibility fn #ident(&self) -> #return_type {
                let value = (self.data >> #shift) & #mask;
                #cast
                #pipe
                value
            }

            #visibility fn #ident_set(&mut self, value: #field_type) {
                let value = value as #data_type;
                self.data = (self.data & !(#mask << #shift)) | ((value & #mask) << #shift);
            }
        });
    }

    let mut data_mask = 0;
    for field in bitfield.fields.iter() {
        data_mask |= field.mask() << field.shift();
    }

    let attributes = &bitfield.attributes;
    let visibility = &bitfield.visibility;
    let ident = &bitfield.ident;

    Ok(quote! {
        #(#attributes)*
        #[derive(Default, Clone, Copy, Debug)]
        #visibility struct #ident {
            data: #data_type,
        }

        impl #ident {
            pub fn new(data: #data_type) -> Self {
                Self { data: data & Self::data_mask() }
            }

            #visibility const fn data_mask() -> #data_type {
                #data_mask as #data_type
            }

            #visibility fn data(&self) -> #data_type {
                self.data
            }

            #visibility fn set_data(&mut self, data: #data_type) {
                self.data = data & Self::data_mask();
            }

            #visibility fn byte(&self, index: usize) -> u8 {
                assert!(index < #data_type_size);
                (self.data >> (8 * index)) as u8
            }

            #visibility fn set_byte(&mut self, index: usize, byte: u8) {
                assert!(index < #data_type_size);
                let mask = (Self::data_mask() >> (8 * index)) as u8;
                let data = ((byte & mask) as #data_type) << (8 * index);
                self.data = (self.data & !(0xFF << (8 * index))) | data;
            }

            #(#functions)*
        }

        impl From<#data_type> for #ident {
            fn from(value: #data_type) -> Self {
                #ident::new(value)
            }
        }

        impl From<#ident> for #data_type {
            fn from(value: #ident) -> Self {
                value.data()
            }
        }
    })
}

fn type_size(ty: &Type) -> usize {
    use std::mem::size_of;
    match ty.to_token_stream().to_string().as_str() {
        "bool" => size_of::<bool>(),
        "u8" => size_of::<u8>(),
        "u16" => size_of::<u16>(),
        "u32" => size_of::<u32>(),
        "u64" => size_of::<u64>(),
        "usize" => size_of::<usize>(),
        _ => unreachable!(),
    }
}

fn type_bits(ty: &Type) -> usize {
    match ty.to_token_stream().to_string().as_str() {
        "bool" => 1,
        "u8" | "u16" | "u32" | "u64" | "usize" => 8 * type_size(ty),
        _ => unreachable!(),
    }
}

struct Bitfield {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub ty: Type,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for Bitfield {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;

        match ty.to_token_stream().to_string().as_str() {
            "u8" | "u16" | "u32" | "u64" | "usize" => (),
            _ => return Err(Error::new(ty.span(), "Bitfield type must be an unsigned")),
        };

        let content;
        braced!(content in input);
        let fields = content.parse_terminated(Field::parse)?;

        Ok(Bitfield {
            attributes,
            visibility,
            ident,
            ty,
            fields,
        })
    }
}

struct Field {
    pub visibility: Visibility,
    pub ident: Ident,
    pub ty: Type,
    pub range_expr: ExprRange,
    pub range: Range<usize>,
    pub pipe: Option<ExprClosure>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        let ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;
        let _: Token![@] = input.parse()?;
        let range_expr = input.parse()?;
        let range = parse_range(&range_expr)?;
        let pipe = if input.parse::<Token![=>]>().is_ok() {
            Some(input.parse::<ExprClosure>()?)
        } else {
            None
        };

        match ty.to_token_stream().to_string().as_str() {
            "bool" | "u8" | "u16" | "u32" | "u64" | "usize" => (),
            _ => {
                return Err(Error::new(
                    ty.span(),
                    "Bitfield field type must be an unsigned int or bool",
                ))
            }
        };

        Ok(Field {
            visibility,
            ident,
            ty,
            range_expr,
            range,
            pipe,
        })
    }
}

impl Field {
    pub fn return_type(&self) -> &Type {
        if let Some(pipe) = &self.pipe {
            if let ReturnType::Type(_, ty) = &pipe.output {
                return ty;
            }
        }
        &self.ty
    }

    pub fn mask(&self) -> u64 {
        u64::MAX >> (u64::BITS as usize - self.range.len())
    }

    pub fn shift(&self) -> u64 {
        self.range.start as u64
    }
}

fn parse_range(range: &ExprRange) -> Result<Range<usize>> {
    if matches!(range.limits, RangeLimits::Closed(_)) {
        return Err(Error::new(
            range.span(),
            "Bitfield expected half-open range",
        ));
    }

    let parse = |expr: &Expr| {
        if let Expr::Lit(expr) = expr {
            if let Lit::Int(literal) = &expr.lit {
                return literal.base10_parse();
            }
        }
        Err(Error::new(expr.span(), "Bitfield expected integer literal"))
    };

    let implicit_bounds_error = || {
        Err(Error::new(
            range.span(),
            "Bitfield expected explicit bounds",
        ))
    };

    Ok(Range {
        start: range
            .from
            .as_ref()
            .map_or_else(implicit_bounds_error, |expr| parse(expr))?,
        end: range
            .to
            .as_ref()
            .map_or_else(implicit_bounds_error, |expr| parse(expr))?,
    })
}
