use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HardwareRental {
    pub hardware_id: String,
    pub user: Pubkey,
    pub start_time: i64,
    pub duration: u64,
    pub is_active: bool,
}

impl HardwareRental {
    pub fn get_account_size() -> usize {
        // Calculate the size based on the fields
        std::mem::size_of::<Self>() + 32 // Add extra space for future upgrades
    }
}