---

description: "Task list for ShortURL Web Frontend"
---

# Tasks: ShortURL Web Frontend

**Input**: Design documents from `/specs/002-svelte-frontend/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md (decisions on proxying and UX patterns)

**Tests**: Tests are OPTIONAL. For this feature we will add targeted Vitest unit coverage and Playwright e2e flows for the highest-priority scenarios to satisfy the constitution’s quality expectations.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Frontend root: `frontend/web/`
- Routes: `frontend/web/src/routes/`
- Components: `frontend/web/src/lib/components/`
- Stores & services: `frontend/web/src/lib/stores/`, `frontend/web/src/lib/services/`
- Utilities: `frontend/web/src/lib/utils/`
- Tests: `frontend/web/tests/unit/`, `frontend/web/tests/e2e/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize the SvelteKit project, Tailwind styling system, and testing toolchain per the implementation plan.

- [ ] T001 Create the SvelteKit project under `frontend/web/` with project scaffolding (`src/`, `static/`, `package.json`, `svelte.config.js`).
- [ ] T002 Configure Tailwind CSS (postcss.config.cjs, tailwind.config.cjs, `src/app.css`) and wire it into the default layout.
- [ ] T003 [P] Add linting/test scripts (`svelte-check`, `eslint`, `vitest`) to `frontend/web/package.json` plus matching config files.
- [ ] T004 [P] Create `.env.example` and Vite config entries in `frontend/web/vite.config.ts` documenting required backend URLs/credentials without exposing secrets.
- [ ] T005 Install and bootstrap Playwright + Vitest test suites (`frontend/web/tests/unit/`, `frontend/web/tests/e2e/`) with base config files.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before any user story work (form, history, or errors) can begin.

**⚠️ CRITICAL**: No user story work can start until this phase is complete.

- [ ] T006 Implement server-only config helper in `frontend/web/src/lib/server/config.ts` to read backend base URL/credentials from environment variables.
- [ ] T007 Build the API proxy route in `frontend/web/src/routes/api/shorten/+server.ts` that forwards requests to the Rust backend while hiding credentials.
- [ ] T008 Create the shared shortening store in `frontend/web/src/lib/stores/shortening.ts` (state machine: idle/loading/success/error with metadata).
- [ ] T009 Establish global layout shell in `frontend/web/src/routes/+layout.svelte` (theme wrappers, responsive container, notification slot) reused by all stories.

**Checkpoint**: Foundation ready—proxying, config, and base state management exist so user stories can proceed independently.

---

## Phase 3: User Story 1 – Shorten a long URL from the browser (Priority: P1) 🎯 MVP

**Goal**: Allow visitors to submit a long URL, call the backend through the proxy, and display a copyable short link with clear loading state.

**Independent Test**: Using the landing page, entering a valid long URL returns a short URL within 5 seconds, shows it on screen, and lets the user copy or open it without leaving the page.

### Implementation for User Story 1

- [ ] T010 [US1] Build the landing page form and layout in `frontend/web/src/routes/+page.svelte` (input field, submit button, responsive sections).
- [ ] T011 [P] [US1] Implement the client service wrapper in `frontend/web/src/lib/services/shorten.ts` that calls `/api/shorten` and normalizes responses.
- [ ] T012 [P] [US1] Create the copy-to-clipboard control in `frontend/web/src/lib/components/CopyButton.svelte` with fallback instructions.
- [ ] T013 [US1] Render success state via `frontend/web/src/lib/components/ShortResultCard.svelte` and wire it into `+page.svelte` (includes open-in-new-tab link).
- [ ] T014 [P] [US1] Add Vitest unit tests in `frontend/web/tests/unit/shorten.spec.ts` covering service success/failure states and store transitions.
- [ ] T015 [US1] Add Playwright e2e test in `frontend/web/tests/e2e/shorten.spec.ts` validating the end-to-end shorten flow and copy action.

**Checkpoint**: User Story 1 is testable end-to-end (form → proxy → backend → result display/copy) and can ship as MVP.

---

## Phase 4: User Story 2 – Review and reuse recently shortened links (Priority: P2)

**Goal**: Persist the last five successful shortenings per browser session and provide controls to copy, open, or clear entries.

**Independent Test**: Shorten multiple URLs, refresh the page, and verify the “Recently shortened” list still shows entries with working copy/open actions and a clear-all control.

### Implementation for User Story 2

