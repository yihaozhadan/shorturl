# Feature Specification: ShortURL Web Frontend

**Feature Branch**: `002-svelte-frontend`  
**Created**: 2025-11-18  
**Status**: Draft  
**Input**: User description: "Add a simple web frontend and integrate with the shorturl backend. Use Sveltekit, TailwindCSS and Vite."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Shorten a long URL from the browser (Priority: P1)

As an anonymous visitor, I want a single-page experience where I can paste a long URL, submit it, and immediately get a short link that I can copy or open so that I can quickly share readable links without touching the backend directly.

**Why this priority**: Provides the primary user value—the ability to create short URLs without using an API client—and is the minimum viable slice that makes the frontend worth shipping.

**Independent Test**: Launch the page in a supported browser, submit a valid URL, and confirm that a short link is returned, rendered, and copyable without depending on any other user stories.

**Acceptance Scenarios**:

1. **Given** a visitor on the landing page with a valid long URL in the form, **When** they submit the form, **Then** the UI shows a loading state and replaces it with the generated short URL plus a copy action.
2. **Given** a returned short link, **When** the visitor clicks it, **Then** it opens the redirect URL in a new tab/window so they can test it immediately.

---

### User Story 2 - Review and reuse recently shortened links (Priority: P2)

As a repeat visitor, I want to see a list of the most recent links I have shortened in this browser session, with clear labels and copy buttons, so I can reuse or share them later without resubmitting the long URL.

**Why this priority**: Improves usability and efficiency for users who shorten multiple URLs, reducing duplicate API calls and providing quick access to prior work.

**Independent Test**: Shorten multiple URLs in sequence, refresh the page, and verify that the session history persists (per defined retention rules) and allows copying or reopening entries without re-contacting the backend.

**Acceptance Scenarios**:

1. **Given** a visitor who has shortened at least one URL, **When** they open the "Recently shortened" section, **Then** they see at least the last five entries with original and short URLs plus timestamps.
2. **Given** a history entry, **When** the visitor selects "Copy" or "Open", **Then** that action operates on the stored short URL without triggering a new creation request.

---

### User Story 3 - Understand errors and backend status (Priority: P3)

As a visitor, I want clear, accessible messaging when my URL is invalid, the backend rejects the request, or the service is unavailable so that I know what to do next (fix the URL, retry later, or contact support) without guessing.

**Why this priority**: Transparent error handling builds trust and reduces support burden, ensuring users understand whether the failure is due to their input or the system.

**Independent Test**: Mock backend responses for validation failures, server errors, and network outages to confirm that each scenario produces distinct UI states, retry guidance, and does not block other user stories.

**Acceptance Scenarios**:

1. **Given** an invalid URL submission, **When** the user attempts to shorten it, **Then** inline validation prevents the request and provides actionable guidance without contacting the backend.
2. **Given** the backend returns a 5xx error or times out, **When** the user submits a valid URL, **Then** the UI shows a non-blocking error banner with retry controls and preserves the original input for resubmission.

---

<!-- Intentionally limited to three prioritized user stories; additional scope requires new stories. -->

### Edge Cases

- Extremely long URLs (>2,000 characters) that pass backend validation but exceed UI layout width must still be displayed (wrapping or truncation with tooltip) without breaking the page.
- Browsers that block clipboard APIs must still allow users to copy short URLs via manual selection with clear fallback instructions.
- Backend downtime or high latency should not freeze the UI; loading indicators must timeout gracefully and allow retries.
- Duplicate long URLs should not create duplicate history entries unless the backend issues a new short link (e.g., due to configuration change).
- Redirect URLs that the backend reports as unsafe or expired must be visually differentiated in the history list to prevent accidental sharing.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: The frontend MUST render an accessible landing view that works on viewport widths from 360px mobile screens upward without horizontal scrolling.
- **FR-002**: The UI MUST provide a single input for long URLs with client-side validation (syntax, presence of scheme) and prevent submission until the input is valid.
- **FR-003**: On submission, the frontend MUST call the existing shorturl backend create endpoint using the configured base URL and display a visual loading indicator until a response (success or error) is received.
- **FR-004**: Successful responses MUST surface the short URL prominently, making it clickable, showing the resolved domain, and providing a copy-to-clipboard control with fallback instructions when clipboard access fails.
- **FR-005**: The UI MUST maintain at least the last five successfully shortened URLs per browser (session storage by default, with graceful degradation if storage is unavailable) and show original URL, short URL, and creation timestamp.
- **FR-006**: Users MUST be able to clear individual history entries or the entire recent list to prevent accidental sharing of outdated links.
- **FR-007**: Error states (invalid input, backend validation failure, network/server error) MUST present distinct messaging, recommended next steps, and leave the input intact for correction or retry.
- **FR-008**: The frontend MUST detect backend unreachability within a configurable timeout (≤5 seconds) and expose a retry action without requiring a page refresh.
- **FR-009**: All interactive controls (submit, copy, retry, clear history) MUST be keyboard-accessible and include visible focus states to meet baseline accessibility expectations.

### Key Entities *(include if feature involves data)*

- **Shortening Request ViewModel**: Represents a pending or completed attempt to shorten a URL. Attributes: original URL, sanitized/validated URL, submission status (idle, loading, success, error), backend response metadata (short URL, error code), timestamp of last action.
- **Recent Link Entry**: Represents a successful shortening stored on the client. Attributes: original URL, short URL, createdAt timestamp, optional label/notes, status badge (active, expired/unsafe as reported by backend).
- **UI Notification**: Represents transient user feedback. Attributes: type (success, info, warning, error), message text, context (form, history list, clipboard), dismissibility, and optional retry callback reference.

### Assumptions & Dependencies

- The backend exposes a stable HTTP endpoint for creating short URLs as described in `specs/001-build-backend-service/spec.md` and supports browser origins via CORS.
- The frontend MUST be implemented in TypeScript on top of the existing SvelteKit + Tailwind stack while minimizing additional third-party libraries beyond what the framework requires.
- No authentication is required; all visitors share the same feature set, and rate limiting (if any) is handled server-side.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: 95% of valid URL submissions performed through the UI produce a short link (or actionable error) within 5 seconds end-to-end on broadband connections.
- **SC-002**: At least 90% of users who shorten a link can copy or open the returned short URL without needing to re-run the request (measured via telemetry or moderated testing).
- **SC-003**: Error states for invalid input vs. backend/service issues are correctly distinguished in 100% of tested scenarios, reducing ambiguous "something went wrong" messages to zero.
- **SC-004**: On mobile devices (≤414px width), all primary actions (submit, copy, view history, retry) remain reachable with no more than two taps and without horizontal scrolling, as verified by responsive testing.

## Clarifications

### Session 2025-11-18

- Q: Which implementation language and dependency strategy should the frontend follow to satisfy content-quality expectations? → A: Use TypeScript with the existing SvelteKit + Tailwind toolchain and keep additional libraries to the bare minimum necessary for functionality.
