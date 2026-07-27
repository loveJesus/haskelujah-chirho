<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	import { linksChirho, scriptureChirho } from '$lib-chirho/data-chirho/content-chirho';
</script>

<footer class="colophon-chirho">
	<div class="section-inner-chirho">
		<div class="colophon-rule-chirho" aria-hidden="true">
			<span class="colophon-line-chirho"></span>
			<span class="colophon-chi-rho-chirho">☧</span>
			<span class="colophon-line-chirho"></span>
		</div>

		<p class="colophon-scripture-chirho">
			“{scriptureChirho.textChirho}” <span class="colophon-ref-chirho">— {scriptureChirho.refChirho}</span>
		</p>

		<p class="colophon-made-chirho">
			Compiler and page alike are written AI-first — a small fleet of AI agents, working in
			one repository under human direction, measured in public. If you want work built this
			way, <a href={linksChirho.contactChirho} class="colophon-link-inline-chirho">write to us</a>.
			How this page was made:
			<a href={linksChirho.guideChirho} class="colophon-link-inline-chirho"
				>the website guide</a
			>.
		</p>

		<nav class="colophon-links-chirho" aria-label="Footer">
			<a href={linksChirho.githubChirho} target="_blank" rel="noopener noreferrer">GitHub</a>
			<a href={linksChirho.donateChirho} target="_blank" rel="noopener noreferrer">Donate</a>
			<a href={linksChirho.newsChirho} target="_blank" rel="noopener noreferrer">News</a>
			<a href={linksChirho.contactChirho}>Contact</a>
		</nav>

		<p class="colophon-close-chirho caps-label-chirho">
			Haskelujah · soli deo gloria
		</p>
	</div>
</footer>

<style>
	.colophon-chirho {
		padding: clamp(3.5rem, 7vw, 5.5rem) 0 3rem;
		background: var(--nave-abyss-chirho);
		color: var(--nave-text-soft-chirho);
		text-align: center;
	}

	.colophon-rule-chirho {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.1rem;
		margin-bottom: 2.2rem;
	}

	.colophon-line-chirho {
		width: clamp(4rem, 16vw, 12rem);
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(201, 162, 39, 0.4), transparent);
	}

	.colophon-chi-rho-chirho {
		font-size: 1.6rem;
		color: var(--gold-chirho);
	}

	.colophon-scripture-chirho {
		font-style: italic;
		font-size: 0.98rem;
		line-height: 1.8;
		max-width: 58ch;
		margin: 0 auto 1.6rem;
		color: var(--nave-text-muted-chirho);
	}

	.colophon-ref-chirho {
		font-style: normal;
		color: var(--gold-chirho);
	}

	.colophon-made-chirho {
		max-width: 62ch;
		margin: 0 auto 2rem;
		font-size: 0.95rem;
		line-height: 1.75;
		color: var(--nave-text-soft-chirho);
	}

	.colophon-link-inline-chirho {
		color: var(--gold-bright-chirho);
		border-bottom: 1px dotted rgba(238, 214, 136, 0.5);
		transition: border-color 0.2s ease;
	}

	.colophon-link-inline-chirho:hover {
		border-bottom-style: solid;
	}

	.colophon-links-chirho {
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 1.8rem;
		margin-bottom: 2rem;
	}

	.colophon-links-chirho a {
		font-family: var(--font-caps-chirho);
		font-size: 0.66rem;
		letter-spacing: 0.22em;
		text-transform: uppercase;
		color: var(--nave-text-soft-chirho);
		transition: color 0.2s ease;
	}

	.colophon-links-chirho a:hover {
		color: var(--gold-bright-chirho);
	}

	.colophon-close-chirho {
		color: var(--nave-text-muted-chirho);
	}
</style>
