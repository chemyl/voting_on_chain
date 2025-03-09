use solana_program::program_error::ProgramError;

pub enum VotingInstructions {
    InitializePool { result: u32 }, // var 0
    Vote { option: u32 },           // var 1
    GetVoteResult,                  // var 2
}

impl VotingInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => Ok(Self::InitializePool { result: 0 }),
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
