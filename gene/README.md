# Gene Library

Rust Library containing functions for modifying Gene Sequences represented as char sequences or binary sequences.
This Gene representation is in its own library due to multiple binary executables using these functions.

## Saving memory

Optimize on space by converting each DNA base to a 2 bit binary numbers. This is because, a 2-bit binary 
number can represent four unique values, one for each DNA base. The following table show the encoding
we will use for the four bases (assuming lowercase for A, T, C, G). We have included the Unicode values
so we can see that it takes 8 times more space!

| DNA Base | 2-bit binary | Unicode (decimal) | 16-bit Unicode      |
| -------- | ------------ | --------------    | ------------------- |
| A        | 00           |  97               | 0000 0000 0110 0001 |
| T        | 11           | 116               | 0000 0000 0111 0100 |
| C        | 01           | 100               | 0000 0000 0110 0100 |
| G        | 10           | 103               | 0000 0000 0110 0111 |

Note that we have made the binary representations for complementary bases be binary complements
as well. For example, complement of the base `A` is `T` --- the complement of `00` is `11`.

With this compact representation, we can store a 31 length subsequence in a 64-bit unsigned int
`u64` primitive type in Rust.

## Docs

Rust Documents can be generated and viewed with `cargo doc --open`.  Generate these docs if you wish to
view the public functions of the BTree Struct
