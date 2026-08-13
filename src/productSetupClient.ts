import { invoke, isTauri } from "@tauri-apps/api/core";

export interface ProductSetupState {
  schemaVersion: 1;
  setupComplete: boolean;
}

export interface ProductSetupClient {
  available: boolean;
  getState(): Promise<ProductSetupState>;
  complete(): Promise<ProductSetupState>;
}

export const completeProductSetupState: ProductSetupState = {
  schemaVersion: 1,
  setupComplete: true,
};

export const nativeProductSetupClient: ProductSetupClient = {
  available: isTauri(),
  getState: () => invoke<ProductSetupState>("get_product_setup_state"),
  complete: () => invoke<ProductSetupState>("complete_product_setup"),
};
