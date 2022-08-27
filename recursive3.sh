anchor build
anchor deploy
node client_invoker.mjs
kill -9 $(lsof -t -i tcp:8899)
rm -rf test-ledger