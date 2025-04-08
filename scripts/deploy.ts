import { execSync } from "child_process";
import fs from "fs";
import path from "path";

// Paths
const PROGRAM_NAME = "solwfr";
const LIB_RS_PATH = path.join(
  __dirname,
  `../programs/${PROGRAM_NAME}/src/lib.rs`
);
const ANCHOR_TOML_PATH = path.join(__dirname, "../Anchor.toml");
const DEPLOY_LOG_PATH = path.join(__dirname, "deployment_log.json");

//solana program close ....... --bypass-warning -ud
//solana program show --programs -ud or --buffers

// Function to extract the Program ID from `lib.rs`
const getProgramIdFromLib = (): string => {
  try {
    const content = fs.readFileSync(LIB_RS_PATH, "utf8");
    const match = content.match(/declare_id!\("([A-Za-z0-9]+)"\);/);
    if (!match) throw new Error("Program ID not found in lib.rs");
    return match[1];
  } catch (error) {
    console.error("❌ Error reading program ID:", (error as Error).message);
    process.exit(1);
  }
};

// Function to get the new program ID from `anchor keys list`
const getNewProgramId = (): string => {
  try {
    const output = execSync("anchor keys list").toString();
    const match = output.match(/(\w+):\s+([A-Za-z0-9]+)/); // Matches: <program_name>: <public_key>
    if (!match) throw new Error("New Program ID not found in anchor keys list");
    return match[2];
  } catch (error) {
    console.error("❌ Error getting new Program ID:", (error as Error).message);
    process.exit(1);
  }
};

// Function to update `Anchor.toml` with the new program ID
const updateAnchorToml = (newProgramId: string): void => {
  try {
    let tomlContent = fs.readFileSync(ANCHOR_TOML_PATH, "utf8");

    // Replace the existing program ID
    tomlContent = tomlContent.replace(
      /(solwfr\s*=\s*")([A-Za-z0-9]+)(")/,
      `$1${newProgramId}$3`
    );

    fs.writeFileSync(ANCHOR_TOML_PATH, tomlContent);
    console.log(`✅ Updated Anchor.toml with new program ID: ${newProgramId}`);
  } catch (error) {
    console.error("❌ Error updating Anchor.toml:", (error as Error).message);
    process.exit(1);
  }
};

// Function to update `lib.rs` with the new program ID
const updateLibRs = (newProgramId: string): void => {
  try {
    let content = fs.readFileSync(LIB_RS_PATH, "utf8");

    // Replace the `declare_id!` line with the new ID
    content = content.replace(
      /declare_id!\("([A-Za-z0-9]+)"\);/,
      `declare_id!("${newProgramId}");`
    );

    fs.writeFileSync(LIB_RS_PATH, content);
    console.log(`✅ Updated lib.rs with new program ID: ${newProgramId}`);
  } catch (error) {
    console.error("❌ Error updating lib.rs:", (error as Error).message);
    process.exit(1);
  }
};

// Function to log deployment history
const logDeployment = (oldProgramId: string, newProgramId: string): void => {
  try {
    let logData: {
      timestamp: string;
      oldProgramId: string;
      newProgramId: string;
    }[] = [];
    if (fs.existsSync(DEPLOY_LOG_PATH)) {
      logData = JSON.parse(fs.readFileSync(DEPLOY_LOG_PATH, "utf8"));
    }

    logData.push({
      timestamp: new Date().toISOString(),
      oldProgramId,
      newProgramId,
    });

    fs.writeFileSync(DEPLOY_LOG_PATH, JSON.stringify(logData, null, 4));
    console.log(`📜 Logged deployment: ${oldProgramId} → ${newProgramId}`);
  } catch (error) {
    console.error("❌ Error writing deployment log:", (error as Error).message);
    process.exit(1);
  }
};

// Main function to execute deployment steps
const main = () => {
  console.log("🚀 Starting deployment script...");

  // 1. Get old program ID
  const OLD_PROGRAM_ID = getProgramIdFromLib();
  console.log(`🔍 Old Program ID: ${OLD_PROGRAM_ID}`);

  // 2. Close existing program
  console.log("\n🔴 Closing existing program...");
  try {
    execSync(`solana program close ${OLD_PROGRAM_ID} --bypass-warning`, {
      stdio: "inherit",
    });
  } catch (error) {
    console.log("Program already closed or not found. Error:", error);
  }

  // 3. Generate new keypair
  console.log("\n🔑 Generating new keypair...");
  execSync(
    "solana-keygen new -o target/deploy/solwfr-keypair.json --force --no-bip39-passphrase",
    { stdio: "inherit" }
  );

  // 5. Get new program ID
  const NEW_PROGRAM_ID = getNewProgramId();
  console.log(`🔄 New Program ID: ${NEW_PROGRAM_ID}`);

  // 6. Update `Anchor.toml`
  updateAnchorToml(NEW_PROGRAM_ID);

  // 7. Update `lib.rs`
  updateLibRs(NEW_PROGRAM_ID);

  // 8. Log deployment history
  logDeployment(OLD_PROGRAM_ID, NEW_PROGRAM_ID);

  // 9. Build & deploy
  console.log("\n🔨 Building Anchor program...");
  execSync("anchor build", { stdio: "inherit" });

  console.log("\n🚀 Deploying Anchor program...");
  execSync("anchor deploy", { stdio: "inherit" });

  console.log("\n🛠 Running init script...");
  execSync("npx esrun scripts/init", { stdio: "inherit" });

  console.log("\n✅ Deployment complete!");
};

// Run the script
main();
