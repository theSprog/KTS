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
    let mut childs = Vec::new();
    let mut draw_pairs = Vec::new();

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
                    let vec = String::from("Vec");
                    let opt = String::from("Option");
                    for segment in segments.iter() {
                        let cur_type = format!("{}", &segment.ident);
                        if cur_type.eq(&vec) || cur_type.eq(&opt) {
                            draw_pairs
                                .push((field.ident.as_ref().unwrap(), Some(format_ident!("id"))));
                        } else {
                            draw_pairs.push((field.ident.as_ref().unwrap(), None));

                            childs.push(field.ident.as_ref().unwrap());
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    let struct_name_str = format!("{}", ident);

    let mut fields = Vec::new();
    let mut pass_id = Vec::new();
    for (field, id) in draw_pairs {
        println!("{}: {:#?}", field, id);
        fields.push(field);
        pass_id.push(id);
    }

    quote! {
        impl Visualizable for #ident {
            fn draw(&self, id: usize) {
                AST_GRAPH::put_node(id, #struct_name_str);

                // // call pairs
                // #(
                //     self.#call_pairs_fields.draw();
                //     AST_GRAPH::put_edge(id, self.#call_pairs_fields.id);
                // )*

                // // draw pairs
                // #(
                //     self.#draw_pairs_fields.draw(id);
                // )*

                #(
                    self.#fields.draw(#pass_id);
                )*

                #(
                    AST_GRAPH::put_edge(id, self.#childs.id);
                )*
            }
        }
    }
    .into()
}

fn draw_func(id: Option<Ident>, field: &Ident) -> TokenStream {
    match id {
        Some(id) => quote! {
            self.#field.draw(id);
        }
        .into(),
        None => quote! {
            self.#field.draw();
        }
        .into(),
    }
}

fn derive_enum(input: &syn::DataEnum, ident: &Ident) -> TokenStream {
    quote!(
        impl Visualizable for #ident {
            fn draw(&self, id: usize) {
                match self {
                    Stat::ImportStat(import_stat) => {
                        AST_GRAPH::put_edge(id, import_stat.id);
                        import_stat.draw();
                    }
                    Stat::Unknown(unknow) => {
                        unknow.draw();
                        AST_GRAPH::put_edge(id, unknow.id);
                    }
                }
            }
        }
    )
    .into()
}

/*
impl Visualizable for Stat {
    fn draw(&self, id: usize) {
        AST_GRAPH::put_node(id, "Stat");

        match self {
            Stat::ImportStat(import_stat) => {
                AST_GRAPH::put_edge(id, import_stat.id);
                import_stat.draw();
            }
            Stat::ExportStat(export_stat) => todo!(),
            Stat::EmptyStat(empty_stat) => todo!(),
            Stat::Block(block) => todo!(),
            Stat::FuncDecl(func_decl) => {
                AST_GRAPH::put_edge(id, func_decl.id);
                func_decl.draw();
            }
            Stat::FuncExpDecl(func_exp_decl) => todo!(),
            Stat::GenFuncDecl(gen_func_decl) => todo!(),

            Stat::Unknown => AST_GRAPH::put_node(id, "StatUnknown"),
        }
    }
}

*/

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
