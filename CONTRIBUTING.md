# Contributing to Memoryling

Thank you for helping Memoryling become a trustworthy desktop life.

## Before you start

- Search existing issues before opening a new one.
- Discuss substantial features or connector designs in an issue before implementation.
- Never include real agent memories, private prompts, credentials, tokens, local databases, or identifiable user data in code, fixtures, screenshots, logs, commits, or issues.
- Keep English and Traditional Chinese user-facing copy in parity.
- Do not describe planned behavior as already implemented.

## Local development

    npm install
    npm run dev

For the desktop shell:

    npm run tauri dev

Before submitting a pull request:

    npm run check
    cargo fmt --manifest-path src-tauri/Cargo.toml --check

## Pull requests

Keep each pull request focused. Explain:

1. the user problem;
2. what changed;
3. how it was verified;
4. any privacy, memory-lineage, reminder, or migration impact;
5. screenshots for visible UI changes.

Connector pull requests must remain read-only by default and include sanitized fixtures. Changes to memory derivation or deletion must preserve source lineage.

By contributing, you agree that your contributions are licensed under the MIT License.
