use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{instruction::TurnstileInstruction, state::State};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // The processor asks instruction.rs to decode the instruction_data argument
    let instruction = TurnstileInstruction::try_from_slice(instruction_data)?;

    // Using the decoded data, the processor will now decide which processing 
    // function to use to process the request.
    match instruction {
        TurnstileInstruction::Initialze { init_state } => {
            initialize(program_id, accounts, init_state)
        }
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
    let initialezer_account_info = next_account_info(account_into_iter)?;

    // rent is deducted from an account's balance according to their space requirements 
    // (i.e. the space an account and its fields take up in memory) regularly. 
    // An account can, however, be made rent-exempt if its balance is higher than some 
    // threshold that depends on the space it's consuming
    let rent = Rent::get()?;
    // Cross Program Invocation of system program.
    // We used the invoke method from solana_program crate which will invoke
    // an input instruction (in our case the create_account)
    invoke(
        // create_account function builds the create_account instruction
        &system_instruction::create_account(
            &initialezer_account_info.key,
            &state_account_info.key,
            // method minimum_balance will tell us how many lamports we have to pay, 
            // in order to make our newly created account rent-exempt
            rent.minimum_balance(State::SERIALZED_SIZE),
            State::SERIALZED_SIZE as u64,
            &program_id,
        ),
        // accounts needed by the create_account instruction
        &[initialezer_account_info.clone(), state_account_info.clone()],
    )?;

    // State serialization using borsh
    let state = State { locked: init_state };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn coin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    // State serialization using borsh
    let state = State { locked: false }; // turnstile unlock
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn push(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    // State serialization using borsh
    let state = State { locked: true }; // turnstile lock
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}
