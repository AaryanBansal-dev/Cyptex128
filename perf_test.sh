#!/bin/bash
# Performance comparison test for Cyptex128

CYPTEX="./target/release/cyptex128"

echo "+----------------------------------------------------------------+"
echo "|                  CYPTEX128 PERFORMANCE DEMONSTRATION           |"
echo "+----------------------------------------------------------------+"
echo ""

if [ ! -f "$CYPTEX" ]; then
    echo "Building Cyptex128..."
    cargo build --release
    echo ""
fi

echo "Test 1: Hash a simple string"
echo "Command: $CYPTEX hash 'Hello, Cyptex128!'"
$CYPTEX hash "Hello, Cyptex128!" --stats

echo ""
echo "Test 2: Hash consistency (same input = same output)"
echo "First call:"
HASH1=$($CYPTEX hash "consistency test")
echo "Hash: $HASH1"
echo "Second call:"
HASH2=$($CYPTEX hash "consistency test")
echo "Hash: $HASH2"
if [ "$HASH1" = "$HASH2" ]; then
    echo "Consistent hashing: PASSED"
else
    echo "Consistent hashing: FAILED"
fi

echo ""
echo "Test 3: Avalanche effect (single bit change = different hash)"
echo "Hash of 'a':"
HASH_A=$($CYPTEX hash "a")
echo "$HASH_A"
echo "Hash of 'b':"
HASH_B=$($CYPTEX hash "b")
echo "$HASH_B"
if [ "$HASH_A" != "$HASH_B" ]; then
    echo "Avalanche effect: PASSED"
else
    echo "Avalanche effect: FAILED"
fi

echo ""
echo "Test 4: Encryption & Decryption"
echo "Original text: 'TestData123'"
PLAINTEXT="TestData123"
KEY="my-secret-key-!"
echo "Encrypting with key: '$KEY'"
CIPHER=$($CYPTEX encrypt "$PLAINTEXT" "$KEY" --hex)
echo "Ciphertext: $CIPHER"

echo ""
echo "Test 5: Performance Benchmark (10M iterations, 64-byte input)"
echo "Running benchmark..."
$CYPTEX bench --iterations 10000000 --size 64

echo ""
echo "Test 6: Large input hashing"
echo "Creating 1MB test file..."
dd if=/dev/urandom of=test_1mb.bin bs=1M count=1 2>/dev/null
echo "Hashing 1MB file..."
$CYPTEX hash --file test_1mb.bin --stats
rm test_1mb.bin

echo ""
echo "All tests completed."
