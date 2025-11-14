# Implementation Plan: URL Shortening Backend Service

**Branch**: `001-build-backend-service` | **Date**: 2025-11-14 | **Spec**: `specs/001-build-backend-service/spec.md`
**Input**: Feature specification from `/specs/001-build-backend-service/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a backend service that converts long URLs into short, easy-to-share URLs
and reliably redirects short URLs back to their original destinations. The
service is implemented in Rust using the Actix web framework, with SurrealDB as
the primary data store for URL mappings and related metadata.

## Technical Context

**Language/Version**: Rust (stable, e.g., 1.80+)  
**Primary Dependencies**: Actix Web (HTTP server and routing), Serde (JSON
serialization), a SurrealDB client library for Rust  
**Storage**: SurrealDB as the primary database for URL mappings and related
metadata  
**Testing**: `cargo test` with unit tests for URL mapping logic and integration
tests for HTTP endpoints  
**Target Platform**: Linux server deployment (container-friendly)  
**Project Type**: web backend service  
**Performance Goals**: Handle typical URL-shortening traffic with p95 redirect
latency under 200ms at normal load, matching spec success criteria  
**Constraints**: <200ms p95 redirect latency, efficient connection management
to SurrealDB, and predictable error responses for a consistent API UX  
**Scale/Scope**: Single backend service for URL shortening and redirection;
scale horizontally by running multiple instances behind a load balancer

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Code Quality First**
  - Plan includes clear separation between routing, business logic, and data
    access modules to keep code simple and maintainable.
  - Rust tooling (`cargo fmt`, `cargo clippy`) will be used to enforce
    formatting and linting.

- **Consistent User Experience**
  - All HTTP APIs will return consistent JSON response structures and status
    codes for success and error cases.
  - Error messages will be actionable and avoid leaking internal details.

- **Performance and Efficiency (NON-NEGOTIABLE)**
  - The plan defines a p95 latency goal of <200ms for redirect requests.
  - SurrealDB access patterns will be designed to avoid unnecessary round
    trips and to use indexed lookups for short-code resolution.

- **Reliability and Observability**
  - Logging will capture URL creation, redirect failures, and invalid input for
    debugging and monitoring.
  - Metrics (where available) will track request rates, error rates, and
    latency to detect regressions.

- **Incremental Delivery with Strong Safety Nets**
  - The implementation will be broken into independently testable user stories:
    create short URL, redirect, and error handling.
  - Automated tests will cover core flows before deployment.

**Gate Evaluation**: No planned violations of the constitution at this stage.
If later design introduces complexity (e.g., additional services or advanced
caching), it must be documented in Complexity Tracking with rationale.

## Project Structure

### Documentation (this feature)

```text
specs/001-build-backend-service/
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
src/
├── routes/        # Actix handlers and routing configuration
├── services/      # URL shortening and lookup business logic
├── db/            # SurrealDB client setup and queries
└── config/        # Configuration loading (DB URL, server settings, etc.)

tests/
├── integration/   # HTTP endpoint tests using a test server
└── unit/          # Unit tests for services and mapping logic
```

**Structure Decision**: Single backend project in `src/` with clear separation
between routes, services, database access, and configuration. Tests are split
into unit and integration tests to align with the constitution's emphasis on
code quality and reliability.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
