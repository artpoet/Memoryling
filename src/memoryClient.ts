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
  kind: "completion" | "agent-memory-document";
  textPreview?: string;
  characterCount: number;
  contentHash: string;
}

export interface ConsentScope {
  schemaVersion: 1 | 2;
  revision: 1;
  sourceId: string;
  adapterId: string;
  adapterVersion: number;
  dataCategories: string[];
  purposes: string[];
  readOnly: true;
  sourceLocatorHash?: string;
  automaticSync?: boolean;
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
  consentScope: ConsentScope;
  consentScopeHash: string;
}

export interface CodexThreadCandidate {
  candidateId: string;
  displayName: string;
  updatedAt: string;
  sourceKind: string;
}

export interface CodexThreadCatalog {
  catalogId: string;
  candidates: CodexThreadCandidate[];
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
  memoryText?: string;
  contentRedacted: boolean;
  characterCount: number;
  contentHash: string;
  consentScopeHash?: string;
  consentRevision?: number;
}

export interface CreatureMark {
  id: string;
  style: "completion-star" | "memory-halo";
  signalType: "completion" | "agent-memory-continuity";
  confidence: number;
  derivationVersion: number;
  explanationKey:
    | "approved_completion_created_star"
    | "approved_agent_memories_created_halo";
  lineage: LineageSource[];
}

export interface MemoryState {
  storeSchemaVersion: number;
  sourceCount: number;
  eventCount: number;
  signalCount: number;
  marks: CreatureMark[];
  activeSource?: {
    sourceId: string;
    adapterId: string;
    displayName: string;
    automaticSync: boolean;
    syncStatus: "manual" | "synced" | "source-missing" | "needs-attention";
    lastSuccessfulSyncAt?: string;
    syncedRecordCount: number;
  };
  agentOperation?: {
    state: "applied";
    appliedAt: string;
    activity:
      | "building" | "research" | "design" | "planning"
      | "debugging" | "writing" | "coordination" | "shipping";
    dialogueCount: number;
  };
}

export interface ApproveImportRequest {
  previewId: string;
  sourceId: string;
  selectedRecordIds: string[];
  consentScopeHash?: string;
}

export interface MemoryClient {
  available: boolean;
  listSources(): Promise<SourceOption[]>;
  getState(): Promise<MemoryState>;
  clearAgentOperation(): Promise<MemoryState>;
  previewSource(sourceId: string): Promise<ImportPreview>;
  listCodexThreads(): Promise<CodexThreadCatalog>;
  previewCodexThread(
    catalogId: string,
    candidateId: string,
  ): Promise<ImportPreview>;
  cancelPreview(previewId: string): Promise<void>;
  approveImport(request: ApproveImportRequest): Promise<MemoryState>;
  forgetSource(sourceId: string): Promise<MemoryState>;
  syncCodexMemories(): Promise<MemoryState>;
}

export const emptyMemoryState: MemoryState = {
  storeSchemaVersion: 5,
  sourceCount: 0,
  eventCount: 0,
  signalCount: 0,
  marks: [],
};

export const nativeMemoryClient: MemoryClient = {
  available: isTauri(),
  listSources: () => invoke<SourceOption[]>("list_memory_sources"),
  getState: () => invoke<MemoryState>("get_memory_state"),
  clearAgentOperation: () => invoke<MemoryState>("clear_agent_operation"),
  previewSource: (sourceId) =>
    invoke<ImportPreview>("preview_memory_source", { sourceId }),
  listCodexThreads: () =>
    invoke<CodexThreadCatalog>("list_codex_threads"),
  previewCodexThread: (catalogId, candidateId) =>
    invoke<ImportPreview>("preview_codex_thread", { catalogId, candidateId }),
  cancelPreview: (previewId) =>
    invoke<void>("cancel_memory_preview", { previewId }),
  approveImport: (request) =>
    invoke<MemoryState>("approve_memory_import", { request }),
  forgetSource: (sourceId) =>
    invoke<MemoryState>("forget_memory_source", { sourceId }),
  syncCodexMemories: () => invoke<MemoryState>("sync_codex_memories"),
};
