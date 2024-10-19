extern crate proc_macro;
use proc_macro::TokenStream;

use syn::DeriveInput;
use quote::quote;
#[proc_macro_derive(StateEditor)]
pub fn derive_state_editor(_input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(_input).unwrap();
    let ident = ast.ident;
    let tokens = quote! {
        impl StateEditor for #ident{
            fn register_to_editor(){
                add_to_registry(stringify!(#ident));
            }
        }

        #[ctor::ctor]
        fn register_class() {
            #ident::register_to_editor();
        }
    };

    tokens.into()
}