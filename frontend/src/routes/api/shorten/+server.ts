import { json } from '@sveltejs/kit';
import { getBackendUrl } from '$lib/server/config';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();
		const backendUrl = getBackendUrl();

		const response = await fetch(`${backendUrl}/shorten`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		const data = await response.json();

		if (!response.ok) {
			return json(data, { status: response.status });
		}

		return json(data);
	} catch (error) {
		console.error('Proxy error:', error);
		return json(
			{
				error: 'Service temporarily unavailable',
				reason: 'Could not connect to backend service'
			},
			{ status: 503 }
		);
	}
};
