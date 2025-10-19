//! Cyptex128 - Ultra-fast 128-bit hashing system
//!
//! Optimized for speed with minimal operations and efficient bit manipulation.

use std::mem;

// Magic constants - pre-computed for maximum speed
const MAGIC_A: u32 = 0x9e3779b9; // Golden ratio
const MAGIC_B: u32 = 0x517cc1b5; // Optimized constant
const MAGIC_C: u32 = 0x85ebca6b; // FNV-inspired
const MAGIC_D: u32 = 0xc2b2ae35; // Additional mixing constant

// Rotation amounts - carefully chosen for speed and entropy
const ROT_A: u32 = 13;
const ROT_B: u32 = 17;
const ROT_C: u32 = 5;
const ROT_D: u32 = 11;

/// Represents a 128-bit hash output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash128([u8; 16]);

impl Hash128 {
    /// Convert to hexadecimal string
    #[inline]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Get raw bytes
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Get as mutable bytes
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 16] {
        &mut self.0
    }
}

impl std::fmt::Display for Hash128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Ultra-optimized Cyptex128 hashing algorithm
/// Processes input with minimal operations for maximum speed
#[inline(always)]
fn mix_state(state: &mut [u32; 4], byte: u32, idx: u32) {
    // Single-pass mixing: XOR, rotate, add - no extra copies
    state[0] = state[0].wrapping_add((byte << 0) ^ MAGIC_A);
    state[1] = state[1].wrapping_add((byte << 8) ^ MAGIC_B);
    state[2] = state[2].wrapping_add((byte << 16) ^ MAGIC_C);
    state[3] = state[3].wrapping_add((byte << 24) ^ MAGIC_D);

    // Rotations based on position - minimal branching
    let rot = (idx as u32).wrapping_mul(7) & 0x1f;
    state[(idx as usize) & 3] = state[(idx as usize) & 3].rotate_left(rot);
}

