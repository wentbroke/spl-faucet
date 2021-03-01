use serde::{Deserialize, Serialize};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
pub struct Airdrop {
  pub is_initialized: bool,
  pub deposit: Pubkey,
  pub withdrawal: Pubkey,
  pub amount: u64,
}

impl Sealed for Airdrop {}

impl IsInitialized for Airdrop {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Pack for Airdrop {
  const LEN: usize = std::mem::size_of::<Self>();

  fn pack_into_slice(&self, dst: &mut [u8]) {
    bincode::serialize_into(dst, &self).unwrap();
  }

  fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
    bincode::deserialize(input).map_err(|_| ProgramError::InvalidAccountData)
  }
}
