import "dotenv/config";
import * as anchor from "@coral-xyz/anchor";
import { Solwfr } from "../target/types/solwfr";
import idl from "../target/idl/solwfr.json";
import fs from "fs";
import { getClusterURL } from "../utils/helper";
import { ComputeBudgetProgram, Connection, Keypair } from "@solana/web3.js";
const { SystemProgram, PublicKey } = anchor.web3;

const main = async (cluster: string) => {
  // Creates a connection to the cluster
  console.log(getClusterURL(cluster));
  const connection = new Connection(
    "https://api.mainnet-beta.solana.com",
    "confirmed"
  );

  // Load the wallet from the deployer's keypair file
  const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const wallet = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(keypairData)
  );

  // Create a provider
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(wallet),
    {
      commitment: "confirmed",
    }
  );

  anchor.setProvider(provider);

  // Load the program
  const program = new anchor.Program<Solwfr>(idl as any, provider);
  const [programStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("program_state")],
    program.programId
  );
  const currentPath = process.cwd();
  const siteAuthority = Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync(`${currentPath}/scripts/site_authority.json`, "utf-8")
      )
    )
  );

  try {
    const state = await program.account.programState.fetch(programStatePda);
    console.log(`Program already initialized, status: ${state.initialized}`);
  } catch (error) {
    console.log(error);
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        programState: programStatePda,
        deployer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        siteAuthority: siteAuthority.publicKey,
      })
      .rpc();

    await connection.confirmTransaction(tx, "finalized");
    console.log("Program initialized successfully.", tx);
  }
};

const cluster: string = process.env.NEXT_PUBLIC_CLUSTER || "mainnet-beta";
main(cluster).catch((error) => console.log(error));
