import { AnchorProvider, Program, Wallet, web3 } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Solwfr } from "../target/types/solwfr";
import { getClusterURL } from "../utils/helper";
import fs from "fs";
import idl from "../target/idl/solwfr.json";
const main = async () => {
  const connection = new Connection(getClusterURL("devnet"), "confirmed");
  const wallet = {
    publicKey: PublicKey.default,
    signTransaction: async () => {
      throw new Error("Read-only provider cannot sign transactions");
    },
    signAllTransaction: async () => {
      throw new Error("Read-only provider cannot sign transactions");
    },
  };
  const provider = new AnchorProvider(connection, wallet as unknown as Wallet, {
    commitment: "confirmed",
  });
  const program = new Program<Solwfr>(idl as any, provider);
  const currentPath = process.cwd();
  const siteAuthority = Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync(`${currentPath}/scripts/site_authority.json`, "utf-8")
      )
    )
  );
  const [programStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("program_state")],
    program.programId
  );

  const accounts = await program.account.playerWallet.all();
  console.log(accounts);
  for (const account of accounts) {
    const tx = await program.methods
      .emergencyCloseWallet()
      .accounts({
        playerWallet: account.publicKey,
        player: account.account.player,
        siteAuthority: siteAuthority.publicKey,
        programState: programStatePda,
      })
      .transaction();
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.feePayer = siteAuthority.publicKey;
    tx.sign(siteAuthority);
    const txHash = await connection.sendRawTransaction(tx.serialize());
    await connection.confirmTransaction(txHash, "confirmed");
    console.log(`Closed ${account.publicKey.toBase58()}`);
  }

  const tables = await program.account.table.all();
  console.log(tables);
  for (const table of tables) {
    const tx = await program.methods
      .emergencyCloseTable()
      .accounts({
        table: table.publicKey,
        dealer: table.account.dealer,
        siteAuthority: siteAuthority.publicKey,
        programState: programStatePda,
      })
      .transaction();
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.feePayer = siteAuthority.publicKey;
    tx.sign(siteAuthority);
    const txHash = await connection.sendRawTransaction(tx.serialize());
    await connection.confirmTransaction(txHash, "confirmed");
    console.log(`Closed ${table.publicKey.toBase58()}`);
  }
};

main();
