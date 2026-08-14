# Privacy Principles

Memoryling's value depends on sensitive context. Privacy is therefore part of the runtime contract, not a footer.

## Current core boundary

Memoryling v0.7.0 is **Agent-operated and local-first**:

- the user installs and opens the App normally, then invokes one update with `醒來吧我的寵物` or `Memoryling, wake up` inside an Agent project; reading a wake-up file or waking the project／Agent／self is explicitly not this authorization;
- the Agent verifies that a compatible App is already open before reading memory for this workflow;
- the current Agent may use only context it is already authorized to read;
- the app receives one bounded derived package through an exact local inbox file;
- the Agent-side helper submits only when a compatible Memoryling 0.7.0-or-newer process is already running and never starts an executable;
- the app does not scan Codex, Claude, or another Agent's files;
- the app does not call an AI API for the core pet loop;
- no telemetry or cloud sync is present.

The slogan authorizes one derived pet update. It does not authorize new connectors, accounts, mail, cloud apps, arbitrary files, external research, paid requests, credentials, or writes to Agent memory.

## Package minimization

Allowed package data:

- coarse Agent family;
- dominant and optional secondary activity;
- coarse journey state;
- opaque evidence IDs, kinds, observed times, and lowercase SHA-256 reference hashes;
- one evidence-qualified `hold`, `change`, or `reset` appearance plan;
- exactly 48 short bilingual dialogue cards, stable theme／semantic identities, and delivery rules.

Forbidden package data:

- raw memory or work text;
- prompts, hidden instructions, reasoning, or tool output;
- file paths, repository URLs, thread IDs, local database identifiers, names, emails, or customer data;
- credentials, tokens, API keys, environment values, or secrets;
- arbitrary attachments or executable instructions.

Hashing is not permission to include a secret. Reference hashes identify stable non-secret pointers and support package-local lineage; they are never returned in the pet render DTO.

## Defense in depth

1. The project Agent skill states the authorization and minimization rules.
2. The PowerShell submit helper validates structure and bounds without echoing content.
3. The submit helper accepts only an exact compatible running process, rejects a closed, stale, or differently named binary, and never prints the executable path.
4. Rust revalidates a strict unknown-field-denying schema.
5. The inbox accepts one exact non-symlink regular file capped at 64 KiB.
6. SQLite stores only the newest operation, current rolling counters, and current／pending appearance state.
7. The pet receives a whitelisted render DTO without evidence hashes, operation digest, paths, or source text.
8. Exact Tauri capabilities and caller-label guards keep sensitive main commands out of the pet surface.

Automated tests and committed examples use synthetic data only. Real Agent memories, private prompts, local databases, and identifiable screenshots must not enter this repository, issues, CI logs, or release evidence.

Opening the App does not authorize memory or recent-work reads. The activation phrase authorizes the bounded update. App-readiness validation may inspect only exact running-process executable metadata; it is not permission for install-location, disk, or `PATH` search.

If readiness fails, the Agent must stop before memory read and show only the locale-appropriate install／open reminder. The helper does not distinguish uninstalled from installed-but-closed by scanning disk; both remain outside the memory boundary and require the user to open a compatible pet before trying again.

## Replacement and forgetting

Each valid operation is an authoritative semantic snapshot. Applying it transactionally deletes prior operation content, then preserves counters only for a stable dialogue ID whose bilingual text is unchanged. Changed or absent cards and their content are deleted; new cards begin unused. This provides variety without creating a local history of retired derived conversations.

Evidence-qualified appearance is separate from dialogue replacement. Only the current visible appearance, one pending plan, and their opaque lineage are retained. A visible change can apply at most once per local day; `reset` requires explicit source-removal evidence.

The user can clear the current pet update from the detail window. That removes Memoryling's local derived operation; it never edits the Agent's source memory. Re-running the slogan can rebuild from current authorized context.

Because the app deliberately does not monitor Agent storage, it cannot notice a source deletion until the user runs Memoryling again. The Agent must compile from current context and omit deleted material. Immediate local removal uses the clear control or app-data deletion. SQLite `secure_delete` is enabled, but Memoryling does not promise cryptographic erasure from storage media, backups, or OS snapshots.

## Dialogue and initiative limits

- English and Traditional Chinese lines are capped at 160 characters and three visual lines.
- Time-sensitive dialogue should expire.
- Repetition is limited by per-card cooldown and max uses.
- Ambient lines are blocked during 22:00–09:00 quiet hours.
- Ambient opportunities use a local 35–70 minute cadence, wait at least ten minutes after any shown line, and are capped at seven lines per local day.
- Click dialogue has a two-second anti-stack interval and does not consume the ambient budget.
- The app never generates new semantic content by itself.
- No line may present an unverified fact, diagnosis, confidential detail, sensitive personality label, or moral judgment.

## Legacy compatibility boundary

Older direct Codex-memory, one-thread, fixture, and BYOK Daily Scout implementations remain in source for compatibility research. They are not the v0.7.0 core, are not started automatically, and are not shown in the primary UX. Daily Scout scheduling is disabled.

Any future reactivation or network feature requires a dedicated accepted ADR, visible data-flow explanation, purpose-specific opt-in, reviewable payload where practical, cost and provider-retention disclosure, deletion behavior, and a fully local path. No old consent carries forward into the Agent-operation protocol.

## Screen and public-output boundary

Pet labels and notifications must remain neutral. Dialogue is derived and can still be personal even when it contains no quotation, so screenshot／streaming privacy mode remains required before public testing with real user context. Browser preview never simulates native Agent state or persistence.
