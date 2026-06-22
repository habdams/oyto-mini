import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Keypair } from "@solana/web3.js";
import { OytoMini } from "../../../../target/types/oyto_mini";
import { expect } from "chai";

describe("anti gaming", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OytoMini as Program<OytoMini>;

  it("rejects tiny contributions", async () => {
    const rule = Keypair.generate();
    const contribution = Keypair.generate();
    const compensation = Keypair.generate();
    let error: any = null;

    const createTx = await program.methods
      .createRule({ mergedPr: {} }, new anchor.BN(10))
      .accounts({
        rule: rule.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([rule])
      .rpc();

    console.log("\x1b[32mCreate Rule Tx: \x1b0m", createTx);

    try {
      const sumbitTx = await program.methods
        .submitContribution(new anchor.BN(5))
        .accounts({
          contributor: provider.wallet.publicKey,
          rule: rule.publicKey,
          contribution: contribution.publicKey,
          compensation: compensation.publicKey,
        })
        .signers([contribution, compensation])
        .rpc();

      console.log("Submit Contribution Tx: ", sumbitTx);

      expect.fail("expected error");
    } catch (err) {
      error = err;
    }
    console.log(
      "\x1b[33mNo Submission Transaction will take place since contibution is below expectation\x1b[0m",
    );

    expect(error.error.errorCode.code).to.be.equal("ContributionTooSmall");
  });
});
