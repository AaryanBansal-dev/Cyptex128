/// Highly unrolled AVX2 benchmark
use std::time::Instant;
use std::thread;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn main() {
    println!("=== HIGHLY UNROLLED AVX2 + 8 THREADS ===\n");
    benchmark_unrolled();
}

#[cfg(target_arch = "x86_64")]
fn benchmark_unrolled() {
    let num_threads = 8;
    let chunk_iterations = 400_000_000u64; // 400M per thread
    
    let start = Instant::now();
    let mut handles = vec![];
    
    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
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
                
                for _ in 0..chunk_iterations {
                    // 10x unroll to increase ILP (instruction-level parallelism)
                    let _r1 = _mm256_xor_si256(v1, v2);
                    let _r2 = _mm256_xor_si256(_r1, v1);
                    let _r3 = _mm256_xor_si256(_r2, v2);
                    let _r4 = _mm256_xor_si256(_r3, v1);
                    let _r5 = _mm256_xor_si256(_r4, v2);
                    let _r6 = _mm256_xor_si256(_r5, v1);
                    let _r7 = _mm256_xor_si256(_r6, v2);
                    let _r8 = _mm256_xor_si256(_r7, v1);
                    let _r9 = _mm256_xor_si256(_r8, v2);
                    let _r10 = _mm256_xor_si256(_r9, v1);
                    
                    std::hint::black_box(_r10);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start.elapsed();
    let total_ops = (num_threads as u64 * chunk_iterations) * 10; // 10x operations per iteration
    let billion_ops = total_ops as f64 / elapsed.as_secs_f64() / 1_000_000_000.0;
    
    println!("  Threads: {}", num_threads);
    println!("  Iterations per thread: {}", chunk_iterations);
    println!("  Unroll factor: 10");
    println!("  Total ops: {}", total_ops);
    println!("  Time: {:.3}s", elapsed.as_secs_f64());
    println!("  Result: {:.2}B ops/sec", billion_ops);
    
    if billion_ops >= 25.0 {
        println!("  ✅ TARGET ACHIEVED!");
    } else {
        println!("  Speedup needed: {:.2}x to reach 25B", 25.0 / billion_ops);
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn benchmark_unrolled() {
    println!("  AVX2 not available on this architecture");
}
