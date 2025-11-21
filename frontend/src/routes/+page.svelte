<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { shorteningStore } from '$lib/stores/shortening';
	import { shortenUrl } from '$lib/services/shorten';
	import { getValidationError } from '$lib/utils/validation';
	import ShortResultCard from '$lib/components/ShortResultCard.svelte';
	import StatusBanner from '$lib/components/StatusBanner.svelte';

	let urlInput = $state('');
	let validationError = $state<string | null>(null);

	const status = $derived($shorteningStore);
	
	// Use PUBLIC_BACKEND_URL if set, otherwise default to window origin for development
	function getShortUrlBase(): string {
		return env.PUBLIC_BACKEND_URL || (typeof window !== 'undefined' ? window.location.origin : 'http://localhost:8080');
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();

		// Client-side validation
		const error = getValidationError(urlInput);
		if (error) {
			validationError = error;
			return;
		}

		validationError = null;
		shorteningStore.startShortening(urlInput);

		try {
			const result = await shortenUrl(urlInput);
			const shortUrlBase = getShortUrlBase();
			const fullShortUrl = `${shortUrlBase}/${result.short_code}`;
			shorteningStore.setSuccess(result.short_code, fullShortUrl);
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to shorten URL';
			shorteningStore.setError('Could not shorten URL', message);
		}
	}

	function handleRetry() {
		if (status.originalUrl) {
			urlInput = status.originalUrl;
			shorteningStore.reset();
		}
	}

	function handleInputChange() {
		validationError = null;
	}
</script>

<svelte:head>
	<title>URL Shortener</title>
	<meta name="description" content="Shorten your long URLs quickly and easily" />
	<meta name="author" content="Hui Zhou" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
</svelte:head>

<main class="mx-auto max-w-2xl px-4 py-12 sm:px-6 lg:px-8">
	<div class="text-center">
		<h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
			URL Shortener
		</h1>
		<p class="mt-4 text-lg text-gray-600">
			Transform long URLs into short, easy-to-share links
		</p>
	</div>

	<div class="mt-12">
		<form onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="url-input" class="block text-sm font-medium text-gray-700">
					Enter your long URL
				</label>
				<div class="mt-2">
					<input
						id="url-input"
						type="url"
						bind:value={urlInput}
						oninput={handleInputChange}
						placeholder="https://example.com/very/long/url/path"
						disabled={status.state === 'loading'}
						class="block w-full rounded-md border border-gray-300 px-4 py-3 text-gray-900 placeholder-gray-400 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:cursor-not-allowed disabled:bg-gray-100"
						aria-invalid={validationError !== null}
						aria-describedby={validationError ? 'url-error' : undefined}
					/>
				</div>
				{#if validationError}
					<p id="url-error" class="mt-2 text-sm text-red-600" role="alert">
						{validationError}
					</p>
				{/if}
			</div>

			<button
				type="submit"
				disabled={status.state === 'loading' || !urlInput.trim()}
				class="w-full rounded-md bg-blue-600 px-4 py-3 text-base font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:bg-gray-400"
			>
				{status.state === 'loading' ? 'Shortening...' : 'Shorten URL'}
			</button>
		</form>

		{#if status.state === 'success' && status.shortUrl}
			<ShortResultCard shortUrl={status.shortUrl} originalUrl={status.originalUrl} />
		{/if}

		{#if status.state === 'error'}
			<StatusBanner
				type="error"
				message={status.error || 'An error occurred'}
				details={status.errorReason}
				onRetry={handleRetry}
				onDismiss={() => shorteningStore.reset()}
			/>
		{/if}
	</div>

	<div class="mt-12 rounded-lg bg-gray-50 p-6">
		<h2 class="text-lg font-semibold text-gray-900">How it works</h2>
		<ol class="mt-4 space-y-2 text-sm text-gray-700">
			<li><strong>1.</strong> Paste your long URL in the input field above</li>
			<li><strong>2.</strong> Click the "Shorten URL" button</li>
			<li><strong>3.</strong> Copy and share your new short URL!</li>
		</ol>
	</div>
</main>
