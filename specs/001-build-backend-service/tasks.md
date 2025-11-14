---

description: "Task list for URL Shortening Backend Service"
---

# Tasks: URL Shortening Backend Service

**Input**: Design documents from `/specs/001-build-backend-service/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL. For this feature, we will include basic tests for core flows to align with the constitution’s quality and reliability principles.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Backend service: `src/` for Rust code, `tests/` for Rust tests
- HTTP handlers and routing: `src/routes/`
- Business logic: `src/services/`
- Database access: `src/db/`
- Configuration: `src/config/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create Rust project structure for backend service in `src/` and `tests/`
- [ ] T002 Initialize Cargo project and dependencies in `Cargo.toml` for Actix Web, Serde, and SurrealDB client
- [ ] T003 [P] Configure Rust formatting and linting tools (`cargo fmt`, `cargo clippy`) in project root
- [ ] T004 [P] Add base configuration loading module in `src/config/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Setup SurrealDB connection client and configuration in `src/db/mod.rs`
- [ ] T006 [P] Define SurrealDB schema or table/record structure for URL mappings in `src/db/schema.rs`
- [ ] T007 [P] Implement application startup with Actix HTTP server in `src/main.rs`
- [ ] T008 Configure basic logging for incoming requests and errors in `src/config/logging.rs`
- [ ] T009 Configure environment-based settings (e.g., SurrealDB URL, server port) in `src/config/env.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Create a short URL from a long URL (Priority: P1) 🎯 MVP

**Goal**: Allow users to submit a long URL and receive a short URL they can share.

**Independent Test**: Given a valid long URL, sending it to the API returns a short URL that redirects back to the original URL.

### Implementation for User Story 1

- [ ] T010 [P] [US1] Define `UrlMapping` data structure in `src/models/url_mapping.rs`
- [ ] T011 [P] [US1] Implement SurrealDB CRUD helper for creating and reading URL mappings in `src/db/url_mapping_repo.rs`
- [ ] T012 [US1] Implement URL shortening service (long → short mapping logic) in `src/services/url_shortener.rs`
- [ ] T013 [US1] Implement HTTP route handler for creating short URLs in `src/routes/create_short_url.rs`
- [ ] T014 [US1] Wire `create short URL` route into Actix application in `src/routes/mod.rs`
- [ ] T015 [P] [US1] Add integration test for creating a short URL via HTTP API in `tests/integration/create_short_url_test.rs`
- [ ] T016 [P] [US1] Add unit tests for URL shortening service (mapping behavior, idempotency) in `tests/unit/url_shortener_tests.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently.

---

## Phase 4: User Story 2 - Handle invalid or unsafe URLs (Priority: P2)

**Goal**: Ensure invalid or clearly unsafe URLs are rejected with clear errors and not stored.

**Independent Test**: Submitting malformed URLs or URLs failing basic validation returns clear error responses and no short URL is created.

### Implementation for User Story 2

- [ ] T017 [P] [US2] Implement URL validation logic (syntax and basic safety checks) in `src/services/url_validation.rs`
- [ ] T018 [US2] Integrate URL validation into `create short URL` flow in `src/services/url_shortener.rs`
- [ ] T019 [US2] Standardize error response format for invalid/unsafe URLs in `src/routes/create_short_url.rs`
- [ ] T020 [P] [US2] Add integration tests for invalid/unsafe URL submissions in `tests/integration/invalid_url_tests.rs`

**Checkpoint**: User Story 1 and User Story 2 both work independently and provide clear error feedback.

---

## Phase 5: User Story 3 - Reuse short URLs for the same long URL (Priority: P3)

**Goal**: Ensure the same long URL consistently maps to the same short URL to avoid duplicates.

**Independent Test**: Submitting the same long URL multiple times returns the same short URL identifier.

### Implementation for User Story 3

- [ ] T021 [P] [US3] Update URL mapping repository to query existing mapping before creating a new one in `src/db/url_mapping_repo.rs`
- [ ] T022 [US3] Update URL shortening service to return existing mapping when available in `src/services/url_shortener.rs`
- [ ] T023 [P] [US3] Add integration tests covering repeated submissions of the same long URL in `tests/integration/reuse_short_url_tests.rs`

**Checkpoint**: All three user stories are independently functional and testable.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T024 [P] Add structured logging for URL creation and redirects in `src/config/logging.rs`
- [ ] T025 [P] Add basic metrics hooks (where feasible) around request counts and latency in `src/services/url_shortener.rs`
- [ ] T026 Improve error message consistency for all endpoints in `src/routes/mod.rs` and individual route files
- [ ] T027 [P] Documentation updates for API usage in `docs/api/url_shortening.md`
- [ ] T028 Performance optimization for URL lookup and redirect path in `src/services/url_shortener.rs`
- [ ] T029 [P] Additional unit tests for edge cases (missing mappings, expired URLs) in `tests/unit/url_mapping_edge_cases_tests.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3–5)**: All depend on Foundational phase completion
  - User stories can proceed in priority order (P1 → P2 → P3)
  - Some tasks marked [P] can run in parallel once dependencies are satisfied
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - independent core flow
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) and after basic create-short-URL path exists (US1)
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) and US1 data model/repository are available

### Within Each User Story

- Models and repository functions before services
- Services before HTTP routes
- Core implementation before integration tests
- Each story should be complete and testable before moving to the next

### Parallel Opportunities

- Setup tasks marked [P]
- Foundational tasks marked [P]
- Within each user story, tasks marked [P] that touch different files
- Integration tests can often be written and run in parallel once endpoints exist

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Create short URL and redirect)
4. **STOP and VALIDATE**: Test User Story 1 independently (end-to-end: create → redirect)
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no hard dependencies
- [Story] label maps the task to a specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing when adding new test cases
- Commit after each task or logical group
- Stop at any checkpoint to validate stories independently
- Avoid vague tasks or tasks that mix concerns across stories
