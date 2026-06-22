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

    const tx = await program.methods
      .createRule({ mergedPr: {} }, new anchor.BN(10))
      .accounts({
        rule: rule.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([rule])
      .rpc();

    console.log("\x1b[32mCreate Rule Tx: \x1b0m", tx);

    const account = await program.account.ruleAccount.fetch(rule.publicKey);

    expect(account.reward.toNumber()).to.be.equals(10);
  });
});
