# Spacy Derive Macro - Usage Examples

## What is this?

The `#[derive(Spacy)]` macro automatically implements the `Spacy` trait for your structs, calculating the size of the struct in bytes. This is useful for Solana programs where you need to know account sizes.

## Basic Usage

### Simple Struct

```rust
use shared::Spacy;

#[derive(Spacy)]
struct SimpleAccount {
    counter: u64,      // 8 bytes
    is_active: bool,   // 1 byte
}

// After implementing the macro, this will automatically have:
// impl Spacy for SimpleAccount {
//     const SIZE: usize = 9;  // 8 + 1
// }

// You can then use it like:
// let size = SimpleAccount::SIZE;  // Returns 9
```

### With Anchor Account

```rust
use anchor_lang::prelude::*;
use shared::Spacy;

#[account]
#[derive(Spacy)]
pub struct NoteData {
    pub content: String,      // TODO: You'll need to handle this!
    pub last_edited: i64,     // 8 bytes
    pub note_id: u64,         // 8 bytes
}

// The SIZE constant will be available:
// NoteData::SIZE
```

## How to Implement the Macro (Your TODO)

Right now, the macro returns 0 for everything because it's not implemented yet. Here's what you need to do:

### Step 1: Handle Basic Types

In `shared-derive/src/lib.rs`, in the `calculate_struct_size` function, you need to:

1. Iterate through the struct fields
2. For each field, get its type
3. Match on the type name to determine size
4. Sum up all the sizes

Example approach:

```rust
Fields::Named(fields) => {
    let mut total_size = 0;

    for field in &fields.named {
        let field_type = &field.ty;
        let type_string = quote!(#field_type).to_string();

        let size = match type_string.as_str() {
            "u8" | "i8" | "bool" => 1,
            "u16" | "i16" => 2,
            "u32" | "i32" => 4,
            "u64" | "i64" => 8,
            "u128" | "i128" => 16,
            "Pubkey" => 32,
            _ => panic!("Unsupported type: {}", type_string),
        };

        total_size += size;
    }

    quote! { #total_size }
}
```

### Step 2: Test It

Create a test struct in one of your programs:

```rust
use shared::Spacy;

#[derive(Spacy)]
struct TestStruct {
    value: u64,    // 8 bytes
    flag: bool,    // 1 byte
}

// Try building - if it compiles, check the size:
// In a test or instruction:
// msg!("TestStruct size: {}", TestStruct::SIZE);
// Should print: TestStruct size: 9
```

### Step 3: Handle Complex Types (Advanced)

Once basic types work, you can tackle:

#### Strings

Strings in Anchor are stored as:
- 4 bytes for length
- N bytes for content (you need to know max length)

You might want to use an attribute:

```rust
#[derive(Spacy)]
struct WithString {
    #[max_len = 100]
    content: String,  // Should calculate as 4 + 100 = 104 bytes
}
```

To implement this:
1. Look at `field.attrs` to find attributes
2. Parse the `max_len` value
3. Calculate size as 4 + max_len

#### Vectors

Similar to String:

```rust
#[derive(Spacy)]
struct WithVec {
    #[max_len = 10]
    items: Vec<u64>,  // Should be 4 + (8 * 10) = 84 bytes
}
```

#### Options

Option adds a 1-byte discriminator:

```rust
#[derive(Spacy)]
struct WithOption {
    maybe_value: Option<u64>,  // Should be 1 + 8 = 9 bytes
}
```

## Testing Your Implementation

### Test 1: Basic Types
```rust
#[derive(Spacy)]
struct Test1 {
    a: u8,      // 1
    b: u16,     // 2
    c: u32,     // 4
    d: u64,     // 8
}
// Expected SIZE: 15
```

### Test 2: With Pubkey
```rust
use anchor_lang::prelude::*;

#[derive(Spacy)]
struct Test2 {
    owner: Pubkey,   // 32
    amount: u64,     // 8
}
// Expected SIZE: 40
```

### Test 3: Your NoteData
```rust
#[derive(Spacy)]
pub struct NoteData {
    pub content: String,      // Depends on your implementation
    pub last_edited: i64,     // 8
    pub note_id: u64,         // 8
}
// Expected SIZE: Depends on how you handle String
```

## Debugging Tips

### See Generated Code

Add this to your macro to print what code it generates:

```rust
let expanded = quote! { ... };
eprintln!("Generated code:\n{}", expanded);
TokenStream::from(expanded)
```

This will show you exactly what the macro is outputting during compilation.

### Check Field Types

Add debug prints in your size calculation:

```rust
for field in &fields.named {
    let type_string = quote!(#field_ty).to_string();
    eprintln!("Field type: {}", type_string);
}
```

### Compile Errors

If the macro generates invalid code, the compiler will tell you. Look at the error message carefully - it will show what code failed to compile.

## Common Solana/Anchor Type Sizes Reference

| Type | Size (bytes) | Notes |
|------|--------------|-------|
| u8, i8 | 1 | |
| bool | 1 | |
| u16, i16 | 2 | |
| u32, i32 | 4 | |
| u64, i64 | 8 | |
| u128, i128 | 16 | |
| Pubkey | 32 | Solana public key |
| String | 4 + max_len | Length prefix + content |
| Vec&lt;T&gt; | 4 + (T * max_len) | Length prefix + elements |
| Option&lt;T&gt; | 1 + T | Discriminator + value |
| Account discriminator | 8 | Anchor adds this automatically |

## Next Steps

1. Implement basic type matching in `calculate_struct_size`
2. Test with simple structs
3. Try it on `NoteData`
4. (Optional) Add support for String, Vec, Option
5. (Optional) Add attribute parsing for max_len

Good luck! Remember: proc macros run at compile time, so any panics or prints will show during `cargo build`.
