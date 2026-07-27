<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import { onMount } from 'svelte';
	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import { tenSecondRunChirho } from '$lib-chirho/data-chirho/content-chirho';
	import { CastPlayerChirho } from '$lib-chirho/util-chirho/cast-player-chirho';

	let termElChirho: HTMLElement;
	let playerChirho: CastPlayerChirho | null = null;

	let startedChirho = $state(false);
	let playingChirho = $state(false);
	let endedChirho = $state(false);
	let speedChirho = $state(1.5);
	let loadFailedChirho = $state(false);
	let copiedChirho = $state(false);

	const speedsChirho = [1, 1.5, 2];

	async function ensureLoadedChirho(): Promise<boolean> {
		if (playerChirho) return true;
		try {
			const pChirho = new CastPlayerChirho(termElChirho, {
				onEndChirho: () => {
					playingChirho = false;
					endedChirho = true;
				}
			});
			await pChirho.loadChirho('/demo-chirho.cast');
			pChirho.setSpeedChirho(speedChirho);
			playerChirho = pChirho;
			return true;
		} catch {
			loadFailedChirho = true;
			return false;
		}
	}

	async function togglePlayChirho(): Promise<void> {
		const okChirho = await ensureLoadedChirho();
		if (!okChirho || !playerChirho) return;
		if (playingChirho) {
			playerChirho.pauseChirho();
			playingChirho = false;
		} else {
			startedChirho = true;
			endedChirho = false;
			playerChirho.playChirho();
			playingChirho = true;
		}
	}

	function restartChirho(): void {
		if (!playerChirho) return;
		playerChirho.restartChirho();
		endedChirho = false;
		playerChirho.playChirho();
		playingChirho = true;
	}

	function cycleSpeedChirho(): void {
		const idxChirho = speedsChirho.indexOf(speedChirho);
		speedChirho = speedsChirho[(idxChirho + 1) % speedsChirho.length];
		playerChirho?.setSpeedChirho(speedChirho);
	}

	async function copyRunChirho(): Promise<void> {
		try {
			await navigator.clipboard.writeText(tenSecondRunChirho);
			copiedChirho = true;
			setTimeout(() => (copiedChirho = false), 2200);
		} catch {
			copiedChirho = false;
		}
	}

	onMount(() => {
		return () => playerChirho?.disposeChirho();
	});
</script>

