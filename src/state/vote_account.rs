pub struct VoteAccount {
    name: String,
    result: u32,
}

impl VoteAccount {
    pub fn new(name: String, result: u32) -> Self {
        VoteAccount { name, result }
    }
}
