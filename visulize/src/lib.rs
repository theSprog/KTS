#![allow(warnings, unused)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Path, Type, TypePath};

use quote::{format_ident, quote};

#[proc_macro_derive(Visualizable)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(ref data) => derive_struct(data, &input.ident),
        Data::Enum(ref data) => derive_enum(data, &input.ident),
        Data::Union(_) => unreachable!(),
    }
}

fn derive_struct(input: &syn::DataStruct, ident: &Ident) -> TokenStream {
    // let mut draw_fields = Vec::new();
    // let mut call_fields = Vec::new();
    let mut fields_vec = Vec::new();
    match &input.fields {
        syn::Fields::Named(fields) => {
            for field in fields.named.iter() {
                if let Type::Path(TypePath {
                    qself: _,
                    path:
                        Path {
                            leading_colon: _,
                            segments,
                        },
                }) = &field.ty
                {
                    fields_vec.push(field.ident.as_ref().unwrap());
                }
            }
        }
        _ => unreachable!(),
    }

    let struct_name_str = format!("{}", ident);

    quote! {
        impl Visualizable for #ident {
            fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
                graph.put_node(self_info, #struct_name_str);

                #(
                    self.#fields_vec.draw(self_info, graph);
                )*
            }
        }
    }
    .into()
}

fn derive_enum(input: &syn::DataEnum, ident: &Ident) -> TokenStream {
    let enum_idents = (&input.variants)
        .into_iter()
        .map(|v| &v.ident)
        .collect::<Vec<_>>();
    let enum_snakes = enum_idents
        .iter()
        .map(|&i| snake(&i.to_string()))
        .map(|i| format_ident!("{}", i))
        .collect::<Vec<_>>();

    let enum_name_str = format!("{}", ident);

    quote!(
        impl Visualizable for #ident {
            fn draw(&self, self_info: NodeInfo, graph: &mut AstGraph) {
                graph.put_node(self_info, #enum_name_str);

                match self {
                    #(
                        #ident::#enum_idents(#enum_snakes) => {
                            #enum_snakes.draw(self_info, graph);
                        }
                    )*
                }
            }
        }
    )
    .into()
}

// 并不是严格的蛇形，在末尾还加了一个 _, 目的是防止和关键字例如 type 冲突
fn snake(input: &str) -> String {
    let mut out = String::default();
    let mut chars = input.chars();
    out.push(chars.next().unwrap().to_ascii_lowercase());
    while let Some(ch) = chars.next() {
        match ch.is_ascii_uppercase() {
            true => {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            }
            false => out.push(ch),
        }
    }
    out.push('_');
    out
}
