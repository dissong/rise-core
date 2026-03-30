// src/validator.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ValidatorStatus {
    Active,
    Unbonding(u32), // Block height when funds become withdrawable
    Inactive,
    Jailed,
}

pub struct ValidatorStats {
    pub address: String,
    pub status: ValidatorStatus,
    pub missed_blocks: u32,
    pub total_staked: u64,
}

impl ValidatorStats {
    pub fn new(addr: &str, stake: u64) -> Self {
        Self {
            address: addr.to_string(),
            status: ValidatorStatus::Active,
            missed_blocks: 0,
            total_staked: stake,
        }
    }

    // Start the exit process (Escrow Handshake)
    pub fn start_unbonding(&mut self, current_block: u32) {
        if self.status == ValidatorStatus::Active {
            let release_at = current_block + 50; // 50 block security delay
            self.status = ValidatorStatus::Unbonding(release_at);
            println!("🔒 Validator {} entered Escrow. Release at block {}", self.address, release_at);
        }
    }

    pub fn check_escrow_release(&mut self, current_block: u32) {
        if let ValidatorStatus::Unbonding(release_height) = self.status {
            if current_block >= release_height {
                self.status = ValidatorStatus::Inactive;
                println!("🔓 Escrow period ended for {}. Funds are now withdrawable.", self.address);
            }
        }
    }
}
