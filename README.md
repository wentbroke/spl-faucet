# SPL Token Faucet
## Install CLI
```
git clone https://github.com/wentbroke/spl-faucet.git
cd frontend
cargo build --release
mv target/release/spl-faucet /usr/local/bin/
export SPL_FAUCET_HOST=http://127.0.0.1:8899
```

## Deploy smart contract
```
git clone https://github.com/wentbroke/spl-faucet.git
cd backend
cargo build-bpf
export SPL_FAUCET_PROGRAM_ADDRESS="$(solana deploy target/deploy/stamm_backend.so | jq .programId -r)"
```