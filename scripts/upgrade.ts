import { execSync } from "child_process";
import fs from "fs";
import path from "path";

const PROGRAM_NAME = "solwfr";
const LIB_RS_PATH = path.join(
  __dirname,
  `../programs/${PROGRAM_NAME}/src/lib.rs`
);

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

const getFileSize = (filePath: string): number => {
  try {
    const stats = fs.statSync(filePath);
    return stats.size;
  } catch (error) {
    console.error("❌ Error getting file size:", (error as Error).message);
    return 0;
  }
};

const main = async () => {
  const programId = getProgramIdFromLib();
  console.log("Program ID:", programId);

  // Get old program size
  const oldProgramPath = "target/deploy/solwfr.so";
  const oldSize = getFileSize(oldProgramPath);
  console.log("Old program size:", oldSize, "bytes");

  // Build new program
  console.log("Building new program...");
  execSync(`anchor build`, { stdio: "inherit" });

  // Get new program size
  const newSize = getFileSize(oldProgramPath);
  console.log("New program size:", newSize, "bytes");

  try {
    // Try to upgrade
    console.log("Attempting upgrade...");
    execSync(
      `anchor upgrade --program-id ${programId} target/deploy/solwfr.so`,
      { stdio: "inherit" }
    );
    console.log("✅ Upgrade completed successfully!");
  } catch (error) {
    console.log("Program needs to be extended");

    // Calculate required extension (round up to nearest 1000 bytes for safety)
    const extensionNeeded = Math.ceil((newSize - oldSize + 1000) / 1000) * 1000;
    console.log(`Extending program by ${extensionNeeded} bytes...`);

    try {
      // Extend program
      execSync(`solana program extend ${programId} ${extensionNeeded} -ud`, {
        stdio: "inherit",
      });

      // Try upgrade again
      console.log("Attempting upgrade again...");
      execSync(
        `anchor upgrade --program-id ${programId} target/deploy/solwfr.so`,
        { stdio: "inherit" }
      );
      console.log("✅ Upgrade completed successfully after extension!");
    } catch (error) {
      console.error(
        "❌ Final upgrade attempt failed:",
        (error as Error).message
      );
      process.exit(1);
    }
  }
};

main().catch((error) => {
  console.error("❌ Unexpected error:", error);
  process.exit(1);
});

//solana program extend 7beRCxYaES6RHt43aVNgjhx3aqUr7xD6GLv5XcQjU4zd 20000 -ud -k ~/.config/solana/id.json
