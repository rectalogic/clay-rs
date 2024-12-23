use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_attribute]
pub fn packed_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input enum
    let input_enum = parse_macro_input!(item as ItemEnum);
    let enum_name = &input_enum.ident;
    let enum_vis = &input_enum.vis;
    let enum_variants = &input_enum.variants;
    let enum_attrs = &input_enum.attrs;

    // Generate code for non-Windows platforms
    let generated_enum = quote! {
        #[cfg(not(target_os = "windows"))]
        #[repr(u8)]
        #(#enum_attrs)*
        #enum_vis enum #enum_name {
            #enum_variants
        }

        #[cfg(target_os = "windows")]
        #[repr(u32)]
        #(#enum_attrs)*
        #enum_vis enum #enum_name {
            #enum_variants
        }
    };

    generated_enum.into()
}
