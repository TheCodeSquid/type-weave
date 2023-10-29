mod parse;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use parse::{Action::*, TypeMap};

macro_rules! unwrap {
    ($result:expr) => {{
        let res: syn::Result<_> = $result;
        match res {
            Ok(value) => value,
            Err(err) => return err.into_compile_error().into(),
        }
    }};
}

#[proc_macro_derive(Weave, attributes(layer, merge))]
pub fn weave(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let Data::Struct(data) = input.data else {
        return quote! { compile_error!("currently only implemented for structs") }.into();
    };

    let mut layer_maps: Vec<TypeMap> = vec![];
    let mut merge_maps: Vec<TypeMap> = vec![];

    for attr in input.attrs {
        if attr.path().is_ident("layer") {
            layer_maps.push(unwrap!(attr.parse_args()));
        } else if attr.path().is_ident("merge") {
            merge_maps.push(unwrap!(attr.parse_args()));
        }
    }

    let mut impls = vec![];
    for map in layer_maps {
        impls.push(map.gen_impl(&name, &data, Layer));
    }
    for map in merge_maps {
        impls.push(map.gen_impl(&name, &data, Merge));
    }

    quote! { #(#impls)* }.into()
}
