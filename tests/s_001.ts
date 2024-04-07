import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { S001 } from "../target/types/s_001";

describe("s_001", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.S001 as Program<S001>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init().rpc();
    console.log("Your transaction signature", tx);
  });
});
