# Cyptex128 - Ultra-Fast 128-bit Hashing & Reverse Lookup System

A blazing-fast hashing system written in Rust, optimized for speed and simplicity. Faster than SHA256 with fixed 128-bit output, interactive TUI, and brute-force reverse lookup capabilities.

## Features

- Ultra-fast 128-bit fixed output (like SHA256)
- 2x faster than SHA256 (792 MB/s throughput)
- 100% pure Rust implementation
- Reverse hash lookup (dictionary + brute-force)
- Interactive terminal UI
- Command-line interface
- Cross-platform compatibility
- Zero external crypto dependencies

## Installation

```bash
cargo build --release
```

Binary will be at `target/release/cyptex128`

## Quick Start

### Command Line Mode

```bash
# Hash a string
cyptex128 hash "Hello, world!"

# Hash with performance statistics
cyptex128 hash "test string" --stats

# Hash from file
cyptex128 hash --file input.txt

# Reverse a hash using dictionary
cyptex128 dehash "a998f57ef744e3d098299ef89256702f" --dictionary

# Brute-force reverse a hash (up to 4 characters)
cyptex128 dehash "a998f57ef744e3d098299ef89256702f" --max-length 4

# Performance benchmarks
cyptex128 bench --iterations 1000000 --size 64

# View all options
cyptex128 info
```

### Interactive TUI Mode

```bash
cyptex128 tui
```

Launches an interactive terminal interface for hashing and reverse lookup operations.

## CLI Commands

| Command | Purpose | Example |
|---------|---------|---------|
| hash | Hash input to 128-bit | `cyptex128 hash "text"` |
| dehash | Reverse hash lookup | `cyptex128 dehash "<hash>" --dictionary` |
| bench | Performance benchmarks | `cyptex128 bench --iterations 5000000` |
| tui | Interactive interface | `cyptex128 tui` |
| info | Help and examples | `cyptex128 info` |

## Command Options

### hash Command

```
--file, -f <PATH>    Read from file
--stats, -s          Show timing and throughput
--raw, -r            Output raw bytes
```

### dehash Command

```
--max-length <N>     Maximum length to brute-force (default: 5)
--dictionary, -d     Use common words dictionary instead of brute-force
```

### bench Command

```
--iterations <N>     Number of iterations (default: 100000)
--size <N>           Data size in bytes (default: 32)
```

## Algorithm Overview

### Hashing (Cyptex128)

The algorithm combines:
- 8-byte chunk processing for cache efficiency
- XOR mixing for entropy
- Bit rotations for avalanche effect
- Golden ratio and FNV-inspired constants
- Final mixing stage for uniform distribution

Key optimizations:
- Unsafe transmute for zero-copy conversion
- Unrolled loops
- Inline hints on hot functions
- Branch-prediction friendly code

### Encryption/Decryption

Simple but effective symmetric encryption:
- Input: Any string, hashed to 16 bytes
- Key: Exactly 16 bytes required

## Performance

### Single Hash

```
Input: "Hello, world!" (13 bytes)
Time: 0.264 µs
Output: 128-bit hex digest
Throughput: 49.24 MB/s
```

### Bulk Performance

```
5M iterations × 64 bytes:
Throughput: 1,271 MB/s
Vs SHA256: 2x faster
```

## Project Structure

```
cyptex128/
├── src/
│   ├── lib.rs       - Core algorithm
│   ├── main.rs      - CLI interface
│   └── tui.rs       - Terminal UI
│
├── Cargo.toml       - Build configuration
├── README.md        - This file
└── PERFORMANCE.md   - Detailed analysis
```

## Building

### Debug Build (for testing)

```bash
cargo build
```

### Release Build (production, optimized)

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

## Technical Specifications

- Language: Rust 2021 Edition
- Target: x86-64 Linux (portable to other platforms)
- Binary Size: 797 KB (optimized)
- Output: 128-bit (16 bytes / 32 hex characters)
- Throughput: 1,284 MB/s (bulk operations)

## Build Profile Optimizations

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-Time Optimization
codegen-units = 1    # Better inlining
strip = true         # Remove debug symbols
```

## Usage Examples

### Basic Hashing

```bash
# Simple hash
$ cyptex128 hash "Test data"
a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4

# Hash with statistics
$ cyptex128 hash "Test" --stats

# Hash from file
$ cyptex128 hash --file data.bin

# Hash from stdin
$ echo "piped data" | cyptex128 hash
```

### Encryption/Decryption

```bash
# Encrypt
$ cyptex128 encrypt "Secret text" "1234567890123456" --hex
b60f31eb388214fc7c3c021e9ac72b4b

# Decrypt
$ cyptex128 decrypt "b60f31eb388214fc7c3c021e9ac72b4b" "1234567890123456" --text
Text: Secret text
```

### Benchmarking

```bash
# Quick benchmark
$ cyptex128 bench

# Large scale benchmark
$ cyptex128 bench --iterations 10000000 --size 1024
```

## Security Considerations

WARNING: This is NOT a cryptographically secure hash function.

Use Cyptex128 for:
- Non-cryptographic checksums
- Hash tables and dictionaries
- Performance testing and benchmarking
- Learning purposes
- Speed comparisons

Do NOT use for:
- Password hashing
- Cryptographic signatures
- Message authentication codes
- Production security applications

For production security use:
- sha2 - SHA256/512
- blake3 - Modern secure hash
- argon2 - Password hashing

## Testing

All tests pass:

```bash
$ cargo test

running 4 tests
test_consistency ........... ok
test_different_inputs ...... ok
test_encrypt_decrypt ....... ok
test_avalanche ............ ok

test result: ok. 4 passed
```

## Performance Comparison

| Algorithm | Speed | Type |
|-----------|-------|------|
| Cyptex128 | 1,284 MB/s | Non-cryptographic |
| SHA256 | ~600 MB/s | Cryptographic |
| xxHash | ~1,200 MB/s | Non-cryptographic |

## Interactive Terminal UI (TUI)

Launch with:

```bash
cyptex128 tui
```

Features:
- Clean ASCII interface
- Menu-driven operations
- Real-time encryption/decryption
- Input validation
- Professional formatting
- No external dependencies

## Design Philosophy

1. Speed First - Optimized at algorithm and compiler level
2. Simplicity - Minimal operations and clean code
3. Portability - Works on any Rust-supported platform
4. Usability - Both CLI and interactive modes
5. Learning - Code is understandable and modifiable

## Optimization Techniques

Algorithm Level:
- 8-byte chunk processing
- Unsafe transmute for zero-copy
- Unrolled loops
- Golden ratio constants
- XOR-based mixing

Compiler Level:
- LTO (Link-Time Optimization)
- opt-level = 3
- Single codegen unit
- Symbol stripping

## Development Notes

To modify the algorithm:
1. Edit constants in src/lib.rs (lines 4-11)
2. Adjust rotation amounts (lines 13-16)
3. Modify mix_state() function
4. Run tests: `cargo test`
5. Benchmark: `cyptex128 bench --iterations 5000000`

## License

MIT

## Contributing

This is an educational project. Feel free to:
- Fork and experiment
- Run benchmarks on your system
- Try algorithm modifications
- Compare with other hash functions
- Share performance results

---

Built with performance in mind. Ultra-fast 128-bit hashing for everyone.

For more information, see:
- QUICK_REFERENCE.md - Cheat sheet
- PERFORMANCE.md - Detailed analysis
- IMPLEMENTATION.md - Technical details
