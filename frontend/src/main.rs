use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::system_instruction;
use solana_sdk::sysvar::rent;
use solana_sdk::transaction::Transaction;
use spl_token::state::Account;
use stamm_backend::instruction::Instruction as StammInstruction;
use stamm_backend::state::Airdrop;
use std::env;
use std::io::stdin;
use std::mem::size_of;

fn main() {
  let mut args = env::args();
  args.next();
  let subcommand = args.next().expect("Give me a subcommand");

  match subcommand.as_ref() {
    "start" => start_airdrop(
      &args.next().expect("Give me a token account"),
      &args.next().expect("Give me an amount to airdrop"),
    ),
    // "finish" => finish_airdrop(args.next().expect("Give me an airdrop account")),
    // "take" => take_airdrop(args.next().expect("Specify an amount to take")),
    _ => panic!("Unknown subcommand"),
  }
}

fn keypair() -> Keypair {
  let bytes: Vec<u8> = serde_json::from_reader(stdin()).unwrap();
  Keypair::from_bytes(&bytes).expect("Malformed keypair")
}

fn program() -> Pubkey {
  env::var("STAMM_PROGRAM")
    .expect("$STAMM_PROGRAM not set")
    .parse()
    .expect("$STAMM_PROGRAM malformed")
}

fn start_airdrop(maker: &str, amount: &str) {
  let maker: Pubkey = maker.parse().expect("Malformed public key");
  let amount: u64 = amount.parse().expect("Give me a number");

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let mint: Pubkey = conn.get_token_account(&maker).unwrap().unwrap().mint.parse().unwrap();

  let key = keypair();
  let deposit = Keypair::new();
  let airdrop = Keypair::new();

  let create_deposit_ix = system_instruction::create_account(
    &key.pubkey(),
    &deposit.pubkey(),
    conn
      .get_minimum_balance_for_rent_exemption(size_of::<Account>())
      .unwrap(),
    size_of::<Account>() as u64,
    &program(),
  );
  let initialize_deposit_ix =
    spl_token::instruction::initialize_account(&program(), &deposit.pubkey(), &mint, &key.pubkey()).unwrap();
  let transfer_to_deposit_ix =
    spl_token::instruction::transfer(&program(), &account, &deposit.pubkey(), &key.pubkey(), &[], amount).unwrap();
  let create_airdrop_account_ix = system_instruction::create_account(
    &key.pubkey(),
    &airdrop.pubkey(),
    conn
      .get_minimum_balance_for_rent_exemption(size_of::<Airdrop>())
      .unwrap(),
    size_of::<Airdrop>() as u64,
    &program(),
  );
  let accounts = vec![
    AccountMeta::new_readonly(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(rent::id(), false),
    AccountMeta::new(deposit.pubkey(), false),
    AccountMeta::new(airdrop.pubkey(), false),
  ];
  let start_airdrop_ix = Instruction::new(program(), &StammInstruction::StartAirdrop, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(
    &[
      create_deposit_ix,
      initialize_deposit_ix,
      transfer_to_deposit_ix,
      create_airdrop_account_ix,
      start_airdrop_ix,
    ],
    Some(&key.pubkey()),
    &[&key],
    hash,
  );
  conn.send_and_confirm_transaction(&tx).unwrap();
}

fn finish_airdrop(airdrop: &str) {
  let airdrop: Pubkey = airdrop.parse().expect("Malformed public key");
  let key = keypair();

  let accounts = vec![
    AccountMeta::new_readonly(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new(deposit.pubkey(), false),
    AccountMeta::new(airdrop.pubkey(), false),
  ];
  let finish_airdrop_ix = Instruction::new(program(), &StammInstruction::FinishAirdrop, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(&[finish_airdrop_ix], Some(&key.pubkey()), &[&key], hash);
  conn.send_and_confirm_transaction(&tx).unwrap();
}

fn take_airdrop(taker: &str, deposit: &str, amount: &str) {
  let taker: Pubkey = taker.parse().expect("Malformed public key");
  let deposit: Pubkey = deposit.parse().expect("Malformed public key");
  let amount: u64 = amount.parse().expect("Give me a number");
  let key = keypair();

  let accounts = vec![
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new(deposit.pubkey(), false),
    AccountMeta::new(taker.pubkey(), false),
  ];
  let take_airdrop_ix = Instruction::new(program(), &StammInstruction::TakeAirdrop { amount }, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(&[take_airdrop_ix], Some(&key.pubkey()), &[&key], hash);
  conn.send_and_confirm_transaction(&tx).unwrap();
}
