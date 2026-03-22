<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import type { Snippet as SnippetChirho } from 'svelte';

	let { children: childrenChirho }: { children: SnippetChirho } = $props();
</script>

<svelte:head>
	<title>Haskelujah Chirho -- A Haskell Compiler Written in Rust</title>
</svelte:head>

{@render childrenChirho()}

<style>
	:global(:root) {
		--bg-outer-chirho: #120f0b;
		--bg-inner-chirho: #1b1711;
		--panel-chirho: rgba(27, 23, 17, 0.84);
		--panel-strong-chirho: rgba(35, 28, 18, 0.96);
		--line-chirho: rgba(246, 190, 96, 0.14);
		--text-main-chirho: #f7f0e3;
		--text-soft-chirho: #d7c8b2;
		--text-muted-chirho: #9f927f;
		--accent-gold-chirho: #f6be60;
		--accent-cyan-chirho: #71d7d1;
		--accent-emerald-chirho: #8dd68f;
		--shadow-chirho: 0 24px 80px rgba(0, 0, 0, 0.34);
	}

	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(html) {
		scroll-behavior: smooth;
	}

	:global(body) {
		font-family: 'Space Grotesk', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		background:
			radial-gradient(circle at top left, rgba(246, 190, 96, 0.1), transparent 32%),
			radial-gradient(circle at top right, rgba(113, 215, 209, 0.1), transparent 24%),
			linear-gradient(180deg, var(--bg-inner-chirho) 0%, var(--bg-outer-chirho) 100%);
		color: var(--text-main-chirho);
		line-height: 1.7;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	:global(code, pre) {
		font-family: 'IBM Plex Mono', 'JetBrains Mono', monospace;
	}

	:global(a) {
		color: var(--accent-cyan-chirho);
		text-decoration: none;
		transition: color 0.2s ease;
	}

	:global(a:hover) {
		color: #b7f0ec;
	}
</style>
