<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import { linksChirho } from '$lib-chirho/data-chirho/content-chirho';

	let litChirho = $state(false);
	let tickingChirho = false;

	function onScrollChirho(): void {
		if (tickingChirho) return;
		tickingChirho = true;
		requestAnimationFrame(() => {
			litChirho = window.scrollY > window.innerHeight * 2.05;
			tickingChirho = false;
		});
	}
</script>

<svelte:window onscroll={onScrollChirho} />

<header class="nav-chirho" class:lit-chirho={litChirho}>
	<nav class="nav-inner-chirho" aria-label="Primary">
		<a href="#top-chirho" class="nav-mark-chirho">
			<span class="nav-chi-rho-chirho" aria-hidden="true">☧</span>
			<span class="nav-word-chirho">Haskelujah</span>
		</a>
		<div class="nav-links-chirho">
			<a href="#craft-chirho" class="nav-link-chirho">The Craft</a>
			<a href="#proof-chirho" class="nav-link-chirho">Proof</a>
			<a href="#tools-chirho" class="nav-link-chirho">Tools</a>
			<a href="#scriptorium-chirho" class="nav-link-chirho">Scriptorium</a>
			<a href="#begin-chirho" class="nav-begin-chirho">Begin</a>
			<a
				href={linksChirho.githubChirho}
				target="_blank"
				rel="noopener noreferrer"
				class="nav-link-chirho">GitHub</a
			>
		</div>
	</nav>
</header>

<style>
	.nav-chirho {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 40;
		background: linear-gradient(180deg, rgba(7, 11, 24, 0.82), rgba(7, 11, 24, 0));
		transition: background 0.5s ease, box-shadow 0.5s ease;
	}

	.nav-chirho.lit-chirho {
		background: rgba(246, 239, 223, 0.88);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		box-shadow: 0 1px 0 rgba(36, 27, 16, 0.14), 0 6px 24px rgba(36, 27, 16, 0.08);
	}

	.nav-inner-chirho {
		max-width: 1180px;
		margin: 0 auto;
		height: 64px;
		padding: 0 var(--edge-pad-chirho);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.nav-mark-chirho {
		display: inline-flex;
		align-items: baseline;
		gap: 0.55rem;
		color: var(--gold-bright-chirho);
	}

	.lit-chirho .nav-mark-chirho {
		color: var(--gold-deep-chirho);
	}

	.nav-chi-rho-chirho {
		font-size: 1.35rem;
		line-height: 1;
	}

	.nav-word-chirho {
		font-family: var(--font-display-chirho);
		font-size: 1.3rem;
		font-weight: 600;
		letter-spacing: 0.02em;
		color: var(--nave-text-chirho);
		transition: color 0.5s ease;
	}

	.lit-chirho .nav-word-chirho {
		color: var(--ink-chirho);
	}

	.nav-links-chirho {
		display: flex;
		align-items: center;
		gap: 1.6rem;
	}

	.nav-link-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.64rem;
		font-weight: 600;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		color: var(--nave-text-soft-chirho);
		transition: color 0.25s ease;
	}

	.lit-chirho .nav-link-chirho {
		color: var(--ink-soft-chirho);
	}

	.nav-link-chirho:hover {
		color: var(--gold-bright-chirho);
	}

	.lit-chirho .nav-link-chirho:hover {
		color: var(--gold-deep-chirho);
	}

	.nav-begin-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.64rem;
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		padding: 0.5rem 1.1rem;
		border-radius: 2px;
		background: var(--gold-foil-chirho);
		color: #1c1408;
		box-shadow: 0 2px 12px rgba(201, 162, 39, 0.25);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.nav-begin-chirho:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 18px rgba(201, 162, 39, 0.4);
	}

	@media (max-width: 820px) {
		.nav-link-chirho {
			display: none;
		}
	}
</style>
