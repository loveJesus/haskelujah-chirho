<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import '@fontsource-variable/cormorant';
	import '@fontsource-variable/cormorant/wght-italic.css';
	import '@fontsource-variable/eb-garamond';
	import '@fontsource-variable/eb-garamond/wght-italic.css';
	import '@fontsource-variable/cinzel';
	import '@fontsource-variable/recursive/full.css';

	import type { Snippet as SnippetChirho } from 'svelte';

	let { children: childrenChirho }: { children: SnippetChirho } = $props();
</script>

<svelte:head>
	<title>Haskelujah — A Haskell Compiler Written in Rust</title>
</svelte:head>

{@render childrenChirho()}

<style>
	/* ============================================================
	   THE ILLUMINATED COMPILER — design tokens
	   Palette drawn from the illuminated-manuscript tradition:
	   parchment, iron-gall ink, lapis lazuli, gold leaf, vermilion,
	   verdigris. Dark "nave" for the hero; lit "codex" for content.
	   ============================================================ */
	:global(:root) {
		/* nave (dark) */
		--nave-abyss-chirho: #070b18;
		--nave-deep-chirho: #0a1024;
		--nave-chirho: #111a3c;
		--nave-line-chirho: rgba(212, 175, 55, 0.16);
		--nave-text-chirho: #f4ecd7;
		--nave-text-soft-chirho: #cfc4a6;
		--nave-text-muted-chirho: #94896f;

		/* parchment (light) */
		--parchment-chirho: #f6efdf;
		--parchment-bright-chirho: #fbf6ea;
		--parchment-deep-chirho: #eaddbe;
		--parchment-shadow-chirho: #d9c99f;

		/* ink */
		--ink-chirho: #221c15;
		--ink-soft-chirho: #4d4030;
		--ink-faint-chirho: #75654a;
		--ink-line-chirho: rgba(36, 27, 16, 0.18);

		/* gold leaf */
		--gold-chirho: #c9a227;
		--gold-bright-chirho: #eed688;
		--gold-deep-chirho: #8e6f14;
		--gold-foil-chirho: linear-gradient(
			135deg,
			#f7e7ae 0%,
			#e3c363 24%,
			#b98f1d 47%,
			#f2da8c 71%,
			#a87f16 100%
		);

		/* lapis lazuli */
		--lapis-chirho: #1e3e8c;
		--lapis-deep-chirho: #14275c;
		--lapis-bright-chirho: #3e63c4;

		/* vermilion + verdigris (fail / pass, period-accurate) */
		--vermilion-chirho: #c23b22;
		--vermilion-bright-chirho: #e34234;
		--vermilion-rubric-chirho: #b02e18;
		--verdigris-chirho: #2e6b57;
		--verdigris-deep-chirho: #24523f;

		/* type */
		--font-display-chirho: 'Cormorant Variable', 'Cormorant', Georgia, serif;
		--font-body-chirho: 'EB Garamond Variable', 'EB Garamond', Georgia, serif;
		--font-caps-chirho: 'Cinzel Variable', 'Cinzel', 'Trajan Pro', serif;
		--font-mono-chirho: 'Recursive Variable', 'Recursive', ui-monospace, monospace;

		/* rhythm */
		--measure-chirho: 68ch;
		--section-pad-chirho: clamp(4.5rem, 9vw, 8rem);
		--edge-pad-chirho: clamp(1.25rem, 4vw, 2.5rem);
	}

	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	@media (prefers-reduced-motion: no-preference) {
		:global(html) {
			scroll-behavior: smooth;
		}
	}

	:global(html) {
		scrollbar-color: var(--gold-deep-chirho) var(--nave-abyss-chirho);
	}

	:global(body) {
		font-family: var(--font-body-chirho);
		font-size: 1.125rem;
		background: var(--nave-abyss-chirho);
		color: var(--nave-text-chirho);
		line-height: 1.72;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		text-rendering: optimizeLegibility;
		overflow-x: hidden;
	}

	:global(::selection) {
		background: rgba(201, 162, 39, 0.35);
		color: inherit;
	}

	:global(a) {
		color: inherit;
		text-decoration: none;
	}

	:global(:focus-visible) {
		outline: 2px solid var(--gold-chirho);
		outline-offset: 3px;
		border-radius: 2px;
	}

	:global(code),
	:global(pre) {
		font-family: var(--font-mono-chirho);
		/* Recursive's MONO axis: 1 = strict monospace for code */
		font-variation-settings: 'MONO' 1;
	}

	:global(img),
	:global(svg),
	:global(video),
	:global(canvas) {
		max-width: 100%;
		display: block;
	}

	/* shared primitives used across section components */
	:global(.caps-label-chirho) {
		font-family: var(--font-caps-chirho);
		font-size: 0.72rem;
		font-weight: 600;
		letter-spacing: 0.22em;
		text-transform: uppercase;
	}

	:global(.section-inner-chirho) {
		max-width: 1140px;
		margin: 0 auto;
		padding-left: var(--edge-pad-chirho);
		padding-right: var(--edge-pad-chirho);
	}

	:global(.visually-hidden-chirho) {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0 0 0 0);
		white-space: nowrap;
		border: 0;
	}
</style>
