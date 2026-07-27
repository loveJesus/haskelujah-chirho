<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import { onMount } from 'svelte';
	import {
		phasesChirho,
		scriptureChirho,
		installCommandChirho,
		linksChirho
	} from '$lib-chirho/data-chirho/content-chirho';
	import type { RoseWindowHandleChirho } from '$lib-chirho/three-chirho/rose-window-chirho';

	let wrapElChirho: HTMLElement;
	let stickyElChirho: HTMLElement;
	let canvasElChirho = $state<HTMLCanvasElement | null>(null);

	let webglOkChirho = $state(true);
	let canvasReadyChirho = $state(false);
	let progressChirho = $state(0);
	let hoverChirho = $state<{ iChirho: number; xChirho: number; yChirho: number } | null>(null);
	let copiedChirho = $state(false);

	let handleChirho: RoseWindowHandleChirho | null = null;
	let visibleChirho = true;
	let rafPendingChirho = false;

	function updateProgressChirho(): void {
		rafPendingChirho = false;
		if (!wrapElChirho) return;
		const rectChirho = wrapElChirho.getBoundingClientRect();
		const rangeChirho = rectChirho.height - window.innerHeight;
		const pChirho = rangeChirho > 0 ? Math.min(Math.max(-rectChirho.top / rangeChirho, 0), 1) : 0;
		progressChirho = pChirho;
		handleChirho?.setScrollProgressChirho(pChirho);
	}

	function onScrollChirho(): void {
		if (!rafPendingChirho) {
			rafPendingChirho = true;
			requestAnimationFrame(updateProgressChirho);
		}
	}

	function onPointerMoveChirho(evChirho: PointerEvent): void {
		if (!stickyElChirho) return;
		const rectChirho = stickyElChirho.getBoundingClientRect();
		const xChirho = ((evChirho.clientX - rectChirho.left) / rectChirho.width) * 2 - 1;
		const yChirho = -(((evChirho.clientY - rectChirho.top) / rectChirho.height) * 2 - 1);
		handleChirho?.setPointerChirho(xChirho, yChirho);
	}

	function onPointerLeaveChirho(): void {
		handleChirho?.setPointerChirho(-10, -10);
	}

	async function copyInstallChirho(): Promise<void> {
		try {
			await navigator.clipboard.writeText(installCommandChirho);
			copiedChirho = true;
			setTimeout(() => (copiedChirho = false), 2200);
		} catch {
			copiedChirho = false;
		}
	}

	onMount(() => {
		const reducedChirho = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		const mobileChirho =
			window.matchMedia('(pointer: coarse)').matches || window.innerWidth < 720;

		const probeChirho = document.createElement('canvas');
		const glChirho = probeChirho.getContext('webgl2');
		if (!glChirho) {
			webglOkChirho = false;
		}

		let ioChirho: IntersectionObserver | null = null;
		let disposedChirho = false;

		function syncRunningChirho(): void {
			handleChirho?.setRunningChirho(visibleChirho && !document.hidden);
		}
		function onVisibilityChirho(): void {
			syncRunningChirho();
		}

		if (webglOkChirho) {
			import('$lib-chirho/three-chirho/rose-window-chirho').then((modChirho) => {
				if (disposedChirho || !canvasElChirho) return;
				try {
					handleChirho = modChirho.createRoseWindowChirho(canvasElChirho, {
						paneHuesChirho: phasesChirho.map((pChirho) => pChirho.hueChirho),
						staticChirho: reducedChirho,
						mobileChirho,
						onPaneHoverChirho: (iChirho, xChirho, yChirho) => {
							hoverChirho = iChirho === null ? null : { iChirho, xChirho, yChirho };
						},
						onContextLostChirho: () => {
							webglOkChirho = false;
							canvasReadyChirho = false;
						}
					});
					canvasReadyChirho = true;
					handleChirho.resizeChirho();
					updateProgressChirho();
					syncRunningChirho();
				} catch {
					webglOkChirho = false;
				}
			});
			ioChirho = new IntersectionObserver(
				(entriesChirho) => {
					visibleChirho = entriesChirho[0]?.isIntersecting ?? false;
					syncRunningChirho();
				},
				{ threshold: 0.02 }
			);
			ioChirho.observe(wrapElChirho);
			document.addEventListener('visibilitychange', onVisibilityChirho);
		}

		updateProgressChirho();

		return () => {
			disposedChirho = true;
			ioChirho?.disconnect();
			document.removeEventListener('visibilitychange', onVisibilityChirho);
			handleChirho?.disposeChirho();
			handleChirho = null;
		};
	});
</script>

<svelte:window onscroll={onScrollChirho} onresize={() => handleChirho?.resizeChirho()} />

<section
	class="nave-chirho"
	bind:this={wrapElChirho}
	aria-label="Haskelujah — the illuminated compiler"
	style="--hero-p-chirho: {progressChirho}"
	onpointermove={onPointerMoveChirho}
	onpointerleave={onPointerLeaveChirho}
