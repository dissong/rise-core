mod validator;
use validator::{ValidatorStats, ValidatorStatus, ProtocolState};

use ed25519_dalek::{SigningKey, Signer}; 
use rand::rngs::OsRng;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 RISE Protocol: Economic Engine Online");

    let mut state = ProtocolState::new();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let mut node = ValidatorStats::new("0xRise_Alpha", 10000);

    for block in 1..=100 {
        // --- Simulate Tokenomics ---
        if block % 10 == 0 {
            state.burned_fees += 25; // Deflationary burn
            state.treasury_balance += 10; // DAO funding
        }

        if block == 30 {
            println!("\n📥 [Block 30] Signing Exit Request...");
            let _sig = signing_key.sign(b"EXIT_PROTOCOL_0xRise_Alpha");
            node.start_unbonding(block);
        }

        node.check_escrow_release(block);

        if block % 25 == 0 {
            println!("📦 Block {}: Validator is {:?}", block, node.status);
        }

        thread::sleep(Duration::from_millis(15));
    }

    // FINAL AUDIT
    state.report();
    println!("✅ Economic Audit Complete. RISE is stable.");
}
