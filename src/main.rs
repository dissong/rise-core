mod validator;
use validator::{ValidatorStats, ValidatorStatus};

use ed25519_dalek::{SigningKey, Signer}; 
use rand::rngs::OsRng;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 RISE Protocol: Escrow & Digital Identity Active");

    // 1. Generate the Identity using the rand_core feature
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    println!("🔑 Validator Public Key: {}", hex::encode(verifying_key.to_bytes()));

    let mut node = ValidatorStats::new("0xRise_Alpha", 10000);

    for block in 1..=100 {
        if block == 30 {
            println!("\n📥 [Block 30] Signing Unstake Request...");
            let message = b"UNSTAKE_0xRise_Alpha";
            let signature = signing_key.sign(message);
            println!("✍️ Signature: [{}...]", hex::encode(&signature.to_bytes()[..8]));
            node.start_unbonding(block);
        }

        node.check_escrow_release(block);

        if block % 20 == 0 {
            println!("📦 Block {}: Status is {:?}", block, node.status);
        }

        thread::sleep(Duration::from_millis(15));
    }
}
