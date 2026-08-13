import { useEffect, useState } from "react";
import CreatureBody from "./CreatureBody";
import type { DailyScoutClient, DailyScoutState } from "./dailyScoutClient";
import type { Locale } from "./locale";
import type { ProductSetupClient } from "./productSetupClient";
import "./ProductSetup.css";

const setupCopy = {
  en: {
    eyebrow: "FIRST WAKE-UP",
    title: "Create your Memoryling",
    intro:
      "Choose how Memoryling speaks and whether to prepare its optional daily research feature.",
    scoutLegend: "Useful daily information",
    language: "Language",
    languageHelp: "You can change this later from the main window.",
    localTitle: "Local pet only",
    localBody:
      "No API key. Memoryling keeps its ordinary pet and local memory features.",
    apiTitle: "Prepare Daily Memory Scout",
    apiBody:
      "Securely save an OpenAI API key now. Daily search still stays off until you approve work context and consent to what may be sent.",
    recommended: "RECOMMENDED TO START",
    apiLabel: "OpenAI API key",
    apiPlaceholder: "Paste an OpenAI API key",
    apiSafety:
      "Saved in Windows Credential Manager. It is never stored in Memoryling's database or shown again. API use may cost money.",
    existingKey: "An OpenAI key is already saved on this device.",
    getKey: "Get an OpenAI API key ↗",
    create: "Wake up my Memoryling",
    creating: "Waking up…",
    localNote:
      "Nothing is uploaded during setup. Memory and Daily Scout remain off until their own review and consent steps.",
    unavailable:
      "Setup could not be saved. You can keep using the local pet and try again later.",
    invalidKey: "Enter a valid OpenAI API key, or choose the local pet option.",
  },
  "zh-TW": {
    eyebrow: "第一次甦醒",
    title: "建立你的 Memoryling",
    intro: "選擇 Memoryling 使用的語言，以及是否先準備選配的每日情報功能。",
    scoutLegend: "每日實用情報",
    language: "語言",
    languageHelp: "之後仍可從主視窗切換。",
    localTitle: "只使用本機寵物",
    localBody: "不用 API key；保留普通寵物與所有本機記憶功能。",
    apiTitle: "準備每日記憶情報",
    apiBody:
      "現在先安全保存 OpenAI API key。直到你核准工作脈絡並同意傳送內容前，每日搜尋仍維持關閉。",
    recommended: "建議先從這裡開始",
    apiLabel: "OpenAI API key",
    apiPlaceholder: "貼上 OpenAI API key",
    apiSafety:
      "Key 只存入 Windows Credential Manager，不會進入 Memoryling 資料庫，也不會再次顯示；API 使用可能產生費用。",
    existingKey: "這台裝置已安全保存一組 OpenAI key。",
    getKey: "申請 OpenAI API key ↗",
    create: "喚醒我的 Memoryling",
    creating: "正在甦醒……",
    localNote:
      "設定過程不會上傳任何內容。記憶與每日情報都要等各自的檢視與同意步驟才會開啟。",
    unavailable: "設定無法保存；你仍可使用本機寵物，稍後再試。",
    invalidKey: "請輸入有效的 OpenAI API key，或選擇只使用本機寵物。",
  },
} as const;

type ScoutChoice = "local" | "openai";

interface ProductSetupProps {
  dailyScoutClient: DailyScoutClient;
  locale: Locale;
  onComplete(): void;
  onLocaleChange(locale: Locale): void;
  setupClient: ProductSetupClient;
}

