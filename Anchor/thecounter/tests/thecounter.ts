import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Thecounter } from "../target/types/thecounter";

describe("thecounter", () => {
  // Set up provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Thecounter as Program<Thecounter>;

  it("Is initialized!", async () => {
    // A new account for MyAccount
    const myAccount = anchor.web3.Keypair.generate();

    // The data you want to initialize with (u64 => use BN)
    const data = new BN(42);

    // Call the program
   const tx = await program.methods
  .initialize(data)
  .accounts({
    myAccount: myAccount.publicKey,
    user: provider.wallet.publicKey,
  }) // 
  .signers([myAccount])
  .rpc();


    console.log("Transaction signature:", tx);

    // Fetch account data from the blockchain
    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    console.log("Stored mybalance:", account.mybalance.toString());
  });
});