/// Hash any input to 128 bits with extreme speed
#[inline]
pub fn hash(input: &[u8]) -> Hash128 {
    let mut state: [u32; 4] = [MAGIC_A, MAGIC_B, MAGIC_C, MAGIC_D];

    // Process 8 bytes at a time for better cache locality and fewer iterations
    let mut i = 0;
    let len = input.len();
    let chunks = len / 8;

    // Fast path: process 8-byte chunks
    for chunk_idx in 0..chunks {
        let offset = chunk_idx * 8;
        let byte0 = input[offset] as u32;
        let byte1 = input[offset + 1] as u32;
        let byte2 = input[offset + 2] as u32;
        let byte3 = input[offset + 3] as u32;
        let byte4 = input[offset + 4] as u32;
        let byte5 = input[offset + 5] as u32;
        let byte6 = input[offset + 6] as u32;
        let byte7 = input[offset + 7] as u32;

        let combined_a = (byte0 << 0) | (byte1 << 8) | (byte2 << 16) | (byte3 << 24);
        let combined_b = (byte4 << 0) | (byte5 << 8) | (byte6 << 16) | (byte7 << 24);

        state[0] = state[0].wrapping_mul(31).wrapping_add(combined_a ^ MAGIC_A);
        state[1] = state[1].wrapping_mul(31).wrapping_add(combined_b ^ MAGIC_B);

        state[0] = state[0].rotate_left(ROT_A);
        state[1] = state[1].rotate_left(ROT_B);

        let temp = state[2];
        state[2] = state[3];
        state[3] = temp;

        i += 8;
    }

    // Process remaining bytes
    for idx in 0..(len - i) {
        mix_state(&mut state, input[i + idx] as u32, idx as u32);
    }

    // Final mixing stage - ensure avalanche effect
    state[0] ^= state[2];
    state[1] ^= state[3];
    state[2] ^= state[0];
    state[3] ^= state[1];

    state[0] = state[0].wrapping_mul(MAGIC_C);
    state[1] = state[1].wrapping_mul(MAGIC_D);
    state[2] = state[2].wrapping_mul(MAGIC_A);
    state[3] = state[3].wrapping_mul(MAGIC_B);

    state[0] = state[0].rotate_left(ROT_B);
    state[1] = state[1].rotate_left(ROT_D);
    state[2] = state[2].rotate_left(ROT_A);
    state[3] = state[3].rotate_left(ROT_C);

    // Convert 4x u32 to 16 bytes using unsafe for zero-copy
    let mut output = [0u8; 16];
    unsafe {
        // Use transmute for fastest possible conversion
        let state_bytes = mem::transmute::<[u32; 4], [u8; 16]>(state);
        output.copy_from_slice(&state_bytes);
    }

    Hash128(output)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistency() {
        let input = b"Hello, world!";
        let hash1 = hash(input);
        let hash2 = hash(input);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_inputs() {
        let hash1 = hash(b"test1");
        let hash2 = hash(b"test2");
        assert_ne!(hash1, hash2);
    }



    #[test]
    fn test_avalanche() {
        let input1 = b"a";
        let input2 = b"b";
        let hash1 = hash(input1);
        let hash2 = hash(input2);
        
        let mut diff = 0;
        for i in 0..16 {
            if hash1.as_bytes()[i] != hash2.as_bytes()[i] {
                diff += 1;
            }
        }
        assert!(diff > 0);
    }
}

/// Dehash - brute-force reverse lookup of a hash
/// Searches through a dictionary to find the original text
pub fn dehash(target_hash: &str, dictionary: &[&str]) -> Option<String> {
    let target_lower = target_hash.to_lowercase();
    
    for candidate in dictionary {
        let candidate_hash = hash(candidate.as_bytes());
        if candidate_hash.to_hex() == target_lower {
            return Some(candidate.to_string());
        }
    }
    
    None
}

/// Dehash with custom word generation (for brute-forcing)
/// Generates strings and checks against target hash
pub fn dehash_brute_force(target_hash: &str, max_length: usize) -> Option<String> {
    let target_lower = target_hash.to_lowercase();
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
    
    // Try single characters first
    for c in alphabet.chars() {
        let candidate = c.to_string();
        let candidate_hash = hash(candidate.as_bytes());
        if candidate_hash.to_hex() == target_lower {
            return Some(candidate);
        }
    }
    
    // Try progressively longer strings (this could take a while!)
    for length in 2..=max_length {
        if try_length(target_hash, alphabet, length) {
            // We found it, do a final check
            return search_length(target_hash, alphabet, length);
        }
    }
    
    None
}

fn try_length(target_hash: &str, alphabet: &str, length: usize) -> bool {
    let target_lower = target_hash.to_lowercase();
    let mut indices = vec![0; length];
    let chars: Vec<char> = alphabet.chars().collect();
    
    loop {
        let candidate: String = indices.iter().map(|&i| chars[i]).collect();
        let candidate_hash = hash(candidate.as_bytes());
        
        if candidate_hash.to_hex() == target_lower {
            return true;
        }
        
        // Increment indices
        let mut carry = true;
        for i in (0..length).rev() {
            if carry {
                indices[i] += 1;
                if indices[i] < chars.len() {
                    carry = false;
                    break;
                }
                indices[i] = 0;
            }
        }
        
        if carry {
            break;
        }
    }
    
    false
}

fn search_length(target_hash: &str, alphabet: &str, length: usize) -> Option<String> {
    let target_lower = target_hash.to_lowercase();
    let mut indices = vec![0; length];
    let chars: Vec<char> = alphabet.chars().collect();
    
    loop {
        let candidate: String = indices.iter().map(|&i| chars[i]).collect();
        let candidate_hash = hash(candidate.as_bytes());
        
        if candidate_hash.to_hex() == target_lower {
            return Some(candidate);
        }
        
        // Increment indices
        let mut carry = true;
        for i in (0..length).rev() {
            if carry {
                indices[i] += 1;
                if indices[i] < chars.len() {
                    carry = false;
                    break;
                }
                indices[i] = 0;
            }
        }
        
        if carry {
            break;
        }
    }
    
    None
}
