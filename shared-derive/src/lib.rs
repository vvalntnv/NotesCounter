// ============================================================================
// PROCEDURAL MACRO GUIDE: Spacy Derive Macro
// ============================================================================
//
// This is a derive macro that automatically implements the Spacy trait.
//
// WHAT IS A PROC MACRO?
// A procedural macro is Rust code that runs at compile time and generates
// more Rust code. It's like a function that takes code as input and outputs
// new code.
//
// HOW THIS WORKS:
// 1. User writes: #[derive(Spacy)]
// 2. Compiler calls our macro with the struct definition
// 3. Our macro analyzes the struct and calculates sizes
// 4. Our macro generates: impl Spacy for StructName { const SIZE: usize = ...; }
// 5. Compiler continues with the generated code
//
// ============================================================================

// Helper module for calculating type sizes
mod size_calculator;

// This import is special - it's provided by the Rust compiler for proc macros
use proc_macro::TokenStream;

// These imports help us work with Rust syntax
use quote::quote; // Helps generate Rust code
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields};

use size_calculator::get_type_size; // Helps parse Rust code

// ============================================================================
// THE MAIN MACRO FUNCTION
// ============================================================================

/// This is the derive macro that users will use like: #[derive(Spacy)]
///
/// The #[proc_macro_derive(Spacy)] attribute tells Rust:
/// "This function implements a derive macro called Spacy"
#[proc_macro_derive(Spacy, attributes(size))]
pub fn derive_spacy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match input.data {
        syn::Data::Struct(ref s) => s,
        _ => {
            return syn::Error::new_spanned(&input, "Spacy can only be derived by structs")
                .to_compile_error()
                .into()
        }
    };

    let size = match calculate_struct_size(data) {
        Ok(size) => size,
        Err(err) => return err.to_compile_error().into(),
    };

    let expanded = quote! {
        impl shared::traits::Spacy for #name {
            const SIZE: usize = #size;
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// SIZE CALCULATION LOGIC
// ============================================================================

/// Calculates the total size of a struct in bytes
///
/// TODO: You need to implement this function!
///
/// HINTS:
/// 1. The `data` parameter contains information about the struct
/// 2. You need to match on whether it's a struct, enum, or union
/// 3. For structs, you need to iterate through the fields
/// 4. For each field, you need to determine its type and size
///
/// COMMON SOLANA/ANCHOR SIZES:
/// - u8, i8: 1 byte
/// - u16, i16: 2 bytes
/// - u32, i32: 4 bytes
/// - u64, i64: 8 bytes
/// - u128, i128: 16 bytes
/// - bool: 1 byte
/// - Pubkey: 32 bytes
/// - String: 4 bytes (length) + actual string bytes (you might need length annotations)
/// - Vec<T>: 4 bytes (length) + (T size * number of elements)
/// - Account discriminator: 8 bytes (Anchor adds this automatically)
fn calculate_struct_size(data: &DataStruct) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut total_size = 8; // 8 bytes for the address
    match &data.fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let size = get_type_size(&field)?;

                total_size += size;
            }
            Ok(quote! { #total_size })
        }

        Fields::Unnamed(_) => {
            // TODO: Handle tuple structs if needed
            return Err(syn::Error::new_spanned(
                &data.fields,
                "Tuple structs are not yet supported",
            ));
        }

        Fields::Unit => {
            // Unit structs have no fields, so size is 0
            Ok(quote! { 0 })
        }
    }
}
