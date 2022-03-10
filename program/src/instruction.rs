use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use spl_token;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TurnstileInstruction {
    /// Initialize a Turnstile state
    ///
    /// Passed accounts:
    ///
    /// (1) [signer, writable] State Account
    /// (2) [signer, writable] Initializer
    /// (3) [signer, writable] Mint Account
    /// (4) [] Treasury Account
    /// (5) [] Rent Sysvar
    /// (6) [] System Program
    /// (7) [] Token Program
    Initialze { init_state: bool },
    /// Exchange SOL for tokens
    ///
    /// Passed Accounts
    ///
    /// (1) [signer, writable] Users wallet
    /// (2) [writable] Treasury Account
    /// (3) [writable] Users token account
    /// (4) [writable] Mint account
    /// (5) [] Mint authority account
    /// (6) [] System Program
    /// (7) [] Token Program
    Exchange,
    /// Push
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Push,
    /// Coin
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] Users token account
    /// (2) [writable] Mint account
    /// (3) [signer] User wallet account (token account owner)
    /// (4) [writable] State Account
    /// (5) [] Token Program
    Coin,
}

// function which builds the Initialize instruction
pub fn initialize(
    turnstile_program: Pubkey,
    state: Pubkey,
    initializer: Pubkey,
    mint: Pubkey,
    treasury: Pubkey,
    init_state: bool,
) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(state, true),
            AccountMeta::new(initializer, true),
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(treasury, false),
            AccountMeta::new_readonly(rent::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: TurnstileInstruction::Initialze { init_state }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn exchange(
    turnstile_program: Pubkey,
    user_wallet: Pubkey,
    treasury: Pubkey,
    user_token_account: Pubkey,
    mint: Pubkey,
) -> Instruction {
    let (mint_authority_address, _mint_authority_bump) =
        Pubkey::find_program_address(&[&mint.to_bytes()], &turnstile_program);
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(user_wallet, true),
            AccountMeta::new(treasury, false),
            AccountMeta::new(user_token_account, false),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(mint_authority_address, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: TurnstileInstruction::Exchange.try_to_vec().unwrap(),
    }
}

pub fn coin(
    turnstile_program: Pubkey,
    user_token_account: Pubkey,
    mint: Pubkey,
    user_wallet: Pubkey,
    state: Pubkey,
) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(user_token_account, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(user_wallet, true),
            AccountMeta::new(state, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: TurnstileInstruction::Coin.try_to_vec().unwrap(),
    }
}

pub fn push(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Push.try_to_vec().unwrap(),
    }
}
