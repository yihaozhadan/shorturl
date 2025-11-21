# ShortURL Frontend

A responsive web frontend for the ShortURL service, built with SvelteKit, TypeScript, and Tailwind CSS.

## Features

- **Simple URL Shortening**: Paste a long URL and get a short, shareable link
- **Client-side Validation**: Instant feedback on URL format before submission
- **Error Handling**: Clear, actionable error messages with retry functionality
- **Responsive Design**: Works seamlessly on mobile, tablet, and desktop
- **Accessible**: Keyboard navigation, ARIA labels, and focus states

## Prerequisites

- Node.js 20+ 
- Running ShortURL backend service (default: `http://localhost:8080`)

## Environment Variables

Create a `.env` file in the frontend directory (see `.env.example`):

```bash
# Server-side only - used for API proxy
BACKEND_URL=http://localhost:8080

# Public-facing - used for generating short URLs
PUBLIC_BACKEND_URL=http://localhost:8080
```

**Important**: 
- `BACKEND_URL` is server-side only and will NOT be exposed to the browser. The frontend proxies requests through `/api/shorten` to keep credentials secure.
- `PUBLIC_BACKEND_URL` is the public-facing URL for your backend where short URLs will redirect. This is typically your production backend domain (e.g., `https://short.yourdomain.com`).

## Installation

```bash
npm install
```

## Development

Start the development server:

```bash
npm run dev

# or start and open in browser
npm run dev -- --open
```

The app will be available at `http://localhost:5173` by default.

## Testing

Run unit tests:
```bash
npm run test:unit
```

Run end-to-end tests:
```bash
npm run test:e2e
```

Run all tests:
```bash
npm test
```

## Building

Create a production build:

```bash
npm run build
```

Preview the production build:

```bash
npm run preview
```

## Code Quality

Format code:
```bash
npm run format
```

Lint and check:
```bash
npm run lint
npm run check
```

## Project Structure

```
frontend/
├── src/
│   ├── lib/
│   │   ├── components/      # Reusable UI components
│   │   ├── services/        # API client functions
│   │   ├── stores/          # Svelte stores for state management
│   │   ├── utils/           # Helper functions
│   │   └── server/          # Server-only code (config)
│   ├── routes/
│   │   ├── +page.svelte     # Landing page
│   │   ├── +layout.svelte   # Global layout
│   │   └── api/shorten/     # Backend proxy endpoint
│   └── app.css              # Tailwind imports
├── tests/                   # Test files
└── static/                  # Static assets
```

## How URL Redirection Works

The architecture separates URL creation from URL redirection:

1. **Creating Short URLs**: Users interact with the frontend (SvelteKit) to create short URLs. The frontend proxies these requests to the backend through `/api/shorten`, keeping backend credentials secure.

2. **Using Short URLs**: When someone clicks a short URL (e.g., `http://localhost:8080/abc123`), the request goes **directly to the backend**, which handles the redirect to the long URL. The frontend is not involved in this flow.

3. **Fallback Route**: The frontend also provides a `[shortCode]` route that can proxy redirects if needed, but the recommended approach is to point short URLs directly to the backend for optimal performance.

## Deployment

The frontend requires:
1. A Node.js environment for the API proxy route
2. The `BACKEND_URL` environment variable set to your backend service
3. The `PUBLIC_BACKEND_URL` environment variable set to your public-facing backend URL

Deploy using SvelteKit adapters for your target platform (Vercel, Netlify, Node.js server, etc.).

**Production Configuration Example**:
```bash
BACKEND_URL=http://backend-internal:8080
PUBLIC_BACKEND_URL=https://short.yourdomain.com
```
