# Implementation Plan: ShortURL Web Frontend

**Branch**: `002-svelte-frontend` | **Date**: 2025-11-18 | **Spec**: [`specs/002-svelte-frontend/spec.md`](./spec.md)
**Input**: Feature specification from `/specs/002-svelte-frontend/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Deliver a single-page ShortURL frontend built with SvelteKit, Tailwind CSS, and Vite that lets visitors submit long URLs, view returned short links, and review recent history with clear error handling. Requests to the existing Rust backend will flow through a SvelteKit server route so backend credentials and base URLs remain on the server, satisfying the requirement to avoid exposing secrets. Session-based history, responsive layout, and structured notification handling align with the feature spec’s user stories and measurable outcomes.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: TypeScript targeting Node.js 20 for SvelteKit server routes and evergreen browsers for the client.  
**Primary Dependencies**: SvelteKit, Vite, Tailwind CSS, @sveltejs/adapter-auto, Vitest + Playwright for unit/e2e coverage.  
**Storage**: Browser sessionStorage for recent links; no new backend persistence required.  
**Testing**: Vitest for UI/state units, Playwright for end-to-end form + history flows; backend already covered by Rust tests.  
**Target Platform**: Web (desktop + mobile) deployed as static assets plus Node adapter (or edge-compatible) for API proxy route.  
**Project Type**: Web frontend addition to existing backend repo.  
**Performance Goals**: 95% of submissions complete within 5 seconds end-to-end, initial paint under 2 seconds on modern devices, UI responsive ≥60fps for interactions.  
**Constraints**: Backend credentials/configs must stay server-side; UI must operate without authentication; maintain accessibility (focus states, contrast) per constitution.  
**Scale/Scope**: Single landing page with one form, history list, and notification system; expected traffic aligns with backend throughput (no heavy multi-user sessions).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

1. **Code Quality First** – Plan includes component/state separation, lint/test tooling (Vitest + Playwright) to keep UI logic simple. ✅
2. **Consistent User Experience** – UI will normalize backend responses and reuse shared terminology from spec to ensure predictable messaging. ✅
3. **Performance & Efficiency** – Defines client-side latency budgets and keeps network hops minimal via server proxy. ✅
4. **Reliability & Observability** – Frontend logs (console + optional SvelteKit hooks) and backend logging already ensure traceability; UI exposes retry guidance. ✅
5. **Incremental Delivery** – User stories map to independent components (form, history, error handling). ✅

**Gate Status**: PASS – no violations detected, so Complexity Tracking remains empty.

## Project Structure

### Documentation (this feature)

```text
specs/002-svelte-frontend/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/                      # Existing Rust backend (unchanged by this feature)
├── routes/
├── services/
├── db/
└── config/

frontend/
└── web/                  # New SvelteKit app
    ├── src/
    │   ├── lib/
    │   │   ├── components/
    │   │   └── stores/
    │   ├── routes/
    │   │   ├── +page.svelte          # Landing page
    │   │   └── api/shorten/+server.ts # Proxy to backend
    │   └── app.d.ts
    ├── static/
    ├── tests/
    │   ├── unit/                      # Vitest state/utility tests
    │   └── e2e/                       # Playwright flows
    ├── package.json
    └── tailwind.config.cjs

tests/
└── (reuse existing Rust test layout)
```

**Structure Decision**: Retain the Rust backend in `src/` and introduce a sibling `frontend/web` SvelteKit project. This keeps responsibilities isolated while allowing shared tooling/scripts at the repo root.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
