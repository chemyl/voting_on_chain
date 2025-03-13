use solana_program::hash::Hash;
use solana_program_test::{BanksClient, ProgramTest, processor};

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

async fn _setup(program_id: Pubkey) -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "voting_on_chain",
        program_id,
        processor!(voting_on_chain::processor::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}
