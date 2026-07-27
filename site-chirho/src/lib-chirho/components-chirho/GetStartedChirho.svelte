<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import SectionHeadChirho from '$lib-chirho/components-chirho/SectionHeadChirho.svelte';
	import {
		installCommandChirho,
		installScriptCommandChirho
	} from '$lib-chirho/data-chirho/content-chirho';
	import { highlightHaskellChirho } from '$lib-chirho/util-chirho/highlight-chirho';

	const firstProgramChirho = `#!/usr/bin/env haskelujah
-- hello.hs — no build step
main :: IO ()
main = putStrLn "Hallelujah."`;

	const firstSegsChirho = highlightHaskellChirho(firstProgramChirho);

	let copiedWhichChirho = $state('');

	async function copyChirho(textChirho: string, whichChirho: string): Promise<void> {
		try {
			await navigator.clipboard.writeText(textChirho);
			copiedWhichChirho = whichChirho;
			setTimeout(() => (copiedWhichChirho = ''), 2200);
		} catch {
			copiedWhichChirho = '';
		}
	}
</script>

<section id="begin-chirho" class="begin-chirho">
	<div class="section-inner-chirho">
		<SectionHeadChirho
			numeralChirho="V"
			kickerChirho="Begin"
			titleChirho="Up and running in a minute"
			subChirho="One install, no GHC, no Stack, no LLVM toolchain. Then write Haskell like a script."
		/>

		<div class="begin-grid-chirho">
			<div class="begin-step-chirho">
				<p class="begin-step-num-chirho caps-label-chirho">1 · install</p>
				<div class="begin-cmd-chirho">
					<code>{installCommandChirho}</code>
					<button
						class="begin-copy-chirho"
						onclick={() => copyChirho(installCommandChirho, 'cargo')}
						aria-label="Copy cargo install command"
					>
						{copiedWhichChirho === 'cargo' ? '✓' : 'copy'}
					</button>
				</div>
				<p class="begin-alt-chirho">
					or the script:
					<button
						class="begin-alt-cmd-chirho"
						onclick={() => copyChirho(installScriptCommandChirho, 'script')}
						aria-label="Copy install script command"
					>
						<code>{installScriptCommandChirho}</code>
						<span class="begin-alt-copy-chirho"
							>{copiedWhichChirho === 'script' ? '✓ copied' : 'copy'}</span
						>
					</button>
				</p>
			</div>

			<div class="begin-step-chirho">
				<p class="begin-step-num-chirho caps-label-chirho">2 · write</p>
				<div class="begin-code-chirho">
					<pre>{#each firstSegsChirho as segChirho, iChirho (iChirho)}<span
								class="hl-{segChirho.kindChirho}-chirho">{segChirho.textChirho}</span
							>{/each}</pre>
				</div>
			</div>

			<div class="begin-step-chirho">
				<p class="begin-step-num-chirho caps-label-chirho">3 · run</p>
				<div class="begin-code-chirho begin-run-chirho">
					<pre><span class="begin-prompt-chirho">$</span> chmod +x hello.hs && ./hello.hs
<span class="begin-out-chirho">Hallelujah.</span></pre>
				</div>
				<p class="begin-note-chirho">
					The same file also builds to a native binary with <code>haskelujah build</code>,
					or to WebAssembly.
				</p>
			</div>
		</div>
	</div>
</section>

<style>
	.begin-chirho {
		padding: var(--section-pad-chirho) 0;
		background: var(--parchment-chirho);
	}

	.begin-grid-chirho {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: clamp(1.2rem, 3vw, 2rem);
		max-width: 1060px;
		margin: 0 auto;
		align-items: start;
	}

	.begin-step-chirho {
		background: var(--parchment-bright-chirho);
		border: 1px solid var(--ink-line-chirho);
		border-radius: 4px;
		padding: 1.4rem 1.4rem 1.5rem;
		box-shadow: 0 12px 34px rgba(36, 27, 16, 0.08);
	}

	.begin-step-num-chirho {
		color: var(--gold-deep-chirho);
		margin-bottom: 0.9rem;
	}

	.begin-cmd-chirho {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.6rem;
		padding: 0.7rem 0.9rem;
		background: var(--nave-deep-chirho);
		border-radius: 4px;
	}

	.begin-cmd-chirho code {
		color: var(--gold-bright-chirho);
		font-size: 0.88rem;
	}

	.begin-copy-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.58rem;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		padding: 0.4rem 0.7rem;
		color: var(--nave-text-soft-chirho);
		background: transparent;
		border: 1px solid rgba(201, 162, 39, 0.35);
		border-radius: 3px;
		cursor: pointer;
		transition: color 0.2s, border-color 0.2s;
	}

	.begin-copy-chirho:hover {
		color: var(--gold-bright-chirho);
		border-color: var(--gold-chirho);
	}

	.begin-alt-chirho {
		margin-top: 0.9rem;
		font-size: 0.9rem;
		color: var(--ink-soft-chirho);
	}

	.begin-alt-cmd-chirho {
		display: block;
		width: 100%;
		margin-top: 0.45rem;
		padding: 0.55rem 0.7rem;
		text-align: left;
		background: rgba(36, 27, 16, 0.05);
		border: 1px dashed rgba(36, 27, 16, 0.28);
		border-radius: 4px;
		cursor: pointer;
		transition: border-color 0.2s, background 0.2s;
	}

	.begin-alt-cmd-chirho:hover {
		border-color: var(--gold-deep-chirho);
		background: rgba(201, 162, 39, 0.08);
	}

	.begin-alt-cmd-chirho code {
		font-size: 0.74rem;
		color: var(--ink-chirho);
		word-break: break-all;
	}

	.begin-alt-copy-chirho {
		display: inline-block;
		margin-left: 0.5rem;
		font-family: var(--font-caps-chirho);
		font-size: 0.56rem;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--gold-deep-chirho);
	}

	.begin-code-chirho {
		padding: 0.9rem 1rem;
		background: var(--nave-deep-chirho);
		border-radius: 4px;
		overflow-x: auto;
	}

	.begin-code-chirho pre {
		font-size: 0.8rem;
		line-height: 1.7;
		color: var(--nave-text-chirho);
	}

	.begin-prompt-chirho {
		color: #8fc7d8;
		font-weight: 600;
	}

	.begin-out-chirho {
		color: #9aa3b8;
	}

	.begin-note-chirho {
		margin-top: 0.9rem;
		font-size: 0.9rem;
		color: var(--ink-soft-chirho);
		line-height: 1.55;
	}

	.begin-note-chirho code {
		font-size: 0.8rem;
		color: var(--ink-chirho);
		background: rgba(36, 27, 16, 0.07);
		padding: 0.1rem 0.35rem;
		border-radius: 3px;
	}

	@media (max-width: 880px) {
		.begin-grid-chirho {
			grid-template-columns: 1fr;
		}
	}
</style>
