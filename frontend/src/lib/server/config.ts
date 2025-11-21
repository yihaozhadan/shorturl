import { env } from '$env/dynamic/private';

export function getBackendUrl(): string {
	return env.BACKEND_URL || 'http://localhost:8080';
}