- [ ] T016 [US2] Implement session-based history store in `frontend/web/src/lib/stores/history.ts` (read/write via `sessionStorage`, graceful fallback when unavailable).
- [ ] T017 [P] [US2] Build the history list UI in `frontend/web/src/lib/components/RecentLinks.svelte` showing original/short URLs, timestamps, and badges.
- [ ] T018 [US2] Integrate the history list and controls (copy, open, clear, remove single entry) into `frontend/web/src/routes/+page.svelte`.
- [ ] T019 [P] [US2] Write unit tests in `frontend/web/tests/unit/history.spec.ts` covering persistence rules, de-duplication, and clear behavior.

**Checkpoint**: Visitors can reuse prior links without re-contacting the backend; story works independently atop US1 foundation.

---

## Phase 5: User Story 3 – Understand errors and backend status (Priority: P3)

**Goal**: Provide inline validation, backend error messaging, and retry guidance so users always know what went wrong and how to proceed.

**Independent Test**: Trigger invalid input, backend rejection, and simulated network failure; each shows distinct messaging, preserves the original URL, and exposes retry guidance without breaking the page.

### Implementation for User Story 3

- [ ] T020 [US3] Add client-side URL validation helpers in `frontend/web/src/lib/utils/validation.ts` and gate submission until input passes.
- [ ] T021 [US3] Display inline validation feedback in `frontend/web/src/routes/+page.svelte` (aria-live messaging, input states).
- [ ] T022 [P] [US3] Extend the proxy handler in `frontend/web/src/routes/api/shorten/+server.ts` to translate backend error payloads into standardized responses.
- [ ] T023 [US3] Implement a reusable notification/retry banner in `frontend/web/src/lib/components/StatusBanner.svelte` and wire it into store transitions.
- [ ] T024 [P] [US3] Add negative-path Playwright tests in `frontend/web/tests/e2e/errors.spec.ts` covering invalid input, backend 5xx, and timeout retries.

**Checkpoint**: Users receive actionable guidance for all failure modes; story can be validated independently.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Hardening tasks that span multiple stories (accessibility, docs, responsive tuning).

- [ ] T025 [P] Audit accessibility (focus states, aria attributes, reduced-motion) across `frontend/web/src/lib/components/` and update Tailwind tokens accordingly.
- [ ] T026 [P] Optimize responsive layouts (mobile ≤414px, tablet, desktop) in `frontend/web/src/routes/+page.svelte` and shared components.
- [ ] T027 Document frontend setup, environment variables, and test commands in `frontend/web/README.md` aligned with backend docs.
- [ ] T028 Add lightweight telemetry/log hooks (console event logging or SvelteKit hooks) in `frontend/web/src/hooks.client.ts` / `hooks.server.ts` to trace submissions and failures.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)** → prerequisite for all other work.
- **Phase 2 (Foundational)** → depends on Phase 1; blocks all user stories until proxy/config/store are ready.
- **Phase 3 (US1 / MVP)** → depends on Phase 2; unlocks MVP delivery.
- **Phase 4 (US2)** → depends on successful completion of US1 form/store so history can hook into existing flows.
- **Phase 5 (US3)** → depends on Phase 2 proxy plus US1 UI scaffolding; may run parallel with late US2 tasks once shared files stabilize.
- **Phase 6 (Polish)** → depends on completion of desired user stories.

### User Story Dependencies

1. **US1 (P1)** → first deliverable (MVP).
2. **US2 (P2)** → requires US1 submission/results logic.
3. **US3 (P3)** → builds on US1 infrastructure but can start after foundational work once error surfaces are defined.

### Parallel Opportunities

- Setup/Foundational tasks marked [P] (lint/test config, env files, history UI) can run concurrently once dependencies met.
- Within each user story, tasks marked [P] target different files (service vs. component vs. tests) and may proceed in parallel.
- Automated tests (Vitest/Playwright) can be authored while implementation stabilizes, as long as APIs are stubbed.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Implement and test all Phase 3 tasks.
3. Demo/ship the basic shortening flow once Vitest + Playwright checks pass.

### Incremental Delivery

1. Ship MVP (US1) → gather feedback.
2. Layer in US2 history experience, release once independently tested.
3. Add US3 error-handling improvements, release after regression tests.
4. Finish with polish tasks to harden accessibility, docs, and telemetry.

### Team Parallelization

- Developer A: Focus on proxy/config + US1 form.
- Developer B: History store/UI (US2) once US1 store events exist.
- Developer C: Error handling + tests (US3) overlapping with US2.

---

## Notes

- Maintain strict separation between client and server runtimes; never expose backend secrets in browser bundles.
- Keep tasks atomic—commit after each checkbox when possible.
- Validate Playwright suites against a running backend or mocked proxy responses before marking tasks complete.
