import * as anchor from "@project-serum/anchor";
import { Connection, PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL, ComputeBudgetProgram } from '@solana/web3.js'

import { BN } from 'bn.js'

import { Program } from "@project-serum/anchor";

import { Codetest } from "../target/types/codetest";
import { expect } from "chai";

import fs from "fs"

// // required for way 01 and 02: setting up anchor provider using a .env file
import dotenv from 'dotenv';
dotenv.config();

// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.local(); // localhost
// const provider = anchor.AnchorProvider.local("https://api.devnet.solana.com");        // In case of devnet             // way 01 : local solana config get


// function airdrop_localhost(player_publickey_string: string) {
//   provider.connection.requestAirdrop(new PublicKey(player_publickey_string), anchor.web3.LAMPORTS_PER_SOL)
//     .then((airdrop_tx_id) => {
//       console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id)
//       provider.connection.getLatestBlockhash()
//         .then((latestBlockhash) => {
//           provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash })
//             .then((confirmation_response) => {
//               console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response)
//             });
//         });
//     });
// }

function airdrop_localhost(player_publickey_string: string) {
  provider.connection.requestAirdrop(new PublicKey(player_publickey_string), anchor.web3.LAMPORTS_PER_SOL)
    .then(async (airdrop_tx_id) => {
      console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
      let latestBlockhash = await provider.connection.getLatestBlockhash();
      let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
      console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
    });
}

