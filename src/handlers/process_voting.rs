use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::state::VoteAccount;

pub fn process_voting(program_id: &Pubkey, accounts: &[AccountInfo], option: u32) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let vote_account = next_account_info(account_iter)?;

    if vote_account.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let mut account_data_ref = vote_account.data.borrow_mut();

    let mut vote_data = VoteAccount::try_from_slice(&account_data_ref)?;

    match option {
        1 => {
            vote_data.result = vote_data
                .result
                .checked_add(1)
                .ok_or(ProgramError::InvalidArgument)?;
        }
        0 => {
            vote_data.result = vote_data
                .result
                .checked_sub(1)
                .ok_or(ProgramError::InvalidArgument)?;
        }
        _ => {
            return Err(ProgramError::InvalidArgument);
        }
    }

    vote_data.serialize(&mut &mut account_data_ref[..])?;

    Ok(())
}
