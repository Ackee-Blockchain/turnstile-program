use crate::processor::process_instruction;
use solana_program::entrypoint;

// We use the entrypoint! macro to declare the process_instruction 
// function the entrypoint to the program. Entrypoints are the only
// way to call a program; all calls go through the function declared 
// as the entrypoint.
entrypoint!(process_instruction);
