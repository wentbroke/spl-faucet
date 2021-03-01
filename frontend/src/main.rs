use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_pack::Pack;
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
      &args.next().expect("Give me an amount to deposit"),
      &args.next().expect("Tell me how much token we can take each time"),
    ),
    "finish" => finish_airdrop(
      &args.next().expect("Give me an airdrop account"),
      &args.next().expect("Give me an account to receive token"),
    ),
    "take" => take_airdrop(
      &args.next().expect("Give me an airdrop account"),
      &args.next().expect("Give me an address to send tokens"),
    ),
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

fn start_airdrop(maker: &str, deposit_amount: &str, take_amount: &str) {
  let maker: Pubkey = maker.parse().expect("Malformed public key");
  let deposit_amount: u64 = deposit_amount.parse().expect("Give me a number");
  let take_amount: u64 = take_amount.parse().expect("Give me a number");

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
    Account::LEN as u64,
    &spl_token::id(),
  );
  let initialize_deposit_ix =
    spl_token::instruction::initialize_account2(&spl_token::id(), &deposit.pubkey(), &mint, &key.pubkey()).unwrap();
  let transfer_to_deposit_ix = spl_token::instruction::transfer(
    &spl_token::id(),
    &maker,
    &deposit.pubkey(),
    &key.pubkey(),
    &[],
    deposit_amount,
  )
  .unwrap();
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
    AccountMeta::new(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(rent::id(), false),
    AccountMeta::new(deposit.pubkey(), false),
    AccountMeta::new(airdrop.pubkey(), false),
  ];
  let start_airdrop_ix = Instruction::new(
    program(),
    &StammInstruction::StartAirdrop { amount: take_amount },
    accounts,
  );

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
    &[&key, &deposit, &airdrop],
    hash,
  );
  conn.send_and_confirm_transaction(&tx).unwrap();

  println!("Airdrop Address: {}", airdrop.pubkey());
}

fn finish_airdrop(airdrop: &str, receiver: &str) {
  let airdrop: Pubkey = airdrop.parse().expect("Malformed public key");
  let receiver: Pubkey = receiver.parse().expect("Malformed public key");
  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let airdrop_bytes = conn.get_account_data(&airdrop).unwrap();
  let airdrop_data = Airdrop::unpack(&airdrop_bytes).unwrap();
  let deposit_owner = conn
    .get_token_account(&airdrop_data.deposit)
    .unwrap()
    .unwrap()
    .owner
    .parse()
    .unwrap();
  let key = keypair();

  let accounts = vec![
    AccountMeta::new_readonly(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(deposit_owner, false),
    AccountMeta::new(airdrop_data.deposit, false),
    AccountMeta::new(receiver, false),
    AccountMeta::new(airdrop, false),
  ];
  let finish_airdrop_ix = Instruction::new(program(), &StammInstruction::FinishAirdrop, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(&[finish_airdrop_ix], Some(&key.pubkey()), &[&key], hash);
  conn.send_and_confirm_transaction(&tx).unwrap();
}

fn take_airdrop(airdrop: &str, taker: &str) {
  let taker: Pubkey = taker.parse().expect("Malformed public key");
  let airdrop: Pubkey = airdrop.parse().expect("Malformed public key");
  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let airdrop_bytes = conn.get_account_data(&airdrop).unwrap();
  let airdrop_data = Airdrop::unpack(&airdrop_bytes).unwrap();
  let deposit_owner = conn
    .get_token_account(&airdrop_data.deposit)
    .unwrap()
    .unwrap()
    .owner
    .parse()
    .unwrap();
  let key = keypair();

  let accounts = vec![
    AccountMeta::new_readonly(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(deposit_owner, false),
    AccountMeta::new(airdrop_data.deposit, false),
    AccountMeta::new(airdrop, false),
    AccountMeta::new(taker, false),
  ];
  let take_airdrop_ix = Instruction::new(program(), &StammInstruction::TakeAirdrop, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(&[take_airdrop_ix], Some(&key.pubkey()), &[&key], hash);
  conn.send_and_confirm_transaction(&tx).unwrap();
}
