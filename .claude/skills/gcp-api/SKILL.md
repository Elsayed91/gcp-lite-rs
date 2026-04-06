---
name: gcp-api
description: Add a new GCP API or extend an existing one. Auto-detects whether to bootstrap a new API or add operations to an existing one.
argument-hint: "[api_name] [version]: [resources/operations needed]"
---

# GCP API Workflow

You are adding or extending a GCP API in gcp-lite-rs. The user's request: "$ARGUMENTS"

This workflow uses **integration-first development**: each operation group is validated against the real GCP API before unit tests are written.

## IMMUTABLE RULES

0. **Integration tests MUST pass for an operation group BEFORE writing unit tests.** Unit tests encode proven behavior, not assumptions.
1. **NEVER edit generated files** (`src/types/`, `src/ops/`, `src/test_support/`) — a hook blocks this. Flag issues → manifest fix → wait for approval.
2. **NEVER modify codegen scripts** without explicit user approval.
3. **NEVER skip or work around authentication failures** — ask user to re-authenticate.
4. **NEVER overwrite existing `src/api/*.rs` files** — only create new or add methods.
5. **NEVER choose the simplest fix by default** — explore solutions, choose the one that generalizes and avoids technical debt.

## Prerequisites

Read these docs:
1. `.claude/docs/architecture.md` — 3-layer pattern
2. `.claude/docs/codegen-reference.md` — manifest format and field overrides
3. `.claude/docs/lro-patterns.md` — LRO pattern selection
4. `.claude/docs/testing-guide.md` — test patterns

## Phase 1: Discovery

1. Parse the user's request: API name, version, requested resources/operations.

2. **Detect add vs extend**:
   - Check for `codegen/manifests/{api_name}.toml` and `src/api/{api_name}.rs`
   - **Both exist** → EXTEND mode
   - **Neither exists** → ADD mode
   - **Manifest only** → resume interrupted add (skip to Phase 3)

3. **ADD mode**: Bootstrap to discover available resources:
   ```
   python3 codegen/bootstrap.py {api_name} {version}
   ```
   Read draft manifest. Determine LRO pattern (cross-ref `.claude/docs/lro-patterns.md`).

4. **EXTEND mode**: Read existing manifest and API client. Use extend.py or fetch_discovery.py to find available additions not yet in the manifest.

5. Present available resources/operations to the user (AskUserQuestion). Group by resource, let user select.

6. **Plan operation groups** — order from simplest to most complex (CRUD first, LROs last).

## Phase 2: Manifest

### ADD mode
Edit `codegen/manifests/{api_name}.toml`:
- `[api]` metadata (name, display_name, version, URLs), `[api.client]`, `[api.lro]` if applicable
- `[[types]]`: `include_fields`, field overrides (`{ required = true }` for names, `{ rust_name = "..._type", serde_rename = "type" }` for reserved words, `{ enum_type = "..." }` for status fields)
- `[[operations]]`: `discovery_resource`/`discovery_method`, `rust_name`, `is_lro`, `list_response`, `query_params`
- Compare with existing manifests: `compute.toml` (selflink LRO), `iam.toml` (no LRO), `service_usage.toml` (name_based LRO)

### EXTEND mode
Add new `[[types]]` and/or `[[operations]]` following existing manifest conventions. Update existing types if adding fields (`include_fields` + overrides).

**COMMIT**: `feat: {add|extend} {api_name} manifest`

## Phase 3: Generation

```
cd codegen && uv run python -m codegen.cli apply
cargo check
```

Fix manifest if cargo check fails (missing field overrides for reserved words, wrong schema name, missing dependency types).

**COMMIT**: `feat: generate types/ops for {api_name}`

## Phase 4: Registration & Scaffolding (ADD mode only)

Skip this phase for EXTEND mode.

1. Add `pub mod {api_name};` and `pub use` to `src/api/mod.rs`
2. Add accessor to `src/client.rs` (check `codegen/generated/registration/client_accessors.rs` for template)
3. If new LRO type needed: add to `src/operation.rs`, export from `lib.rs`
4. Create `src/api/{api_name}.rs` with struct shell (no methods yet)
5. Create `tests/integration/{api_name}.rs` with module header
6. Add integration test task to `Taskfile.yml`
7. `cargo check`

**COMMIT**: `feat: scaffold {api_name} API client`

## Phase 5: Incremental Development

For each operation group:

### Step A: Write API Methods
Add methods to `src/api/{api_name}.rs` for THIS GROUP ONLY:
- Section comments: `// ── Resource Name ──────...`
- Ergonomic signatures: `project`, `zone`, `name` — not raw resource paths
- Construct resource name paths: `format!("projects/{}/...", project)`
- LRO dual methods: `method()` (blocking) + `method_start()` (returns Operation)

### Step B: Write Integration Test
Add test function in `tests/integration/{api_name}.rs`:
- Real create → get → list → update → delete lifecycle
- Use library's validated methods for setup, gcloud CLI for ops not yet implemented
- Always-cleanup pattern, step-numbered `println!`

### Step C: Run Integration Test
```
cargo nextest run --test integration {api_name} -- --ignored --test-threads=1 --nocapture
```
Fix failures (missing body fields, special URLs, empty query params, path encoding). Re-run until passing.

### Step D: Write Unit Tests
In `src/api/{api_name}.rs`, encode proven-correct URL patterns, body fields, parameter handling using MockClient. Every test verifies actual data — never just `is_ok()`.

### Step E: Run Full Test Suite
```
cargo nextest run --lib
```

### Step F: Commit
```
feat: add {api_name} {group_name} operations
```

**Repeat Steps A-F for each operation group.**

## Phase 6: Documentation

- **ADD**: Create `docs/{api_name}/` with api.md, operations.md, usage.md (see `.claude/docs/api-doc-template.md`)
- **EXTEND**: Update existing `docs/{api_name}/operations.md` and `usage.md`

**COMMIT**: `docs: {add|update} {api_name} documentation`

## Phase 7: Quality Gate

```
cargo check
cargo clippy -- -D warnings
cargo nextest run --lib
cargo nextest run --test integration {api_name} -- --ignored --test-threads=1 --nocapture
```

Report results. Fix and re-run if anything fails.
