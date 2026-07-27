<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import { onMount } from 'svelte';
	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import {
		phasesChirho,
		pipelineStagesChirho,
		sampleSourceChirho
	} from '$lib-chirho/data-chirho/content-chirho';
	import { highlightHaskellChirho } from '$lib-chirho/util-chirho/highlight-chirho';

	const sourceSegsChirho = highlightHaskellChirho(sampleSourceChirho);

	let activeStageChirho = $state(0);
	let stageElsChirho = $state<HTMLElement[]>([]);

	// mini rose window sector paths
	const CX_CHIRHO = 50;
	const CY_CHIRHO = 50;
	const R0_CHIRHO = 26;
	const R1_CHIRHO = 46;
	const GAP_DEG_CHIRHO = 2.2;

	function polarChirho(rChirho: number, degChirho: number): string {
		const radChirho = ((degChirho - 90) * Math.PI) / 180;
		return `${(CX_CHIRHO + rChirho * Math.cos(radChirho)).toFixed(2)} ${(
			CY_CHIRHO + rChirho * Math.sin(radChirho)
		).toFixed(2)}`;
	}

	function sectorPathChirho(iChirho: number): string {
		const a0Chirho = (iChirho * 360) / 12 + GAP_DEG_CHIRHO / 2;
		const a1Chirho = ((iChirho + 1) * 360) / 12 - GAP_DEG_CHIRHO / 2;
		return [
			`M ${polarChirho(R1_CHIRHO, a0Chirho)}`,
			`A ${R1_CHIRHO} ${R1_CHIRHO} 0 0 1 ${polarChirho(R1_CHIRHO, a1Chirho)}`,
			`L ${polarChirho(R0_CHIRHO, a1Chirho)}`,
			`A ${R0_CHIRHO} ${R0_CHIRHO} 0 0 0 ${polarChirho(R0_CHIRHO, a0Chirho)}`,
			'Z'
		].join(' ');
	}

	const sectorPathsChirho = phasesChirho.map((_pChirho, iChirho) => sectorPathChirho(iChirho));

	function paneStateChirho(paneIdxChirho: number, stageIdxChirho: number): 'lit' | 'done' | 'dim' {
		const [aChirho, bChirho] = pipelineStagesChirho[stageIdxChirho].phaseIdxChirho;
		if (paneIdxChirho === aChirho || paneIdxChirho === bChirho) return 'lit';
		if (paneIdxChirho < aChirho) return 'done';
		return 'dim';
	}

	onMount(() => {
		const ioChirho = new IntersectionObserver(
			(entriesChirho) => {
				for (const entryChirho of entriesChirho) {
					if (entryChirho.isIntersecting) {
						const idxChirho = stageElsChirho.indexOf(entryChirho.target as HTMLElement);
						if (idxChirho >= 0) activeStageChirho = idxChirho;
					}
				}
			},
			{ rootMargin: '-42% 0px -42% 0px' }
		);
		for (const elChirho of stageElsChirho) {
			if (elChirho) ioChirho.observe(elChirho);
		}
		return () => ioChirho.disconnect();
	});
</script>

