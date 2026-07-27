<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import { limitationsChirho } from '$lib-chirho/data-chirho/content-chirho';
</script>

<section id="confessio-chirho" class="confessio-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			numeralChirho="VI"
			kickerChirho="Confessio"
			titleChirho="Where we still fall short"
			subChirho="An honest ledger, kept current. Trust is built from what a project admits, not what it advertises."
		/>

		<ul class="confessio-list-chirho">
			{#each limitationsChirho as limitationChirho (limitationChirho.titleChirho)}
				<li class="confessio-item-chirho">
					<span class="confessio-mark-chirho" aria-hidden="true">✗</span>
					<div>
						<h3 class="confessio-title-chirho">{limitationChirho.titleChirho}</h3>
						<p class="confessio-body-chirho">{limitationChirho.bodyChirho}</p>
					</div>
				</li>
			{/each}
		</ul>
	</div>
</section>

<style>
	.confessio-chirho {
		padding: var(--section-pad-chirho) 0;
		background: linear-gradient(180deg, var(--parchment-chirho), var(--parchment-deep-chirho));
	}

	.confessio-list-chirho {
		list-style: none;
		max-width: 760px;
		margin: 0 auto;
	}

	.confessio-item-chirho {
		display: flex;
		gap: 1.1rem;
		padding: 1.25rem 0;
		border-bottom: 1px solid var(--ink-line-chirho);
	}

	.confessio-item-chirho:last-child {
		border-bottom: none;
	}

	.confessio-mark-chirho {
		flex-shrink: 0;
		display: inline-grid;
		place-items: center;
		width: 1.7rem;
		height: 1.7rem;
		margin-top: 0.15rem;
		border: 1px solid color-mix(in srgb, var(--vermilion-rubric-chirho) 55%, transparent);
		border-radius: 50%;
		color: var(--vermilion-rubric-chirho);
		font-size: 0.8rem;
	}

	.confessio-title-chirho {
		font-family: var(--font-display-chirho);
		font-size: 1.28rem;
		font-weight: 600;
		color: var(--ink-chirho);
		margin-bottom: 0.25rem;
	}

	.confessio-body-chirho {
		font-size: 1rem;
		color: var(--ink-soft-chirho);
		line-height: 1.65;
	}
</style>
