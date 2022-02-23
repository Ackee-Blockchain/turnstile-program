use std::env;

use poc::assert_tx_success;
use poc_framework::{
    keypair, solana_sdk::signer::Signer, Environment, LocalEnvironment, PrintableTransaction,
};
use solana_program::{native_token::sol_to_lamports, pubkey::Pubkey, system_program};

use turnstile::state::State;

pub struct Challenge {
    pub initializer: Pubkey,
    pub turnstile_program: Pubkey,
    pub state: Pubkey,
}

pub fn main() {
    let (mut env, fixture) = setup();
    test(&mut env, &fixture);
}

pub fn test(env: &mut LocalEnvironment, fixture: &Challenge) {
    println!(
        "State after init: {:?}",
        env.get_deserialized_account::<State>(fixture.state)
            .unwrap()
    );
    // Unlock the turnstile by inserting a coin
    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::coin(
            fixture.turnstile_program,
            fixture.state,
        )],
        &[],
    ));

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

    let mut env = LocalEnvironment::builder()
        .add_program(turnstile_program, path)
        .add_account_with_lamports(
            initializer.pubkey(),
            system_program::ID,
            sol_to_lamports(10.0),
        )
        .build();

    // Init TurnstileState
    assert_tx_success(env.execute_as_transaction(
        &[turnstile::instruction::initialize(
            turnstile_program,
            state.pubkey(),
            initializer.pubkey(),
            true,
        )],
        &[&initializer, &state],
    ))
    .print();

    let challenge = Challenge {
        initializer: initializer.pubkey(),
        turnstile_program,
        state: state.pubkey(),
    };

    (env, challenge)
}
