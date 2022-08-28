### How to run 
- copy and rename `.env.example` to `.env` and change `ANCHOR_WALLET` address
- copy and rename `Anchor.toml.example` to `Anchor.toml` and change provider.wallet address
- open 3 split terminals, parallely
- on first terminal: `sh recursive2.sh`
- on second terminal: `solana logs`
- on third terminal: `sh recursive3.sh` Here you'll find the output of how methods are invoked in `client_invoker.mjs`
