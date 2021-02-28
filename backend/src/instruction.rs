use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Instruction {
  // [signer] token holder
  // [] token program
  // [] rent sysvar
  // [writable] token deposit address
  // [writable] data account
  StartAirdrop,

  // [signer] token holder
  // [] token program
  // [writable] token deposit address
  // [writable] data account
  FinishAirdrop,

  // [] token program
  // [writable] token deposit address
  // [writable] taker address
  TakeAirdrop { amount: u64 },
}