<section id="craft-chirho" class="craft-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			toneChirho="nave"
			numeralChirho="II"
			kickerChirho="The craft"
			titleChirho="Twelve phases, one small program"
			subChirho="Follow six lines of Haskell from text to machine. Dumps are illustrative — drawn in the true shape of each intermediate form."
		/>

		<div class="craft-grid-chirho">
			<div class="craft-rail-chirho">
				<svg
					class="mini-rose-chirho"
					viewBox="0 0 100 100"
					role="img"
					aria-label="Miniature rose window; panes light as each compilation phase is described"
				>
					<circle cx="50" cy="50" r="48.6" fill="none" stroke="rgba(201,162,39,0.4)" stroke-width="1.6" />
					<circle cx="50" cy="50" r="23.4" fill="none" stroke="rgba(201,162,39,0.4)" stroke-width="1.2" />
					{#each phasesChirho as phaseChirho, iChirho (phaseChirho.numeralChirho)}
						<path
							d={sectorPathsChirho[iChirho]}
							fill={phaseChirho.hueChirho}
							class="mini-pane-chirho"
							class:mini-pane-lit-chirho={paneStateChirho(iChirho, activeStageChirho) === 'lit'}
							class:mini-pane-done-chirho={paneStateChirho(iChirho, activeStageChirho) === 'done'}
						/>
					{/each}
					<text
						x="50"
						y="50"
						text-anchor="middle"
						dominant-baseline="central"
						class="mini-lambda-chirho">λ</text
					>
				</svg>

				<div class="craft-source-chirho">
					<p class="craft-source-label-chirho caps-label-chirho">Main.hs</p>
					<pre class="craft-source-code-chirho">{#each sourceSegsChirho as segChirho, iChirho (iChirho)}<span
							class="hl-{segChirho.kindChirho}-chirho">{segChirho.textChirho}</span
						>{/each}</pre>
				</div>
			</div>

			<div class="craft-stages-chirho">
				{#each pipelineStagesChirho as stageChirho, iChirho (stageChirho.titleChirho)}
					{@const phaseAChirho = phasesChirho[stageChirho.phaseIdxChirho[0]]}
					{@const phaseBChirho = phasesChirho[stageChirho.phaseIdxChirho[1]]}
					<article
						class="stage-chirho"
						class:stage-active-chirho={activeStageChirho === iChirho}
						style="--stage-hue-chirho: {phaseAChirho.hueChirho}"
						bind:this={stageElsChirho[iChirho]}
					>
						<p class="stage-phases-chirho caps-label-chirho">
							<span>{phaseAChirho.numeralChirho} · {phaseAChirho.nameChirho}</span>
							<span class="stage-phase-sep-chirho">—</span>
							<span>{phaseBChirho.numeralChirho} · {phaseBChirho.nameChirho}</span>
						</p>
						<h3 class="stage-title-chirho">{stageChirho.titleChirho}</h3>
						<pre class="stage-code-chirho">{stageChirho.codeChirho}</pre>
						<p class="stage-caption-chirho">{stageChirho.captionChirho}</p>
					</article>
				{/each}
			</div>
		</div>
	</div>
</section>

<style>
	.craft-chirho {
		background:
			radial-gradient(ellipse 80% 50% at 50% 0%, rgba(30, 62, 140, 0.22), transparent 65%),
			linear-gradient(180deg, var(--nave-deep-chirho), var(--nave-abyss-chirho));
		padding: var(--section-pad-chirho) 0;
		color: var(--nave-text-chirho);
	}

	.craft-grid-chirho {
		display: grid;
		grid-template-columns: minmax(0, 5fr) minmax(0, 7fr);
		gap: clamp(2rem, 5vw, 4.5rem);
		align-items: start;
	}

	.craft-rail-chirho {
		position: sticky;
		top: 12vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.8rem;
	}

	.mini-rose-chirho {
		width: min(100%, 300px);
		filter: drop-shadow(0 0 26px rgba(30, 62, 140, 0.4));
	}

	.mini-pane-chirho {
		opacity: 0.16;
		transition: opacity 0.7s ease, filter 0.7s ease;
	}

	.mini-pane-done-chirho {
		opacity: 0.5;
	}

	.mini-pane-lit-chirho {
		opacity: 1;
		filter: drop-shadow(0 0 7px currentColor) brightness(1.18);
	}

	.mini-lambda-chirho {
		font-family: var(--font-display-chirho);
		font-size: 21px;
		fill: var(--gold-bright-chirho);
	}

	.craft-source-chirho {
		width: 100%;
		max-width: 340px;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 4px;
		background: rgba(7, 11, 24, 0.7);
		overflow: hidden;
	}

	.craft-source-label-chirho {
		padding: 0.55rem 1rem;
		color: var(--gold-chirho);
		border-bottom: 1px solid var(--nave-line-chirho);
	}

	.craft-source-code-chirho {
		padding: 1rem 1.1rem;
		font-size: 0.82rem;
		line-height: 1.7;
		overflow-x: auto;
	}

	:global(.hl-keyword-chirho) {
		color: var(--gold-bright-chirho);
	}
	:global(.hl-type-chirho) {
		color: #8fa8d8;
	}
	:global(.hl-operator-chirho) {
		color: #d9822b;
	}
	:global(.hl-number-chirho) {
		color: #e3c363;
	}
	:global(.hl-string-chirho) {
		color: #9dc4a8;
	}
	:global(.hl-comment-chirho) {
		color: #6d6248;
		font-style: italic;
	}
	:global(.hl-plain-chirho) {
		color: var(--nave-text-chirho);
	}

	.craft-stages-chirho {
		display: flex;
		flex-direction: column;
		gap: clamp(3.5rem, 9vh, 7rem);
		padding: 4vh 0 10vh;
	}

	.stage-chirho {
		border-left: 2px solid color-mix(in srgb, var(--stage-hue-chirho) 45%, transparent);
		padding-left: clamp(1.1rem, 2.5vw, 1.8rem);
		opacity: 0.45;
		transition: opacity 0.5s ease, border-color 0.5s ease, transform 0.5s ease;
	}

	.stage-active-chirho {
		opacity: 1;
		border-color: var(--stage-hue-chirho);
		transform: translateX(2px);
	}

	.stage-phases-chirho {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		color: var(--stage-hue-chirho);
		margin-bottom: 0.6rem;
	}

	.stage-phase-sep-chirho {
		opacity: 0.5;
	}

	.stage-title-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(1.5rem, 3vw, 2rem);
		font-weight: 600;
		margin-bottom: 1rem;
		color: var(--nave-text-chirho);
	}

	.stage-code-chirho {
		font-size: 0.84rem;
		line-height: 1.65;
		padding: 1.1rem 1.25rem;
		background: rgba(7, 11, 24, 0.72);
		border: 1px solid color-mix(in srgb, var(--stage-hue-chirho) 30%, transparent);
		border-radius: 4px;
		overflow-x: auto;
		color: var(--nave-text-chirho);
		box-shadow: 0 10px 34px rgba(0, 0, 0, 0.35);
	}

	.stage-caption-chirho {
		margin-top: 0.85rem;
		font-size: 0.98rem;
		font-style: italic;
		color: var(--nave-text-soft-chirho);
		max-width: 52ch;
	}

	@media (max-width: 880px) {
		.craft-grid-chirho {
			grid-template-columns: 1fr;
		}
		.craft-rail-chirho {
			position: static;
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: center;
		}
		.mini-rose-chirho {
			width: min(58vw, 240px);
		}
		.craft-stages-chirho {
			gap: 3rem;
			padding-top: 1rem;
		}
		.stage-chirho {
			opacity: 1;
		}
	}
</style>
