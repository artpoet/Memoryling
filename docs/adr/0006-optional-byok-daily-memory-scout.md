# ADR-0006: Optional BYOK Daily Memory Scout

- Status: Proposed
- Date: 2026-08-13
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md), [ADR-0005](0005-codex-thread-history-source-pilot.md)
- Product draft: [Daily Memory Scout](../drafts/deep-interview-daily-memory-scout-2026-08-13.md)

## Context

Memoryling should become useful as well as emotionally present. The requested feature may use approved recent work to bring back one timely, source-linked model update, workflow improvement, or practical technique per day. This changes the current runtime fact that no memory-derived content leaves the device.

The feature must remain optional. Ordinary local pet behavior cannot require an API key, network consent, or a successful provider call. OpenAI's official API documentation also requires API keys to remain secret, supports Web Search through the Responses API, returns cited URLs in `url_citation` annotations, and documents retention limits that cannot be represented as zero retention for an ordinary API account.

## Proposed decision

Memoryling will add an optional **Daily Memory Scout** with the following v1 contract:

1. **Off by default.** No key, consent, or enabled setting means zero OpenAI requests. Turning it off preserves the local pet and stops future scheduled attempts.
2. **One provider.** The first version supports only OpenAI at the fixed official `https://api.openai.com/v1` host. The WebView cannot set a base URL, Authorization header, model, tool list, or system instructions.
3. **Local BYOK.** The user supplies a dedicated OpenAI API key. Rust writes it to Windows Credential Manager through an abstract credential-vault boundary. The key is never returned to the WebView or stored in SQLite, JSON settings, localStorage, logs, fixtures, screenshots, or git. It still exists briefly in process memory during a request and cannot be protected from software already running with the user's authority.
4. **Rust-only API client.** Rust uses the Responses API with pinned `gpt-5.6-terra`, `store: false`, the stable `web_search` tool, bounded output, a fixed timeout, and no Conversations, background mode, files, remote MCP, shell, computer use, or arbitrary fetch.
5. **Minimized recent-work context.** A deterministic local compiler may read approved Memoryling events but sends only allowlisted public tool／model names, coarse work domains, bounded generic goals, non-sensitive constraints, and a coarse evidence window. It excludes prompts, final-answer text, names, paths, repository URLs, IDs, contact／payment data, secrets, and arbitrary extracted phrases. Unsupported or insufficient context produces no network request.
6. **Purpose-specific consent.** Enabling shows the exact outbound summary, provider, purpose, categories, daily limit, cost boundary, and retention notice. Consent binds the current approved source, compiler version, provider, model, categories, purpose, maximum context size, and revision. A source／provider／purpose／category expansion requires fresh consent.
7. **At most one attempt per local date.** The app creates one transactional attempt row before a paid search. Restarts, concurrent triggers, failure, sleep, or local-date rollback cannot create a second attempt for the same or earlier local date. A failed attempt is reported and is not retried automatically that day.
8. **App-running schedule only.** No OS background service is installed. When Memoryling is running, it checks after the configured daytime delivery time. Missed days are not backfilled.
9. **Citation-only links and honest fallback.** A result is accepted only when the response contains a completed Web Search call, an explicit current-update／fallback-tip marker, a bounded message, and at least one valid HTTPS `url_citation`. A fallback is visibly labeled as a practical tip without claiming major news. The UI displays only citation URLs returned by the API annotations, never model-authored plain-text URLs, with at most three visible sources. The system browser opens a result only after Rust verifies that exact URL is persisted in the local citation table; the two OpenAI setup links are fixed constants.
10. **Content-minimized pet boundary.** The floating pet receives only an `insight-ready` state and a neutral prompt to open details. The full message, relevance explanation, outbound summary, errors, and source links remain main-window-only.
11. **Lineage and forgetting.** A saved insight links to every local source used by its compiled context. Forgetting one of those sources removes dependent insights and disables the now-invalid consent without spending another search. It does not delete provider-side retention records.
12. **Clear controls.** The main window can replace or delete the key, test the connection without Web Search, turn the feature off, mark an insight read, clear local insight history, or perform a combined off＋key deletion＋history deletion action.

