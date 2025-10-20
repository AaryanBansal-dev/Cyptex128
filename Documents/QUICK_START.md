# Cyptex128 - Hash-Only Edition - Quick Reference

## Installation
```bash
cargo build --release
# Binary: target/release/cyptex128
# Add to PATH: export PATH="/home/aaryan/Projects/Cyptex128/target/release:$PATH"
```

## Usage

### Basic Hashing
```bash
cyptex128 hash "Hello World"
# Output: 128-bit hex digest (32 characters)
```

### Hash from File
```bash
cyptex128 hash --file document.txt
```

### Hash with Statistics
```bash
cyptex128 hash "data" --stats
# Shows: input size, output, time, throughput
```

### Pipe Input
```bash
echo "piped text" | cyptex128 hash
```

### Benchmarking
```bash
cyptex128 bench                              # Default: 100k iterations, 32 bytes
cyptex128 bench --iterations 1000000         # 1 million iterations
cyptex128 bench --size 1024                  # Benchmark 1KB chunks
```

### Help & Examples
```bash
cyptex128 info                               # Full documentation and examples
cyptex128 hash --help                        # Hash command options
```

### Interactive TUI
```bash
cyptex128 tui                                # Launch terminal interface
# Menu options:
# [1] Hash Text      - Enter text and get instant 128-bit hash
# [2] Exit           - Quit application
```

## Specifications

| Property | Value |
|----------|-------|
| Output Size | 128 bits (16 bytes / 32 hex chars) |
| Speed | 1,271 MB/s (5x faster than SHA256) |
| Implementation | Pure Rust, zero external crypto deps |
| Cross-platform | Linux, macOS, Windows |
| License | MIT/Apache 2.0 |

## Example Outputs

```bash
$ cyptex128 hash "The quick brown fox"
afe107503b0cd7b12eb8d8acb56f162e

$ cyptex128 hash "Hello"
28840a1371c2b224ca0e861aa29529f4

$ cyptex128 hash "" 
# Empty input produces consistent hash
```

## Performance Characteristics

- **Throughput**: 1,271 MB/s (bulk hashing)
- **Single Hash**: 0.3 µs latency
- **Algorithm**: 8-byte chunk processing with XOR + rotation mixing
- **Consistency**: Deterministic - same input always produces same output

## Features

✓ Ultra-fast 128-bit hashing  
✓ Command-line interface  
✓ Interactive TUI  
✓ File input support  
✓ Stdin pipe support  
✓ Statistics/benchmarking  
✓ Cross-platform  
✓ Zero external dependencies  

## File Structure

```
src/lib.rs      - Core hashing algorithm (172 lines)
src/main.rs     - CLI commands and logic (200 lines)
src/tui.rs      - Terminal user interface (104 lines)
Cargo.toml      - Dependencies and build config
README.md       - Full documentation
```

## Notes

- All inputs are hashed to exactly 128 bits
- Output is always hexadecimal (32 characters)
- No encryption features (hash-only)
- Deterministic: same input = same output always
- Perfect for checksums, fingerprints, and deduplication
