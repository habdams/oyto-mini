import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Keypair } from "@solana/web3.js";
import { OytoMini } from "../../../../target/types/oyto_mini";
import { expect } from "chai";

describe("compensation generation", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OytoMini as Program<OytoMini>;

  it("creates compensation", async () => {
    const rule = Keypair.generate();

    const contribution = Keypair.generate();

    const compensation = Keypair.generate();

    const createTx = await program.methods
      .createRule({ mergedPr: {} }, new anchor.BN(10))
      .accounts({
        rule: rule.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([rule])
      .rpc();

    console.log("\x1b[32mCreate Rule Tx: \x1b0m", createTx);
    console.log("---");

    const submitTx = await program.methods
      .submitContribution(new anchor.BN(50))
      .accounts({
        contributor: provider.wallet.publicKey,
        rule: rule.publicKey,
        contribution: contribution.publicKey,
        compensation: compensation.publicKey,
      })
      .signers([contribution, compensation])
      .rpc();

    console.log("\x1b[32mSubmit Contribution Tx: \x1b0m", submitTx);

    const account = await program.account.compensationAccount.fetch(
      compensation.publicKey,
    );

    expect(account.amount.toNumber()).to.be.equals(10);
  });
});
