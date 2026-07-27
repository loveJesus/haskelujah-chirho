<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import NavChirho from '$lib-chirho/components-chirho/NavChirho.svelte';
	import RoseWindowHeroChirho from '$lib-chirho/components-chirho/RoseWindowHeroChirho.svelte';
	import PlaqueChirho from '$lib-chirho/components-chirho/PlaqueChirho.svelte';
	import PipelineStoryChirho from '$lib-chirho/components-chirho/PipelineStoryChirho.svelte';
	import CanonTableChirho from '$lib-chirho/components-chirho/CanonTableChirho.svelte';
	import ScriptoriumChirho from '$lib-chirho/components-chirho/ScriptoriumChirho.svelte';
	import GetStartedChirho from '$lib-chirho/components-chirho/GetStartedChirho.svelte';
	import ConfessioChirho from '$lib-chirho/components-chirho/ConfessioChirho.svelte';
	import RoadmapChirho from '$lib-chirho/components-chirho/RoadmapChirho.svelte';
	import ColophonChirho from '$lib-chirho/components-chirho/ColophonChirho.svelte';
</script>

<div id="top-chirho" class="page-chirho">
	<NavChirho />

	<main>
		<RoseWindowHeroChirho />

		<div class="dawn-chirho" aria-hidden="true"></div>

		<PlaqueChirho />
		<PipelineStoryChirho />
		<CanonTableChirho />
		<ScriptoriumChirho />
		<GetStartedChirho />
		<ConfessioChirho />
		<RoadmapChirho />
	</main>

	<ColophonChirho />

	<!-- a whisper of grain over everything; parchment must never look like a JPEG -->
	<svg class="grain-chirho" aria-hidden="true">
		<filter id="grain-filter-chirho">
			<feTurbulence type="fractalNoise" baseFrequency="0.72" numOctaves="2" stitchTiles="stitch" />
			<feColorMatrix type="saturate" values="0" />
		</filter>
		<rect width="100%" height="100%" filter="url(#grain-filter-chirho)" />
	</svg>
</div>

<style>
	.page-chirho {
		position: relative;
	}

	/* the emergence: night gives way to parchment as you pass through the window */
	.dawn-chirho {
		height: 26vh;
		background: linear-gradient(
			180deg,
			var(--nave-abyss-chirho) 0%,
			#3d3a51 34%,
			#907d63 62%,
			#d8c8a4 82%,
			var(--parchment-chirho) 100%
		);
	}

	.grain-chirho {
		position: fixed;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0.028;
		pointer-events: none;
		z-index: 60;
		mix-blend-mode: soft-light;
	}
</style>