>
	<div class="nave-sticky-chirho" bind:this={stickyElChirho}>
		<div class="nave-bg-chirho" aria-hidden="true"></div>

		{#if webglOkChirho}
			<canvas
				class="nave-canvas-chirho"
				class:ready-chirho={canvasReadyChirho}
				bind:this={canvasElChirho}
				aria-hidden="true"
			></canvas>
		{:else}
			<div class="css-window-chirho" aria-hidden="true">
				<span class="css-window-lambda-chirho">λ</span>
			</div>
		{/if}
		<p class="visually-hidden-chirho">
			A rose window of twelve stained-glass panes — one for each of the twelve compilation
			phases, from lexing to linking — surrounding a golden λ.
		</p>

		{#if hoverChirho}
			{@const paneChirho = phasesChirho[hoverChirho.iChirho]}
			<div
				class="pane-tip-chirho"
				style="left: {hoverChirho.xChirho}px; top: {hoverChirho.yChirho}px; --tip-hue-chirho: {paneChirho.hueChirho}"
				role="status"
			>
				<span class="pane-tip-numeral-chirho">{paneChirho.numeralChirho}</span>
				<span class="pane-tip-name-chirho">{paneChirho.nameChirho}</span>
				<span class="pane-tip-detail-chirho">{paneChirho.detailChirho}</span>
			</div>
		{/if}

		<div class="nave-content-chirho">
			<p class="nave-scripture-chirho">
				“{scriptureChirho.textChirho}”
				<span class="nave-scripture-ref-chirho">— {scriptureChirho.refChirho}</span>
			</p>

			<h1 class="nave-title-chirho"><span class="foil-chirho">Haskelujah</span> ☧</h1>
			<p class="nave-sub-chirho">A Haskell compiler, written in Rust, honest about itself.</p>
			<p class="nave-desc-chirho">
				Compiler · package manager · REPL · test runner · LSP · AI — one binary. No GHC, no
				Stack, no LLVM toolchain required.
			</p>

			<div class="nave-cartouche-chirho">
				<code class="nave-install-chirho">{installCommandChirho}</code>
				<button
					class="nave-copy-chirho"
					onclick={copyInstallChirho}
					aria-label="Copy install command"
				>
					{copiedChirho ? '✓ copied' : 'copy'}
				</button>
			</div>

			<div class="nave-actions-chirho">
				<a href="#begin-chirho" class="nave-btn-primary-chirho">Begin</a>
				<a
					href={linksChirho.githubChirho}
					target="_blank"
					rel="noopener noreferrer"
					class="nave-btn-ghost-chirho">GitHub</a
				>
			</div>

			<div class="nave-hint-chirho" aria-hidden="true">
				<span>descend</span>
				<span class="nave-hint-arrow-chirho">▾</span>
			</div>
		</div>
	</div>
</section>

<style>
	.nave-chirho {
		position: relative;
		height: 240vh;
		background: var(--nave-abyss-chirho);
	}

	.nave-sticky-chirho {
		position: sticky;
		top: 0;
		height: 100vh;
		overflow: hidden;
		display: grid;
		place-items: center;
	}

	.nave-bg-chirho {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(ellipse 90% 62% at 50% 30%, rgba(30, 58, 138, 0.34), transparent 68%),
			radial-gradient(ellipse 55% 42% at 50% 34%, rgba(201, 162, 39, 0.1), transparent 70%),
			linear-gradient(180deg, #0b1026 0%, var(--nave-abyss-chirho) 78%);
	}

	.nave-canvas-chirho {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
		transition: opacity 1.4s ease;
	}

	.nave-canvas-chirho.ready-chirho {
		opacity: 1;
	}

	/* CSS-only rose window for the no-WebGL few */
	.css-window-chirho {
		position: absolute;
		top: 8%;
		left: 50%;
		transform: translateX(-50%);
		width: min(58vmin, 460px);
		aspect-ratio: 1;
		border-radius: 50%;
		background: conic-gradient(
			from 90deg,
			#3e63c4, #2e8fb8, #2fa089, #3e7c64, #6fa34a, #b8a032,
			#e3c363, #d9822b, #c23b22, #a93a63, #7c4fa8, #4a5fb0, #3e63c4
		);
		-webkit-mask: radial-gradient(circle, transparent 0 29%, #000 31% 66%, transparent 68%);
		mask: radial-gradient(circle, transparent 0 29%, #000 31% 66%, transparent 68%);
		filter: saturate(0.85) brightness(0.9);
		display: grid;
		place-items: center;
	}

	.css-window-lambda-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(3rem, 10vmin, 5.5rem);
		color: var(--gold-bright-chirho);
		text-shadow: 0 0 34px rgba(238, 214, 136, 0.55);
	}

	.pane-tip-chirho {
		position: absolute;
		transform: translate(-50%, -120%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.1rem;
		padding: 0.55rem 0.95rem;
		background: rgba(7, 11, 24, 0.88);
		border: 1px solid var(--tip-hue-chirho);
		border-radius: 4px;
		pointer-events: none;
		text-align: center;
		box-shadow: 0 8px 28px rgba(0, 0, 0, 0.55);
		z-index: 4;
	}

	.pane-tip-numeral-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.7rem;
		letter-spacing: 0.24em;
		color: var(--tip-hue-chirho);
	}

	.pane-tip-name-chirho {
		font-family: var(--font-display-chirho);
		font-size: 1.15rem;
		font-weight: 600;
		color: var(--nave-text-chirho);
	}

	.pane-tip-detail-chirho {
		font-size: 0.85rem;
		color: var(--nave-text-soft-chirho);
	}

	.nave-content-chirho {
		position: relative;
		z-index: 2;
		text-align: center;
		max-width: 720px;
		padding: 0 var(--edge-pad-chirho);
		margin-top: 53vh;
		opacity: calc(1 - var(--hero-p-chirho) * 2.4);
		transform: translateY(calc(var(--hero-p-chirho) * -8vh));
	}

	.nave-scripture-chirho {
		font-style: italic;
		font-size: 0.93rem;
		line-height: 1.7;
		color: var(--nave-text-muted-chirho);
		max-width: 60ch;
		margin: 0 auto 1.5rem;
	}

	.nave-scripture-ref-chirho {
		display: block;
		margin-top: 0.4rem;
		font-style: normal;
		font-family: var(--font-caps-chirho);
		font-size: 0.68rem;
		letter-spacing: 0.26em;
		color: var(--gold-chirho);
	}

	.nave-title-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(3rem, 8vw, 5rem);
		font-weight: 560;
		line-height: 1.04;
		letter-spacing: 0.01em;
		color: var(--gold-bright-chirho);
		margin-bottom: 0.6rem;
	}

	.foil-chirho {
		background: var(--gold-foil-chirho);
		background-size: 220% 220%;
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		color: transparent;
		filter: drop-shadow(0 2px 18px rgba(201, 162, 39, 0.35));
	}

	@media (prefers-reduced-motion: no-preference) {
		.foil-chirho {
			animation: foil-sheen-chirho 9s ease-in-out infinite;
		}
		.nave-hint-arrow-chirho {
			animation: hint-bob-chirho 2.4s ease-in-out infinite;
		}
	}

	@keyframes foil-sheen-chirho {
		0%, 100% { background-position: 0% 30%; }
		50% { background-position: 90% 70%; }
	}

	@keyframes hint-bob-chirho {
		0%, 100% { transform: translateY(0); opacity: 0.75; }
		50% { transform: translateY(6px); opacity: 1; }
	}

	.nave-sub-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(1.2rem, 2.6vw, 1.5rem);
		font-weight: 500;
		font-style: italic;
		color: var(--nave-text-chirho);
		margin-bottom: 0.7rem;
	}

	.nave-desc-chirho {
		font-size: 1.02rem;
		color: var(--nave-text-soft-chirho);
		max-width: 56ch;
		margin: 0 auto 1.8rem;
	}

	.nave-cartouche-chirho {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.5rem 0.5rem 0.5rem 1.2rem;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 999px;
		background: rgba(11, 16, 38, 0.72);
		box-shadow:
			inset 0 0 24px rgba(30, 58, 138, 0.25),
			0 6px 30px rgba(0, 0, 0, 0.45);
		margin-bottom: 1.2rem;
	}

	.nave-install-chirho {
		font-size: 0.95rem;
		color: var(--gold-bright-chirho);
		letter-spacing: 0.01em;
	}

	.nave-copy-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.62rem;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		padding: 0.5rem 0.9rem;
		border: 1px solid var(--nave-line-chirho);
		border-radius: 999px;
		background: transparent;
		color: var(--nave-text-soft-chirho);
		cursor: pointer;
		transition: color 0.2s, border-color 0.2s, background 0.2s;
	}

	.nave-copy-chirho:hover {
		color: var(--gold-bright-chirho);
		border-color: var(--gold-chirho);
		background: rgba(201, 162, 39, 0.08);
	}

	.nave-actions-chirho {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.nave-btn-primary-chirho,
	.nave-btn-ghost-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.72rem;
		font-weight: 600;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		padding: 0.85rem 2.1rem;
		border-radius: 2px;
		transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s, color 0.2s;
	}

	.nave-btn-primary-chirho {
		background: var(--gold-foil-chirho);
		color: #1c1408;
		box-shadow: 0 4px 22px rgba(201, 162, 39, 0.3);
	}

	.nave-btn-primary-chirho:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(201, 162, 39, 0.45);
	}

	.nave-btn-ghost-chirho {
		border: 1px solid var(--nave-line-chirho);
		color: var(--nave-text-soft-chirho);
	}

	.nave-btn-ghost-chirho:hover {
		color: var(--gold-bright-chirho);
		border-color: var(--gold-chirho);
	}

	.nave-hint-chirho {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.15rem;
		font-family: var(--font-caps-chirho);
		font-size: 0.6rem;
		letter-spacing: 0.34em;
		text-transform: uppercase;
		color: var(--nave-text-muted-chirho);
	}

	@media (max-width: 720px) {
		.nave-chirho {
			height: 200vh;
		}
		.nave-content-chirho {
			margin-top: 52vh;
		}
		.nave-scripture-chirho {
			margin-bottom: 1.4rem;
		}
	}
</style>
