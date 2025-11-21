<script lang="ts">
	import CopyButton from './CopyButton.svelte';

	interface Props {
		shortUrl: string;
		originalUrl: string;
	}

	let { shortUrl, originalUrl }: Props = $props();
</script>

<div
	class="mt-6 rounded-lg border border-green-200 bg-green-50 p-6"
	role="region"
	aria-label="Shortened URL result"
>
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<h3 class="text-sm font-medium text-gray-700">Your short URL:</h3>
			<a
				href={shortUrl}
				target="_blank"
				rel="noopener noreferrer"
				class="mt-2 block break-all text-lg font-semibold text-blue-600 hover:text-blue-800 hover:underline"
			>
				{shortUrl}
			</a>
			<p class="mt-2 text-sm text-gray-600">
				Original: <span class="break-all">{originalUrl}</span>
			</p>
		</div>
		<div class="ml-4">
			<CopyButton text={shortUrl} label="Copy" />
		</div>
	</div>
</div>
