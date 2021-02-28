use crate::prelude::*;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
};

pub(crate) fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
  let instruction = if let Ok(instruction) = bincode::deserialize(instruction_data) {
    instruction
  } else {
    return Err(Error::MalformedInstruction.into());
  };

  match instruction {
    Instruction::Genesis => {
      genesis(program_id, accounts)?;
    }
  }

  Ok(())
}

// Transfer the ownership of tokens to the PDA after ICO or something
fn genesis(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
  let (pda, _bump_seed) = Pubkey::find_program_address(&[b"stamm"], program_id);
  let accounts = &mut accounts.iter();

  let founder = next_account_info(accounts)?;
  if !founder.is_signer {
    return Err(ProgramError::MissingRequiredSignature)?;
  }

  let token_program = next_account_info(accounts)?;

  let cash_mint = next_account_info(accounts)?;
  if Ok(*cash_mint.key) != env!("STAMM_CASH").parse() {
    return Err(ProgramError::InvalidInstructionData)?;
  }
  let stop_minting_cash_ix = spl_token::instruction::set_authority(
    token_program.key,
    cash_mint.key,
    Some(&pda),
    spl_token::instruction::AuthorityType::MintTokens,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_minting_cash_ix,
    &[cash_mint.clone(), founder.clone(), token_program.clone()],
  )?;
  let stop_freezing_cash_ix = spl_token::instruction::set_authority(
    token_program.key,
    cash_mint.key,
    None,
    spl_token::instruction::AuthorityType::FreezeAccount,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_freezing_cash_ix,
    &[cash_mint.clone(), founder.clone(), token_program.clone()],
  )?;

  let bond_mint = next_account_info(accounts)?;
  if Ok(*bond_mint.key) != env!("STAMM_BOND").parse() {
    return Err(ProgramError::InvalidInstructionData)?;
  }
  let stop_minting_bond_ix = spl_token::instruction::set_authority(
    token_program.key,
    bond_mint.key,
    Some(&pda),
    spl_token::instruction::AuthorityType::MintTokens,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_minting_bond_ix,
    &[bond_mint.clone(), founder.clone(), token_program.clone()],
  )?;
  let stop_freezing_bond_ix = spl_token::instruction::set_authority(
    token_program.key,
    bond_mint.key,
    None,
    spl_token::instruction::AuthorityType::FreezeAccount,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_freezing_bond_ix,
    &[bond_mint.clone(), founder.clone(), token_program.clone()],
  )?;

  let share_mint = next_account_info(accounts)?;
  if Ok(*share_mint.key) != env!("STAMM_SHARE").parse() {
    return Err(ProgramError::InvalidInstructionData)?;
  }
  let stop_minting_share_ix = spl_token::instruction::set_authority(
    token_program.key,
    share_mint.key,
    Some(&pda),
    spl_token::instruction::AuthorityType::MintTokens,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_minting_share_ix,
    &[share_mint.clone(), founder.clone(), token_program.clone()],
  )?;
  let stop_freezing_share_ix = spl_token::instruction::set_authority(
    token_program.key,
    share_mint.key,
    Nonee,
    spl_token::instruction::AuthorityType::FreezeAccount,
    founder.key,
    &[&founder.key],
  )?;
  invoke(
    &stop_freezing_share_ix,
    &[share_mint.clone(), founder.clone(), token_program.clone()],
  )?;

  Ok(())
}
