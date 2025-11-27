import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AppProgram } from "../target/types/app_program";

describe("global-counter-service", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.app_program as Program<AppProgram>;
  const payer = program.provider.wallet.payer;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createNote(
        "This is the newest note that I have created. I think it is less than 200 symbols type shi"
      )
      .accounts({
        signer: program.provider.wallet.publicKey,
      })
      .signers([payer])
      .rpc();

    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("notedata"), payer.publicKey.toBytes()],
      program.programId
    );
    const data = await program.account.noteData.fetch(pda);
    console.log(data);

    console.log("Your transaction signature", tx);
  });
});
