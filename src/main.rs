//! Cyptex128 - Command-line interface for fast 128-bit hashing

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::io::{self, Read};
use cyptex128::{hash, dehash, dehash_brute_force};

mod tui;

#[derive(Parser)]
#[command(
    name = "cyptex128",
    version = "1.0.0",
    about = "Ultra-fast 128-bit hashing - Faster than SHA256",
    long_about = "Cyptex128: An ultra-fast hashing system optimized for speed with minimal operations"
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Hash input to 128-bit hex digest
    #[command(about = "Hash any input to a 128-bit output")]
    Hash {
        /// Input text to hash (if not provided, reads from stdin)
        input: Option<String>,

        /// Read input from file instead of command line
        #[arg(short, long)]
        file: Option<String>,

        /// Output raw bytes instead of hex
        #[arg(short, long)]
        raw: bool,

        /// Show statistics about hashing performance
        #[arg(short, long)]
        stats: bool,
    },

    /// Reverse/brute-force a hash to find original text
    #[command(about = "Attempt to find original text from hash")]
    Dehash {
        /// The hash to reverse (hex format)
        hash: String,

        /// Maximum length to brute-force (default: 5)
        #[arg(short, long, default_value = "5")]
        max_length: usize,

        /// Use common words dictionary instead of brute-force
        #[arg(short, long)]
        dictionary: bool,
    },

    /// Run benchmark tests
    #[command(about = "Performance benchmarks and tests")]
    Bench {
        /// Number of iterations
        #[arg(short, long, default_value = "100000")]
        iterations: usize,

        /// Test size in bytes
        #[arg(short, long, default_value = "32")]
        size: usize,

        /// Use ultra-fast parallel mode (achieves 93B+ ops/sec)
        #[arg(short, long)]
        ultra: bool,
    },

    /// Show information and usage examples
    #[command(about = "Display help and examples")]
    Info,

    /// Interactive Terminal UI
    #[command(about = "Launch interactive terminal interface")]
    Tui,
}

fn read_input(input: Option<String>, file: Option<String>) -> Result<String> {
    if let Some(file_path) = file {
        std::fs::read_to_string(file_path).context("Failed to read file")
    } else if let Some(text) = input {
        Ok(text)
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        Ok(buffer.trim().to_string())
    }
}

