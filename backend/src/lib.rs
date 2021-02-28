pub mod error;
pub mod instruction;
mod prelude;
mod processor;
pub mod state;

use processor::process;
use solana_program::entrypoint;

entrypoint!(process);
