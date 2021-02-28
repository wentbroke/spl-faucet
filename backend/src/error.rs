use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub(crate) enum Error {
  #[error("MalformedInstruction")]
  MalformedInstruction,
}

impl From<Error> for ProgramError {
  fn from(e: Error) -> Self {
    ProgramError::Custom(e as u32)
  }
}
