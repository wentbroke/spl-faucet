mod error;
mod instruction;
mod prelude;
mod processor;
mod state;
use processor::process;
use solana_program::entrypoint;

entrypoint!(process);
