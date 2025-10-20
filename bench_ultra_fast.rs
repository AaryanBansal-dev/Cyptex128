/// Ultra-high-performance benchmark for Cyptex128 minimal hash
/// Tests the absolute maximum speed achievable

use std::convert::TryInto;

fn main() {
    use std::time::Instant;
    
    // Test 1: Fixed 128-bit input (UUID-like)
    println!("=== ULTRA-FAST BENCHMARK ===\n");
    
    println!("Test 1: Fixed 128-bit Hash (simulate UUID hashing)");
    let data = [1u8; 16];
    let start = Instant::now();
    let iterations = 10_000_000_000u64; // 10 billion!
    
    for _ in 0..iterations {
        let a = u64::from_ne_bytes(data[0..8].try_into().unwrap());
        let b = u64::from_ne_bytes(data[8..16].try_into().unwrap());
        let _h0 = a.wrapping_mul(0x9e3779b97f4a7c15) ^ b;
        let _h1 = b.wrapping_mul(0x517cc1b727220a95) ^ a;
        std::hint::black_box(_h0);
        std::hint::black_box(_h1);
    }
    
    let elapsed = start.elapsed();
    let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
    let billion_ops = ops_per_sec / 1_000_000_000.0;
    
    println!("  Iterations: {}", iterations);
    println!("  Time: {:.3}s", elapsed.as_secs_f64());
    println!("  Ops/sec: {:.0}", ops_per_sec);
    println!("  Billion ops/sec: {:.2}B", billion_ops);
    println!("  Estimated throughput: {:.2} TB/s on 128-bit data\n", billion_ops * 0.016);
}
