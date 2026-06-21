import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { OytoMini } from "../../../../target/types/oyto_mini";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("create rule", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OytoMini as Program<OytoMini>;

  it("creates a rule", async () => {
    setTimeout(() => {}, 60000);
    const rule = Keypair.generate();

    await program.methods
      .createRule({ mergedPr: {} }, new anchor.BN(10))
      .accounts({
        rule: rule.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([rule])
      .rpc();

    const account = await program.account.ruleAccount.fetch(rule.publicKey);

    expect(account.reward.toNumber()).to.be.equals(10);
  });
});
