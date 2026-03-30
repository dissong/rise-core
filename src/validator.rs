#[derive(Debug, Clone, PartialEq)]
pub enum ValidatorStatus {
    Active,
    Unbonding(u32), 
    Inactive,
}

pub struct ProtocolState {
    pub total_supply: u64,
    pub burned_fees: u64,
    pub treasury_balance: u64,
}

impl ProtocolState {
    pub fn new() -> Self {
        Self {
            total_supply: 1_000_000,
            burned_fees: 0,
            treasury_balance: 50_000,
        }
    }

    pub fn report(&self) {
        println!("\n📊 --- RISE PROTOCOL STATE EXPLORER ---");
        println!("🔥 Total $RYZE Burned:   {} units", self.burned_fees);
        println!("🏦 Treasury Holdings:   {} units", self.treasury_balance);
        println!("📈 Net Supply:          {} units", self.total_supply - self.burned_fees);
        println!("----------------------------------------\n");
    }
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

    pub fn start_unbonding(&mut self, current_block: u32) {
        let release_at = current_block + 50; 
        self.status = ValidatorStatus::Unbonding(release_at);
        println!("🔒 Escrow Engaged: Funds held until block {}", release_at);
    }

    pub fn check_escrow_release(&mut self, current_block: u32) {
        if let ValidatorStatus::Unbonding(release_height) = self.status {
            if current_block >= release_height {
                self.status = ValidatorStatus::Inactive;
                println!("🔓 Escrow Released: Validator may now withdraw stake.");
            }
        }
    }
}
