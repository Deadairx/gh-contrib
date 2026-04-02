# gh-contrib Roadmap

## Purpose

`gh-contrib` should evolve from a CLI-first experiment into two things at once:

- a maintained terminal app for viewing GitHub contribution data
- a reusable Rust source of GitHub contribution logic for a future Swift + UniFFI macOS widget project

This document is an execution handoff artifact. It is written to help the next agent refactor the repo without breaking the current CLI and without letting terminal assumptions leak into reusable Rust code.

## Current Codebase Snapshot

Today the repo is still a single binary crate with a CLI-first flow:

- `src/main.rs` loads dotenv, reads CLI args and env vars, selects a color palette, calls the API layer, flattens the GraphQL response, and renders the grid
- `src/api.rs` builds the GraphQL request, performs the HTTP call, and currently reads the GitHub token from env itself
- `src/model.rs` contains `ContributionDay` plus other response-layer structs, some of which appear stale or unused by the current app path
- `src/render.rs` is terminal-specific rendering logic using `crossterm`

Implications:

- there is not yet a reusable library boundary
- generated GraphQL response types are too close to the app surface
- CLI concerns and reusable GitHub/domain concerns are still mixed together

## Current Risks And Realities

The next execution agent should account for the repo as it exists today, not as the target design imagines it.

- There are currently no tests protecting behavior.
- `src/detailed_render.rs` and `src/graphql/detailed_contributions.graphql` exist but are not part of the current main execution path.
- `src/detailed_render.rs` appears incomplete and should not drive the first extraction.
- The worktree already has preexisting dirty state in some files. Do not assume every diff in the repo belongs to the refactor you are performing.

## Target Direction

Recommended eventual workspace shape:

```text
gh-contrib/
  Cargo.toml
  crates/
    gh_contrib_core/
    gh_contrib_cli/
    gh_contrib_uniffi/
```

### `gh_contrib_core`

Owns:

- GitHub GraphQL querying
- contribution-domain models
- normalization and summary logic
- streak calculations
- widget- and consumer-friendly snapshot construction
- explicit config and error types for non-CLI consumers

Should not own:

- environment-variable UX
- CLI argument parsing
- terminal rendering
- crossterm-specific palettes or output formatting
- Swift- or UniFFI-specific translation concerns

### `gh_contrib_cli`

Owns:

- command-line arguments
- env var convenience
- dotenv loading
- terminal output and palettes
- any output formatting specific to CLI ergonomics

Design rule:

- `gh_contrib_cli` should depend on public APIs from `gh_contrib_core` and should not reach into generated GraphQL types or re-own domain logic.

### `gh_contrib_uniffi`

Owns:

- UniFFI facade around the core crate
- typed Swift-facing records, enums, and errors
- translation from internal core models into a narrow FFI-safe public surface

Should not own:

- GitHub business logic that belongs in the core crate
- widget rendering concerns
- duplicate summary logic

## Refactor Principles

The next agent should treat these as guardrails, not suggestions.

1. Preserve the current CLI behavior while changing structure.
2. Introduce tests before or alongside structural refactors.
3. Keep generated GraphQL types internal to the core implementation.
4. Move env-var reads out of reusable code and into the CLI shell.
5. Keep terminal rendering entirely CLI-owned.
6. Do not let incomplete detailed-contributions work distort the first extraction.
7. Every major step should leave the repo compiling.
8. Prefer explicit config types in core over CLI-shaped inputs.

## Desired End State

The ideal end state is that:

- the CLI still works
- the core is reusable by non-CLI consumers
- the public core API uses stable domain-oriented types
- terminal concerns remain isolated
- the UniFFI layer is thin
- Swift can consume typed Rust output without importing terminal assumptions

## Non-Goals For This Refactor

The first execution pass should stay focused on architecture and safety.

- Do not add new end-user features while splitting the repo.
- Do not expand the detailed-contributions path unless it is explicitly adopted, completed, and tested.
- Do not generalize the project into a broader GitHub dashboard product.
- Do not start the macOS widget implementation from this repo.
- Do not introduce UniFFI until the core API and crate boundaries are stable.

## Phased Execution Plan

### Phase 0: Characterize Existing Behavior

Before large structural moves, add enough tests to pin current behavior.

Priority tests:

- flattening from GitHub response data preserves order across weeks and days
- mapping preserves `date`, `contribution_count`, and `color` exactly
- CLI username resolution prefers positional arg over `GH_USER`
- missing username still produces a clear error
- no-color rendering for a small fixed fixture remains stable

Definition of done:

- current behavior has enough coverage to refactor with confidence

Implementation note:

- prefer tests that pin semantics and shape over brittle exact-string matching where wording is likely to change during cleanup

### Phase 1: Establish A Reusable Core Boundary In The Existing Code

Before splitting into multiple crates, identify the real seams and make them explicit.

Goals:

- define stable domain types around contribution data
- hide GraphQL response details behind normalization functions
- move fetch logic toward explicit config input rather than env access
- make the current binary depend on reusable functions instead of inlining everything in `main.rs`

Definition of done:

- reusable fetch + normalize logic has a coherent API shape
- CLI code is visibly thinner, even if the repo is not yet a workspace

### Phase 2: Convert To A Workspace

