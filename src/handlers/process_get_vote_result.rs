use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::state::VoteAccount;

pub fn process_get_vote_result(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let vote_account = next_account_info(&mut accounts.iter())?;

    if vote_account.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }
    let account_data_ref = vote_account.data.borrow_mut();
    let vote_data = VoteAccount::try_from_slice(&account_data_ref)?;

    msg!("Vote Pool: Name{:?}", vote_data.name);
    msg!("Vote Pool: Result{:?}", vote_data.result);

    Ok(())
}