export function ProductSetup({
  dailyScoutClient,
  locale,
  onComplete,
  onLocaleChange,
  setupClient,
}: ProductSetupProps) {
  const t = setupCopy[locale];
  const [choice, setChoice] = useState<ScoutChoice>("local");
  const [apiKey, setApiKey] = useState("");
  const [scoutState, setScoutState] = useState<DailyScoutState | null>(null);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState("");
  const apiAvailable = dailyScoutClient.available;

  useEffect(() => {
    if (!dailyScoutClient.available) return;
    let active = true;
    void dailyScoutClient
      .getState()
      .then((state) => {
        if (active) setScoutState(state);
      })
      .catch(() => undefined);
    return () => {
      active = false;
    };
  }, [dailyScoutClient]);

  async function openApiKeys() {
    setNotice("");
    try {
      await dailyScoutClient.openLink("api-keys");
    } catch {
      setNotice(t.unavailable);
    }
  }

  async function completeSetup() {
    const needsKey =
      choice === "openai" && apiAvailable && !scoutState?.hasApiKey;
    if (needsKey && apiKey.trim().length < 20) {
      setNotice(t.invalidKey);
      return;
    }
    setBusy(true);
    setNotice("");
    try {
      if (needsKey) {
        const next = await dailyScoutClient.saveApiKey(apiKey.trim());
        setScoutState(next);
        setApiKey("");
      }
      await setupClient.complete();
      onComplete();
    } catch {
      setNotice(t.unavailable);
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="product-setup-shell">
      <section className="product-setup" aria-labelledby="product-setup-title">
        <div className="setup-creature" aria-hidden="true">
          <CreatureBody />
        </div>

        <div className="setup-content">
          <p className="setup-eyebrow">{t.eyebrow}</p>
          <h1 id="product-setup-title">{t.title}</h1>
          <p className="setup-intro">{t.intro}</p>

          <fieldset className="setup-fieldset setup-language">
            <legend>{t.language}</legend>
            <div className="setup-language-options">
              <button
                aria-pressed={locale === "en"}
                className={locale === "en" ? "selected" : ""}
                onClick={() => onLocaleChange("en")}
                type="button"
              >
                English
              </button>
              <button
                aria-pressed={locale === "zh-TW"}
                className={locale === "zh-TW" ? "selected" : ""}
                onClick={() => onLocaleChange("zh-TW")}
                type="button"
              >
                繁體中文
              </button>
            </div>
            <small>{t.languageHelp}</small>
          </fieldset>

          <fieldset className="setup-fieldset setup-scout-choice">
            <legend>{t.scoutLegend}</legend>
            <label className={choice === "local" ? "setup-choice selected" : "setup-choice"}>
              <input
                checked={choice === "local"}
                name="scout-choice"
                onChange={() => setChoice("local")}
                type="radio"
                value="local"
              />
              <span>
                <span className="setup-choice-heading">
                  <strong>{t.localTitle}</strong>
                  <em>{t.recommended}</em>
                </span>
                <small>{t.localBody}</small>
              </span>
            </label>
            <label className={choice === "openai" ? "setup-choice selected" : "setup-choice"}>
              <input
                checked={choice === "openai"}
                disabled={!apiAvailable}
                name="scout-choice"
                onChange={() => setChoice("openai")}
                type="radio"
                value="openai"
              />
              <span>
                <strong>{t.apiTitle}</strong>
                <small>{t.apiBody}</small>
              </span>
            </label>
          </fieldset>

          {choice === "openai" && apiAvailable && (
            <div className="setup-api-panel">
              {scoutState?.hasApiKey ? (
                <p className="setup-key-connected">✓ {t.existingKey}</p>
              ) : (
                <label>
                  <span>{t.apiLabel}</span>
                  <input
                    aria-label={t.apiLabel}
                    autoComplete="off"
                    onChange={(event) => setApiKey(event.target.value)}
                    placeholder={t.apiPlaceholder}
                    spellCheck={false}
                    type="password"
                    value={apiKey}
                  />
                </label>
              )}
              <p>{t.apiSafety}</p>
              <button className="setup-link" onClick={() => void openApiKeys()} type="button">
                {t.getKey}
              </button>
            </div>
          )}

          <p className="setup-notice" aria-live="polite" role="status">
            {notice}
          </p>
          <button
            className="setup-submit"
            disabled={busy}
            onClick={() => void completeSetup()}
            type="button"
          >
            {busy ? t.creating : t.create}
          </button>
          <p className="setup-local-note">{t.localNote}</p>
        </div>
      </section>
    </main>
  );
}

export default ProductSetup;
