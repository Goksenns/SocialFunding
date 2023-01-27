import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SocialFunding } from "../target/types/social_funding";

describe("SocialFunding", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SocialFunding as Program<SocialFunding>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
