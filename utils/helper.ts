import "dotenv/config";

export const getClusterURL = (cluster: string): string => {
  const clusterUrls: any = {
    "mainnet-beta": process.env.MAINNET_RPC_URL,
    testnet: process.env.TESTNET_RPC_URL,
    devnet: process.env.DEVNET_RPC_URL,
    localhost: "http://127.0.0.1:8899",
  };

  return clusterUrls[cluster];
};
