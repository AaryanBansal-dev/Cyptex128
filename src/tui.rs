//! TUI (Terminal User Interface) for Cyptex128
//! Professional terminal interface for hashing and dehashing

use std::io::{self, Write};
use crate::{hash, dehash, dehash_brute_force};

const BANNER: &str = r#"
╔════════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║                        CYPTEX128 - TUI INTERFACE                          ║
║                                                                            ║
║                   Ultra-fast 128-bit Hashing System                        ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
"#;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn print_header() {
    println!("{}", BANNER);
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn display_menu() {
    println!();
    println!("  ┌────────────────────────────────────────────────────────────┐");
    println!("  │                      MAIN MENU                            │");
    println!("  ├────────────────────────────────────────────────────────────┤");
    println!("  │                                                            │");
    println!("  │  [1] Hash Text                                             │");
    println!("  │  [2] Dehash (Reverse Lookup)                               │");
    println!("  │  [3] Exit                                                  │");
    println!("  │                                                            │");
    println!("  └────────────────────────────────────────────────────────────┘");
    println!();
}

fn display_hash_result(result: &str) {
    println!();
    println!("  ┌────────────────────────────────────────────────────────────┐");
    println!("  │                    HASH SUCCESSFUL                        │");
    println!("  ├────────────────────────────────────────────────────────────┤");
    println!("  │                                                            │");
    println!("  │  Hash (128-bit hex):                                       │");
    println!("  │  {}", result);
    println!("  │                                                            │");
    println!("  └────────────────────────────────────────────────────────────┘");
    println!();
}

fn hash_operation() {
    clear_screen();
    print_header();
    
    println!("  ┌────────────────────────────────────────────────────────────┐");
    println!("  │                      HASH TEXT                            │");
    println!("  └────────────────────────────────────────────────────────────┘\n");
    
    let data = read_input("  Enter text to hash: ");
    
    if data.is_empty() {
        return;
    }
    
    let hash_result = hash(data.as_bytes());
    let result = hash_result.to_string();
    
    display_hash_result(&result);
    let _ = read_input("  Press Enter to continue: ");
}

fn dehash_operation() {
    clear_screen();
    print_header();
    
    println!("  ┌────────────────────────────────────────────────────────────┐");
    println!("  │                   REVERSE HASH (DEHASH)                   │");
    println!("  └────────────────────────────────────────────────────────────┘\n");
    
    let target_hash = read_input("  Enter hash to reverse (hex): ");
    
    if target_hash.is_empty() || target_hash.len() != 32 {
        println!("\n  [!] Invalid hash. Must be 32 hex characters.");
        let _ = read_input("  Press Enter to continue: ");
        return;
    }
    
    println!("  [1] Use dictionary (common words)");
    println!("  [2] Brute-force (up to 6 characters)\n");
    
    let method = read_input("  Choose method [1-2]: ");
    
    match method.as_str() {
        "1" => {
            let common_words = vec![
                "hello", "world", "test", "password", "admin", "user", "data",
                "hash", "security", "rust", "code", "python", "javascript",
                "database", "server", "client", "network", "internet", "web",
                "application", "system", "process", "thread", "memory", "cpu",
                "gpu", "storage", "cache", "queue", "stack", "tree",
                "graph", "algorithm", "function", "module", "class", "object",
                "array", "string", "number", "boolean", "value", "key",
                "pair", "map", "list", "set", "collection", "interface",
            ];
            
            println!("\n  Searching dictionary...");
            match dehash(&target_hash, &common_words) {
                Some(result) => {
                    println!("\n  SUCCESS! Found: \"{}\"", result);
                    println!("  Verification: {}", hash(result.as_bytes()));
                }
                None => {
                    println!("\n  [!] Not found in dictionary.");
                }
            }
        }
        "2" => {
            println!("\n  Brute-forcing (this may take a moment)...");
            match dehash_brute_force(&target_hash, 4) {
                Some(result) => {
                    println!("\n  SUCCESS! Found: \"{}\"", result);
                    println!("  Verification: {}", hash(result.as_bytes()));
                }
                None => {
                    println!("\n  [!] Not found within 4 characters.");
                }
            }
        }
        _ => {
            println!("\n  [!] Invalid option.");
        }
    }
    
    let _ = read_input("  Press Enter to continue: ");
}

/// Run the interactive TUI
pub fn run() {
    loop {
        clear_screen();
        print_header();
        display_menu();
        
        let choice = read_input("  Select option [1-3]: ");
        
        match choice.as_str() {
            "1" => hash_operation(),
            "2" => dehash_operation(),
            "3" => {
                clear_screen();
                print_header();
                println!("  [*] Exiting Cyptex128 TUI...\n");
                break;
            }
            _ => {
                println!("  [!] Invalid option. Press Enter to continue...");
                let _ = read_input("");
            }
        }
    }
}
