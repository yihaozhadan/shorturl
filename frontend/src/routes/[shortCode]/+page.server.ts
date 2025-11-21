import { redirect } from '@sveltejs/kit';
import { getBackendUrl } from '$lib/server/config';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { shortCode } = params;
	const backendUrl = getBackendUrl();

	try {
		// Call backend redirect endpoint to get the long URL
		const response = await fetch(`${backendUrl}/${shortCode}`, {
			method: 'GET',
			redirect: 'manual' // Don't follow redirects automatically
		});

		// Backend returns 302 Found with Location header
		if (response.status === 302 || response.status === 301) {
			const location = response.headers.get('Location');
			if (location) {
				throw redirect(302, location);
			}
		}

		// If not found, backend returns 404
		if (response.status === 404) {
			return {
				notFound: true,
				shortCode
			};
		}

		// Unexpected status
		throw new Error(`Unexpected response status: ${response.status}`);
	} catch (err) {
		// If it's a redirect error, rethrow it
		if (err && typeof err === 'object' && 'status' in err && 'location' in err) {
			throw err;
		}

		// Otherwise, show error page
		return {
			error: true,
			shortCode,
			message: err instanceof Error ? err.message : 'Failed to resolve short URL'
		};
	}
};
