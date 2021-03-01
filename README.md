# SPL Token Faucet
With SPL Token Faucet, developers are now able to try DeFi products on Solana testnet much easier.

[Currently running on devnet.](https://explorer.solana.com/address/BiXcrzmpiCuesk4mPhNQLKEKux3qCoUrEgsNgLD8hk3k?cluster=devnet)

## Usage
```
export SPL_FAUCET_RPC=https://devnet.solana.com
export SPL_FAUCET_PROGRAM_ID=8uZDZjPjrBzPgK1KuE9EMgPWARMeKmSAHCLzsqZPqvJP
```

### Open a faucet
`$ cat ~/.config/solana/id.json | spl-faucet open <your token account> <amount you would like to put> <amount one will take each time>`
This command opens a faucet with your funds, and gives us its faucet address.

---

# Usage
## Take token from a faucet
`$ cat ~/.config/solana/id.json | spl-faucet take <faucet address> <your token account>`
This command make the faucet send you a token. qty is specified by who opened the faucet, as described above.

---

# Usage
## Close a faucet
`$ cat ~/.config/solana/id.json | spl-faucet close <faucet address> <your token account>`
This command will close the faucet you opened, and sends you back its funds.

## Development
### Install CLI
```
git clone https://github.com/wentbroke/spl-faucet.git
cd frontend
cargo build --release
mv target/release/spl-faucet-frontend /usr/local/bin/spl-faucet
export SPL_FAUCET_RPC=http://127.0.0.1:8899
```

### Deploy smart contract
```
git clone https://github.com/wentbroke/spl-faucet.git
cd backend
cargo build-bpf
export SPL_FAUCET_PROGRAM_ID="$(solana deploy target/deploy/spl_faucet_backend.so | jq .programId -r)"
```