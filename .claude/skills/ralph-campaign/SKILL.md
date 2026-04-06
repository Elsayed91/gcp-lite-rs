---
name: ralph-campaign
description: Generate .ralph campaign files (PROMPT.md + fix_plan.md) for autonomous Ralph loop execution
user-invokable: true
---

# Ralph Campaign Generator

Generate a `.ralph/campaigns/{name}/` directory with a PROMPT.md and fix_plan.md tailored to the current project's conventions.

## Phase 1: Understand the Task

If no task description was provided as an argument, ask the user:
- What should Ralph build? (e.g., "Add GCP VM, Storage, and Networking APIs")
- Desired campaign name (suggest one derived from the task, e.g., `gcp-compute`)

Extract: **what** is being built, **scope** (new API? extend existing? standalone feature?), and **campaign name**.

## Phase 2: Read Project Context

Read these files to understand conventions (skip any that don't exist):

1. `.claude/CLAUDE.md` — build/test commands, quality gates, commit conventions, architecture
2. `.ralph/PROMPT.md` — existing prompt as **structural reference** for tone and format
3. Relevant `.claude/docs/` for the target domain (e.g., architecture, testing guide)

Extract and note:
- Exact build & test commands (e.g., `cargo check`, `cargo nextest run --lib`)
- Quality gate commands (e.g., `cargo clippy -- -D warnings`)
- Commit message conventions
- Whether the project uses: codegen pipeline, integration-first testing, generated vs hand-written layers
- Test resource naming convention (e.g., `cloud-lite-test-{slug}-`)

## Phase 3: Generate fix_plan.md

Decompose the task into a phased checklist. Rules:

- **Task IDs:** `{phase}.{sequence}` — e.g., 1.1, 1.2, 2.1
- **IDs ending in .1** are scaffold/setup tasks (create structure, generate code, bootstrap)
- **IDs ending in .2+** are implementation + test tasks
- **Sequence matters:** scaffold before implementation, read operations before write operations
- **Each task line lists specific work** — e.g., `1.2: compute ListDisks + GetDisk + integration tests`
- **Group by logical unit** (service, module, feature area)
- Tasks within a group are sequential; groups can be parallelized if independent

Format:
```
# {Campaign Title}

Ralph picks the first unchecked task, completes it, checks it off, moves to the next.
Tasks within a group are SEQUENTIAL. {Testing mandate if applicable.}

## Phase 1 -- {Phase Description}

### {Group Name} ({metadata like wire format, protocol})
- [ ] 1.1: {scaffold/setup description}
- [ ] 1.2: {operations} + {test type}
- [ ] 1.3: {more operations} + {test type}

## Phase 2 -- {Phase Description}
...
```

## Phase 4: Generate PROMPT.md

This is the critical file. Follow this exact skeleton — adapt each section to the project and task. **Target under 120 lines** to minimize per-loop token cost.

```
# Ralph Development Instructions

## Context
You are Ralph, an autonomous AI development agent [role derived from task].
[1-2 sentences about what you're building and the project.]

**Project Type:** [detected from project]

## Step 0: Orient
1. Read `{campaign fix_plan path}` -- find the first unchecked (`- [ ]`) task
2. If NO unchecked tasks remain, set EXIT_SIGNAL: true
3. [Any domain-specific docs to read per task]
4. Check existing work before starting

## Step 1: Claim Task
Commit: `chore: start {task_id} -- {task_description}`

## Step 2: Execute Task
[Define task types based on the project's workflow]

### Type A: [Scaffold tasks] (IDs ending in .1)
[Step-by-step with ACTUAL commands from the project]
[Verification steps]

### Type B: [Implementation tasks] (IDs ending in .2+)
[Step-by-step with ACTUAL commands]
[Testing requirements — integration-first if project uses it]
[Failure protocol: max 3 fix attempts, then:]
1. Document issue in `.claude/backlog/issue-tracker/{service}-{description}.md`
2. Mark task BLOCKED in fix_plan.md: append `[BLOCKED: reason]`
3. Commit: `chore: block {task_id} -- {reason}`
4. Move to NEXT task

## Step 3: Quality Gate
[ACTUAL quality commands from CLAUDE.md — not placeholders]

## Step 4: Mark Done
Mark task done in fix_plan.md -- `- [ ]` to `- [x]`.
Commit: `chore: complete {task_id}`

## Key Principles
- **ONE task per loop -- HARD STOP.** Complete one task, output status, stop.
- Search the codebase before assuming something isn't implemented
- [Project-specific principles from CLAUDE.md]
- [Test resource naming convention if applicable]

## Status Reporting (CRITICAL)
[Include the RALPH_STATUS block verbatim — same format always]
```

**Rules for the generated PROMPT.md:**
- Use actual commands from the project, never placeholders like "run tests"
- Include the blocking protocol — prevents Ralph from spinning on failures
- ONE task per loop is non-negotiable — prevents context exhaustion
- Status reporting block is mandatory and always the same format
- Reference the campaign's fix_plan.md path, not the root one
- Do NOT include spec-by-example exit scenarios (the ralph CLI handles those)

## Phase 5: Write and Report

1. Create directory `.ralph/campaigns/{name}/`
2. Write `fix_plan.md`
3. Write `PROMPT.md`
4. Tell the user how to run it:
   ```
   ralph --prompt .ralph/campaigns/{name}/PROMPT.md
   ```
