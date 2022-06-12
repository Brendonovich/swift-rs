use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item, ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn swift_object(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let struct_input: proc_macro2::TokenStream = input.clone().into();
    let input = parse_macro_input!(input as Item);

    let impls = match input {
        Item::Struct(def) => swift_object_impl(def),
        _ => panic!("'swift_object' can only be applied to structs"),
    };

    quote! {
        #[repr(C)]
        #struct_input
        #impls
    }
    .into()
}

fn swift_object_impl(struct_def: ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = struct_def.ident;

    quote! {
        unsafe impl swift_rs::SwiftRet for #struct_name {
            type SwiftType = swift_rs::SRObject<#struct_name>;

            fn __retain(v: &Self::SwiftType) {
                v.__retain();
            }
        }
    }
}
