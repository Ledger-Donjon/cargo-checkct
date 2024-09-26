use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn checkct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    let name = &item_fn.sig.ident;

    let entrypoint_descriptor_name = Ident::new(
        &format!("__checkct_entrypoint_descriptor__{name}"),
        Span::call_site(),
    );

    quote! {
        // This variable is used to prevent the compiler from removing the code of the entrypoint
        // function if it is unused. Moreover this variable name has a special format which allows
        // cargo-checkct to find it.
        // Make sure that the section .note.checkct is kept by the linker.
        #[link_section = ".note.checkct"]
        #[used]
        pub static #entrypoint_descriptor_name: fn() = #name;

        #item_fn
    }
    .into()
}
