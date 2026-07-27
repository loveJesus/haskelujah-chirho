<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import {
		shouldCompileChirho,
		shouldFailChirho,
		artifactRepoPathsChirho
	} from '$lib-chirho/data-chirho/compat-chirho';
	import {
		statsStripChirho,
		statsProvenanceChirho,
		linksChirho
	} from '$lib-chirho/data-chirho/content-chirho';

	const artifactUrlChirho = (pathChirho: string): string =>
		`${linksChirho.githubChirho}/blob/main_chirho/${pathChirho}`;
</script>

<section id="proof-chirho" class="proof-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			numeralChirho="I"
			kickerChirho="Truth in public"
			titleChirho="Measured, both ways"
			subChirho="Compatibility against GHC's own test suite — including the axis most compilers don't publish."
		/>

		<div class="manifesto-chirho">
			<p class="manifesto-lead-chirho">
				<span class="drop-cap-chirho" aria-hidden="true">T</span><span
					class="visually-hidden-chirho">T</span
				>here are many ways to claim compatibility, and one way to measure it: GHC's own
				test corpus, run both directions. A compiler must accept the programs GHC accepts —
				and it must <em>reject</em> the programs GHC rejects, because a compiler that
				accepts wrong programs is more dangerous than one that refuses right ones.
			</p>
			<p>
				These numbers are parsed at build time from measurement artifacts committed to the
				repository — corpus, method, and every failing file listed. This page cannot claim
				what the repo cannot prove.
			</p>
		</div>

		<div class="axes-chirho">
			<article class="axis-chirho axis-accept-chirho">
				<span class="axis-seal-chirho" aria-hidden="true">✓</span>
				<p class="axis-kicker-chirho caps-label-chirho">should_compile · accepts</p>
				<p class="axis-value-chirho">
					{shouldCompileChirho.passChirho}<span class="axis-of-chirho"
						>/ {shouldCompileChirho.totalChirho}</span
					>
				</p>
				<p class="axis-pct-chirho">{shouldCompileChirho.displayPercentChirho}</p>
				<p class="axis-desc-chirho">
					programs GHC accepts that Haskelujah also accepts
				</p>
				<a
					class="axis-meta-chirho"
					href={artifactUrlChirho(artifactRepoPathsChirho.shouldCompileChirho)}
					target="_blank"
					rel="noopener noreferrer"
				>
					measured {shouldCompileChirho.measuredDateChirho} @ {shouldCompileChirho.measuredCommitChirho}
				</a>
				{#if shouldCompileChirho.stabilityChirho}
					<p class="axis-stability-chirho">
						stability · {shouldCompileChirho.stabilityChirho}
					</p>
				{/if}
			</article>

			<article class="axis-chirho axis-reject-chirho">
				<span class="axis-seal-chirho" aria-hidden="true">✗</span>
				<p class="axis-kicker-chirho caps-label-chirho">should_fail · rejects</p>
				<p class="axis-value-chirho">
					{shouldFailChirho.passChirho}<span class="axis-of-chirho"
						>/ {shouldFailChirho.totalChirho}</span
					>
				</p>
				<p class="axis-pct-chirho">{shouldFailChirho.displayPercentChirho}</p>
				<p class="axis-desc-chirho">
					programs GHC rejects that Haskelujah also rejects — the honest number, and the
					current front of the work
				</p>
				<a
					class="axis-meta-chirho"
					href={artifactUrlChirho(artifactRepoPathsChirho.shouldFailChirho)}
					target="_blank"
					rel="noopener noreferrer"
				>
					measured {shouldFailChirho.measuredDateChirho} @ {shouldFailChirho.measuredCommitChirho}
				</a>
				{#if shouldFailChirho.stabilityChirho}
					<p class="axis-stability-chirho">
						stability · {shouldFailChirho.stabilityChirho}
					</p>
				{/if}
			</article>
		</div>

		<p class="method-note-chirho">
			Method: each axis is a single timed sweep of the corpus, 15-second per-file timeout,
			counts as committed in the artifacts above. A run-to-run variance of ±1 file has been
			observed — the type checker has a
			<a
				class="method-link-chirho"
				href={artifactUrlChirho('spec-chirho/bug-nondeterministic-typecheck-chirho.md')}
				target="_blank"
				rel="noopener noreferrer">known nondeterminism under investigation</a
			> — so percentages are floored to the whole, deliberately, never above their own range.
			The two axes carry different stability confidence, so each card quotes its artifact's
			stability line verbatim rather than flattening them into one claim. When a sweep is
			re-run and its artifact updated, this page updates with it.
		</p>

		<div class="stats-strip-chirho">
			{#each statsStripChirho as statChirho (statChirho.labelChirho)}
				<div class="stat-chirho">
					<span class="stat-value-chirho">{statChirho.valueChirho}</span>
					<span class="stat-label-chirho">{statChirho.labelChirho}</span>
				</div>
			{/each}
		</div>
		<p class="stats-provenance-chirho">{statsProvenanceChirho}</p>
	</div>
</section>

<style>
	.proof-chirho {
		padding: var(--section-pad-chirho) 0;
		background: var(--parchment-chirho);
	}

	.manifesto-chirho {
		max-width: var(--measure-chirho);
		margin: 0 auto clamp(2.5rem, 5vw, 3.6rem);
		color: var(--ink-chirho);
		font-size: 1.16rem;
		line-height: 1.8;
	}

	.manifesto-chirho p + p {
		margin-top: 1.1rem;
	}

	.manifesto-lead-chirho em {
		font-style: italic;
		color: var(--vermilion-rubric-chirho);
	}

	.drop-cap-chirho {
		float: left;
		font-family: var(--font-display-chirho);
		font-weight: 600;
		font-size: 4.6rem;
		line-height: 0.78;
		padding: 0.35rem 0.65rem 0.15rem 0;
		color: var(--lapis-chirho);
	}

	.axes-chirho {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: clamp(1.2rem, 3vw, 2.2rem);
		max-width: 900px;
		margin: 0 auto;
	}

	.axis-chirho {
		--axis-accent-chirho: var(--verdigris-chirho);
		position: relative;
		text-align: center;
		padding: clamp(1.8rem, 4vw, 2.8rem) clamp(1.2rem, 3vw, 2.2rem);
		background: var(--parchment-bright-chirho);
		border: 1px solid var(--ink-line-chirho);
		border-radius: 3px;
		box-shadow:
			0 1px 0 rgba(250, 244, 228, 0.9) inset,
			0 14px 40px rgba(36, 27, 16, 0.09);
	}

	.axis-chirho::before {
		content: '';
		position: absolute;
		inset: 8px;
		border: 1px solid color-mix(in srgb, var(--axis-accent-chirho) 42%, transparent);
		border-radius: 2px;
		pointer-events: none;
	}

	.axis-reject-chirho {
		--axis-accent-chirho: var(--vermilion-chirho);
	}

	.axis-seal-chirho {
		position: absolute;
		top: -1.05rem;
		left: 50%;
		transform: translateX(-50%) rotate(-4deg);
		width: 2.1rem;
		height: 2.1rem;
		display: grid;
		place-items: center;
		border-radius: 50%;
		background: radial-gradient(
			circle at 34% 30%,
			color-mix(in srgb, var(--axis-accent-chirho) 78%, #fff),
			var(--axis-accent-chirho) 58%,
			color-mix(in srgb, var(--axis-accent-chirho) 68%, #000)
		);
		color: var(--parchment-bright-chirho);
		font-size: 0.95rem;
		font-weight: 700;
		box-shadow:
			0 3px 8px rgba(36, 27, 16, 0.3),
			inset 0 1px 2px rgba(255, 255, 255, 0.35);
	}

	.axis-kicker-chirho {
		color: var(--axis-accent-chirho);
		margin-bottom: 1.1rem;
	}

	.axis-value-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(3rem, 7vw, 4.6rem);
		font-weight: 600;
		line-height: 1;
		color: var(--ink-chirho);
	}

	.axis-of-chirho {
		font-size: 0.42em;
		font-weight: 500;
		color: var(--ink-faint-chirho);
		margin-left: 0.3rem;
	}

	.axis-pct-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 1.02rem;
		font-weight: 700;
		letter-spacing: 0.12em;
		color: var(--axis-accent-chirho);
		margin: 0.5rem 0 0.9rem;
	}

	.axis-desc-chirho {
		font-size: 1rem;
		color: var(--ink-soft-chirho);
		max-width: 34ch;
		margin: 0 auto 1.1rem;
		line-height: 1.55;
	}

	.axis-meta-chirho {
		font-family: var(--font-mono-chirho);
		font-variation-settings: 'MONO' 1;
		font-size: 0.72rem;
		color: var(--ink-faint-chirho);
		border-bottom: 1px dotted var(--ink-faint-chirho);
		transition: color 0.2s ease, border-color 0.2s ease;
	}

	.axis-meta-chirho:hover {
		color: var(--lapis-chirho);
		border-color: var(--lapis-chirho);
	}

	.axis-stability-chirho {
		margin-top: 0.7rem;
		font-family: var(--font-mono-chirho);
		font-variation-settings: 'MONO' 1;
		font-size: 0.66rem;
		letter-spacing: 0.02em;
		color: var(--ink-faint-chirho);
		overflow-wrap: anywhere;
	}

	.method-note-chirho {
		max-width: 66ch;
		margin: 1.6rem auto 0;
		text-align: center;
		font-size: 0.88rem;
		font-style: italic;
		line-height: 1.65;
		color: var(--ink-faint-chirho);
	}

	.method-link-chirho {
		color: var(--ink-soft-chirho);
		border-bottom: 1px dotted var(--ink-faint-chirho);
		transition: color 0.2s ease, border-color 0.2s ease;
	}

	.method-link-chirho:hover {
		color: var(--lapis-chirho);
		border-color: var(--lapis-chirho);
	}

	.stats-strip-chirho {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 1.2rem;
		max-width: 900px;
		margin: clamp(2.5rem, 5vw, 3.6rem) auto 0;
		padding-top: 2rem;
		border-top: 1px solid var(--ink-line-chirho);
	}

	.stat-chirho {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.2rem;
		text-align: center;
	}

	.stat-value-chirho {
		font-family: var(--font-display-chirho);
		font-size: 1.9rem;
		font-weight: 600;
		color: var(--lapis-chirho);
	}

	.stat-label-chirho {
		font-size: 0.88rem;
		color: var(--ink-soft-chirho);
		max-width: 20ch;
	}

	.stats-provenance-chirho {
		text-align: center;
		font-family: var(--font-mono-chirho);
		font-variation-settings: 'MONO' 1;
		font-size: 0.72rem;
		color: var(--ink-faint-chirho);
		margin-top: 1.1rem;
	}

	@media (max-width: 720px) {
		.axes-chirho {
			grid-template-columns: 1fr;
		}
		.stats-strip-chirho {
			grid-template-columns: repeat(2, minmax(0, 1fr));
			row-gap: 1.6rem;
		}
	}
</style>
