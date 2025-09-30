use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::Seed,
    program_error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

use crate::state::Escrow;

pub struct Make {}

pub fn process_make_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // validate accounts
    let [
        maker,
        escrow,
        vault,
        maker_ata_a,
        maker_ata_b,
        mint_a,
        mint_b,
        token_program,
        system_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // TODO finish up make

    let signer_seeds = [Seed::from(b"escrow"), Seed::from(maker.key()), Seed::from()];

    // create and init escrow account
    CreateAccount {
        from: maker,
        to: escrow,
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        space: Escrow::LEN as u64,
        owner: &crate::id(),
    }
    .invoke()?;
    Ok(())
}
