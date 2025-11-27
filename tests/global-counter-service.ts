import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AppProgram } from "../target/types/app_program";
import { CounterService } from "../target/types/counter_service";

describe("global-counter-service", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.app_program as Program<AppProgram>;
  const counterProgram = anchor.workspace.counter_service as Program<CounterService>
  const payer = program.provider.wallet.payer;

  it("Should create a note", async () => {
    // Add your test here.
    const noteAccount = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .createNote(
        "This is the newest note that I have created. I think it is less than 200 symbols type shi"
      )
      .accounts({
        signer: program.provider.wallet.publicKey,
        note: noteAccount.publicKey,
      })
      .signers([payer, noteAccount])
      .rpc();

    const data = await program.account.noteData.fetch(noteAccount.publicKey);
    console.log(data);

    const pda = 0; // todo get the counter data
    const counterData = counterProgram.account;

    console.log("Your transaction signature", tx);
  });

  it("Should edit a note", async () => {
    const noteAccount = anchor.web3.Keypair.generate();
  })
});
