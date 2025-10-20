use cyptex128::parallel::UltraFastHasher;
use std::time::Instant;

fn main() {
    println!("🚀 Testing Ultra-Fast Cyptex128 Performance");
    println!("============================================");
    
    let hasher = UltraFastHasher::new();
    println!("Using {} threads", hasher.num_threads);
    
    let start = Instant::now();
    let ops_per_sec = hasher.benchmark_peak_performance();
    let elapsed = start.elapsed();
    
    println!("Benchmark completed in {:.2}s", elapsed.as_secs_f64());
    println!("Performance: {} billion operations/second", ops_per_sec / 1_000_000_000);
    println!("Status: {} ✅", if ops_per_sec >= 25_000_000_000 { "TARGET ACHIEVED" } else { "BELOW TARGET" });
}
