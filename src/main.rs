// src/main.rs
mod validator;

use validator::{ValidatorStats, SlashReason};
use std::thread;
use std::time::Duration;

struct ProtocolParams {
    total_gas_per_block: f64,
    burn_rate: f64,
    validator_cut: f64,
}

struct Proposal {
    description: String,
    passed: bool,
}

fn main() {
    println!("🚀 RISE Protocol: Full Security & Governance Suite Active");

    let mut node = ValidatorStats::new("0xRise_Alpha", 10000);
    let mut params = ProtocolParams { total_gas_per_block: 20.0, burn_rate: 0.30, validator_cut: 0.50 };
    let mut dao_proposal = Proposal { description: String::from("Increase Burn to 40%"), passed: false };
    let mut last_seen_hash = String::new();

    for block in 1..=130 {
        // 1. DAO Governance Check (Block 80 Update)
        if block == 80 && !dao_proposal.passed {
            dao_proposal.passed = true;
            params.burn_rate = 0.40;
            params.validator_cut = 0.40;
            println!("\n🏛️  DAO VOTE PASSED: {} | New Burn: 40%\n", dao_proposal.description);
        }

        // 2. Double-Signing Security Check (Simulate Attack at Block 115)
        let incoming_hash = if block == 115 { "MALICIOUS_HASH" } else { "VALID_HASH" };
        
        if let Some(reason) = node.verify_signature(block, incoming_hash, &mut last_seen_hash) {
            println!("🚨 SECURITY ALERT: {:?} detected at Block {}!", reason, block);
            println!("💀 Validator 0xRise_Alpha PERMANENTLY banned. Stake slashed.");
            break; 
        }

        // 3. Economic Summary
        if block % 25 == 0 {
            let burned = params.total_gas_per_block * params.burn_rate;
            println!("📦 Block {}: {:.2} $RYZE Burned via Protocol", block, burned);
        }

        // 4. Liveness Check
        if block > 120 { node.missed_blocks += 1; }
        if let Some(reason) = node.check_slashing() {
            println!("⚠️ ALERT: Validator Jailed for {:?}.", reason);
            break;
        }

        thread::sleep(Duration::from_millis(30));
    }
    println!("\n🏁 Protocol Simulation Finished.");
}
