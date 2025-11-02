use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use bytemuck::{Pod, Zeroable};

// vault_state, fundraiser_state

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("27abzM8KfWuiYyiy6T3Dv1EeJWSPuBK7DDjtBQoapEfP");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    match discriminator {
        0 => {
            // seeds
            // size of data .. [seeds, size, data]

            // tyest deser account
            let [mut first_account] = accounts else {
                return Err(pinocchio::program_error::ProgramError::InvalidInstructionData);
            };

            let our_data[0..IxHandler::LEN] = data;

            let initial_bump = [0u8];
            let bump = [initial_bump];
            let seed = [
                Seed::from(b"fundraiser"),
                Seed::from(first_account.key()),
                Seed::from(&bump),
            ];
            let seeds = Signer::from(&seed);
            CreateAccount {
                from: maker,
                lamports: Rent::get()?.minimum_balance(Fundraiser::LEN),
                owner: &crate::ID,
                space: Fundraiser::LEN as u64,
                to: fundraiser,
            }
            .invoke_signed(&[seeds])?;

            let mut some_fist_account = first_account.try_borrow_mut_data().unwrap();

            // this will modify the account state
            some_fist_account.copy_from_slice(&[0u8, 3u8, 5u8, 5u8]);

            Ok(())
        }
        _ => return Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct IxHandler {
    seeds: [u8; 8],
    size: [u8; 8],
}

impl IxHandler {
    pub const LEN: usize = core::mem::size_of::<IxHandler>();
}
