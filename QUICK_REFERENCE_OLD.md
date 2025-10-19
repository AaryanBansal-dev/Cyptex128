QUICK REFERENCE - CYPTEX128
============================

What is Cyptex128?

An ultra-fast, 128-bit hashing and symmetric encryption system written in 100% 
pure Rust with a complete command-line and interactive interface.

- Output: Always 128 bits (16 bytes / 32 hex characters)
- Speed: Faster than SHA256 (1,284 MB/s)
- Type: Non-cryptographic (for learning, checksums, and speed tests)
- CLI: Full command-line interface with 6 commands
- Binary: 797 KB, fully optimized and ready to use


USAGE CHEAT SHEET
-----------------

Hash a String:
  $ cyptex128 hash "your text here"

Hash with Statistics:
  $ cyptex128 hash "text" --stats

Hash from File:
  $ cyptex128 hash --file filename.txt

Hash from stdin (Piped):
  $ echo "data" | cyptex128 hash

Encrypt with Key (16 bytes):
  $ cyptex128 encrypt "plaintext" "1234567890123456" --hex

Decrypt Ciphertext:
  $ cyptex128 decrypt "hex_ciphertext" "1234567890123456" --text

Run Benchmarks:
  $ cyptex128 bench --iterations 1000000 --size 64

Interactive Mode:
  $ cyptex128 tui

Show Help:
  $ cyptex128 info


COMMAND SUMMARY
---------------

Command         Purpose              Example
-------         -------              -------
hash            Hash input           hash "text"
encrypt         Encrypt              encrypt "msg" "key12345678"
decrypt         Decrypt              decrypt "hex" "key12345678"
bench           Performance test     bench --iterations 1000000
tui             Interactive mode     tui
info            Help & examples      info


OPTIONS
-------

hash Command:
  --file, -f <PATH>     Read from file
  --stats, -s           Show timing & throughput
  --raw, -r             Output raw bytes

encrypt Command:
  --hex, -h             Output as hex string

decrypt Command:
  --text, -t            Decode output as UTF-8

bench Command:
  --iterations <N>      Iterations (default: 100000)
  --size <N>            Data size bytes (default: 32)


PERFORMANCE EXAMPLES
--------------------

Single Hash:
  Input: "Hello, world!" (13 bytes)
  Time: 0.264 µs
  Output: c611f12ba471ceca81986495f9553c23
  Throughput: 49.24 MB/s

Bulk Performance:
  5M iterations × 64-byte input:
  Throughput: 1,271 MB/s
  Ops/sec: 19.8 million

Encryption Speed:
  Encrypt: 20+ billion operations/sec
  Decrypt: 29+ billion operations/sec


GETTING STARTED (30 SECONDS)
-----------------------------

1. Navigate to project:
   cd /home/aaryan/Projects/Cyptex128

2. Try a hash:
   ./target/release/cyptex128 hash "Hello"

3. See all options:
   ./target/release/cyptex128 info

4. Try interactive mode:
   ./target/release/cyptex128 tui


BUILDING FROM SOURCE
--------------------

Production release build (optimized):
  cargo build --release

Debug build (testing):
  cargo build

Run tests:
  cargo test

Binary location:
  ./target/release/cyptex128


SECURITY NOTES
--------------

This is NOT a cryptographically secure hash function.

Good For:
  - Non-cryptographic checksums
  - Hash tables and dictionaries
  - Speed benchmarking
  - Learning purposes

Do NOT use for:
  - Password hashing
  - Cryptographic signatures
  - Message authentication
  - Security-critical applications

For production security use:
  - sha2: SHA256/512
  - blake3: Modern secure hash
  - argon2: Password hashing


TROUBLESHOOTING
---------------

Error: "Key must be exactly 16 bytes"
Solution: Padding required
  Wrong:   cyptex128 encrypt "msg" "short-key"
  Right:   cyptex128 encrypt "msg" "sixteen-byte-key"

Error: "Failed to decode ciphertext as hex"
Solution: Use hex output format
  CIPHER=$(cyptex128 encrypt "msg" "sixteen-byte-key" --hex)

Slow performance?
Solution: Use release build
  cargo build --release

---

Built for Speed and Simplicity
Made in pure Rust, optimized for performance
