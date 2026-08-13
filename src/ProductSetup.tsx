import { useState } from "react";
import CreatureBody from "./CreatureBody";
import type { Locale } from "./locale";
import type { ProductSetupClient } from "./productSetupClient";
import "./ProductSetup.css";

const setupCopy = {
  en: {
    eyebrow: "FIRST WAKE-UP",
    title: "Create your Memoryling",
    intro:
      "Choose how Memoryling speaks. Then wake it through the Agent already working with you.",
    language: "Language",
    languageHelp: "You can change this later from the main window.",
    agentTitle: "Wake it from your Agent project",
    slogan: "Run Memoryling",
    agentBody:
      "The Agent reads only the context it is already authorized to use, compiles a bounded pet update, and hands it to this local app.",
    create: "Wake up my Memoryling",
    creating: "Waking up…",
    localNote:
      "No API key is needed. The app never scans Agent memory or calls an AI service by itself.",
    unavailable:
      "Setup could not be saved. You can keep using the local pet and try again later.",
  },
  "zh-TW": {
    eyebrow: "第一次甦醒",
    title: "建立你的 Memoryling",
    intro: "選擇 Memoryling 使用的語言；接著由原本就在跟你工作的 Agent 喚醒牠。",
    language: "語言",
    languageHelp: "之後仍可從主視窗切換。",
    agentTitle: "從你的 Agent 專案喚醒牠",
    slogan: "運作 Memoryling",
    agentBody:
      "Agent 只讀取它原本獲准使用的脈絡，編譯成有邊界的寵物更新包，再交給這個本機 App。",
    create: "喚醒我的 Memoryling",
    creating: "正在甦醒……",
    localNote: "不需要 API key；App 不會自行掃描 Agent 記憶，也不會自行呼叫 AI 服務。",
    unavailable: "設定無法保存；你仍可使用本機寵物，稍後再試。",
  },
} as const;

interface ProductSetupProps {
  locale: Locale;
  onComplete(): void;
  onLocaleChange(locale: Locale): void;
  setupClient: ProductSetupClient;
}

export function ProductSetup({
  locale,
  onComplete,
  onLocaleChange,
  setupClient,
}: ProductSetupProps) {
  const t = setupCopy[locale];
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState("");

  async function completeSetup() {
    setBusy(true);
    setNotice("");
    try {
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

          <section className="setup-agent-route" aria-label={t.agentTitle}>
            <p className="setup-agent-title">{t.agentTitle}</p>
            <strong className="setup-slogan">“{t.slogan}”</strong>
            <p>{t.agentBody}</p>
          </section>

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
