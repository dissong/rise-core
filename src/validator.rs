// src/validator.rs

#[derive(Debug, Clone)]
pub enum SlashReason {
    DoubleSigning,    // Malicious: Signing two different hashes for one block
    Downtime,         // Negligent: Offline for too long
    InvalidProposal,  // Medium: Submitting corrupt data
}

pub struct ValidatorStats {
    pub address: String,
    pub missed_blocks: u32,
    pub is_active: bool,
    pub total_staked: u64,
}

impl ValidatorStats {
    pub fn new(addr: &str, stake: u64) -> Self {
        Self {
            address: addr.to_string(),
            missed_blocks: 0,
            is_active: true,
            total_staked: stake,
        }
    }

    // NEW: Logic to detect if a validator is trying to fork the chain
    pub fn verify_signature(&mut self, _height: u32, block_hash: &str, last_hash: &mut String) -> Option<SlashReason> {
        if !last_hash.is_empty() && block_hash != *last_hash {
            self.is_active = false;
            return Some(SlashReason::DoubleSigning);
        }
        *last_hash = block_hash.to_string();
        None
    }

    pub fn check_slashing(&mut self) -> Option<SlashReason> {
        if self.missed_blocks > 100 {
            self.is_active = false;
            return Some(SlashReason::Downtime);
        }
        None
    }
}
