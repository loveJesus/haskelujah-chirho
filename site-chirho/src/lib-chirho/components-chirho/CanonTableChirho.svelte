<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import { toolsChirho } from '$lib-chirho/data-chirho/content-chirho';
</script>

<section id="tools-chirho" class="canon-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			numeralChirho="III"
			kickerChirho="One binary"
			titleChirho="The whole toolchain, under one name"
			subChirho="Twelve subcommands — compiler to editor — verified against the CLI source, not a brochure."
		/>

		<ul class="canon-grid-chirho">
			{#each toolsChirho as toolChirho (toolChirho.commandChirho)}
				<li class="canon-arch-chirho">
					<code class="canon-cmd-chirho">
						<span class="canon-cmd-bin-chirho">haskelujah</span>
						{toolChirho.commandChirho}
					</code>
					<p class="canon-blurb-chirho">{toolChirho.blurbChirho}</p>
				</li>
			{/each}
		</ul>
	</div>
</section>

<style>
	.canon-chirho {
		padding: var(--section-pad-chirho) 0;
		background: linear-gradient(180deg, var(--parchment-chirho), var(--parchment-deep-chirho));
	}

	.canon-grid-chirho {
		list-style: none;
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: clamp(0.9rem, 2vw, 1.4rem);
		max-width: 1060px;
		margin: 0 auto;
	}

	.canon-arch-chirho {
		--arch-r-chirho: 90px;
		background: var(--parchment-bright-chirho);
		border: 1px solid var(--ink-line-chirho);
		border-top-left-radius: var(--arch-r-chirho) 70px;
		border-top-right-radius: var(--arch-r-chirho) 70px;
		border-bottom-left-radius: 3px;
		border-bottom-right-radius: 3px;
		padding: 2.4rem 1.1rem 1.4rem;
		text-align: center;
		position: relative;
		box-shadow: 0 10px 30px rgba(36, 27, 16, 0.07);
		transition: transform 0.25s ease, box-shadow 0.25s ease, border-color 0.25s ease;
	}

	.canon-arch-chirho::before {
		content: '';
		position: absolute;
		inset: 7px 7px auto 7px;
		height: 3rem;
		border: 1px solid rgba(142, 111, 20, 0.35);
		border-bottom: none;
		border-top-left-radius: calc(var(--arch-r-chirho) - 7px) 62px;
		border-top-right-radius: calc(var(--arch-r-chirho) - 7px) 62px;
		pointer-events: none;
	}

	.canon-arch-chirho:hover {
		transform: translateY(-4px);
		border-color: rgba(142, 111, 20, 0.5);
		box-shadow: 0 16px 40px rgba(36, 27, 16, 0.13);
	}

	.canon-cmd-chirho {
		display: block;
		font-size: 0.92rem;
		font-weight: 600;
		color: var(--ink-chirho);
		margin-bottom: 0.5rem;
	}

	.canon-cmd-bin-chirho {
		display: block;
		font-size: 0.66rem;
		font-weight: 400;
		color: var(--ink-faint-chirho);
		letter-spacing: 0.06em;
		margin-bottom: 0.15rem;
	}

	.canon-blurb-chirho {
		font-size: 0.9rem;
		line-height: 1.5;
		color: var(--ink-soft-chirho);
	}

	@media (max-width: 980px) {
		.canon-grid-chirho {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 700px) {
		.canon-grid-chirho {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 440px) {
		.canon-grid-chirho {
			grid-template-columns: 1fr;
		}
	}
</style>
