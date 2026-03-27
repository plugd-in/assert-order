use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Derive macro for implementing `VariantOrder` based on
/// the definition of an enum.
#[proc_macro_derive(VariantOrder)]
pub fn variant_order_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_def = match ast.data {
        Data::Enum(ref enum_def) => enum_def,
        Data::Union(union_def) => {
            return syn::Error::new_spanned(
                &union_def.union_token,
                "VariantOrder can only be applied to enums.",
            )
            .to_compile_error()
            .into();
        }
        Data::Struct(struct_def) => {
            return syn::Error::new_spanned(
                &struct_def.struct_token,
                "VariantOrder can only be applied to enums.",
            )
            .to_compile_error()
            .into();
        }
    };

    let variants = enum_def
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let variant_count = variants.len();

    let name = &ast.ident;
    let order_impl = quote! {
        impl VariantOrder for #name {
            fn order() -> &'static [&'static str] {
                static VARIANTS: [&'static str; #variant_count] = [
                    #(stringify!(#variants)),*
                ];

                return &VARIANTS;
            }
        }
    };

    order_impl.into()
}
