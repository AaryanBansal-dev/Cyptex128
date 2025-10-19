# Cyptex128 - Ultra-Fast 128-bit Hashing & Encryption

A blazing-fast hashing and symmetric encryption system written in Rust, optimized for speed and simplicity. **Faster than SHA256** with fixed 128-bit output.

## Features

- **Blazing Fast** - Optimized bit operations and minimal overhead
- **128-bit Fixed Output** - Like SHA256, always 16 bytes / 32 hex characters
- **Optional Encryption** - Reversible symmetric encryption with 16-byte keys
- **No External Dependencies** - Pure Rust implementation (only CLI deps)
- **Simple & Educational** - Easy to understand and modify
- **Cross-Platform** - Works on Linux, macOS, Windows, etc.
- **Interactive TUI** - Terminal user interface for easy interaction

## Installation

```bash
cargo build --release
```

Binary will be at `target/release/cyptex128`

## Usage

### Hash a String

```bash
cyptex128 hash "Hello, world!"
# Output: 128-bit hex digest
```

### Hash from File

```bash
cyptex128 hash --file input.txt
```

### Hash with Statistics

```bash
cyptex128 hash "test" --stats
# Shows timing and throughput info
```

### Hash from stdin

```bash
echo "piped data" | cyptex128 hash
```

### Encrypt Data

```bash
cyptex128 encrypt "plaintext" "16byte-key-1234" --hex
```

### Decrypt Data

```bash
cyptex128 decrypt "<hex_ciphertext>" "16byte-key-1234" --text
```

### Run Benchmarks

```bash
cyptex128 bench --iterations 1000000 --size 1024
```

### Interactive Mode

```bash
cyptex128 tui
```

### View Help & Examples

```bash
cyptex128 info
```

## Algorithm Details

### Hashing (Cyptex128)

The algorithm combines:
1. **8-byte chunk processing** - Better cache locality
2. **XOR mixing** - Fast bit operations
3. **Bit rotations** - Avalanche effect
4. **Magic constants** - Golden ratio and FNV-inspired values
5. **Final mixing stage** - Ensures uniform distribution

**Key Optimizations:**
- Unrolled loops for reduced overhead
- Unsafe transmute for zero-copy u32→u8 conversion
- Combined operations to maximize instruction pipeline efficiency
- Rotation amounts chosen for entropy and speed

### Encryption/Decryption

Simple but effective symmetric encryption:
- **Input:** Any string → hashed to 16 bytes
- **Key:** 16 bytes exactly
- **Process:** XOR + bit rotation
- **Fully reversible:** Decrypt recovers original

## Performance

Designed to be **faster than SHA256** through:
- Fewer mathematical operations
- Better memory locality with 8-byte chunks
- Direct bit manipulation without allocation
- Aggressive compiler optimizations (LTO, opt-level=3)

Run benchmarks to see actual numbers on your system:

```bash
cargo build --release && ./target/release/cyptex128 bench --iterations 10000000
```

## Project Structure

```
cyptex128/
├── src/
│   ├── main.rs      # CLI interface
│   ├── lib.rs       # Core hashing & encryption
│   └── tui.rs       # Terminal user interface
│
├── Cargo.toml       # Rust configuration
├── README.md        # This file
└── CONTEXT.md       # Project specification
```

## Build Profiles

**Debug Build** (testing):
```bash
cargo build
```

**Release Build** (production - optimized):
```bash
cargo build --release
```

Release build includes:
- LTO (Link-Time Optimization)
- Optimization level 3 (`opt-level = 3`)
- Single codegen unit
- Symbol stripping

## CLI Commands

| Command | Purpose |
|---------|---------|
| `hash [INPUT]` | Hash input to 128-bit digest |
| `encrypt <INPUT> <KEY>` | Encrypt input with key |
| `decrypt <CIPHERTEXT> <KEY>` | Decrypt with key |
| `bench` | Performance benchmarks |
| `tui` | Interactive terminal interface |
| `info` | Help & examples |

## Command Options

### hash

```
--file, -f <PATH>    Read from file
--raw, -r            Output raw bytes
--stats, -s          Show timing & throughput
```

### encrypt

```
--hex, -h            Output as hex string
```

### decrypt

```
--text, -t           Try to decode as UTF-8 text
```

### bench

```
--iterations <N>     Number of hash operations (default: 100000)
--size <N>           Data size in bytes (default: 32)
```

## Example Output

```
$ cyptex128 hash "Cyptex128" --stats

Hash Statistics:
  Input size: 9 bytes
  Output: a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4
  Time: 1.234µs
  Throughput: 7.31 MB/s
```

## Testing

```bash
cargo test
```

Tests include:
- Consistency (same input → same output)
- Differentiation (different inputs → different outputs)
- Encryption/Decryption reversibility
- Avalanche effect verification

## Design Philosophy

1. **Learning-First** - Code is understandable and modifiable
2. **Speed-Optimized** - Minimal operations, maximum throughput
3. **Portable** - Works on any Rust-supported platform
4. **Professional** - Clean ASCII art and no excessive decoration

## Quick Start

```bash
# Build
cargo build --release

# Run
./target/release/cyptex128 info

# Hash something
./target/release/cyptex128 hash "your text here"

# Or interactive mode
./target/release/cyptex128 tui

# Benchmark
./target/release/cyptex128 bench --iterations 5000000
```

## Security Considerations

Cyptex128 is **NOT** suitable for production cryptographic applications. It's designed for:
- Learning about hash algorithms
- Fast non-cryptographic hashing
- Fun educational projects
- Performance comparisons

For production security, use established libraries like:
- `sha2` - SHA-256/512
- `blake3` - BLAKE3
- `argon2` - Password hashing

## License

MIT

---

**Built for Speed and Simplicity**
