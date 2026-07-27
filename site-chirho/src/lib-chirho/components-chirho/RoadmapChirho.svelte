<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import { roadmapChirho } from '$lib-chirho/data-chirho/content-chirho';
</script>

<section id="roadmap-chirho" class="roadmap-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			toneChirho="nave"
			numeralChirho="VII"
			kickerChirho="God willing"
			titleChirho="Where this road goes"
		/>

		<ol class="roadmap-grid-chirho">
			{#each roadmapChirho as itemChirho, iChirho (itemChirho.titleChirho)}
				<li class="roadmap-item-chirho">
					<span class="roadmap-star-chirho" aria-hidden="true">✦</span>
					<h3 class="roadmap-title-chirho">{itemChirho.titleChirho}</h3>
					<p class="roadmap-body-chirho">{itemChirho.bodyChirho}</p>
					<span class="roadmap-num-chirho" aria-hidden="true">{iChirho + 1}</span>
				</li>
			{/each}
		</ol>
	</div>
</section>

<style>
	.roadmap-chirho {
		padding: var(--section-pad-chirho) 0;
		background:
			radial-gradient(ellipse 75% 55% at 50% 100%, rgba(30, 62, 140, 0.24), transparent 70%),
			linear-gradient(180deg, var(--nave-deep-chirho), var(--nave-abyss-chirho));
		color: var(--nave-text-chirho);
	}

	.roadmap-grid-chirho {
		list-style: none;
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: clamp(1rem, 2.5vw, 1.6rem);
		max-width: 1020px;
		margin: 0 auto;
	}

	.roadmap-item-chirho {
		position: relative;
		padding: 1.5rem 1.4rem 1.6rem;
		background: rgba(11, 16, 38, 0.66);
		border: 1px solid var(--nave-line-chirho);
		border-radius: 4px;
		overflow: hidden;
		transition: border-color 0.25s ease, transform 0.25s ease;
	}

	.roadmap-item-chirho:hover {
		border-color: rgba(201, 162, 39, 0.45);
		transform: translateY(-3px);
	}

	.roadmap-star-chirho {
		color: var(--gold-chirho);
		font-size: 0.9rem;
	}

	.roadmap-title-chirho {
		font-family: var(--font-display-chirho);
		font-size: 1.3rem;
		font-weight: 600;
		margin: 0.5rem 0 0.5rem;
		color: var(--nave-text-chirho);
	}

	.roadmap-body-chirho {
		font-size: 0.95rem;
		color: var(--nave-text-soft-chirho);
		line-height: 1.6;
	}

	.roadmap-num-chirho {
		position: absolute;
		right: 0.6rem;
		bottom: -0.9rem;
		font-family: var(--font-display-chirho);
		font-size: 4.4rem;
		font-weight: 600;
		color: rgba(201, 162, 39, 0.09);
		line-height: 1;
		pointer-events: none;
	}

	@media (max-width: 880px) {
		.roadmap-grid-chirho {
			grid-template-columns: 1fr;
		}
	}
</style>
