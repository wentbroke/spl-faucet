use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Instruction {
  // [signer] token holder
  // [] token program
  // [] rent sysvar
  // [writable] token deposit address
  // [writable] data account
  StartAirdrop { amount: u64 },

  // [signer] token holder
  // [] token program
  // [] owner of token deposit address
  // [writable] token deposit address
  // [writable] receiver address
  // [writable] data account
  FinishAirdrop,

  // [signer] taker sol address
  // [] token program
  // [] owner of token deposit address
  // [writable] token deposit address
  // [writable] data account
  // [writable] taker token address
  TakeAirdrop,
}
