Cyptex128 - Project Context
Project Overview

Cyptex128 is a hashing and encryption system written in Rust designed to convert any input into a fixed 128-bit output. Its main goals are:

Simplicity – The algorithm is easy to understand and implement.

Speed – Optimized for performance in Rust.

Fixed 128-bit output – Every input, regardless of size, generates exactly 128 bits.

Optional reversible encryption – While hashes are normally one-way, a symmetric encrypter/decrypter is included for experimental reversible 128-bit transformations.

Components
1. Hashing Algorithm

Cyptex128 uses a simple yet effective combination of XOR operations, rotations, and mixing with constants to create a unique 128-bit hash from any input string or byte array.

Algorithm Steps:

Initialize four 32-bit state variables: A, B, C, D (totals 128 bits).

Convert input into bytes and process in 16-byte blocks.

For each block:

XOR each byte with the current state.

Rotate bits in each state by a constant offset.

Add a fixed constant (pre-defined "magic numbers") to each state.

After processing all blocks, combine A, B, C, D into a 128-bit hash.

Output can be represented as 16 bytes or a 32-character hexadecimal string.

Example pseudo-code:

fn cyptex128_hash(input: &[u8]) -> [u8; 16] {
    let mut state = [0x12345678, 0x9abcdef0, 0xfedcba98, 0x87654321]; // 128-bit state
    let constants = [0x11111111, 0x22222222, 0x33333333, 0x44444444];

    for chunk in input.chunks(16) {
        for i in 0..chunk.len() {
            state[i % 4] ^= chunk[i] as u32;
            state[i % 4] = state[i % 4].rotate_left((i * 7 % 32) as u32);
            state[i % 4] = state[i % 4].wrapping_add(constants[i % 4]);
        }
    }

    let mut output = [0u8; 16];
    for i in 0..4 {
        output[i*4..i*4+4].copy_from_slice(&state[i].to_be_bytes());
    }
    output
}

2. Reversible Encryption/Decryption (Optional)

Since standard hashes cannot be reversed, we provide a simple symmetric encryption that produces 128-bit outputs. This is AES-style inspired, but minimal for learning purposes.

Algorithm Steps:

Input is padded/truncated to 16 bytes (128 bits).

Use a secret key of 16 bytes.

XOR input bytes with key bytes.

Rotate each byte by a fixed offset.

Output is exactly 128 bits.

Pseudo-code:

fn cyptex128_encrypt(input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let mut output = [0u8; 16];
    for i in 0..16 {
        output[i] = input[i] ^ key[i];
        output[i] = output[i].rotate_left(3);
    }
    output
}

fn cyptex128_decrypt(input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let mut output = [0u8; 16];
    for i in 0..16 {
        output[i] = input[i].rotate_right(3);
        output[i] ^= key[i];
    }
    output
}


✅ Notes:

Encryption is fully reversible with the key.

Hashing is irreversible, but the 128-bit output is consistent for the same input.

Both are optimized for speed and simplicity in Rust.

3. Rust Project Structure
Cyptex128/
│
├─ src/
│   ├─ lib.rs        # Contains hash & encryption/decryption functions
│   └─ main.rs       # CLI interface for testing
│
├─ Cargo.toml        # Rust project configuration
├─ README.md         # Project overview
└─ CONTEXT.md        # This file

4. Features
Feature	Description
Fixed 128-bit hash	All inputs produce 128-bit output
Simple and fast	Minimal operations, optimized in Rust
Reversible encryption	XOR + rotation based symmetric encryption
Rust-native implementation	No external dependencies required
Optional CLI usage	Test hashing & encryption with your input
5. Usage Example (CLI)
cargo run --example hash "Hello, world!"
cargo run --example encrypt "16-byte input" "16-byte-key"
cargo run --example decrypt "ciphertext" "16-byte-key"

6. Design Philosophy

Learning-first: Easy for beginners to understand and modify.

Portable: Works on any platform Rust supports.

Secure-ish for fun projects: Not meant for production-grade cryptography.