<script lang="ts">
	import '../app.css';
	import './layout.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" type="image/svg+xml" href="/favicon.svg" />
	<link rel="alternate icon" type="image/png" sizes="96x96" href="/favicon-96x96.png" />
	<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
	<link rel="manifest" href="/site.webmanifest" />
	<meta name="theme-color" content="#ffffff" />
</svelte:head>

{@render children()}

<footer class="mt-12 text-center text-sm text-gray-500">
	&copy; {new Date().getFullYear()} <a href="https://huizhou.dev">Hui Zhou</a>. All rights reserved.
</footer>