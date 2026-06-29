# Project Instructions for AI Agents

This file provides instructions and context for AI coding agents working on this project.

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:ca08a54f -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

## Session Completion

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
<!-- END BEADS INTEGRATION -->


## What This Is

A Rust library providing a full client-side implementation of the JMAP protocol suite:
- JMAP Core (RFC 8620) — session, request/response, error handling, set operations
- JMAP for Mail (RFC 8621) — email, mailbox, thread, submission, identity, vacation response
- JMAP over WebSocket (RFC 8887) — async WebSocket streams
- JMAP for Sieve Scripts (draft-ietf-jmap-sieve) — Sieve script management

Upstream repo: https://github.com/stalwartlabs/jmap-client (this is a fork/working copy)

## Build & Test

```bash
# Build (default: async + websockets + aws_lc_rs)
cargo build

# Test all features
cargo test --all-features

# Test blocking mode (mutually exclusive with async)
cargo test --no-default-features --features blocking,ring

# Lint
cargo clippy --all-features -- -D warnings

# Format check
cargo fmt --check

# Docs
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `async` | yes | Async HTTP + EventSource via `reqwest`/`futures` |
| `websockets` | yes | JMAP over WebSocket via `tokio-tungstenite` + `rustls` |
| `aws_lc_rs` | yes | rustls crypto backend (AWS-LC) |
| `ring` | no | rustls crypto backend (ring); mutually exclusive with `aws_lc_rs` |
| `blocking` | no | Blocking HTTP via `reqwest`; mutually exclusive with `async` |
| `debug` | no | Enable debug output |

`blocking` and `async` are mutually exclusive — `maybe-async` selects at compile time.

## Architecture

```
src/
├── lib.rs              URI, Method, DataType enums; Error type; crate root
├── client.rs           Client struct, builder, request dispatch (async path)
├── client_ws.rs        WebSocket client (websockets feature)
├── core/               JMAP Core — session, get/set/query/changes/request/response/errors
├── blob/               Blob copy
├── email/              Email — get, set, query, changes, import, parse, search_snippet
├── email_submission/   EmailSubmission methods
├── event_source/       EventSource async stream (async feature)
├── identity/           Identity methods
├── mailbox/            Mailbox methods
├── principal/          Principal methods
├── push_subscription/  PushSubscription methods
├── sieve/              Sieve script methods
├── thread/             Thread methods
└── vacation_response/  VacationResponse methods
```

Each object module follows the JMAP verb layout: `get.rs`, `set.rs`, `query.rs`,
`changes.rs`, with a `mod.rs` re-exporting types.

## Conventions

- `#[forbid(unsafe_code)]` at crate root — no unsafe anywhere
- Use `maybe-async` macros (`#[maybe_async]`, `#[async_impl]`, `#[sync_impl]`)
  for async/blocking duality — do not use `#[cfg(feature = "async")]` for branching
- Wire format is camelCase JSON; Rust identifiers are snake_case — use `#[serde(rename)]`
- Use `AHashMap` (from `ahash`) instead of `std::collections::HashMap`
- `parking_lot::RwLock` for shared mutable state in `Client`
- `From` impls on `Error` for all transport/parse/method error types
- No `.unwrap()` or `.expect()` in library code — propagate errors

## Branch Naming

- `feat/<short-description>` — new features
- `fix/<short-description>` — bug fixes

## Related Specs

`~/PROJECT/jmap-chat-spec/` contains draft JMAP extension specs (JMAP Chat, federation,
FileNode, push, WSS, CID) that are candidates for future client support. Consult these
before implementing any new JMAP extension methods.
