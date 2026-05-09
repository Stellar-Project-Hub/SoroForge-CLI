import type { NetworkOptions } from "./types";

/**
 * High-level client for interacting with a SoroForge-managed Soroban environment.
 */
export class SoroForgeClient {
  constructor(private readonly network: NetworkOptions) {}

  getNetworkName(): string {
    return this.network.name;
  }

  getRpcUrl(): string {
    return this.network.rpcUrl;
  }
}
