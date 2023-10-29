use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    spanned::Spanned,
    DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident, Result, Token, Type,
};

pub struct TypePair {
    pub left: Type,
    pub right: Option<Type>,
}

impl Parse for TypePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let left = Type::Path(input.parse()?);

        let right: Option<Type> = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { left, right })
    }
}

impl TypePair {
    pub fn gen_impl(self, name: &Ident, data: &DataStruct, action: Action) -> TokenStream {
        let left: Ident = parse_quote!(left);
        let right: Ident = parse_quote!(right);

        let producer = self.producer(data, action, &left, &right);

        let left = self.left;
        let right = self.right.unwrap_or_else(|| left.clone());

        let trait_ty = action.trait_ty();
        let trait_method = action.trait_method();

        quote! {
            impl #trait_ty <#left, #right> for #name {
                fn #trait_method (left: #left, right: #right) -> Self {
                    #producer
                }
            }
        }
    }

    fn producer(
        &self,
        data: &DataStruct,
        action: Action,
        left: &Ident,
        right: &Ident,
    ) -> TokenStream {
        let trait_ty = action.trait_ty();
        let trait_method = action.trait_method();

        match &data.fields {
            Fields::Unit => quote!(Self),
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields: Vec<_> = named
                    .iter()
                    .map(|field| {
                        let ident = field.ident.as_ref().unwrap();
                        quote_spanned! {field.span()=>
                            #ident: #trait_ty::#trait_method(#left.#ident, #right.#ident)
                        }
                    })
                    .collect();

                quote! {
                    Self { #(#fields),* }
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let fields: Vec<_> = unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let span = field.span();
                        let index = syn::Index {
                            index: i as u32,
                            span,
                        };

                        quote_spanned! {span=>
                            #trait_ty::#trait_method(#left.#index, #right.#index)
                        }
                    })
                    .collect();

                quote! {
                    Self(#(#fields),*)
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Layer,
    Merge,
}

impl Action {
    fn trait_ty(&self) -> Type {
        use Action::*;
        match self {
            Layer => parse_quote!(type_weave::Layer),
            Merge => parse_quote!(type_weave::Merge),
        }
    }

    fn trait_method(&self) -> Ident {
        use Action::*;
        match self {
            Layer => parse_quote!(into_layered),
            Merge => parse_quote!(into_merged),
        }
    }
}
