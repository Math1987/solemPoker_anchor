### How to run Way 01

anchor test
anchor test --skip-local-validator
anchor test --skip-local-validator --skip-deploy

### How to run Way 02:

- copy and rename `.env.example` to `.env` and change `ANCHOR_WALLET` address
- copy and rename `Anchor.toml.example` to `Anchor.toml` and change provider.wallet address
- open 3 split terminals, parallely
- on first terminal: `sh recursive2.sh`
- on second terminal: `solana logs`
- on third terminal: `sh recursive3.sh` Here you'll find the output of how methods are invoked in `client_invoker.mjs`


### Install Soteria
- sh -c "$(curl -k https://supercompiler.xyz/install)"
- export PATH=$PWD/soteria-linux-develop/bin/:$PATH
- To run soteria, invoke `soteria .` or `soteria -analyzeAll .` under the smart contract’s directory (where Xargo.toml exists)

### Prerequisites
- Update Solana version
- Update rustup rustc cargo version

### Notes
- When trying to run with same PDA accounts, but different GameType account on same ledger; it will result to an empty array "activeGamesInOneType: []" for test cases
- In this case prefer `anchor test`