<section id="scriptorium-chirho" class="scriptorium-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			toneChirho="nave"
			numeralChirho="IV"
			kickerChirho="Scriptorium"
			titleChirho="Watch a real session"
			subChirho="A recorded terminal session, replayed faithfully — init, build, REPL, MCP. Not staged HTML."
		/>

		<div class="term-frame-chirho">
			<div class="term-bar-chirho">
				<span class="term-title-chirho caps-label-chirho">haskelujah · recorded 80×24</span>
				<div class="term-controls-chirho">
					<button
						class="term-btn-chirho"
						onclick={togglePlayChirho}
						aria-label={playingChirho ? 'Pause replay' : 'Play replay'}
					>
						{playingChirho ? '❚❚' : '▶'}
					</button>
					<button
						class="term-btn-chirho"
						onclick={restartChirho}
						disabled={!startedChirho}
						aria-label="Restart replay">↺</button
					>
					<button
						class="term-btn-chirho term-speed-chirho"
						onclick={cycleSpeedChirho}
						aria-label="Change replay speed">{speedChirho}×</button
					>
				</div>
			</div>

			<div class="term-body-chirho">
				<pre class="term-screen-chirho" bind:this={termElChirho} aria-hidden="true"></pre>
				<p class="visually-hidden-chirho">
					Recorded terminal demo: haskelujah init creates a project, check type-checks it,
					test runs the suite, fmt formats it, and mcp lists AI tools.
				</p>
				{#if !startedChirho}
					<button class="term-poster-chirho" onclick={togglePlayChirho}>
						<span class="term-poster-ring-chirho" aria-hidden="true">▶</span>
						<span class="caps-label-chirho">begin the session</span>
					</button>
				{/if}
				{#if endedChirho}
					<div class="term-ended-chirho">
						<button class="term-again-chirho" onclick={restartChirho}>↺ watch again</button>
					</div>
				{/if}
				{#if loadFailedChirho}
					<p class="term-failed-chirho">
						The recording could not be loaded — the session lives on in your terminal
						instead:
					</p>
				{/if}
			</div>
		</div>

		<div class="ten-second-chirho">
			<p class="ten-second-label-chirho">Prefer your own terminal? Ten seconds:</p>
			<div class="ten-second-row-chirho">
				<code class="ten-second-cmd-chirho">{tenSecondRunChirho}</code>
				<button
					class="ten-second-copy-chirho"
					onclick={copyRunChirho}
					aria-label="Copy the install-and-repl command"
				>
					{copiedChirho ? '✓ copied' : 'copy'}
				</button>
			</div>
		</div>
	</div>
</section>

<style>
	.scriptorium-chirho {
		padding: var(--section-pad-chirho) 0;
		background:
			radial-gradient(ellipse 70% 46% at 50% 8%, rgba(30, 62, 140, 0.2), transparent 68%),
			var(--nave-abyss-chirho);
		color: var(--nave-text-chirho);
	}

	.term-frame-chirho {
		max-width: 820px;
		margin: 0 auto;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 6px;
		overflow: hidden;
		background: rgba(7, 11, 24, 0.85);
		box-shadow:
			0 0 0 1px rgba(7, 11, 24, 0.6),
			0 24px 70px rgba(0, 0, 0, 0.5),
			inset 0 0 60px rgba(30, 62, 140, 0.08);
	}

	.term-bar-chirho {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.6rem 1rem;
		border-bottom: 1px solid var(--nave-line-chirho);
		background: rgba(201, 162, 39, 0.05);
	}

	.term-title-chirho {
		color: var(--nave-text-muted-chirho);
	}

	.term-controls-chirho {
		display: flex;
		gap: 0.45rem;
	}

	.term-btn-chirho {
		min-width: 2.1rem;
		height: 1.75rem;
		padding: 0 0.55rem;
		font-family: var(--font-mono-chirho);
		font-variation-settings: 'MONO' 1;
		font-size: 0.75rem;
		color: var(--nave-text-soft-chirho);
		background: transparent;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 3px;
		cursor: pointer;
		transition: color 0.2s, border-color 0.2s;
	}

	.term-btn-chirho:hover:not(:disabled) {
		color: var(--gold-bright-chirho);
		border-color: var(--gold-chirho);
	}

	.term-btn-chirho:disabled {
		opacity: 0.35;
		cursor: default;
	}

	.term-body-chirho {
		position: relative;
	}

	.term-screen-chirho {
		height: 380px;
		overflow-y: auto;
		padding: 1.1rem 1.25rem;
		font-size: 0.83rem;
		line-height: 1.55;
		font-variation-settings: 'MONO' 1;
		color: #cfd6e4;
		scrollbar-width: thin;
	}

	:global(.cast-bold-chirho) {
		font-weight: 700;
	}
	:global(.cast-fg-gold-chirho) {
		color: var(--gold-bright-chirho);
	}
	:global(.cast-fg-cyan-chirho) {
		color: #8fc7d8;
	}
	:global(.cast-fg-green-chirho) {
		color: #8fd8a8;
	}
	:global(.cast-fg-red-chirho) {
		color: #e58a76;
	}
	:global(.cast-fg-blue-chirho) {
		color: #8fa8d8;
	}
	:global(.cast-fg-magenta-chirho) {
		color: #c79bd8;
	}
	:global(.cast-fg-gray-chirho) {
		color: #7a8298;
	}
	:global(.cast-fg-white-chirho) {
		color: #f0f2f7;
	}
	:global(.cast-fg-black-chirho) {
		color: #4a4f60;
	}

	.term-poster-chirho {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		background:
			radial-gradient(ellipse 60% 55% at 50% 45%, rgba(30, 62, 140, 0.3), transparent 75%),
			rgba(7, 11, 24, 0.82);
		border: none;
		color: var(--nave-text-soft-chirho);
		cursor: pointer;
		transition: color 0.25s ease;
	}

	.term-poster-chirho:hover {
		color: var(--gold-bright-chirho);
	}

	.term-poster-ring-chirho {
		display: grid;
		place-items: center;
		width: 4.4rem;
		height: 4.4rem;
		border: 1px solid var(--gold-chirho);
		border-radius: 50%;
		font-size: 1.3rem;
		color: var(--gold-bright-chirho);
		box-shadow: 0 0 34px rgba(201, 162, 39, 0.25);
		transition: transform 0.25s ease, box-shadow 0.25s ease;
	}

	.term-poster-chirho:hover .term-poster-ring-chirho {
		transform: scale(1.06);
		box-shadow: 0 0 48px rgba(201, 162, 39, 0.4);
	}

	.term-ended-chirho {
		position: absolute;
		inset: auto 0 0 0;
		display: flex;
		justify-content: center;
		padding: 0.8rem;
		background: linear-gradient(180deg, transparent, rgba(7, 11, 24, 0.9));
	}

	.term-again-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.62rem;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		padding: 0.5rem 1.2rem;
		color: var(--gold-bright-chirho);
		background: rgba(7, 11, 24, 0.8);
		border: 1px solid var(--gold-chirho);
		border-radius: 999px;
		cursor: pointer;
	}

	.term-failed-chirho {
		padding: 0 1.25rem 1rem;
		color: var(--nave-text-muted-chirho);
		font-style: italic;
	}

	.ten-second-chirho {
		margin-top: 2.4rem;
		text-align: center;
	}

	.ten-second-label-chirho {
		font-style: italic;
		color: var(--nave-text-soft-chirho);
		margin-bottom: 0.8rem;
	}

	.ten-second-row-chirho {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.45rem 0.45rem 0.45rem 1.1rem;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 999px;
		background: rgba(11, 16, 38, 0.72);
	}

	.ten-second-cmd-chirho {
		font-size: 0.88rem;
		color: var(--gold-bright-chirho);
	}

	.ten-second-copy-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.6rem;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		padding: 0.45rem 0.85rem;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 999px;
		background: transparent;
		color: var(--nave-text-soft-chirho);
		cursor: pointer;
		transition: color 0.2s, border-color 0.2s;
	}

	.ten-second-copy-chirho:hover {
		color: var(--gold-bright-chirho);
		border-color: var(--gold-chirho);
	}

	@media (max-width: 600px) {
		.term-screen-chirho {
			height: 300px;
			font-size: 0.72rem;
		}
		.ten-second-row-chirho {
			flex-direction: column;
			border-radius: 12px;
			padding: 0.8rem;
		}
	}
</style>
