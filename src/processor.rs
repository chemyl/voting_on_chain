use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instructions::voting::VotingInstructions;

use crate::handlers::process_get_vote_result;
use crate::handlers::process_initialize_voting;
use crate::handlers::process_voting;

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VotingInstructions::unpack(instruction_data)?;

    match instruction {
        VotingInstructions::InitializePool { result } => process_initialize_voting(result)?,
        VotingInstructions::Vote { option } => process_voting(option)?,
        VotingInstructions::GetVoteResult => process_get_vote_result()?,
    }
    Ok(())
}
