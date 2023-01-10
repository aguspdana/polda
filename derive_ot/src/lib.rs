use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DataUnion;
use syn::DeriveInput;
use syn::Fields;
use syn::FieldsNamed;
use syn::FieldsUnnamed;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_transformable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_transformable(&ast)
}

fn impl_transformable(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;

    let body = match &ast.data {
        Data::Struct(s) => try_apply_for_struct(s),
        Data::Enum(e) => try_apply_for_enum(e),
        Data::Union(u) => try_apply_for_union(u)
    };

    let expanded = quote! {
        impl Transformable for #ident {
            fn try_apply(&mut self, op: Operation) -> Result<Option<Operation>, Error> {
                #body
            }
        }
    };

    expanded.into()
}

fn try_apply_for_struct(data: &DataStruct) -> TokenStream2 {
    match &data.fields {
        Fields::Named(n) => todo!(),
        Fields::Unnamed(u) => todo!(),
        Fields::Unit => todo!()
    }
}

fn try_apply_for_named_struct(fields: &FieldsNamed) -> TokenStream2 {
    let f = fields.named.iter();
    quote!{}
}

fn try_apply_for_unnamed_struct(fields: &FieldsUnnamed) -> TokenStream2 {
    quote!{}
}

fn try_apply_for_unit_struct() -> TokenStream2 {
    quote!{}
}

fn try_apply_for_enum(data: &DataEnum) -> TokenStream2 {
    todo!()
}

fn try_apply_for_union(data: &DataUnion) -> TokenStream2 {
    todo!()
}