Once the seams are clear, perform the structural split.

Recommended shape:

- root `Cargo.toml` becomes the workspace manifest
- `crates/gh_contrib_core` owns GraphQL files, build script, transport, normalization, domain models, and tests
- `crates/gh_contrib_cli` owns CLI parsing, dotenv/env convenience, palette selection, and terminal rendering

Recommended package naming:

- preserve the installed binary name as `gh-contrib`
- use the workspace split to change internal crate/package boundaries, not the user-facing command name

Important design choice:

- keep the GraphQL codegen and query files in `gh_contrib_core`, since that is where the data-fetching responsibility belongs

Definition of done:

- workspace builds successfully
- CLI crate compiles against core crate only
- running `cargo run -p gh_contrib_cli -- <user>` is a valid development path, even if the final binary is still named `gh-contrib`

### Phase 3: Stabilize The Core API

Once the workspace exists, improve the public core API so it is ready for future consumers.

Goals:

- ensure public APIs return domain types rather than generated response structs
- define explicit config/input types suitable for CLI and future non-CLI callers
- introduce summary and snapshot-oriented models if they can be added without destabilizing the CLI

Definition of done:

- core crate has a clear public API that is not CLI-shaped
- no env var reads remain in the core crate

### Phase 4: Keep The CLI Honest

The CLI should become a thin shell over the core.

Goals:

- retain current UX and terminal output behavior
- keep rendering and palette logic isolated in `gh_contrib_cli`
- ensure the CLI is consuming the same domain data future consumers will use

Definition of done:

- CLI still works as before
- CLI-specific code is clearly separated from reusable logic

### Phase 5: Quarantine Or Remove Stale Paths

After the main extraction succeeds, re-evaluate incomplete or stale code.

Candidates:

- `src/detailed_render.rs`
- `src/graphql/detailed_contributions.graphql`
- unused structs in the current `src/model.rs`

Do not force these into the first split unless they are actively adopted and tested.

Definition of done:

- the main architecture is not cluttered by incomplete side paths

### Phase 6: Prepare For `gh_contrib_uniffi`

Only start this once the core crate feels stable.

Goals:

- add a narrow facade crate around `gh_contrib_core`
- expose typed records and errors suitable for Swift
- keep the public FFI surface intentionally smaller than the full internal Rust model

Definition of done:

- UniFFI layer is thin and depends on a stable core

## Recommended First Execution Sequence

If the next agent wants a concrete order of operations, follow this sequence:

1. Inspect the dirty worktree and avoid overwriting unrelated changes.
2. Add characterization tests for normalization, username resolution, and no-color rendering.
3. Introduce an in-place library seam so fetch/normalize logic can be moved without changing behavior.
4. Create the workspace and move reusable logic into `crates/gh_contrib_core`.
5. Move CLI-specific code into `crates/gh_contrib_cli` and preserve the `gh-contrib` binary name.
6. Make the CLI depend only on public core APIs.
7. Run tests and manual smoke checks to confirm parity.
8. Only then decide whether to quarantine, delete, or revive stale detailed-contributions code.

This sequence is intentionally conservative. It favors preserving a working CLI over achieving the cleanest architecture in one jump.

## Test Strategy

The first execution agent should leave the repo safer than they found it.

Minimum recommended test coverage:

- core normalization tests with small synthetic fixtures
- unit tests for summary or streak logic once introduced
- render tests for deterministic no-color output
- CLI argument/env precedence tests
- at least one build-level smoke test that proves the workspace compiles after the split

Optional but useful:

- snapshot-style tests for text output
- fixture-based tests for GraphQL response normalization

## Migration Safety Rules

These are the safety rails for the refactor.

- Do not break the existing CLI in the name of future architecture.
- Do not expose generated GraphQL types as the public core API.
- Do not keep token/env resolution inside reusable core logic.
- Do not move terminal rendering into `gh_contrib_core`.
- Do not let widget-specific concerns reshape the CLI crate.
- Do not overbuild `gh_contrib_uniffi` before the core API is proven.
- Do not accidentally rename or remove the user-facing `gh-contrib` command during the split.

## Extraction Notes

Good extraction candidates from the current code:

- GraphQL query definitions
- HTTP and GraphQL request logic currently in `src/api.rs`
- contribution-domain types centered on `ContributionDay`
- future summary and snapshot builders derived from normalized contribution data

Likely CLI-owned or rewrite-or-isolate candidates:

- argument parsing in `src/main.rs`
- env and dotenv convenience in `src/main.rs`
- terminal rendering and color logic in `src/render.rs`

## Coordination With The Widget Project

The future `Projects/github-hub-widget` project should treat this repo as a donor of Rust domain logic, not as a UI or architectural template.

What that means in practice:

- `gh-contrib` should produce reusable contribution and summary models
- the widget project should not inherit CLI UX assumptions
- `gh_contrib_uniffi` should be a narrow bridge over core logic, not a second business-logic home

## Final Definition Of Done

This roadmap is fulfilled when:

- the repo is a workspace
- `gh_contrib_cli` depends on `gh_contrib_core`
- the current CLI behavior still works
- reusable domain logic is isolated from terminal concerns
- the core crate is shaped so a future UniFFI layer can stay thin
