<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Short URL - {data.shortCode}</title>
</svelte:head>

<main class="mx-auto min-h-screen max-w-2xl px-4 py-12 sm:px-6 lg:px-8">
	<div class="text-center">
		{#if data.notFound}
			<div class="rounded-lg border border-red-200 bg-red-50 p-8">
				<h1 class="text-3xl font-bold text-red-900">Short URL Not Found</h1>
				<p class="mt-4 text-lg text-red-700">
					The short code <code class="rounded bg-red-100 px-2 py-1 font-mono">{data.shortCode}</code> does not exist or has expired.
				</p>
				<a
					href="/"
					class="mt-6 inline-block rounded-md bg-blue-600 px-6 py-3 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
				>
					Create a Short URL
				</a>
			</div>
		{:else if data.error}
			<div class="rounded-lg border border-yellow-200 bg-yellow-50 p-8">
				<h1 class="text-3xl font-bold text-yellow-900">Error Loading URL</h1>
				<p class="mt-4 text-lg text-yellow-700">
					{data.message || 'An unexpected error occurred while resolving the short URL.'}
				</p>
				<div class="mt-6 flex justify-center gap-4">
					<button
						onclick={() => window.location.reload()}
						class="rounded-md bg-yellow-600 px-6 py-3 text-base font-medium text-white hover:bg-yellow-700 focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2"
					>
						Retry
					</button>
					<a
						href="/"
						class="rounded-md bg-blue-600 px-6 py-3 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
					>
						Go Home
					</a>
				</div>
			</div>
		{:else}
			<div class="rounded-lg border border-blue-200 bg-blue-50 p-8">
				<div class="animate-pulse">
					<h1 class="text-2xl font-semibold text-blue-900">Redirecting...</h1>
					<p class="mt-4 text-blue-700">
						You should be redirected automatically. If not, please check the URL.
					</p>
				</div>
			</div>
		{/if}
	</div>
</main>
