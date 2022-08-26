import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Codetest } from "../target/types/codetest";
import { expect } from "chai";

const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;


describe("Codetest", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Codetest as Program<Codetest>;
  let gameList = anchor.web3.Keypair.generate() ;
  // let data = anchor.web3.Keypair.generate();
  // let x;
  // let [gameListPda] = await anchor.web3.PublicKey.findProgramAddress(
  //   [Buffer.from("GAME_LIST")],
  //   program.programId
  // );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init().accounts({
      server: provider.wallet.publicKey,
      gameList:gameList.publicKey,
      systemProgram: SystemProgram.programId

    }).signers([gameList]).rpc();
    // console.log("Your transaction signature", tx);
    // const gameResult = await program.account.gameList.fetch(gameListPda);
    // // console.log("🚀 ~ file: Codetest.ts ~ line 33 ~ it ~ gameResult", gameResult.gameTypeIndex.valueOf())
    // x = gameResult.gameTypeIndex.toString();
    // // console.log("🚀 ~ file: Codetest.ts ~ line 36 ~ it ~ x", x)

    // const gameResult2 = await program.account.data.fetch(data.publicKey);
    // // console.log("🚀 ~ file: Codetest.ts ~ line 35 ~ it ~ gameResult2", gameResult2)

  });

  // it("it will Create a Game Type !", async () => {

  //   console.log("inside create")
  //   let [gametype] = await anchor.web3.PublicKey.findProgramAddress(
  //     // [(Buffer.from("GAME_TYPE"),Buffer.from(x))], 
  //     [
  //       Buffer.from("GAME_TYPE"),
  //       new Uint8Array(x)
  //     ],

  //     program.programId
  //   );
  //   console.log("🚀 ~ file: Codetest.ts ~ line 40 ~ it ~ gametype", gametype.toBase58())
  //   let [gamelist] = await anchor.web3.PublicKey.findProgramAddress(
  //     [Buffer.from("GAME_LIST")],
  //     program.programId
  //   );
  //   console.log("🚀 ~ file: Codetest.ts ~ line 45 ~ it ~ gamelist", gamelist.toBase58())

  //   try {
  //     await program.methods.createGameType(new anchor.BN(1 * LAMPORTS_PER_SOL), 3, 6).accounts({
  //       gameListPda: gamelist,
  //       authority: provider.wallet.publicKey,
  //       gameTypePda: gametype,
  //       systemProgram: SystemProgram.programId,

  //     }).signers([]).rpc();
  //   }
  //   catch (error) {
  //     console.log(error);
  //   }
  //   const gameResult = await program.account.gameList.fetch(gameListPda);
  //   console.log("🚀 ~ file: Codetest.ts ~ line 64 ~ it ~ gameResult", gameResult)
  //   const gameResult2 = await program.account.gameList.fetch(gametype);
  //   console.log("🚀 ~ file: Codetest.ts ~ line 66 ~ it ~ gameResult2", gameResult2)

  //   // console.log("Your transaction signature", tx);
  // });

});
