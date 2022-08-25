import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Codetest } from "../target/types/codetest";
import { expect } from "chai";

const { SystemProgram ,LAMPORTS_PER_SOL} = anchor.web3 ;


describe("Codetest", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Codetest as Program<Codetest>;
  let gameList = anchor.web3.Keypair.generate() ;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init().accounts({
      gameList : gameList.publicKey,
        server : provider.wallet.publicKey,
        systemProgram : SystemProgram.programId

    }).signers([gameList]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("it will Create a Game Type !", async () => {
    let [gamePda] = await anchor.web3.PublicKey.findProgramAddress( 
      [gameList.publicKey.toBytes()], 
      program.programId
      );
      let [gametype] = await anchor.web3.PublicKey.findProgramAddress( 
        [Buffer.from("GAME_TYPE")], 
        program.programId
        );

    const tx = await program.methods.createGameType(new anchor.BN(1*LAMPORTS_PER_SOL),3,6).accounts({
      gameList:gameList.publicKey, 
      gamePda:gamePda,
      authority:provider.wallet.publicKey,
      game:gametype,
      systemProgram: SystemProgram.programId,

    }).signers([provider.wallet.payer]).rpc();
    console.log("Your transaction signature", tx);
  });

});
