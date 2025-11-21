<script lang="ts">
	interface Props {
		type: 'error' | 'info' | 'warning';
		message: string;
		details?: string;
		onRetry?: () => void;
		onDismiss?: () => void;
	}

	let { type, message, details, onRetry, onDismiss }: Props = $props();

	const bgColors = {
		error: 'bg-red-50 border-red-200',
		warning: 'bg-yellow-50 border-yellow-200',
		info: 'bg-blue-50 border-blue-200'
	};

	const textColors = {
		error: 'text-red-800',
		warning: 'text-yellow-800',
		info: 'text-blue-800'
	};
</script>

<div
	class="mt-4 rounded-lg border p-4 {bgColors[type]}"
	role="alert"
	aria-live="polite"
>
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<p class="font-medium {textColors[type]}">{message}</p>
			{#if details}
				<p class="mt-1 text-sm {textColors[type]} opacity-80">{details}</p>
			{/if}
		</div>
		<div class="ml-4 flex gap-2">
			{#if onRetry}
				<button
					onclick={onRetry}
					class="rounded-md bg-white px-3 py-1 text-sm font-medium shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 {textColors[type]}"
				>
					Retry
				</button>
			{/if}
			{#if onDismiss}
				<button
					onclick={onDismiss}
					class="rounded-md bg-white px-3 py-1 text-sm font-medium shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 {textColors[type]}"
					aria-label="Dismiss"
				>
					×
				</button>
			{/if}
		</div>
	</div>
</div>
