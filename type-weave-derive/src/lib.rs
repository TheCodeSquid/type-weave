use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields,
    FieldsNamed, FieldsUnnamed, GenericParam, Generics, Ident, Index, Result,
};

macro_rules! unwrap {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

/// Implement `Weave` for a struct whose fields all implement the trait.
#[proc_macro_derive(Weave)]
pub fn weave(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = unwrap!(struct_data(input.data));

    let generics = add_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let other = Ident::new("other", Span::call_site());
    let over = producer("over", &other, &data.fields);
    let under = producer("under", &other, &data.fields);

    quote! {
        impl #impl_generics type_weave::Weave for #name #ty_generics #where_clause {
            fn over(self, #other: Self) -> Self {
                #over
            }

            fn under(self, #other: Self) -> Self {
                #under
            }
        }
    }
    .into()
}

fn struct_data(data: Data) -> Result<DataStruct> {
    let err = "Weave derive macro only supports structs";
    match data {
        Data::Struct(data) => Ok(data),
        Data::Enum(data) => Err(Error::new(data.enum_token.span(), err)),
        Data::Union(data) => Err(Error::new(data.union_token.span(), err)),
    }
}

fn producer(method: &str, other: &Ident, fields: &Fields) -> TokenStream {
    let method = Ident::new(method, Span::call_site());
    match fields {
        Fields::Unit => quote! { Self },
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields: Vec<_> = named
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    quote_spanned! {field.span()=>
                        #ident: type_weave::Weave::#method(self.#ident, #other.#ident)
                    }
                })
                .collect();
            quote! { Self { #(#fields,)* } }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let fields: Vec<_> = unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let index = Index {
                        index: i as u32,
                        span: field.span(),
                    };
                    quote_spanned! {field.span()=>
                        type_weave::Weave::#method(self.#index, #other.#index)
                    }
                })
                .collect();
            quote! { Self(#(#fields),*) }
        }
    }
}

fn add_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(parse_quote!(type_weave::Weave));
        }
    }
    generics
}
