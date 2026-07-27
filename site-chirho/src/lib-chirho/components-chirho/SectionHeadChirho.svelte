<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	let {
		numeralChirho,
		kickerChirho,
		titleChirho,
		subChirho = '',
		toneChirho = 'parchment'
	}: {
		numeralChirho: string;
		kickerChirho: string;
		titleChirho: string;
		subChirho?: string;
		toneChirho?: 'parchment' | 'nave';
	} = $props();
</script>

<div class="head-chirho" class:nave-tone-chirho={toneChirho === 'nave'}>
	<div class="head-rule-chirho" aria-hidden="true">
		<span class="head-fleuron-chirho">❧</span>
		<span class="head-line-chirho"></span>
		<span class="head-numeral-chirho">{numeralChirho}</span>
		<span class="head-line-chirho"></span>
		<span class="head-fleuron-chirho head-fleuron-flip-chirho">❧</span>
	</div>
	<p class="head-kicker-chirho caps-label-chirho">{kickerChirho}</p>
	<h2 class="head-title-chirho">{titleChirho}</h2>
	{#if subChirho}
		<p class="head-sub-chirho">{subChirho}</p>
	{/if}
</div>

<style>
	.head-chirho {
		text-align: center;
		margin-bottom: clamp(2.5rem, 5vw, 4rem);
		--head-accent-chirho: var(--gold-deep-chirho);
		--head-title-color-chirho: var(--ink-chirho);
		--head-sub-color-chirho: var(--ink-soft-chirho);
		--head-line-color-chirho: rgba(36, 27, 16, 0.25);
	}

	.head-chirho.nave-tone-chirho {
		--head-accent-chirho: var(--gold-chirho);
		--head-title-color-chirho: var(--nave-text-chirho);
		--head-sub-color-chirho: var(--nave-text-soft-chirho);
		--head-line-color-chirho: rgba(201, 162, 39, 0.3);
	}

	.head-rule-chirho {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.9rem;
		margin-bottom: 1.3rem;
	}

	.head-fleuron-chirho {
		color: var(--head-accent-chirho);
		font-size: 1.05rem;
		opacity: 0.85;
	}

	.head-fleuron-flip-chirho {
		display: inline-block;
		transform: scaleX(-1);
	}

	.head-line-chirho {
		width: clamp(3rem, 9vw, 7rem);
		height: 1px;
		background: linear-gradient(90deg, transparent, var(--head-line-color-chirho), transparent);
	}

	.head-numeral-chirho {
		font-family: var(--font-caps-chirho);
		font-size: 0.85rem;
		font-weight: 700;
		letter-spacing: 0.14em;
		color: var(--head-accent-chirho);
		border: 1px solid var(--head-line-color-chirho);
		border-radius: 50%;
		width: 2.4rem;
		height: 2.4rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.head-kicker-chirho {
		color: var(--head-accent-chirho);
		margin-bottom: 0.7rem;
	}

	.head-title-chirho {
		font-family: var(--font-display-chirho);
		font-size: clamp(2.1rem, 4.6vw, 3.2rem);
		font-weight: 600;
		line-height: 1.12;
		letter-spacing: 0.005em;
		color: var(--head-title-color-chirho);
	}

	.head-sub-chirho {
		margin: 0.9rem auto 0;
		max-width: 56ch;
		font-size: 1.08rem;
		color: var(--head-sub-color-chirho);
	}
</style>
