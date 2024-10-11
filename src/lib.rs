use solana_program::entrypoint;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

entrypoint!(processor::process_instruction);

# src/error.rs
use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug)]
pub enum ZKloudError {
    #[error("Hardware not available")]
    HardwareNotAvailable,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Invalid rental")]
    InvalidRental,
    #[error("Rental not active")]
    RentalNotActive,
}

impl From<ZKloudError> for ProgramError {
    fn from(e: ZKloudError) -> Self {
        ProgramError::Custom(e as u32)
    }
}