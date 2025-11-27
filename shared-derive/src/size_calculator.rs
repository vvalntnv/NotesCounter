// ============================================================================
// SIZE CALCULATOR HELPER MODULE
// ============================================================================
//
// This module helps calculate the size of different Rust types in bytes.
// You can use this to organize your size calculation logic.
//
// ============================================================================

use quote::quote;
use syn::{Field, GenericArgument, LitInt, PathArguments, Type};

static UTF_8_CHAR_LEN: u8 = 4;

/// Returns the size in bytes for a given type string
///
/// TODO: Implement this function!
///
/// This function takes a type name (like "u64" or "String") and returns
/// how many bytes it takes up in a Solana account.
pub fn get_type_size(field: &Field) -> Result<usize, syn::Error> {
    let field_type = &field.ty;
    let type_name = quote! { #field_type }.to_string();

    match type_name.as_str() {
        // Integer types
        "u8" | "i8" => Ok(1),
        "u16" | "i16" => Ok(2),
        "u32" | "i32" => Ok(4),
        "u64" | "i64" => Ok(8),
        "u128" | "i128" => Ok(16),

        // Boolean
        "bool" => Ok(1),

        // Solana specific types
        "Pubkey" => Ok(32), // Solana public keys are 32 bytes

        "String" => calculate_string_size(&field),
        "Vec" => calculate_vec_size(&field),

        // Some types to consider:
        // - Option<T> (1 byte discriminator + T size)
        // - Custom structs (would need recursive calculation)

        // If we don't recognize the type, return None
        _ => Err(syn::Error::new_spanned(&field, "Unrecognized type!")),
    }
}

/// Calculates size for String types
///
///
/// In Anchor, Strings are typically stored with:
/// - 4 bytes for the length
/// - N bytes for the actual string data (max length)
///
/// You might want to read an attribute like #[max_len = 100] from the field
pub fn calculate_string_size(field: &Field) -> Result<usize, syn::Error> {
    let size = get_complex_attr_size(field)?.expect("No size passed!");

    Ok(4 + (size * UTF_8_CHAR_LEN as usize))
}

/// Calculates size for Vec types
///
///
/// Similar to String, Vec needs:
/// - 4 bytes for the length
/// - size_of(T) * max_count bytes for the data
pub fn calculate_vec_size(field: &syn::Field) -> Result<usize, syn::Error> {
    let size = get_complex_attr_size(field)?.expect("No size passed!");
    let inner_type = extract_vec_inner(&field.ty).expect("No inner type provided!");
    let inner_type_string = quote! { #inner_type }.to_string();

    let inner_type_size = match inner_type_string.as_str() {
        "u8" | "i8" => Some(1),
        "u16" | "i16" => Some(2),
        "u32" | "i32" => Some(4),
        "u64" | "i64" => Some(8),
        "u128" | "i128" => Some(16),

        // Boolean
        "bool" => Some(1),

        // Solana specific types
        "Pubkey" => Some(32), // Solana public keys are 32 bytes
        _ => None,
    };

    if let Some(s) = inner_type_size {
        Ok(4 + (s * size))
    } else {
        Err(syn::Error::new_spanned(
            &field,
            "Unsupported Inner Type of the vector",
        ))
    }
}

/// Calculates size for Option types
///
/// TODO: Implement this if you want to support Option fields!
///
/// Option<T> in Anchor is typically:
/// - 1 byte for Some/None discriminator
/// - size_of(T) for the value if Some
pub fn _calculate_option_size(_field: &syn::Field, _inner_type: &str) -> Result<usize, syn::Error> {
    // TODO: Implement this!

    // You'll need to:
    // 1. Get the size of the inner type
    // 2. Return 1 + inner_size

    panic!("Options not yet supported!");
    // Ok(1)
}

fn get_complex_attr_size(field: &Field) -> Result<Option<usize>, syn::Error> {
    let mut attr_size: Option<usize> = None;

    for attr in &field.attrs {
        if !attr.path().is_ident("size") {
            continue;
        }

        let size: LitInt = attr.parse_args()?;
        attr_size = Some(size.base10_parse::<usize>()?);

        // attr.parse_nested_meta(|meta| {
        //     let content;
        //     parenthesized!(content in meta.input);
        //     let size: LitInt = content.parse()?;
        //     attr_size = Some(size.base10_parse::<usize>()?);
        //
        //     Ok(())
        // })?;
    }

    Ok(attr_size)
}

fn extract_vec_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        let segment = tp.path.segments.first()?;

        if segment.ident == "Vec" {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    return Some(inner_ty);
                }
            }
        }
    }
    None
}
