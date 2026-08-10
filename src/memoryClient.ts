import { invoke, isTauri } from "@tauri-apps/api/core";

export interface SourceOption {
  id: string;
  adapterId: string;
  adapterVersion: number;
  displayName: string;
  locator: string;
  fixtureOnly: boolean;
}

export interface PreviewRecord {
  id: string;
  sourceTimestamp: string;
  kind: "completion";
  textPreview: string;
  contentHash: string;
}

export interface ImportPreview {
  previewId: string;
  source: SourceOption;
  recordCount: number;
  timeRange: {
    start: string;
    end: string;
  };
  records: PreviewRecord[];
  accessScope: {
    readOnly: boolean;
    sourceWriteAccess: boolean;
    networkAccess: boolean;
    arbitraryPathAccess: boolean;
  };
}

export interface LineageSource {
  memoryEventId: string;
  memoryEventSchemaVersion: number;
  sourceId: string;
  sourceLabel: string;
  adapterId: string;
  adapterVersion: number;
  sourceRecordId: string;
  sourceTimestamp: string;
  memoryText: string;
  contentHash: string;
}

export interface CreatureMark {
  id: string;
  style: "completion-star";
  signalType: "completion";
  confidence: number;
  derivationVersion: number;
  explanationKey: "approved_completion_created_star";
  lineage: LineageSource[];
}

export interface MemoryState {
  storeSchemaVersion: number;
  sourceCount: number;
  eventCount: number;
  signalCount: number;
  marks: CreatureMark[];
}

export interface ApproveImportRequest {
  previewId: string;
  sourceId: string;
  selectedRecordIds: string[];
}

export interface MemoryClient {
  available: boolean;
  listSources(): Promise<SourceOption[]>;
  getState(): Promise<MemoryState>;
  previewSource(sourceId: string): Promise<ImportPreview>;
  cancelPreview(previewId: string): Promise<void>;
  approveImport(request: ApproveImportRequest): Promise<MemoryState>;
  forgetSource(sourceId: string): Promise<MemoryState>;
}

export const emptyMemoryState: MemoryState = {
  storeSchemaVersion: 1,
  sourceCount: 0,
  eventCount: 0,
  signalCount: 0,
  marks: [],
};

export const nativeMemoryClient: MemoryClient = {
  available: isTauri(),
  listSources: () => invoke<SourceOption[]>("list_memory_sources"),
  getState: () => invoke<MemoryState>("get_memory_state"),
  previewSource: (sourceId) =>
    invoke<ImportPreview>("preview_memory_source", { sourceId }),
  cancelPreview: (previewId) =>
    invoke<void>("cancel_memory_preview", { previewId }),
  approveImport: (request) =>
    invoke<MemoryState>("approve_memory_import", { request }),
  forgetSource: (sourceId) =>
    invoke<MemoryState>("forget_memory_source", { sourceId }),
};
