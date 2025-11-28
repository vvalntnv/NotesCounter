import * as anchor from "@coral-xyz/anchor";
import { SendTransactionError } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { ProgramError } from "@coral-xyz/anchor";
import { AppProgram } from "../target/types/app_program";
import { CounterService } from "../target/types/counter_service";
import { assert } from "chai";

describe("global-counter-service", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.app_program as Program<AppProgram>;
  const counterProgram = anchor.workspace
    .counter_service as Program<CounterService>;
  const payer = program.provider.wallet.payer;

  it("Should create a note", async () => {
    // Add your test here.
    const noteAccount = anchor.web3.Keypair.generate();
    const seeds = [Buffer.from("userinfo"), payer.publicKey.toBytes()];
    const [pda] = await anchor.web3.PublicKey.findProgramAddress(
      seeds,
      counterProgram.programId
    );

    try {
      const tx = await program.methods
        .createNote(
          "This is the newest note that I have created. I think it is less than 200 symbols type shi"
        )
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount])
        .rpc();
    } catch (err) {
      if (err instanceof SendTransactionError) {
        console.log(await err.getLogs(program.provider.connection));
      }

      throw err;
    }

    const data = await program.account.noteData.fetch(noteAccount.publicKey);
    console.log(data);

    const counterData = await counterProgram.account.userCounter.fetch(pda);

    assert(counterData.createdCount.toNumber() === 1);

    console.log(counterData);
  });

  it("Should create second and third notes and verify counter", async () => {
    const noteAccount2 = anchor.web3.Keypair.generate();
    const noteAccount3 = anchor.web3.Keypair.generate();
    const seeds = [Buffer.from("userinfo"), payer.publicKey.toBytes()];
    const [pda] = await anchor.web3.PublicKey.findProgramAddress(
      seeds,
      counterProgram.programId
    );

    try {
      // Create second note
      await program.methods
        .createNote("This is my second note!")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount2.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount2])
        .rpc();

      // Check counter after second note
      let counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After second note:", counterData);
      assert(counterData.createdCount.toNumber() === 2);

      // Create third note
      await program.methods
        .createNote("This is my third note!")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount3.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount3])
        .rpc();

      // Check counter after third note
      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After third note:", counterData);
      assert(counterData.createdCount.toNumber() === 3);
    } catch (err) {
      if (err instanceof SendTransactionError) {
        console.log(await err.getLogs(program.provider.connection));
      }
      throw err;
    }
  });

  it("Should edit one note multiple times and verify edited counter", async () => {
    const noteAccount = anchor.web3.Keypair.generate();
    const seeds = [Buffer.from("userinfo"), payer.publicKey.toBytes()];
    const [pda] = await anchor.web3.PublicKey.findProgramAddress(
      seeds,
      counterProgram.programId
    );

    try {
      // First create the note
      await program.methods
        .createNote("Initial note content")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount])
        .rpc();

      // Get initial counter state
      let counterData = await counterProgram.account.userCounter.fetch(pda);
      const initialCreatedCount = counterData.createdCount.toNumber();
      console.log("Initial created count:", initialCreatedCount);
      console.log("initial edited", counterData.editedCount.toNumber());
      assert(counterData.editedCount.toNumber() === 0);

      // Edit the note first time
      const tx_id = await program.methods
        .editNote("First edit")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      const tx = await program.provider.connection.getTransaction(tx_id, {
        commitment: "confirmed",
      });

      console.log("logito, favorito", tx.meta.logMessages);

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After first edit:", counterData);
      assert(counterData.editedCount.toNumber() === 1);
      assert(counterData.createdCount.toNumber() === initialCreatedCount);

      // Edit the note second time
      await program.methods
        .editNote("Second edit")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After second edit:", counterData);
      assert(counterData.editedCount.toNumber() === 2);
      assert(counterData.createdCount.toNumber() === initialCreatedCount);

      // Edit the note third time
      await program.methods
        .editNote("Third edit")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After third edit:", counterData);
      assert(counterData.editedCount.toNumber() === 3);
      assert(counterData.createdCount.toNumber() === initialCreatedCount);
    } catch (err) {
      if (err instanceof SendTransactionError) {
        console.log(await err.getLogs(program.provider.connection));
      }
      throw err;
    }
  });

  it("Should edit two different notes and verify both counters", async () => {
    const noteAccount1 = anchor.web3.Keypair.generate();
    const noteAccount2 = anchor.web3.Keypair.generate();
    const seeds = [Buffer.from("userinfo"), payer.publicKey.toBytes()];
    const [pda] = await anchor.web3.PublicKey.findProgramAddress(
      seeds,
      counterProgram.programId
    );

    try {
      // Create first note
      await program.methods
        .createNote("First note content")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount1.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount1])
        .rpc();

      let counterData = await counterProgram.account.userCounter.fetch(pda);
      const createdAfterFirst = counterData.createdCount.toNumber();
      console.log("After creating first note:", counterData);

      // Create second note
      await program.methods
        .createNote("Second note content")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount2.publicKey,
          counter: pda,
        })
        .signers([payer, noteAccount2])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      const createdAfterSecond = counterData.createdCount.toNumber();
      console.log("After creating second note:", counterData);
      assert(createdAfterSecond === createdAfterFirst + 1);

      // Edit first note
      await program.methods
        .editNote("Edited first note")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount1.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After editing first note:", counterData);
      assert(counterData.editedCount.toNumber() === 1);
      assert(counterData.createdCount.toNumber() === createdAfterSecond);

      // Edit second note
      await program.methods
        .editNote("Edited second note")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount2.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After editing second note:", counterData);
      assert(counterData.editedCount.toNumber() === 2);
      assert(counterData.createdCount.toNumber() === createdAfterSecond);

      // Edit first note again
      await program.methods
        .editNote("Edited first note again")
        .accounts({
          signer: program.provider.wallet.publicKey,
          note: noteAccount1.publicKey,
          counter: pda,
        })
        .signers([payer])
        .rpc();

      counterData = await counterProgram.account.userCounter.fetch(pda);
      console.log("After editing first note again:", counterData);
      assert(counterData.editedCount.toNumber() === 3);
      assert(counterData.createdCount.toNumber() === createdAfterSecond);
    } catch (err) {
      if (err instanceof SendTransactionError) {
        console.log(await err.getLogs(program.provider.connection));
      }
      throw err;
    }
  });
});
