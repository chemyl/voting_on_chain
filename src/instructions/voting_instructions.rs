use borsh::{BorshDeserialize, BorshSerialize};
// use borsh::ser::BorshSerialize;
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VotingInstructions {
    InitializePool,
    Vote { option: u32 },
    GetVoteResult,
}

impl VotingInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => {
                let mut name_bytes = [0u8; 32];
                let rest_len = rest.len().min(32);
                name_bytes[..rest_len].copy_from_slice(&rest[..rest_len]);

                Ok(
                    Self::InitializePool, //     {
                                          //     name: name_bytes,
                                          //     result: 0,
                                          // }
                )
            }
            1 => {
                let user_option = u32::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Ok(Self::Vote {
                    option: user_option,
                })
            }
            2 => Ok(Self::GetVoteResult),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
