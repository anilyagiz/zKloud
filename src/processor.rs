use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    system_instruction,
    program::{invoke, invoke_signed},
    clock::Clock,
    sysvar::Sysvar,
    rent::Rent,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    error::ZKloudError,
    instruction::ZKloudInstruction,
    state::HardwareRental,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ZKloudInstruction::try_from_slice(instruction_data)?;

    match instruction {
        ZKloudInstruction::InitHardwareRental { hardware_id, rental_duration } => {
            process_init_hardware_rental(program_id, accounts, hardware_id, rental_duration)
        }
        ZKloudInstruction::CompleteRental { rental_id } => {
            process_complete_rental(accounts, rental_id)
        }
    }
}

fn process_init_hardware_rental(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    hardware_id: String,
    rental_duration: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let rental_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let blinks_program = next_account_info(account_info_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !verify_hardware_availability(&hardware_id) {
        return Err(ZKloudError::HardwareNotAvailable.into());
    }

    let rental_id = generate_rental_id(&hardware_id, user_account.key);
    let rent = Rent::get()?;
    let space = HardwareRental::get_account_size();
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            user_account.key,
            rental_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[user_account.clone(), rental_account.clone(), system_program.clone()],
    )?;

    let clock = Clock::get()?;
    let rental = HardwareRental {
        hardware_id,
        user: *user_account.key,
        start_time: clock.unix_timestamp,
        duration: rental_duration,
        is_active: true,
    };

    rental.serialize(&mut &mut rental_account.data.borrow_mut()[..])?;

    let rental_cost = calculate_rental_cost(&rental);
    process_blinks_payment(rental_cost, user_account, blinks_program)?;

    msg!("Hardware rental initialized: {}", rental_id);
    Ok(())
}

fn process_complete_rental(accounts: &[AccountInfo], rental_id: String) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let rental_account = next_account_info(account_info_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut rental = HardwareRental::try_from_slice(&rental_account.data.borrow())?;

    if rental.user != *user_account.key {
        return Err(ZKloudError::InvalidRental.into());
    }

    if !rental.is_active {
        return Err(ZKloudError::RentalNotActive.into());
    }

    rental.is_active = false;
    rental.serialize(&mut &mut rental_account.data.borrow_mut()[..])?;

    msg!("Rental completed: {}", rental_id);
    Ok(())
}

fn verify_hardware_availability(hardware_id: &str) -> bool {
    // TODO: Implement actual hardware availability check
    true
}

fn generate_rental_id(hardware_id: &str, user: &Pubkey) -> String {
    let clock = Clock::get().unwrap();
    format!("{}-{}-{}", hardware_id, user, clock.unix_timestamp)
}

fn calculate_rental_cost(rental: &HardwareRental) -> u64 {
    // TODO: Implement actual cost calculation logic
    let base_rate = 100; // Lamports per second
    base_rate * rental.duration
}

fn process_blinks_payment(amount: u64, payer: &AccountInfo, blinks_program: &AccountInfo) -> ProgramResult {
    // TODO: Implement actual Blinks payment integration
    msg!("Processing payment of {} lamports using Blinks", amount);
    Ok(())
}