## Data flow

```text
approved local event text
  -> deterministic allowlist compiler in Rust
  -> reviewable DailySearchContextV1 (no source prose)
  -> explicit DailyInsightConsentScopeV1
  -> one reserved local-date attempt
  -> fixed OpenAI Responses API + web_search (store: false)
  -> validated short text + url_citation annotations
  -> local SQLite insight + source lineage
  -> neutral pet-ready state + detailed main-window card
```

## Threats and controls

| Threat | Control |
|---|---|
| key leakage | OS credential vault; write-only WebView flow; no key echo／log／SQLite |
| scope creep | canonical consent hash over source, categories, purpose, provider, model, compiler, and size limit |
| private-text upload | finite allowlist compiler; no raw phrase extraction; outbound preview before consent |
| duplicate cost | immediate SQLite reservation keyed by local date plus monotonic date guard |
| prompt injection from web | fixed developer instructions; Web Search only; no local tools or side effects |
| invented／exfiltration links | accept only HTTPS `url_citation` annotations; Rust opener requires a persisted citation or one fixed OpenAI setup URL |
| response drift | strict JSON-shape parsing, size／length limits, completed-search requirement, fail closed |
| source deletion leaves ghosts | source-linked insight deletion and consent invalidation during forgetting |

## Consequences

### Positive

- Memoryling gains a practical daily behavior without making cloud AI mandatory.
- The outgoing context is understandable and substantially smaller than the approved local record.
- A database reservation makes the paid daily budget enforceable across restarts and concurrent triggers.
- The existing pet／main privacy split remains intact.

### Costs and limits

- BYOK in a desktop process is residual risk even with Windows Credential Manager.
- The allowlist compiler initially supports a bounded set of Agent-coding and AI-video signals; unknown work stays local and produces no search.
- Web Search and model usage can cost money on the user's OpenAI API account.
- `store: false` does not by itself promise zero abuse-monitoring retention. Provider-side deletion is outside Memoryling's control.
- A source implementation and synthetic tests do not prove paid live behavior, provider account eligibility, packaged Windows behavior, or information quality.

## Rejected alternatives

- **Memoryling-hosted proxy:** rejected for v1 because it introduces accounts, centralized secrets, server operations, and a much larger trust boundary.
- **Store the key in SQLite or JSON:** rejected because local app data is not a credential vault.
- **Send approved final answers directly:** rejected because it is unnecessary for the bounded search goal and exposes private prose.
- **Search on every app open or offer “try again”:** rejected because it breaks the once-per-day cost boundary.
- **Let the model invent or browse arbitrary tools:** rejected because the workflow permits only hosted Web Search and has no side-effecting tools.
- **Show the full insight on the floating surface:** rejected until screenshot／screen-sharing privacy behavior is accepted.

## Privacy impact

When enabled, a reviewable, minimized work-context summary leaves the device and is processed by OpenAI and the Web Search tool. The enable screen must explain possible cost, ordinary API retention, `store: false`, and the difference between deleting Memoryling's local copy and provider-side retention. OpenAI's current official references are the [Web Search guide](https://developers.openai.com/api/docs/guides/tools-web-search), [data controls](https://developers.openai.com/api/docs/guides/your-data), [API authentication reference](https://developers.openai.com/api/reference/overview#authentication), and [GPT-5.6 Terra model page](https://developers.openai.com/api/docs/models/gpt-5.6-terra).

The ADR remains Proposed until synthetic security／budget／forgetting tests, visible bilingual desktop smoke, one explicitly authorized paid live smoke with non-private synthetic context, and packaged Windows acceptance pass. No private source content may be used merely to exercise the API.
