use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instructions;
pub mod state;

entrypoint!(process_instruction);

use pinocchio_pubkey::declare_id;

use crate::instructions::process_make_instruction;
declare_id!("YourProgramIdHere");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_discriminator, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match ix_discriminator {
        0 => process_make_instruction(accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    };

    Ok(())
}
