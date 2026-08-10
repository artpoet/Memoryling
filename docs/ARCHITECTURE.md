# Architecture

## Status

This document describes the intended architecture. As of 2026-08-10, only the Tauri desktop shell and React concept UI are implemented.

## System shape

    External durable-memory sources
        → read-only source adapters
        → import preview and consent gate
        → normalized local memory events
        → derivation engine
        → lineage-aware local store
        → creature state, stories, conversations, reminders
        → bilingual Tauri UI

## Layers

| Layer | Responsibility | Current state |
|---|---|---|
| Desktop shell | Native window, lifecycle, notifications | Tauri 2 shell exists; notifications not implemented |
| Experience UI | Creature, habitat, stories, controls, explanations | Interactive concept shell |
| Source adapters | Read selected durable-memory formats without mutating them | Not implemented |
| Import gate | Preview scope, explain access, obtain consent | Not implemented |
| Normalizer | Convert source records into a versioned local event schema | Not implemented |
| Derivation engine | Produce traits, tensions, story hooks, reminder candidates | Not implemented |
| Local store | Persist normalized events, derived effects, lineage, and settings | Not implemented |
| Conversation layer | Ground dialogue in approved local context | Not implemented; provider decision open |
| Reminder policy | Enforce quiet hours, budget, urgency, and snooze state | UI concept only |

## Proposed core records

### SourceMemory

- stable source adapter ID
- opaque source record ID
- source path or locator retained locally
- observed timestamp and source timestamp
- user-approved normalized text or structured fields
- content hash

### DerivedSignal

- type: completion, recurrence, promise, value, conflict, preference, or relationship
- confidence and derivation version
- one or more SourceMemory references
- creation and invalidation timestamps

### WorldEffect

- type: trait, visual mark, habitat change, story event, dialogue fact, or reminder candidate
- state and lifecycle
- DerivedSignal references
- explanation payload

This graph allows deletion to flow from a source record through every dependent effect.

## Connector contract

A connector must:

1. declare exactly which paths and formats it can read;
2. perform no writes to the source tool's files;
3. show an import preview before persistence;
4. normalize deterministic, testable records;
5. use synthetic fixtures in the repository;
6. fail closed when a format is unknown;
7. never collect credentials from source files.

The first planned adapter targets approved Codex durable-memory files. It will not scan arbitrary home directories.

## Trust boundaries

- **External source files:** user-owned, untrusted input, read-only.
- **Local Memoryling store:** user-owned derived state, never committed.
- **UI:** displays explanations but must not render source content as trusted HTML.
- **Future model provider:** optional boundary requiring a separate ADR and explicit consent before any memory-derived context leaves the device.

## Open decisions

- embedded local model versus optional remote conversation provider;
- SQLite schema and migration strategy;
- adapter discovery and permission UX;
- Windows resident-app lifecycle and notification integration;
- deterministic versus model-assisted derivation boundaries.

Major decisions are recorded in [docs/adr](adr/INDEX.md).