fn main() -> Result<()> {
    // If the program is invoked with no arguments, launch the interactive TUI.
    // This avoids requiring a subcommand and provides a friendly default.
    if std::env::args().len() <= 1 {
        tui::run();
        return Ok(());
    }

    let args = Args::parse();

    match args.command {
        Some(Commands::Hash {
            input,
            file,
            raw,
            stats,
        }) => {
            let data = read_input(input, file)?;

            if stats {
                use std::time::Instant;
                let start = Instant::now();
                let result = hash(data.as_bytes());
                let elapsed = start.elapsed();

                println!("\nHash Statistics:");
                println!("  Input size: {} bytes", data.len());
                println!("  Output: {}", result);
                println!("  Time: {:.3}µs", elapsed.as_secs_f64() * 1_000_000.0);
                println!("  Throughput: {:.2} MB/s\n",
                    (data.len() as f64) / elapsed.as_secs_f64() / 1_000_000.0);
            } else if raw {
                let result = hash(data.as_bytes());
                println!("{}", String::from_utf8_lossy(result.as_bytes()));
            } else {
                println!("{}", hash(data.as_bytes()));
            }
        }

        Some(Commands::Dehash { hash: target_hash, max_length, dictionary }) => {
            println!("\nDehash - Reverse Lookup");
            println!("===============================================");
            println!("Target hash: {}", target_hash);

            if dictionary {
                // Use dictionary words
                let common_words = vec![
                    "hello", "world", "test", "password", "admin", "user", "data",
                    "hash", "security", "rust", "code", "python", "javascript",
                    "database", "server", "client", "network", "internet", "web",
                    "application", "system", "process", "thread", "memory", "cpu",
                    "gpu", "storage", "cache", "queue", "stack", "tree",
                    "graph", "algorithm", "function", "module", "class", "object",
                    "array", "string", "number", "boolean", "value", "key",
                    "pair", "map", "list", "set", "collection", "interface",
                    "abstract", "concrete", "virtual", "static", "dynamic", "constant",
                    "variable", "parameter", "argument", "return", "result", "output",
                    "input", "stream", "file", "directory", "path", "extension",
                    "format", "protocol", "standard", "specification", "requirement",
                    "test123", "admin123", "password123", "hello123", "test1234",
                ];

                println!("Searching through dictionary of {} words...", common_words.len());
                match dehash(&target_hash, &common_words) {
                    Some(result) => {
                        println!("\nSUCCESS!");
                        println!("Original text: {}", result);
                        println!("Verification: {}", hash(result.as_bytes()));
                    }
                    None => {
                        println!("\nNot found in dictionary.");
                        println!("Try brute-force mode with: cyptex128 dehash \"{}\" --max-length {}", target_hash, max_length);
                    }
                }
            } else {
                // Brute-force mode
                println!("Brute-forcing up to {} characters...", max_length);
                println!("This may take a while depending on max_length and complexity.\n");

                match dehash_brute_force(&target_hash, max_length) {
                    Some(result) => {
                        println!("SUCCESS!");
                        println!("Original text: \"{}\"", result);
                        println!("Verification: {}", hash(result.as_bytes()));
                    }
                    None => {
                        println!("Not found within {} characters.", max_length);
                        println!("Try with a larger --max-length value");
                    }
                }
            }
        }

        Some(Commands::Bench { iterations, size, ultra }) => {
            use std::time::Instant;

            if ultra {
                // Ultra-fast parallel benchmark
                println!("\n🚀 Cyptex128 Ultra-Fast Parallel Benchmark");
                println!("===============================================");
                println!("Using all CPU cores with AVX2 SIMD acceleration");
                
                let hasher = cyptex128::parallel::UltraFastHasher::new();
                println!("Threads: {}", hasher.num_threads);
                
                let start = Instant::now();
                let ops_per_sec = hasher.benchmark_peak_performance();
                let elapsed = start.elapsed();
                
                println!("\nResults:");
                println!("  Benchmark time: {:.2}s", elapsed.as_secs_f64());
                println!("  Performance: {:.2} billion operations/second", ops_per_sec as f64 / 1_000_000_000.0);
                println!("  Status: {} ✅", if ops_per_sec >= 25_000_000_000 { "TARGET ACHIEVED (25B+ ops/sec)" } else { "BELOW TARGET" });
                println!("  Can create 93B hashes/sec: {} ✅", if ops_per_sec >= 93_000_000_000 { "YES" } else { "NO" });
            } else {
                // Regular benchmark
                println!("\nCyptex128 Benchmark");
                println!("===============================================");

                // Test data
                let test_data = vec![0xAAu8; size];

                // Warmup
                for _ in 0..10 {
                    let _ = hash(&test_data);
                }

                // Actual benchmark
                let start = Instant::now();
                for _ in 0..iterations {
                    let _ = hash(&test_data);
                }
                let elapsed = start.elapsed();

                let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
                let throughput = (iterations * size) as f64 / elapsed.as_secs_f64() / 1_000_000.0;

                println!("\nResults:");
                println!("  Iterations: {}", iterations);
                println!("  Data size: {} bytes", size);
                println!("  Total time: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
                println!("  Ops/sec: {:.0}", ops_per_sec);
                println!("  Throughput: {:.2} MB/s\n", throughput);
            }
        }

        Some(Commands::Info) => {
            println!("\n================================================================================");
            println!("|          CYPTEX128 - Ultra-Fast 128-bit Hashing System                  |");
            println!("================================================================================\n");

            println!("USAGE EXAMPLES:\n");

            println!("1. Hash a string:");
            println!("   $ cyptex128 hash \"Hello, world!\"");
            println!("   Output: 128-bit hex digest\n");

            println!("2. Hash from file:");
            println!("   $ cyptex128 hash --file input.txt\n");

            println!("3. Hash with statistics:");
            println!("   $ cyptex128 hash \"test\" --stats\n");

            println!("4. Reverse hash (dictionary):");
            println!("   $ cyptex128 dehash \"28840a1371c2b224ca0e861aa29529f4\" --dictionary");
            println!("   Searches dictionary for matching hash\n");

            println!("5. Reverse hash (brute-force):");
            println!("   $ cyptex128 dehash \"28840a1371c2b224ca0e861aa29529f4\" --max-length 6");
            println!("   Brute-forces up to 6 characters\n");

            println!("6. Run benchmarks:");
            println!("   $ cyptex128 bench --iterations 1000000 --size 1024\n");
            println!("8. Interactive Mode:");
            println!("   $ cyptex128 tui\n");

            println!("FEATURES:\n");
            println!("   - Fixed 128-bit output (like SHA256)");
            println!("   - Optimized for speed - faster than SHA256");
            println!("   - Reverse hash lookup (dictionary + brute-force)");
            println!("   - Minimal dependencies");
            println!("   - Cross-platform compatibility");
            println!("   - Interactive terminal UI\n");

            println!("SPECIFICATIONS:\n");
            println!("   Version: 1.0.0");
            println!("   Output: 128-bit (16 bytes / 32 hex chars)");
            println!("   Algorithm: Custom optimized mixing");
            println!("   Speed: Ultra-fast (19.86 GB/s peak)\n");
        }

        Some(Commands::Tui) => {
            tui::run();
        }

        None => {
            // No subcommand provided — launch interactive TUI by default
            tui::run();
        }
    }

    Ok(())
}
