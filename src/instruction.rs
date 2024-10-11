use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ZKloudInstruction {
    /// Initializes a new hardware rental
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account of the person initializing the rental
    /// 1. `[writable]` The rental account, it will hold all necessary info about the rental
    /// 2. `[]` The system program
    /// 3. `[]` The Blinks program
    InitHardwareRental {
        hardware_id: String,
        rental_duration: u64,
    },

    /// Completes an existing rental
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account of the person who initialized the rental
    /// 1. `[writable]` The rental account to be closed
    CompleteRental {
        rental_id: String,
    },
}