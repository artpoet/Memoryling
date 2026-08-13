import { invoke, isTauri } from "@tauri-apps/api/core";

export interface DailySearchContext {
  schemaVersion: 1;
  workDomains: string[];
  publicToolsAndModels: string[];
  currentGoals: string[];
  nonSensitiveConstraints: string[];
  evidenceWindow: {
    startDate: string;
    endDate: string;
  };
  preferredInsightCategories: string[];
}

export interface DailyCitation {
  title: string;
  url: string;
}

export interface DailyInsight {
  id: string;
  localDate: string;
  provider: "openai";
  model: string;
  petMessage: string;
  strength: "practical" | "quiet";
  relevanceReason: string;
  searchedAt: string;
  read: boolean;
  citations: DailyCitation[];
}

export interface DailyScoutState {
  enabled: boolean;
  hasApiKey: boolean;
  canEnable: boolean;
  provider: "openai";
  model: string;
  deliveryTime: string;
  status:
    | "off"
    | "needs-key"
    | "needs-memory"
    | "scheduled"
    | "running"
    | "ready"
    | "failed"
    | "complete";
  contextPreview?: DailySearchContext;
  latestInsight?: DailyInsight;
  todayAttempt?: {
    localDate: string;
    status: "running" | "succeeded" | "failed";
    errorCode?: string;
  };
}

export interface ConfigureDailyScoutRequest {
  locale: "en" | "zh-TW";
  deliveryTime: string;
  consentAccepted: boolean;
}

export interface DailyScoutClient {
  available: boolean;
  getState(): Promise<DailyScoutState>;
  saveApiKey(apiKey: string): Promise<DailyScoutState>;
  testApiKey(): Promise<void>;
  configure(request: ConfigureDailyScoutRequest): Promise<DailyScoutState>;
  disable(): Promise<DailyScoutState>;
  deleteApiKey(): Promise<DailyScoutState>;
  clearHistory(): Promise<DailyScoutState>;
  reset(): Promise<DailyScoutState>;
  markRead(): Promise<DailyScoutState>;
  openLink(kind: "api-keys" | "quickstart" | "citation", url?: string): Promise<void>;
}

export const emptyDailyScoutState: DailyScoutState = {
  enabled: false,
  hasApiKey: false,
  canEnable: false,
  provider: "openai",
  model: "gpt-5.6-terra",
  deliveryTime: "10:00",
  status: "off",
};

export const nativeDailyScoutClient: DailyScoutClient = {
  available: isTauri(),
  getState: () => invoke<DailyScoutState>("get_daily_scout_state"),
  saveApiKey: (apiKey) =>
    invoke<DailyScoutState>("save_openai_api_key", { apiKey }),
  testApiKey: () => invoke<void>("test_openai_api_key"),
  configure: (request) =>
    invoke<DailyScoutState>("configure_daily_scout", { request }),
  disable: () => invoke<DailyScoutState>("disable_daily_scout"),
  deleteApiKey: () => invoke<DailyScoutState>("delete_openai_api_key"),
  clearHistory: () => invoke<DailyScoutState>("clear_daily_scout_history"),
  reset: () => invoke<DailyScoutState>("reset_daily_scout"),
  markRead: () => invoke<DailyScoutState>("mark_daily_insight_read"),
  openLink: (kind, url) =>
    invoke<void>("open_daily_scout_link", { request: { kind, url } }),
};
