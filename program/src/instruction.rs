use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

// instruction.rs defines the "API" of a program

// by deriving BorshDeserialize and BorshSerialize we are able to easily 
// serialize and deserialize this enum
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TurnstileInstruction {
    /// Initialize a Turnstile state
    ///
    /// Passed accounts:
    ///
    /// (1) [signer, writable] State Account
    /// (2) [signer, writable] Initializer
    /// (3) [] System Program
    Initialze { init_state: bool },     // (de)serialization   Intialize {init_state: true} <-> [0,1]  
                                        //                     Intialize {init_state: false} <-> [0,0]
    /// Push
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Push,                               // (de)serialization   Push <-> [1]  
    /// Coin
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Coin,                               // (de)serialization   Coin <-> [2]  
}

// function which builds the Initialize instruction
pub fn initialize(
    turnstile_program: Pubkey,
    state: Pubkey,
    initializer: Pubkey,
    init_state: bool,
) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(state, true),
            AccountMeta::new(initializer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TurnstileInstruction::Initialze { init_state } // [0, init_state]
            .try_to_vec()
            .unwrap(),
    }
}

// function which builds the Coin instruction
pub fn coin(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Coin.try_to_vec().unwrap(), // [2]
    }
}

// function which builds the Push instruction
pub fn push(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Push.try_to_vec().unwrap(), // [1]
    }
}
