# Research: ShortURL Web Frontend

## Backend integration without leaking credentials
- **Decision**: Proxy all shortening requests through a SvelteKit server route that reads the backend base URL and credentials from server-only environment variables.
- **Rationale**: Server routes execute on the backend side of SvelteKit, keeping secrets out of the client bundle while still letting the single-page UI call a relative path (e.g., `/api/shorten`).
- **Alternatives considered**:
  - *Direct browser calls with public env vars*: rejected because Vite/SvelteKit would embed the values in the client bundle, exposing credentials.
  - *Standalone BFF service*: overkill for a single page and would increase deployment complexity.

## Error handling and retry UX
- **Decision**: Normalize backend error responses into structured UI notifications (validation, backend rejection, network failure) and keep the original input for retries.
- **Rationale**: Matches spec success criteria for clear messaging, helps distinguish user vs. system issues, and enables retry without retyping URLs.
- **Alternatives considered**:
  - *Generic toast for all failures*: rejected because it conflicts with the constitution’s consistency principle and spec requirement for actionable guidance.

## Recent links storage scope
- **Decision**: Store the last five successful shortenings in `sessionStorage`, falling back to in-memory state if storage is unavailable or disabled.
- **Rationale**: Session storage keeps data per-tab without requiring backend persistence, satisfying the “reuse recently shortened links” story while respecting privacy expectations.
- **Alternatives considered**:
  - *LocalStorage*: rejected to avoid sharing history across browser tabs/devices when users may expect isolation.
  - *Backend persistence per user*: would require auth and introduce new backend scope outside this feature.

## Styling and layout system
- **Decision**: Use Tailwind CSS utility classes layered on top of a minimal design token set (spacing, colors) defined in `tailwind.config.js` to ensure consistent responsive spacing and focus states.
- **Rationale**: Aligns with request to use Tailwind, accelerates building responsive layouts, and makes it easier to guarantee accessibility (focus rings, contrast) without bespoke CSS.
- **Alternatives considered**:
  - *Pure CSS modules*: would slow down iteration and duplicate utilities Tailwind already provides.
  - *Component library*: unnecessary overhead for a single-page UI.
