import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

async function closeBuffers() {
  try {
    // Get buffer list
    const { stdout } = await execAsync("solana program show --buffers -ud");

    // Parse buffer addresses from the output
    const bufferAddresses = stdout
      .split("\n")
      .slice(2) // Skip header lines
      .filter((line) => line.trim()) // Remove empty lines
      .map((line) => line.split("|")[0].trim()); // Get first column (buffer address)

    console.log(`Found ${bufferAddresses.length} buffers to close`);

    // Close each buffer
    for (const address of bufferAddresses) {
      try {
        console.log(`Closing buffer: ${address}`);
        await execAsync(`solana program close ${address}`);
        console.log(`Successfully closed ${address}`);
      } catch (error) {
        console.error(`Failed to close buffer ${address}:`, error);
      }
    }

    console.log("Finished closing buffers");
  } catch (error) {
    console.error("Error getting buffer list:", error);
  }
}

// Run the script
closeBuffers();
