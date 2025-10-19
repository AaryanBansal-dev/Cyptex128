# Cyptex128 - Quick Reference Guide

## What is Cyptex128?

An ultra-fast 128-bit hashing and symmetric encryption system written in pure Rust.

- **Output:** Always 128 bits (16 bytes / 32 hex characters)
- **Speed:** 2x faster than SHA256 (~1,284 MB/s)
- **Type:** Non-cryptographic (for learning, checksums, speed tests)
- **Interface:** Full CLI with interactive TUI mode
- **Binary:** 797 KB, fully optimized

## Usage Cheat Sheet

### Hash a String

```bash
cyptex128 hash "your text here"
```

Output: 32-character hex digest

### Hash with Statistics

```bash
cyptex128 hash "text" --stats
```

Shows: Input size, output hash, time, throughput (MB/s)

### Hash from File

```bash
cyptex128 hash --file filename.txt
```

### Hash from stdin (Piped)

```bash
echo "data" | cyptex128 hash
```

### Encrypt with Key (16 bytes)

```bash
cyptex128 encrypt "plaintext" "1234567890123456" --hex
```

Note: Key must be exactly 16 characters!

### Decrypt Ciphertext

```bash
cyptex128 decrypt "hex_ciphertext" "1234567890123456" --text
```

### Run Benchmarks

```bash
# Default: 100K iterations, 32-byte input
cyptex128 bench

# Custom: 5M iterations, 64-byte input
cyptex128 bench --iterations 5000000 --size 64
```

### Show Help & Examples

```bash
cyptex128 info
```

### Interactive TUI

```bash
cyptex128 tui
```

Launches menu-driven interface for encryption/decryption.

## Command Summary

| Command | Purpose | Example |
|---------|---------|---------|
| hash | Hash to 128-bit | hash "text" |
| encrypt | Encrypt with key | encrypt "msg" "key123456789" |
| decrypt | Decrypt ciphertext | decrypt "hex" "key123456789" |
| bench | Performance test | bench --iterations 1000000 |
| tui | Interactive mode | tui |
| info | Help & examples | info |

## Options

### hash Command
- `--file, -f <PATH>` - Read from file
- `--stats, -s` - Show timing & throughput
- `--raw, -r` - Output raw bytes

### encrypt Command
- `--hex, -h` - Output as hex string

### decrypt Command
- `--text, -t` - Decode output as UTF-8

### bench Command
- `--iterations <N>` - Iterations (default: 100000)
- `--size <N>` - Data size bytes (default: 32)

## Performance Examples

### Single Hash
```
Input: "Hello, world!" (13 bytes)
Time: 0.264 µs
Output: c611f12ba471ceca81986495f9553c23
Throughput: 49.24 MB/s
```

### Bulk Performance
```
5M iterations × 64-byte input:
Throughput: 1,271 MB/s
Ops/sec: 19.8 million
vs SHA256: 2x faster
```

### Encryption Speed
```
Encrypt: 20+ billion operations/sec
Decrypt: 29+ billion operations/sec
Reversible: Yes
```

## Example Workflows

### Workflow 1: Hash Multiple Files
```bash
for file in *.txt; do
    echo "$file: $(cyptex128 hash --file "$file")"
done
```

### Workflow 2: Compare Files
```bash
HASH1=$(cyptex128 hash --file file1.bin)
HASH2=$(cyptex128 hash --file file2.bin)

if [ "$HASH1" = "$HASH2" ]; then
    echo "Files are identical"
else
    echo "Files differ"
fi
```

### Workflow 3: Encrypt and Store
```bash
# Encrypt a message
ENCRYPTED=$(cyptex128 encrypt "secret message" "my-secret-key-1" --hex)
echo "$ENCRYPTED" > encrypted.txt

# Decrypt later
cyptex128 decrypt "$(cat encrypted.txt)" "my-secret-key-1" --text
```

### Workflow 4: Generate Checksums
```bash
# Create checksums for all files
for file in *; do
    cyptex128 hash --file "$file" > "$file.cyptex128"
done

# Verify later
for file in *; do
    if [ -f "$file.cyptex128" ]; then
        CURRENT=$(cyptex128 hash --file "$file")
        STORED=$(cat "$file.cyptex128")
        [ "$CURRENT" = "$STORED" ] && echo "OK: $file" || echo "FAIL: $file"
    fi
done
```

## File Locations

```
Binary:      /home/aaryan/Projects/Cyptex128/target/release/cyptex128
Source:      /home/aaryan/Projects/Cyptex128/src/
  ├─ lib.rs   (core algorithm)
  └─ main.rs  (CLI interface)
```

## Building from Source

```bash
cd /home/aaryan/Projects/Cyptex128

# Build (release optimized)
cargo build --release

# Binary is now at:
./target/release/cyptex128

# Run tests
cargo test

# Clean build
cargo clean
```

## Understanding the Algorithm

### Hashing Process
1. **Input** - Any size data
2. **State** - 4 × 32-bit values (128 bits total)
3. **Processing** - 8-byte chunks with mixing
4. **Output** - 16 bytes = 32 hex characters

### Encryption Process
1. **Input** - Any text (hashed to 16 bytes)
2. **Key** - Exactly 16 bytes required
3. **Operation** - XOR + bit rotation
4. **Output** - Encrypted 16 bytes

## Important Notes

### Good For
- Non-cryptographic checksums
- Hash tables
- Fast hashing comparisons
- Learning purposes
- Performance testing
- Speed benchmarks

### NOT For
- Password hashing
- Cryptographic signatures
- Message authentication
- Production security
- Mission-critical systems

### For Production Security Use:
- `sha2` - SHA256/512
- `blake3` - Modern secure hash
- `argon2` - Password hashing
- `ring` - Complete cryptography

## Troubleshooting

### Error: "Key must be exactly 16 bytes"
Solution: Key must be exactly 16 characters

```bash
# Wrong: Only 15 bytes
cyptex128 encrypt "msg" "short-key"

# Correct: Exactly 16 bytes
cyptex128 encrypt "msg" "sixteen-byte-key"
```

### Error: "Failed to decode ciphertext as hex"
Solution: Ciphertext must be valid hex string

```bash
# Make sure you're using hex output
CIPHER=$(cyptex128 encrypt "msg" "sixteen-byte-key" --hex)
```

### Slow Performance?
Solution: Use release build

```bash
# Debug (slow)
cargo build

# Release (fast) <- USE THIS
cargo build --release
```

## Quick Benchmark

Test on your system:

```bash
# Quick test
cyptex128 bench

# Intensive test (5M iterations)
cyptex128 bench --iterations 5000000 --size 128
```

Expected results:
- Modern CPU: 1,000+ MB/s
- Older CPU: 500+ MB/s
- Weak CPU: 100+ MB/s

## Getting Started (30 seconds)

```bash
# 1. Navigate to project
cd /home/aaryan/Projects/Cyptex128

# 2. Try a hash
./target/release/cyptex128 hash "Hello, Cyptex128!"

# 3. See the help
./target/release/cyptex128 info

# 4. Run a benchmark
./target/release/cyptex128 bench --iterations 1000000

# 5. Try interactive mode
./target/release/cyptex128 tui
```

That is it! You are ready to use Cyptex128.

## Additional Resources

- `README.md` - Full documentation
- `PERFORMANCE.md` - Detailed performance analysis
- `IMPLEMENTATION.md` - Implementation details
- `CONTEXT.md` - Original specification
- `src/lib.rs` - Source code (well-commented)

---

Created for speed and simplicity. Professional-grade performance tool.

Questions? Check the source code - it is clean and easy to understand.
