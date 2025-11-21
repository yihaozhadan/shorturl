import { writable } from 'svelte/store';

export type ShorteningState = 'idle' | 'loading' | 'success' | 'error';

export interface ShorteningStatus {
	state: ShorteningState;
	originalUrl: string;
	shortCode?: string;
	shortUrl?: string;
	error?: string;
	errorReason?: string;
}

function createShorteningStore() {
	const { subscribe, set, update } = writable<ShorteningStatus>({
		state: 'idle',
		originalUrl: ''
	});

	return {
		subscribe,
		reset: () =>
			set({
				state: 'idle',
				originalUrl: ''
			}),
		startShortening: (url: string) =>
			update((state) => ({
				...state,
				state: 'loading',
				originalUrl: url,
				error: undefined,
				errorReason: undefined
			})),
		setSuccess: (shortCode: string, shortUrl: string) =>
			update((state) => ({
				...state,
				state: 'success',
				shortCode,
				shortUrl
			})),
		setError: (error: string, reason?: string) =>
			update((state) => ({
				...state,
				state: 'error',
				error,
				errorReason: reason
			}))
	};
}

export const shorteningStore = createShorteningStore();
