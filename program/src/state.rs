use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// we use state.rs to encode state into or decode the state of an account which
// has been passed into the entrypoint.

// by deriving BorshDeserialize and BorshSerialize we are able to easily
// serialize and deserialize this structure

// Example
//
// now we are able to convert state into byte-array and vice versa
//
// State { locked: false } <-> [0]
// State { locked: true } <-> [1]
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct State {
    pub locked: bool,
    // mint uniquely defines a token. This token will then be accepted by our turnstile
    pub mint: Pubkey,
    // an account that will receive SOLs earned by our turnstile
    pub treasury: Pubkey,
}

impl State {
    pub const SERIALZED_SIZE: usize = 1 + 32 + 32;
}
