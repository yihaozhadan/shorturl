<script lang="ts">
	import '../app.css';
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}

<footer class="mt-12 text-center text-sm text-gray-500">
	&copy; {new Date().getFullYear()} <a href="https://huizhou.dev">Hui Zhou</a>. All rights reserved.
</footer>