describe("Codetest", async () => {

  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider connection", provider.connection.rpcEndpoint);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider wallet", provider.wallet.publicKey.toBase58());
  anchor.setProvider(provider);

  const CodetestProgram = anchor.workspace.Codetest as Program<Codetest>;
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram programId", CodetestProgram.programId.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram rpc", CodetestProgram.rpc);


  let gameList = anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 37 ~ main ~ gameList", gameList.publicKey.toBase58());
  let gameType = anchor.web3.Keypair.generate();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 39 ~ main ~ gameType", gameType.publicKey.toBase58());

  // Derive globalTresuryPda // this is used only for transferring commission to solemInc
  let [globalTreasuryPda] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("GlobalTreasury"),
    ],
    CodetestProgram.programId
  );
  console.log("🚀 ~ file: client_invoker.mjs ~ line 48 ~ main ~ globalTreasuryPda", globalTreasuryPda.toBase58());

  const solemInc = new anchor.web3.PublicKey("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb");

  // let player1=anchor.web3.Keypair.generate();
  let player1 = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/player1.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ player1", player1.publicKey.toBase58())

  // // // airdrop for localhost only
  // if (provider.connection.rpcEndpoint == "http://localhost:8899") {
  //   let airdrop_tx_id = await provider.connection.requestAirdrop(player1.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
  //   let latestBlockhash = await provider.connection.getLatestBlockhash();
  //   let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
  // }

  // let player2=anchor.web3.Keypair.generate();
  let player2 = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/player2.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 43 ~ main ~ player2", player2.publicKey.toBase58())

  // // // airdrop for localhost only
  // if (provider.connection.rpcEndpoint == "http://localhost:8899") {
  //   let airdrop_tx_id = await provider.connection.requestAirdrop(player2.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
  //   let latestBlockhash = await provider.connection.getLatestBlockhash();
  //   let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
  // }

  // let player3=anchor.web3.Keypair.generate();
  let player3 = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/player3.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 45 ~ main ~ player3", player3.publicKey.toBase58())

  // // // airdrop for localhost only
  // if (provider.connection.rpcEndpoint == "http://localhost:8899") {
  //   let airdrop_tx_id = await provider.connection.requestAirdrop(player3.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
  //   let latestBlockhash = await provider.connection.getLatestBlockhash();
  //   let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
  // }

  // let player4=anchor.web3.Keypair.generate();
  let player4 = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/player4.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ player4", player4.publicKey.toBase58())

  // // // airdrop for localhost only
  // if (provider.connection.rpcEndpoint == "http://localhost:8899") {
  //   let airdrop_tx_id = await provider.connection.requestAirdrop(player4.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
  //   let latestBlockhash = await provider.connection.getLatestBlockhash();
  //   let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
  // }

  // let player5=anchor.web3.Keypair.generate();
  let player5 = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/player5.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 49 ~ main ~ player5", player5.publicKey.toBase58())

  // // // airdrop for localhost only
  // if (provider.connection.rpcEndpoint == "http://localhost:8899") {
  //   let airdrop_tx_id = await provider.connection.requestAirdrop(player5.publicKey, anchor.web3.LAMPORTS_PER_SOL);
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 44 ~ main ~ airdrop_tx_id", airdrop_tx_id);
  //   let latestBlockhash = await provider.connection.getLatestBlockhash();
  //   let confirmation_response = await provider.connection.confirmTransaction({ signature: airdrop_tx_id, ...latestBlockhash });
  //   console.log("🚀 ~ file: client_invoker.mjs ~ line 47 ~ main ~ confirmation_response", confirmation_response);
  // }

  // synchronous airdrop
  airdrop_localhost(player1.publicKey.toBase58());
  airdrop_localhost(player2.publicKey.toBase58());
  airdrop_localhost(player3.publicKey.toBase58());
  airdrop_localhost(player4.publicKey.toBase58());
  airdrop_localhost(player5.publicKey.toBase58());

  /*            AIRDROP_COMPLETE        */
  console.log("Line 92: Airdrop Complete!")

  // create a data_account // this should be created only with the init method, and later should be saved
  // let data_account = anchor.web3.Keypair.generate();
  let data_account = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/data_account.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 35 ~ main ~ data_account", data_account.publicKey.toBase58());

  // console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ provider.wallet.payer", provider.wallet.payer.publicKey.toBase58());
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ typeof(provider.wallet.payer)", typeof (provider.wallet.payer));

  // default compute budget == 200_000 // addPlayer for 3rd player or the player that fulls the room takes compute units more than 200_000
  const additionalComputeBudgetInstruction = ComputeBudgetProgram.requestUnits({
    units: 400000,
    additionalFee: 0,
  });


  it("init method!", async () => {
    // Invoking init Endpoint // required to invoke only one time
    console.log("🚀 ~ file: Codetest.ts ~ line 129 ~ it ~ Invoking init Endpoint");
    let transaction_id = await CodetestProgram.methods
      .init()
      .accounts({
        server: provider.wallet.publicKey,
        gameList: gameList.publicKey, // data account
        systemProgram: SystemProgram.programId
      })
      .signers([gameList])
      .rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 42 ~ main ~ init transaction_id: ", transaction_id);

  });

  it("createGameType method!", async () => {
    console.log("🚀 ~ file: Codetest.ts ~ line 129 ~ it ~ Invoking createGameType Endpoint");
    let entry_fee_client = 0.01;
    let max_games_available_in_game_type = 3;
    let max_players_in_game_client = 3;
    // Invoking createGameType Endpoint // required to invoke every time we create a new game type (different based on entry fee)
    let transaction_id = await CodetestProgram.methods
      .createGameType(new BN(entry_fee_client * LAMPORTS_PER_SOL), max_games_available_in_game_type, max_players_in_game_client)
      .accounts({
        gameList: gameList.publicKey,
        authority: provider.wallet.publicKey,
        gameType: gameType.publicKey,   // data account
        systemProgram: SystemProgram.programId
      })
      .signers([gameType])
      .rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 102 ~ main ~ createGameType transaction_id", transaction_id);

  });

  it("addPlayer method: For Adding player 1", async () => {

    /*

  // Adding player 1

  */

    // Fetching Data from gameType data account => pub struct GameList == CodetestProgram.account.gameList
    let gameTypeResult_for_P1 = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypeResult_for_P1", gameTypeResult_for_P1);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 162 ~ main ~ gameTypeResult_for_P1.lastGameIndex", gameTypeResult_for_P1.lastGameIndex);

    let last_game_index_for_P1 = gameTypeResult_for_P1.lastGameIndex // this is required to chnage gamePda


    // First Time GAME PDA Check
    let [gamePda_P1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        // new Uint8Array(last_game_index) // // numeric type check - It's not working here
        Buffer.from(last_game_index_for_P1.toString()) // // string is working
      ],
      CodetestProgram.programId
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 191 ~ main ~ gamePda_P1: ${gamePda_P1.toBase58()}, last_game_index_for_P1: ${last_game_index_for_P1}`)

    const tx_P1 = await CodetestProgram.methods.addPlayer().accounts({
      player: player1.publicKey,
      gameList: gameList.publicKey,
      solemInc: solemInc,
      authority: provider.wallet.publicKey,
      globalTreasuryPda: globalTreasuryPda,
      gameType: gameType.publicKey,
      gamePda: gamePda_P1,
      systemProgram: SystemProgram.programId
    }).signers([player1]).preInstructions([additionalComputeBudgetInstruction]).rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 161 ~ tx ~ tx_P1", tx_P1)

    // After Player 1 is added check
    // Fetching Data from gamePda account => pub struct Game == CodetestProgram.account.game
    let gamePda_P1_Result = await CodetestProgram.account.game.fetch(gamePda_P1);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 210 ~ main ~ P1 gamePda_P1_Result after update: ", gamePda_P1_Result);

  });

  it("addPlayer method: For Adding player 2", async () => {

    /*

  // Adding player 2

  */

    // lastGameIndex isnt updated yet
    // if full = true, lastGameIndex will be updated
    let gameTypeResult_for_P2 = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypeResult_for_P2", gameTypeResult_for_P2);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 176 ~ main ~ gameTypeResult_for_P2.lastGameIndex", gameTypeResult_for_P2.lastGameIndex);

    let last_game_index_for_P2 = gameTypeResult_for_P2.lastGameIndex

    // Second Time GAME PDA Check
    let [gamePda_P2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        Buffer.from(last_game_index_for_P2.toString()) // // string is working
      ],
      CodetestProgram.programId
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 249 ~ main ~ gamePda_P2: ${gamePda_P2.toBase58()}, last_game_index_for_P2: ${last_game_index_for_P2}`)

    const tx_P2 = await CodetestProgram.methods.addPlayer().accounts({
      player: player2.publicKey,
      gameList: gameList.publicKey,
      solemInc: solemInc,
      authority: provider.wallet.publicKey,
      globalTreasuryPda: globalTreasuryPda,
      gameType: gameType.publicKey,
      gamePda: gamePda_P2,
      systemProgram: SystemProgram.programId
    }).signers([player2]).preInstructions([additionalComputeBudgetInstruction]).rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 200 ~ consttx_P2=awaitCodetestProgram.methods.addPlayer ~ tx_P2", tx_P2);

    // After Player 2 is added check
    // Fetching Data from gamePda account => pub struct Game == CodetestProgram.account.game
    let gamePda_P2_Result = await CodetestProgram.account.game.fetch(gamePda_P2);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 205 ~ main ~ gamePda_P2_Result after update:", gamePda_P2_Result);

  });

  it("addPlayer method: For Adding player 3", async () => {

    /*
 
   // Adding player 3
 
   */

    // lastGameIndex isnt updated yet
    // if full = true, lastGameIndex will be updated
    let gameTypeResult_for_P3 = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypeResult_for_P3", gameTypeResult_for_P3);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 212 ~ main ~ gameTypeResult_for_P3.lastGameIndex", gameTypeResult_for_P3.lastGameIndex);

    let last_game_index_for_P3 = gameTypeResult_for_P3.lastGameIndex

    // Third Time GAME PDA Check
    let [gamePda_P3] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        Buffer.from(last_game_index_for_P3.toString()) // // string is working
      ],
      CodetestProgram.programId
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 281 ~ main ~ gamePda_P3: ${gamePda_P3.toBase58()}, last_game_index_for_P3: ${last_game_index_for_P3}`)


    const tx_P3 = await CodetestProgram.methods.addPlayer().accounts({
      player: player3.publicKey,
      gameList: gameList.publicKey,
      solemInc: solemInc,
      authority: provider.wallet.publicKey,
      globalTreasuryPda: globalTreasuryPda,
      gameType: gameType.publicKey,
      gamePda: gamePda_P3,
      systemProgram: SystemProgram.programId
    }).signers([player3]).preInstructions([additionalComputeBudgetInstruction]).rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 236 ~ consttx_P3=awaitCodetestProgram.methods.addPlayer ~ tx_P3", tx_P3);

    // After Player 3 is added check
    // Fetching Data from gamePda account => pub struct Game == CodetestProgram.account.game
    let gamePda_P3_Result = await CodetestProgram.account.game.fetch(gamePda_P3);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 241 ~ main ~ gamePda_P3_Result", gamePda_P3_Result);

  });

  it("addPlayer method: For Adding player 4", async () => {

    /*
 
   // Adding player 4
 
   */

    // lastGameIndex isnt updated yet
    // if full = true, lastGameIndex will be updated
    let gameTypeResult_for_P4 = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypeResult_for_P4", gameTypeResult_for_P4);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 249 ~ main ~ gameTypeResult_for_P4.lastGameIndex", gameTypeResult_for_P4.lastGameIndex);

    let last_game_index_for_P4 = gameTypeResult_for_P4.lastGameIndex

    // Forth Time GAME PDA Check
    let [gamePda_P4] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        Buffer.from(last_game_index_for_P4.toString()) // // string is working
      ],
      CodetestProgram.programId
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 314 ~ main ~ gamePda_P4: ${gamePda_P4.toBase58()}, last_game_index_for_P4: ${last_game_index_for_P4}`)

    const tx_P4 = await CodetestProgram.methods.addPlayer().accounts({
      player: player4.publicKey,
      gameList: gameList.publicKey,
      solemInc: solemInc,
      authority: provider.wallet.publicKey,
      globalTreasuryPda: globalTreasuryPda,
      gameType: gameType.publicKey,
      gamePda: gamePda_P4,  // client side is sending the new game PDA
      systemProgram: SystemProgram.programId
    }).signers([player4]).preInstructions([additionalComputeBudgetInstruction]).rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 272 ~ consttx_P4=awaitCodetestProgram.methods.addPlayer ~ tx_P4", tx_P4);

    // After Player 4 is added check
    // Fetching Data from gamePda account => pub struct Game == CodetestProgram.account.game
    let gamePda_P4_Result = await CodetestProgram.account.game.fetch(gamePda_P4);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 276 ~ main ~ gamePda_P4_Result", gamePda_P4_Result);

  });

  it("addPlayer method: For Adding player 5", async () => {

    /*
  
    // Adding player 5
  
    */

    // lastGameIndex isnt updated yet
    // if full = true, lastGameIndex will be updated
    let gameTypeResult_for_P5 = await CodetestProgram.account.gameType.fetch(gameType.publicKey);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 281 ~ main ~ gameTypeResult_for_P5", gameTypeResult_for_P5);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 283 ~ main ~ gameTypeResult_for_P5.lastGameIndex", gameTypeResult_for_P5.lastGameIndex);

    let last_game_index_for_P5 = gameTypeResult_for_P5.lastGameIndex

    // Fifth Time GAME PDA Check
    let [gamePda_P5] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("GAME"),
        Buffer.from(last_game_index_for_P5.toString()) // // string is working
      ],
      CodetestProgram.programId
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 346 ~ main ~ gamePda_P5: ${gamePda_P5.toBase58()}, last_game_index_for_P5: ${last_game_index_for_P5}`)


    const tx_P5 = await CodetestProgram.methods.addPlayer().accounts({
      player: player5.publicKey,
      gameList: gameList.publicKey,
      solemInc: solemInc,
      authority: provider.wallet.publicKey,
      globalTreasuryPda: globalTreasuryPda,
      gameType: gameType.publicKey,
      gamePda: gamePda_P5,
      systemProgram: SystemProgram.programId
    }).signers([player5]).preInstructions([additionalComputeBudgetInstruction]).rpc();
    console.log("🚀 ~ file: client_invoker.mjs ~ line 307 ~ consttx_P5=awaitCodetestProgram.methods.addPlayer ~ tx_P5", tx_P5);

    // After Player 5 is added check
    // Fetching Data from gamePda account => pub struct Game == CodetestProgram.account.game
    let gamePda_P5_Result = await CodetestProgram.account.game.fetch(gamePda_P5);
    console.log("🚀 ~ file: client_invoker.mjs ~ line 312 ~ main ~ gamePda_P5_Result", gamePda_P5_Result);

    // Printing Balance of Global Treasury PDA
    let global_treasury_bal = await provider.connection.getBalance(globalTreasuryPda)
    console.log("🚀 ~ file: client_invoker.mjs ~ line 358 ~ main ~ global_treasury_bal", global_treasury_bal / anchor.web3.LAMPORTS_PER_SOL);

  });

});

