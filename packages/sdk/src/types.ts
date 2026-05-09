export interface NetworkOptions {
  name: "testnet" | "mainnet" | "local";
  rpcUrl: string;
  networkPassphrase: string;
}

export interface DeployOptions {
  wasmPath: string;
  contractName: string;
  network: NetworkOptions;
}
