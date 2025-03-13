use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct VoteAccount {
    pub name: Vec<u8>,
    pub result: u32,
}
