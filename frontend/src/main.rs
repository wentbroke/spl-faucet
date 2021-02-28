use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use stamm_backend::instruction::Instruction as StammInstruction;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
  let mut args = env::args();
  args.next();
  let subcommand = args.next().expect("Give me a subcommand");

  match subcommand.as_ref() {
    "genesis" => genesis(),
    _ => panic!("Unknown subcommand"),
  }
}

fn keypair() -> Keypair {
  let file = File::open("~/.config/solana/id.json").expect("~/.config/solana/id.json not found");
  let reader = BufReader::new(file);
  let bytes: Vec<u8> = serde_json::from_reader(reader).unwrap();
  Keypair::from_bytes(&bytes).expect("Malformed keypair")
}

fn genesis() {
  let program: Pubkey = env::var("STAMM_PROGRAM")
    .expect("$STAMM_PROGRAM not set")
    .parse()
    .expect("$STAMM_PROGRAM malformed");
  let cash: Pubkey = env::var("STAMM_CASH")
    .expect("$STAMM_CASH not set")
    .parse()
    .expect("$STAMM_CASH malformed");
  let bond: Pubkey = env::var("STAMM_BOND")
    .expect("$STAMM_BOND not set")
    .parse()
    .expect("$STAMM_BOND malformed");
  let share: Pubkey = env::var("STAMM_SHARE")
    .expect("$STAMM_SHARE not set")
    .parse()
    .expect("$STAMM_SHARE malformed");

  let key = keypair();
  let accounts = vec![
    AccountMeta::new(key.pubkey(), true),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new(cash, false),
    AccountMeta::new(bond, false),
    AccountMeta::new(share, false),
  ];
  let ix = Instruction::new(program, &StammInstruction::Genesis, accounts);

  let conn = RpcClient::new("http://127.0.0.1:8899".to_string());
  let (hash, _) = conn.get_recent_blockhash().unwrap();
  let tx = Transaction::new_signed_with_payer(&[ix], Some(&key.pubkey()), &[&key], hash);
  conn.send_and_confirm_transaction(&tx).unwrap();
}
