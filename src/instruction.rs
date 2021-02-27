use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum Instruction {
  // [signer] founder
  // [] token program
  // [writable] cash mint address
  // [writable] bond mint address
  // [writable] share mint address
  Genesis,
  // [signer] who wants to use stamm
  // Register,

  // [signer] who stakes sol
  // [writable] share account
  // BuyShare { amount: u64 },

  // [signer] who withdraws sol
  // [writable] share account
  // [writable] cash account
  // SellShare { amount: u64 },

  // [signer] who buys bonds
  // [writable] bond account
  // [writable] cash account
  // BuyBond { amount: u64 },

  // [signer] who sells bonds
  // [writable] bond account
  // [writable] cash account
  // SellBond { amount: u64 },

  // [signer] who buys sol
  // [writable] cash account
  // BuySol { amount: u64 },

  // [signer] who sells sol
  // [writable] cash account
  // SellSol { amount: u64 }
}
