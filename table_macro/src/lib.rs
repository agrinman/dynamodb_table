use proc_macro::TokenStream;
use quote::{quote};
use syn::{self, DeriveInput};

#[proc_macro_derive(
    DynamodbTable,
    attributes(
        dynamodb_table_name,
        dynamodb_table_name_prefix,
        dynamodb_table_inherit_from
    )
)]
pub fn derive_dynamo_table(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let result = impl_dynamodb_table(&ast);
    proc_macro::TokenStream::from(result)
}

fn impl_dynamodb_table(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    match ast.data {
        syn::Data::Struct(_) => {}
        _ => {
            panic!("#[derive(DynamodbTable)] is only defined for structs.");
        }
    };

    if let Some(syn::Meta::Path(inherit)) =
        get_attr::<syn::Meta>("dynamodb_table_inherit_from", ast)
    {
        return quote! {
            impl DynamodbTable for #name {
                fn table_name() -> String {
                    #inherit::table_name()
                }
            }
        };
    }

    let table = match get_attr::<syn::LitStr>("dynamodb_table_name", ast) {
        Some(name) => quote! { #name.to_string() },
        None => {
            let name = name.to_string().to_lowercase();

            if let Some(prefix) = get_attr::<syn::LitStr>("dynamodb_table_name_prefix", ast) {
                quote! {
                    format!("{}{}", #prefix, #name)
                }
            } else {
                quote! { #name.to_string() }
            }
        }
    };

    quote! {
        impl DynamodbTable for #name {
            fn table_name() -> String {
                #table
            }
        }
    }
}

fn get_attr<P: syn::parse::Parse>(named: &'static str, ast: &syn::DeriveInput) -> Option<P> {
    let attr = ast.attrs.iter().filter(|a| a.path.is_ident(named)).next()?;

    attr.parse_args::<P>()
        .map(Some)
        .expect(&format!("expecting an attribute: #[{}(<value>)]", named))
}
