use std::env;

use poc::assert_tx_success;
use poc_framework::{
    keypair, solana_sdk::{signer::Signer, signature::Keypair}, Environment, LocalEnvironment, PrintableTransaction,
};
use solana_program::{native_token::sol_to_lamports, pubkey::Pubkey, system_program};

use turnstile::state::State;
use spl_token::state::{Account, Mint};

pub struct Challenge {
    pub initializer: Pubkey,
    pub turnstile_program: Pubkey,
    pub state: Pubkey,
    pub mint: Pubkey,
    pub treasury: Pubkey,
    pub user_wallet: Keypair,
    pub user_token_account: Pubkey,
}

pub fn main() {
    let (mut env, fixture) = setup();
    test(&mut env, &fixture);
}

pub fn test(env: &mut LocalEnvironment, fixture: &Challenge) {

    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::exchange(
            fixture.turnstile_program,
            fixture.user_wallet.pubkey(),
            fixture.treasury,
            fixture.user_token_account,
            fixture.mint
        )],
        &[&fixture.user_wallet] 
    ))
    .print();

    let token_acc = env.get_unpacked_account::<Account>(fixture.user_token_account).unwrap();
    println!{"User token balance after exchange: {:?}", token_acc.amount};

    println!(
        "State after init: {:?}",
        env.get_deserialized_account::<State>(fixture.state)
            .unwrap()
    );
    // Unlock the turnstile by inserting a coin
    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::coin(
            fixture.turnstile_program,
            fixture.user_token_account,
            fixture.mint,
            fixture.user_wallet.pubkey(),
            fixture.state,
        )],
        &[&fixture.user_wallet],
    ));

    let token_acc = env.get_unpacked_account::<Account>(fixture.user_token_account).unwrap();
    println!{"User token balance after coin: {:?}", token_acc.amount};

    println!(
        "State after coin: {:?}",
        env.get_deserialized_account::<State>(fixture.state)
            .unwrap()
    );
    // Passing through turnstile thus making it locked again
    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::push(
            fixture.turnstile_program,
            fixture.state,
        )],
        &[],
    ));

    println!(
        "State after push: {:?}",
        env.get_deserialized_account::<State>(fixture.state)
            .unwrap()
    );
}

pub fn setup() -> (LocalEnvironment, Challenge) {
    let mut dir = env::current_exe().unwrap();
    let path = {
        dir.pop();
        dir.pop();
        dir.pop();
        dir.pop();
        dir.push("program/target/deploy/turnstile.so");
        dir.to_str()
    }
    .unwrap();

    let turnstile_program = keypair(1).pubkey();
    let initializer = keypair(42);
    let state = keypair(21);
    let treasury = keypair(22);
    let mint = keypair(23);
    let user = keypair(99);
    let user_token_account = keypair(98);

    let mut env = LocalEnvironment::builder()
        .add_program(turnstile_program, path)
        .add_account_with_lamports(
            initializer.pubkey(),
            system_program::ID,
            sol_to_lamports(10.0),
        )
        .add_account_with_lamports(
            user.pubkey(),
            system_program::ID,
            sol_to_lamports(10.0),
        )
        .add_account_with_tokens(
            user_token_account.pubkey(),
            mint.pubkey(),
            user.pubkey(), 
            0
        )
        .build();

    // Init TurnstileState
    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::initialize(
            turnstile_program,
            state.pubkey(),
            initializer.pubkey(),
            mint.pubkey(),
            treasury.pubkey(),
            true,
        )],
        &[&initializer, &state, &mint],
    ))
    .print();

    let minto = env.get_unpacked_account::<Mint>(mint.pubkey()).unwrap();
    println!{"Mint: {:?}", minto};
    let roken = env.get_unpacked_account::<Account>(user_token_account.pubkey()).unwrap();
    println!{"Token account: {:?}", roken};

    let challenge = Challenge {
        initializer: initializer.pubkey(),
        turnstile_program,
        state: state.pubkey(),
        mint: mint.pubkey(),
        treasury: treasury.pubkey(),
        user_wallet: user,
        user_token_account: user_token_account.pubkey(),
    };

    (env, challenge)
}
