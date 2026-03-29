use std::thread;
use std::time::Duration;

// This defines what a Transaction looks like in RISE
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

// THE GUARDIAN: Logic to catch malicious activity
fn validate_transaction(tx: &Transaction) -> bool {
    println!("[VALIDATOR] Analyzing tx from {}...", tx.sender);
    
    // Security Rule 1: No negative transfers (The "Infinite Money" hack)
    if tx.amount <= 0.0 {
        println!("[⚠️ ALERT] Malicious Activity: Negative amount detected!");
        return false;
    }

    // Security Rule 2: Blacklist Check
    if tx.sender == "Scammer" {
        println!("[❌ REJECTED] Address is on the RISE Blacklist.");
        return false;
    }

    println!("[✅ APPROVED] Integrity check passed.");
    true
}

fn main() {
    println!("🚀 RISE Core Infrastructure - Validator v1.1");
    println!("🛡️ Security Layer: ACTIVE");

    let transactions = vec![
        Transaction { sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: 500.0 },
        Transaction { sender: "Scammer".to_string(), receiver: "Alice".to_string(), amount: 10.0 },
        Transaction { sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: -100.0 },
    ];

    let monitor_handle = thread::spawn(move || {
        for tx in transactions {
            thread::sleep(Duration::from_secs(2));
            if validate_transaction(&tx) {
                println!("[NETWORK LOG] Confirmed: {} sent {} RYZE", tx.sender, tx.amount);
            } else {
                println!("[NETWORK LOG] Blocked: Transaction dropped by Validator.");
            }
            println!("---------------------------------------------------");
        }
    });

    monitor_handle.join().unwrap();
    println!("📡 RISE Node: All checks complete. System healthy.");
}
