use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Ident, ItemFn, Result, Token,
};

struct AttributeArgs {
    descriptor_link_section: Option<String>,
}

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                descriptor_link_section: None,
            });
        }

        let arg_name: Ident = input.parse()?;
        if arg_name != "descriptor_link_section" {
            return Err(Error::new(
                arg_name.span(),
                "expected `descriptor_link_section`",
            ));
        }
        input.parse::<Token![=]>()?;
        let descriptor_link_section: Literal = input.parse()?;

        Ok(Self {
            descriptor_link_section: Some(descriptor_link_section.to_string()),
        })
    }
}

#[proc_macro_attribute]
pub fn checkct(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    // By default the descriptor will be stored in the .note.checkct section
    let descriptor_link_section = Literal::string(
        args.descriptor_link_section
            .as_deref()
            .unwrap_or(".note.checkct"),
    );

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
        // Make sure that the descriptor link section is kept by the linker.
        #[link_section = #descriptor_link_section]
        #[used]
        pub static #entrypoint_descriptor_name: fn() = #name;

        #item_fn
    }
    .into()
}
