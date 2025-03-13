use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::system_program;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::state::VoteAccount;

pub fn process_initialize_voting(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // name: [u8; 32],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let vote_account = next_account_info(account_iter)?;
    let payer_account = next_account_info(account_iter)?;
    let system_account = next_account_info(account_iter)?;

    if !system_program::check_id(system_account.key) {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if vote_account.data.borrow().len() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vote_name = "TEST POOL #1".to_string();

    let actual_size = std::mem::size_of::<VoteAccount>() + vote_name.as_bytes().len();
    let rent = Rent::get()?.minimum_balance(actual_size);

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            vote_account.key,
            rent,
            actual_size as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            vote_account.clone(),
            system_account.clone(),
        ],
    )?;

    let vote_data = VoteAccount {
        name: vote_name.try_to_vec()?,
        result: 0,
    }
    .try_to_vec()?;
    let serialized_data = borsh::to_vec(&vote_data)?;
    vote_account.data.borrow_mut()[..serialized_data.len()].copy_from_slice(&serialized_data);

    Ok(())
}
