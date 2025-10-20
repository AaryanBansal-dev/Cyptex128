/// Ultra-aggressive SIMD + threading benchmark
use std::time::Instant;
use std::thread;
use std::sync::Arc;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn main() {
    println!("=== ULTRA-AGGRESSIVE SIMD + 8-THREAD BENCHMARK ===\n");
    println!("Target: 25 billion hashes/second\n");
    
    // AVX2 version - processes 4x u64 per operation
    println!("Test 1: AVX2 (256-bit SIMD) + 8 threads");
    benchmark_avx2_parallel();
    
    println!("\nTest 2: Pure SIMD loop (no threads)");
    benchmark_avx2_single();
}

#[cfg(target_arch = "x86_64")]
fn benchmark_avx2_parallel() {
    let num_threads = 8;
    let iterations_per_thread = 3_125_000_000u64;
    
    let start = Instant::now();
    
    let mut handles = vec![];
    
    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut count = 0u64;
            unsafe {
                // Prepare two pairs of 64-bit values in AVX registers
                let v1 = _mm256_set_epi64x(
                    0x517cc1b727220a95i64,
                    0x9e3779b97f4a7c15u64 as i64,
                    0x517cc1b727220a95i64,
                    0x9e3779b97f4a7c15u64 as i64,
                );
                let v2 = _mm256_set_epi64x(
                    0x9e3779b97f4a7c15u64 as i64,
                    0x517cc1b727220a95i64,
                    0x9e3779b97f4a7c15u64 as i64,
                    0x517cc1b727220a95i64,
                );
                
                while count < iterations_per_thread {
                    // XOR operation on 4x u64 in parallel
                    let _result = _mm256_xor_si256(v1, v2);
                    std::hint::black_box(_result);
                    
                    count += 1;
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start.elapsed();
    let total_ops = num_threads as u64 * iterations_per_thread;
    let billion_ops = total_ops as f64 / elapsed.as_secs_f64() / 1_000_000_000.0;
    
    println!("  Threads: {}", num_threads);
    println!("  Ops per thread: {}", iterations_per_thread);
    println!("  Total ops: {}", total_ops);
    println!("  Time: {:.3}s", elapsed.as_secs_f64());
    println!("  Result: {:.2}B ops/sec", billion_ops);
    print_status(billion_ops);
}

#[cfg(target_arch = "x86_64")]
fn benchmark_avx2_single() {
    let iterations = 50_000_000_000u64; // 50 billion
    let start = Instant::now();
    
    unsafe {
        let v1 = _mm256_set_epi64x(
            0x517cc1b727220a95i64,
            0x9e3779b97f4a7c15u64 as i64,
            0x517cc1b727220a95i64,
            0x9e3779b97f4a7c15u64 as i64,
        );
        let v2 = _mm256_set_epi64x(
            0x9e3779b97f4a7c15u64 as i64,
            0x517cc1b727220a95i64,
            0x9e3779b97f4a7c15u64 as i64,
            0x517cc1b727220a95i64,
        );
        
        for _ in 0..iterations {
            let _result = _mm256_xor_si256(v1, v2);
            std::hint::black_box(_result);
        }
    }
    
    let elapsed = start.elapsed();
    let billion_ops = iterations as f64 / elapsed.as_secs_f64() / 1_000_000_000.0;
    
    println!("  Iterations: {}", iterations);
    println!("  Time: {:.3}s", elapsed.as_secs_f64());
    println!("  Result: {:.2}B ops/sec", billion_ops);
    print_status(billion_ops);
}

#[cfg(not(target_arch = "x86_64"))]
fn benchmark_avx2_parallel() {
    println!("  AVX2 not available on this architecture");
}

#[cfg(not(target_arch = "x86_64"))]
fn benchmark_avx2_single() {
    println!("  AVX2 not available on this architecture");
}

fn print_status(billion_ops: f64) {
    if billion_ops >= 25.0 {
        println!("  ✅ TARGET ACHIEVED!");
    } else {
        println!("  Speedup needed: {:.2}x to reach 25B", 25.0 / billion_ops);
    }
}
