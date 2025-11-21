export function isValidUrl(input: string): boolean {
	if (!input || input.trim() === '') {
		return false;
	}

	const trimmed = input.trim();

	// Explicitly check if URL starts with http:// or https://
	if (!trimmed.startsWith('http://') && !trimmed.startsWith('https://')) {
		return false;
	}

	try {
		const url = new URL(trimmed);
		// Verify protocol again and ensure there's a hostname
		return (url.protocol === 'http:' || url.protocol === 'https:') && url.hostname !== '';
	} catch {
		return false;
	}
}

export function getValidationError(input: string): string | null {
	if (!input || input.trim() === '') {
		return 'Please enter a URL';
	}

	if (!isValidUrl(input)) {
		return 'Please enter a valid http:// or https:// URL';
	}

	return null;
}
