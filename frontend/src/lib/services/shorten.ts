export interface ShortenResponse {
	short_code: string;
	long_url: string;
}

export interface ErrorResponse {
	error: string;
	reason?: string;
}

export async function shortenUrl(longUrl: string): Promise<ShortenResponse> {
	const response = await fetch('/api/shorten', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ long_url: longUrl })
	});

	const data = await response.json();

	if (!response.ok) {
		const errorData = data as ErrorResponse;
		throw new Error(errorData.reason || errorData.error || 'Failed to shorten URL');
	}

	return data as ShortenResponse;
}
