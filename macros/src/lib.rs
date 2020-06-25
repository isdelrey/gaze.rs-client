mod message;

use proc_macro::TokenStream;
use quote::quote;


#[proc_macro_attribute]
pub fn message_type(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let metadata = syn::parse_macro_input!(metadata as syn::ExprLit);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();
    let type_name = metadata.lit;

    println!("{:?}", type_name);

    let output = quote!{
        #[derive(Serialize, Debug)]
        #input

        impl #name {
            fn filter(filter: serde_json::Value) -> serde_json::Value {
                filter
            }
        }

        impl WithMessageType for #name {
            fn get_message_type(&self) -> String {
                String::from(#type_name)
            }
        }
    };

    TokenStream::from(output)
}