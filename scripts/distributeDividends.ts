import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getMint, getAccount } from "@solana/spl-token";
import { Program, AnchorProvider, web3, Wallet } from "@coral-xyz/anchor";
import fs from "fs";
import { getClusterURL } from "../utils/helper";
import { Solwfr } from "../target/types/solwfr";
import idl from "../target/idl/solwfr.json";
// Define the holder type
interface TokenHolder {
  address: string;
  balance: number;
  share: number;
}

async function distributeDividends(
  connection: Connection,
  program: any,
  treasuryKeypair: web3.Keypair,
  tokenMintAddress: string
) {
  // 0. First withdraw fees from program state to treasury
  try {
    // Get program state PDA
    const [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId
    );

    const programState = await program.account.programState.fetch(
      programStatePda
    );
    console.log(
      "Program state fees collected:",
      programState.feesCollected.toNumber()
    );

    // Get treasury address (using the treasury keypair's public key)
    const treasuryAddress = treasuryKeypair.publicKey;

    const currentPath = process.cwd();
    const siteAuthority = Keypair.fromSecretKey(
      Uint8Array.from(
        JSON.parse(
          fs.readFileSync(`${currentPath}/scripts/site_authority.json`, "utf-8")
        )
      )
    );

    // Call the withdraw_fees instruction
    console.log("Withdrawing fees from program state...");
    const tx = await program.methods
      .withdrawFees()
      .accounts({
        programState: programStatePda,
        treasury: treasuryAddress,
        siteAuthority: siteAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .transaction();
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.feePayer = siteAuthority.publicKey;
    tx.sign(siteAuthority);
    const txId = await connection.sendRawTransaction(tx.serialize());
    await connection.confirmTransaction(txId, "confirmed");
    console.log("Fees withdrawn successfully");
  } catch (error) {
    console.error("Error withdrawing fees:", error);
    throw error;
  }
  return;
  // 1. Get the token mint info
  const mintPubkey = new PublicKey(tokenMintAddress);
  const mintInfo = await getMint(connection, mintPubkey);

  // 2. Get all token accounts for this mint
  const tokenAccounts = await connection.getProgramAccounts(TOKEN_PROGRAM_ID, {
    filters: [
      { dataSize: 165 }, // Size of token account
      { memcmp: { offset: 0, bytes: mintPubkey.toBase58() } }, // Filter by mint
    ],
  });

  // 3. Calculate total supply and each holder's share
  const totalSupply = Number(mintInfo.supply);
  const holders: TokenHolder[] = [];

  for (const account of tokenAccounts) {
    const tokenAccount = await getAccount(connection, account.pubkey);
    const balance = Number(tokenAccount.amount);

    if (balance > 0) {
      holders.push({
        address: tokenAccount.owner.toString(),
        balance,
        share: balance / totalSupply,
      });
    }
  }

  const RENT_EXEMPTION = 1890880;

  // 4. Get treasury balance
  const treasuryBalanceBeforeRent = await connection.getBalance(
    treasuryKeypair.publicKey
  );
  const treasuryBalance = treasuryBalanceBeforeRent - RENT_EXEMPTION;

  console.log(`Treasury balance: ${treasuryBalance} lamports`);
  console.log(`Found ${holders.length} token holders`);

  // 5. Distribute dividends to each holder in batches
  const BATCH_SIZE = 10; // Number of transfers per transaction
  const batches: TokenHolder[][] = [];

  // Group holders into batches
  for (let i = 0; i < holders.length; i += BATCH_SIZE) {
    batches.push(holders.slice(i, i + BATCH_SIZE));
  }

  console.log(
    `Splitting ${holders.length} holders into ${batches.length} batches`
  );

  // Process each batch
  for (let batchIndex = 0; batchIndex < batches.length; batchIndex++) {
    const batch = batches[batchIndex];
    console.log(
      `Processing batch ${batchIndex + 1}/${batches.length} with ${
        batch.length
      } holders`
    );

    // Create a transaction for this batch
    const transaction = new Transaction();

    // Add transfer instructions for each holder in the batch
    for (const holder of batch) {
      const dividendAmount = Math.floor(treasuryBalance * holder.share);

      if (dividendAmount > 0) {
        console.log(
          `Adding transfer of ${dividendAmount} lamports to ${holder.address}`
        );

        transaction.add(
          SystemProgram.transfer({
            fromPubkey: treasuryKeypair.publicKey,
            toPubkey: new PublicKey(holder.address),
            lamports: dividendAmount,
          })
        );
      }
    }

    // Skip empty transactions
    if (transaction.instructions.length === 0) {
      console.log(`Skipping empty batch ${batchIndex + 1}`);
      continue;
    }

    // Sign and send transaction
    transaction.feePayer = treasuryKeypair.publicKey;
    transaction.recentBlockhash = (
      await connection.getRecentBlockhash()
    ).blockhash;
    transaction.sign(treasuryKeypair);

    try {
      const signature = await connection.sendRawTransaction(
        transaction.serialize()
      );
      console.log(`Batch ${batchIndex + 1} sent with signature: ${signature}`);

      // Wait for confirmation
      await connection.confirmTransaction(signature, "confirmed");
      console.log(`Batch ${batchIndex + 1} confirmed`);
    } catch (error) {
      const currentPath = process.cwd();
      fs.writeFileSync(
        `${currentPath}/scripts/error.json`,
        JSON.stringify({ batch, error }, null, 2)
      );
      console.error(`Error processing batch ${batchIndex + 1}:`, error);
      // Continue with next batch even if this one fails
    }
  }
  console.log("Dividends distributed successfully");
}

const main = async () => {
  const connection = new Connection(getClusterURL("mainnet-beta"), "confirmed");
  const keypairPath = `${process.env.HOME}/.config/solana/id.json`;
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const wallet = web3.Keypair.fromSecretKey(Uint8Array.from(keypairData));
  const provider = new AnchorProvider(connection, new Wallet(wallet), {
    commitment: "confirmed",
  });
  const program = new Program<Solwfr>(idl as any, provider);
  const currentPath = process.cwd();
  const treasuryKeypair = Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync(`${currentPath}/scripts/treasury.json`, "utf-8")
      )
    )
  );

  await distributeDividends(
    connection,
    program,
    treasuryKeypair,
    "wfrQpcxsp7eoyp2f4uH4VGK3nFxUzurA7YgStnikWy2"
  );
};

main();

//treasury: T3527F9f5bbUUJTCmBPaQ6d45ZMck2E7LgVzgrF9WFR
