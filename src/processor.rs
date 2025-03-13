use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instructions::voting_instructions::VotingInstructions;

use crate::handlers::process_get_vote_result;
use crate::handlers::process_initialize_voting;
use crate::handlers::process_voting;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VotingInstructions::unpack(instruction_data)?;

    match instruction {
        VotingInstructions::InitializePool
        //  { name, result } 
         => {
            process_initialize_voting(program_id, accounts)?
        }
        VotingInstructions::Vote { option } => process_voting(program_id, accounts, option)?,
        VotingInstructions::GetVoteResult => process_get_vote_result(program_id, accounts)?,
    }

    Ok(())
}
