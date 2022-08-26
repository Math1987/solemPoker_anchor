import * as anchor from '@project-serum/anchor'
import { Connection, PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js'

import { BN } from 'bn.js'

import { Program } from "@project-serum/anchor";

import fs from "fs"

// // required for way 01 and 02: setting up anchor provider using a .env file
import dotenv from 'dotenv';
dotenv.config();


async function main() {


  // // required for way 02: setting up anchor provider using a .env file
  // let connection = new Connection("http://127.0.0.1:8899");
  // let wallet = Keypair.fromSecretKey(new Uint8Array([174,43,21,124,235,242,183,95,89,171,100,127,116,183,210,163,62,17,158,201,65,188,108,8,115,61,176,91,62,135,167,250,118,94,241,20,85,34,25,213,153,93,160,113,100,20,36,164,213,160,146,24,173,202,35,132,251,152,207,201,46,251,197,14]))

  const provider = anchor.AnchorProvider.local();                     // way 01 : local solana config get
  // const provider = anchor.AnchorProvider.env();                    // way 02 : using a .env file
  // const provider = new anchor.AnchorProvider(connection, wallet )  // way 03`: Hardcoding the details for provider here itself

  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider connection", provider.connection._rpcEndpoint);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider wallet", provider.wallet.publicKey.toBase58());

  anchor.setProvider(provider)

  const CodetestProgram = anchor.workspace.Codetest;
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram programId", CodetestProgram.programId.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram rpc", CodetestProgram.rpc);


  let gameList = anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 37 ~ main ~ gameList", gameList.publicKey.toBase58());
  let gameType = anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 39 ~ main ~ gameType", gameType.publicKey.toBase58());
  let player1=anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ player1", player1.publicKey.toBase58())
  let player2=anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 43 ~ main ~ player2", player2.publicKey.toBase58())
  let player3=anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 45 ~ main ~ player3", player3.publicKey.toBase58())
  let player4=anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ player4", player4.publicKey.toBase58())
  let player5=anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 49 ~ main ~ player5", player5.publicKey.toBase58())

  // create a data_account // this should be created only with the init method, and later should be saved
  // let data_account = anchor.web3.Keypair.generate();
  let data_account = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/data_account.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 35 ~ main ~ data_account", data_account.publicKey.toBase58());

  // // derive a global gameListPda account
  // let [gameListPda] = await anchor.web3.PublicKey.findProgramAddress(
  //   [Buffer.from("GAME_LIST")],
  //   CodetestProgram.programId
  // );
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ gameListPda", gameListPda.toBase58());

  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ provider.wallet.payer", provider.wallet.payer.publicKey.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ typeof(provider.wallet.payer)", typeof (provider.wallet.payer));


  // Invoking init Endpoint // required to invoke only one time
  let transaction_id = await CodetestProgram.methods
    .init()
    .accounts({
      server: provider.wallet.publicKey,
      gameList: gameList.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([gameList])
    .rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 42 ~ main ~ init transaction_id: ", transaction_id);

  // // Fetching Data from gameListPda account => pub struct GameList == CodetestProgram.account.gameList
  // let gameListPdaResult = await CodetestProgram.account.gameList.fetch(gameListPda);
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 61 ~ main ~ gameListPdaResult", gameListPdaResult);

  // // Assigning last_game_type_index to a variable
  // let last_game_type_index = gameListPdaResult.gameTypeIndex;
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 68 ~ main ~ last_game_type_index", last_game_type_index);
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 68 ~ main ~ typeof(last_game_type_index)", typeof (last_game_type_index));

  // // Fetching data inside the data account => pub struct Data == CodetestProgram.account.data
  // let data_accountResult = await CodetestProgram.account.data.fetch(data_account.publicKey);
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 73 ~ main ~ data_accountResult", data_accountResult);
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 73 ~ main ~ data_accountResult", data_accountResult.selectId);

  // // derive a last_game_type_index specific gameTypePda account
  // let [gameTypePda] = await anchor.web3.PublicKey.findProgramAddress(
  //   [
  //     Buffer.from("GAME_TYPE"),
  //     // new Uint8Array(last_game_type_index) // // numeric type check - It's not working here
  //     Buffer.from(last_game_type_index.toString()) // // hardcoded is working
  //   ],
  //   CodetestProgram.programId
  // );
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameListPda", gameListPda.toBase58());
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameTypePda", gameTypePda.toBase58());


  // Invoking createGameType Endpoint // required to invoke every time we create a new game type (different based on entry fee)
  transaction_id = await CodetestProgram.methods
    .createGameType(new BN(1 * LAMPORTS_PER_SOL), 3, 3)
    .accounts({
      gameList: gameList.publicKey,
      authority: provider.wallet.publicKey,
      gameType: gameType.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([gameType])
    .rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 102 ~ main ~ createGameType transaction_id", transaction_id);

  // Fetching Data from gameType account => pub struct GameList == CodetestProgram.account.gameList
  let gameTypeResult = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypeResult", gameTypeResult);
  console.log("====================>",gameTypeResult.lastGameIndex);
  let last_game_type_index=gameTypeResult.lastGameIndex
  // // gameListPda EJrpvgQEh7cVQ58H9WvXu3fmRw2TY3WG7Nj7XJtjPtsD
  // // gameTypePda with numeric type DZmAu2u9LtoXfUrhro3s4aTYpFsLPzjAJMEeDFMSaHMZ
  // // with "1" as hard coded string gameTypePda: 2K5s7K1Go45ThtQkRjXPeWnKTNVXyP5r5GSdrc6PRLAn
  // // authority GruSFAjP7gtmJ9k3SBAiCrMXyUByGJKR885MhKWM9KJD

  // transaction_id = await CodetestProgram.methods
  //   .createGameType(new BN(1 * LAMPORTS_PER_SOL), 3, 6)
  //   .accounts({
  //     gameList: gameList.publicKey,
  //     authority: provider.wallet.publicKey,
  //     gameType: gameType.publicKey,
  //     systemProgram: SystemProgram.programId
  //   })
  //   .signers([gameType])
  //   .rpc();
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 102 ~ main ~ createGameType transaction_id", transaction_id);
  const solemInc = new anchor.web3.PublicKey("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb") ;

  let [gameTypePda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        // new Uint8Array(last_game_type_index) // // numeric type check - It's not working here
        Buffer.from(last_game_type_index.toString()) // // hardcoded is working
      ],
      CodetestProgram.programId
    );
  console.log("🚀 ~ file: client_invoker.mjs ~ line 139 ~ main ~ gameTypePda", gameTypePda.toBase58())
  let [gameTreasuryPda] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("Treasury"),
    ],
    CodetestProgram.programId
  );
  const tx = await CodetestProgram.methods.addPlayer().accounts({
    player:player1.publicKey,
    gameList:gameList.publicKey,
    solemInc:solemInc,
    authority: provider.wallet.publicKey,
    gameTreasuryPda:gameTreasuryPda,
    gameType:gameType.publicKey,
    gamePda:gameTypePda,
    systemProgram: SystemProgram.programId
  }).signers([player1]).rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 161 ~ tx ~ tx", tx)

  const tx2 = await CodetestProgram.methods.addPlayer().accounts({
    player:player2.publicKey,
    gameList:gameList.publicKey,
    solemInc:solemInc,
    authority: provider.wallet.publicKey,
    gameTreasuryPda:gameTreasuryPda,
    gameType:gameType.publicKey,
    gamePda:gameTypePda,
    systemProgram: SystemProgram.programId
  }).signers([player2]).rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 173 ~ tx2 ~ tx2", tx2)
  const tx3 = await CodetestProgram.methods.addPlayer().accounts({
    player:player3.publicKey,
    gameList:gameList.publicKey,
    solemInc:solemInc,
    authority: provider.wallet.publicKey,
    gameTreasuryPda:gameTreasuryPda,
    gameType:gameType.publicKey,
    gamePda:gameTypePda,
    systemProgram: SystemProgram.programId
  }).signers([player3]).rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 184 ~ tx3 ~ tx3", tx3)
  const tx4 = await CodetestProgram.methods.addPlayer().accounts({
    player:player4.publicKey,
    gameList:gameList.publicKey,
    solemInc:solemInc,
    authority: provider.wallet.publicKey,
    gameTreasuryPda:gameTreasuryPda,
    gameType:gameType.publicKey,
    gamePda:gameTypePda,
    systemProgram: SystemProgram.programId
  }).signers([player4]).rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 195 ~ tx4 ~ tx4", tx4)
  console.log("🚀 ~ file: client_invoker.mjs ~ line 199~ main ~ gameTypeResult", gameTypeResult);
  const tx5 = await CodetestProgram.methods.addPlayer().accounts({
    player:player5.publicKey,
    gameList:gameList.publicKey,
    solemInc:solemInc,
    authority: provider.wallet.publicKey,
    gameTreasuryPda:gameTreasuryPda,
    gameType:gameType.publicKey,
    gamePda:gameTypePda,
    systemProgram: SystemProgram.programId
  }).signers([player5]).rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 213 ~ tx5 ~ tx5", tx5)

}

main()


// Privilege Extension
// CPIs extend the privileges of the caller to the callee. 
// The puppet account was passed as a mutable account to the puppet-master 
// but it was still mutable in the puppet program as well

// Privilege extension is convenient but also dangerous. 
// If a CPI is unintentionally made to a malicious program, 
// this program has the same privileges as the caller. 
// Anchor protects you from CPIs to malicious programs with two measures. 
// First, the Program<'info, T> type checks that the given account is the expected program T. 
// Should you ever forget to use the Program type, the automatically generated cpi function 
// (in the previous example this was puppet::cpi::set_data) also checks that the cpi_program argument equals the expected program.