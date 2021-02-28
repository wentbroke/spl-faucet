use crate::prelude::*;
use solana_program::program::invoke_signed;
use solana_program::program_pack::Pack;
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
};
use spl_token::state::Account;
use std::mem::size_of;

pub(crate) fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
  let instruction = if let Ok(instruction) = bincode::deserialize(instruction_data) {
    instruction
  } else {
    return Err(Error::MalformedInstruction.into());
  };

  match instruction {
    Instruction::StartAirdrop => {
      start_airdrop(program_id, accounts)?;
    }
    Instruction::FinishAirdrop => {
      finish_airdrop(program_id, accounts)?;
    }
    Instruction::TakeAirdrop { amount } => take_airdrop(program_id, accounts, amount)?,
  }

  Ok(())
}

fn start_airdrop(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
  let (pda, _bump_seed) = Pubkey::find_program_address(&[b"stamm"], program_id);
  let accounts = &mut accounts.iter();

  let wealthy = next_account_info(accounts)?;
  if !wealthy.is_signer {
    return Err(ProgramError::MissingRequiredSignature)?;
  }

  let token_program = next_account_info(accounts)?;
  let rent = &Rent::from_account_info(next_account_info(accounts)?)?;
  let deposit = next_account_info(accounts)?;
  let deposit_info = Account::unpack(&deposit.data.borrow())?;
  if 0 >= deposit_info.amount {
    return Err(ProgramError::InsufficientFunds);
  }
  let airdrop = next_account_info(accounts)?;
  if !rent.is_exempt(airdrop.lamports(), size_of::<Airdrop>()) {
    return Err(ProgramError::AccountNotRentExempt);
  }

  let ix = spl_token::instruction::set_authority(
    token_program.key,
    deposit.key,
    Some(&pda),
    spl_token::instruction::AuthorityType::AccountOwner,
    wealthy.key,
    &[&wealthy.key],
  )?;
  invoke(&ix, &[deposit.clone(), wealthy.clone(), token_program.clone()])?;

  let mut airdrop_data = Airdrop::unpack_unchecked(&airdrop.data.borrow())?;
  if airdrop_data.is_initialized {
    return Err(ProgramError::AccountAlreadyInitialized);
  }
  airdrop_data.is_initialized = true;
  airdrop_data.deposit = *deposit.key;
  airdrop_data.withdrawal = *wealthy.key;
  Airdrop::pack(airdrop_data, &mut airdrop.data.borrow_mut())?;

  Ok(())
}

fn finish_airdrop(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
  let (pda, bump_seed) = Pubkey::find_program_address(&[b"stamm"], program_id);
  let accounts = &mut accounts.iter();

  let wealthy = next_account_info(accounts)?;
  if !wealthy.is_signer {
    return Err(ProgramError::MissingRequiredSignature)?;
  }
  let token_program = next_account_info(accounts)?;
  let deposit = next_account_info(accounts)?;
  let deposit_data = Account::unpack(&deposit.data.borrow())?;
  let airdrop = next_account_info(accounts)?;
  let airdrop_data = Airdrop::unpack_unchecked(&airdrop.data.borrow())?;
  if wealthy.key != &airdrop_data.withdrawal {
    return Err(ProgramError::InvalidAccountData);
  }

  let ix = spl_token::instruction::transfer(
    token_program.key,
    deposit.key,
    &airdrop_data.withdrawal,
    &pda,
    &[&pda],
    deposit_data.amount,
  )?;
  invoke_signed(
    &ix,
    &[deposit.clone(), wealthy.clone(), token_program.clone()],
    &[&[&b"stamm"[..], &[bump_seed]]],
  )?;

  let ix = spl_token::instruction::close_account(token_program.key, deposit.key, wealthy.key, &pda, &[&pda])?;
  invoke_signed(
    &ix,
    &[deposit.clone(), wealthy.clone(), token_program.clone()],
    &[&[&b"stamm"[..], &[bump_seed]]],
  )?;

  **wealthy.lamports.borrow_mut() = wealthy
    .lamports()
    .checked_add(airdrop.lamports())
    .ok_or(Error::LamportsOverflow)?;
  **airdrop.lamports.borrow_mut() = 0;

  Ok(())
}

fn take_airdrop(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
  let (pda, bump_seed) = Pubkey::find_program_address(&[b"stamm"], program_id);
  let accounts = &mut accounts.iter();

  let token_program = next_account_info(accounts)?;
  let deposit = next_account_info(accounts)?;
  let receiver = next_account_info(accounts)?;

  let ix = spl_token::instruction::transfer(token_program.key, deposit.key, receiver.key, &pda, &[&pda], amount)?;
  invoke_signed(
    &ix,
    &[deposit.clone(), receiver.clone(), token_program.clone()],
    &[&[&b"stamm"[..], &[bump_seed]]],
  )?;

  Ok(())
}
