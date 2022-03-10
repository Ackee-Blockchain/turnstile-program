use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    native_token::sol_to_lamports,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::{
    instruction::{burn, initialize_mint, mint_to},
    state::Mint,
};

use crate::{instruction::TurnstileInstruction, state::State};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TurnstileInstruction::try_from_slice(instruction_data)?;

    match instruction {
        TurnstileInstruction::Initialze { init_state } => {
            initialize(program_id, accounts, init_state)
        }
        TurnstileInstruction::Exchange => exchange(program_id, accounts),
        TurnstileInstruction::Coin => coin(program_id, accounts),
        TurnstileInstruction::Push => push(program_id, accounts),
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    init_state: bool,
) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;
    // someone who wants to create a turnstile (payer)
    let initialezer_account_info = next_account_info(account_into_iter)?;
    // mint uniquely defines a token. This token will then be accepted by our turnstile
    let mint_account_info = next_account_info(account_into_iter)?;
    // an account that will receive SOLs earned by our turnstile
    let treasury_account_info = next_account_info(account_into_iter)?;
    let rent_account_info = next_account_info(account_into_iter)?;

    // creation of state account
    let rent = Rent::get()?;
    invoke(
        &system_instruction::create_account(
            &initialezer_account_info.key,
            &state_account_info.key,
            rent.minimum_balance(State::SERIALZED_SIZE),
            State::SERIALZED_SIZE as u64,
            &program_id,
        ),
        &[initialezer_account_info.clone(), state_account_info.clone()],
    )?;

    // as our mint authority we want to use turnstile program and that's why
    // we must use a PDA derived from `program_id` as authority
    // This will ensure that only our program will be able to mint new tokens and no one else
    let (mint_authority_address, _mint_authority_bump) =
        Pubkey::find_program_address(&[&mint_account_info.key.to_bytes()], program_id);

    // creation of token mint account
    invoke(
        &system_instruction::create_account(
            &initialezer_account_info.key,
            &mint_account_info.key,
            rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &spl_token::id(),
        ),
        &[initialezer_account_info.clone(), mint_account_info.clone()],
    )?;
    
    // initialization of token mint account
    invoke(
        &initialize_mint(
            &spl_token::id(),
            mint_account_info.key,
            &mint_authority_address,
            None,
            0,
        )?,
        &[mint_account_info.clone(), rent_account_info.clone()],
    )?;

    // initialization of our state
    let state = State {
        locked: init_state,
        mint: *mint_account_info.key,
        treasury: *treasury_account_info.key,
    };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn exchange(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let user_wallet_info = next_account_info(account_into_iter)?;
    let treasury_info = next_account_info(account_into_iter)?;
    let user_token_account_info = next_account_info(account_into_iter)?;
    let mint_info = next_account_info(account_into_iter)?;
    let mint_authority_info = next_account_info(account_into_iter)?;

    // simple transfer from user's wallet to treasury wallet
    // `user_wallet` must sign exchange instruction
    invoke(
        &system_instruction::transfer(
            user_wallet_info.key,
            treasury_info.key,
            sol_to_lamports(1.0),
        ),
        &[user_wallet_info.clone(), treasury_info.clone()],
    )?;

    let (mint_authority_address, mint_authority_bump) =
        Pubkey::find_program_address(&[&mint_info.key.to_bytes()], program_id);

    // minting of new tokens signed by our program
    invoke_signed(
        &mint_to(
            &spl_token::id(),
            mint_info.key,
            user_token_account_info.key,
            &mint_authority_address,
            &[],
            5,
        )?,
        &[
            mint_info.clone(),
            user_token_account_info.clone(),
            mint_authority_info.clone(),
        ],
        // runtime will take this seeds and id of inovking program and hash them together ([signer_seeds] + program_id = PDA)
        // then it will search for an account with the same address as resulting PDA and marks it as signer
        &[&[&mint_info.key.to_bytes(), &[mint_authority_bump]]],
    )?;

    Ok(())
}

pub fn coin(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let user_token_account_info = next_account_info(account_into_iter)?;
    let mint_info = next_account_info(account_into_iter)?;
    let user_wallet_info = next_account_info(account_into_iter)?;
    let state_account_info = next_account_info(account_into_iter)?;

    // token burning -> simulates inserting of tokens into a turnstile
    invoke(
        &burn(
            &spl_token::id(),
            user_token_account_info.key,
            mint_info.key,
            user_wallet_info.key,
            &[],
            5,
        )?,
        &[
            user_token_account_info.clone(),
            mint_info.clone(),
            user_wallet_info.clone(),
        ],
    )?;

    // state update
    let mut state = State::try_from_slice(*state_account_info.data.borrow())?;

    state.locked = false;

    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn push(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    // state update
    let mut state = State::try_from_slice(*state_account_info.data.borrow())?;

    state.locked = true;

    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}
