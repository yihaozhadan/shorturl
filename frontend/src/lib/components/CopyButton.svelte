<script lang="ts">
	interface Props {
		text: string;
		label?: string;
	}

	let { text, label = 'Copy' }: Props = $props();
	let copied = $state(false);
	let showFallback = $state(false);

	async function handleCopy() {
		try {
			await navigator.clipboard.writeText(text);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (err) {
			showFallback = true;
		}
	}
</script>

<button
	onclick={handleCopy}
	class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
	aria-label={copied ? 'Copied!' : label}
>
	{copied ? 'Copied!' : label}
</button>

{#if showFallback}
	<p class="mt-2 text-sm text-gray-600">
		Your browser doesn't support automatic copying. Please select and copy the text manually.
	</p>
{/if}
