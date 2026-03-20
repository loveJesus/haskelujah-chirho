<!-- For God so loved the world, that He gave His only begotten Son,
     that all who believe in Him should not perish but have everlasting life. John 3:16 -->

<script lang="ts">
	// For God so loved the world, that He gave His only begotten Son,
	// that all who believe in Him should not perish but have everlasting life. John 3:16

	const statsChirho = [
		{ labelChirho: 'GHC Compatibility', valueChirho: '91.8%', detailChirho: '861 / 938 tests' },
		{ labelChirho: 'Lines of Rust', valueChirho: '150K+', detailChirho: '21 crates' },
		{ labelChirho: 'Total Tests', valueChirho: '2,121', detailChirho: '0 failures' },
		{ labelChirho: 'Backends', valueChirho: '3', detailChirho: 'Cranelift, LLVM, Wasm' }
	];

	const benchmarksChirho = [
		{ nameChirho: 'Cranelift (default)', timeChirho: '0.91s', barChirho: 83, colorChirho: '#7dd3fc' },
		{ nameChirho: 'GHC 9.14 -O2', timeChirho: '1.10s', barChirho: 100, colorChirho: '#64748b' }
	];

	const benchmarkSuiteChirho = [
		{ nameChirho: 'fib(42)', descChirho: 'Recursive Fibonacci', ghcChirho: '1.10s', clChirho: '0.91s', llvmChirho: '0.91s', ratioChirho: '1.21x faster' },
		{ nameChirho: 'ack(3,10)', descChirho: 'Ackermann function', ghcChirho: '0.23s', clChirho: '0.14s', llvmChirho: '--', ratioChirho: '1.64x faster' },
		{ nameChirho: 'euler1(100M)', descChirho: '100M iterations (TCO)', ghcChirho: '0.09s', clChirho: '0.15s', llvmChirho: '0.35s', ratioChirho: '0.60x' },
		{ nameChirho: 'tak(30,20,10)', descChirho: 'Takeuchi function', ghcChirho: '0.10s', clChirho: '0.16s', llvmChirho: '0.27s', ratioChirho: '0.63x' },
		{ nameChirho: 'qsort(1K)', descChirho: 'Quicksort random', ghcChirho: '0.22s', clChirho: '0.20s', llvmChirho: '0.15s', ratioChirho: 'LLVM 1.47x' },
		{ nameChirho: 'primes(10K)', descChirho: 'Sieve of Eratosthenes', ghcChirho: '0.28s', clChirho: '0.59s', llvmChirho: '0.18s', ratioChirho: 'LLVM 1.56x' },
	];

	const featuresChirho = [
		{
			titleChirho: 'No LLVM Required',
			descriptionChirho:
				'Cranelift as the default backend means zero external dependencies. Install and compile immediately -- no LLVM toolchain setup, no version mismatches, no headaches.',
			iconChirho: 'C'
		},
		{
			titleChirho: 'LLVM When You Want It',
			descriptionChirho:
				'Opt in to the LLVM backend for maximum optimization. Production builds benefit from LLVM 18 without forcing it on every developer.',
			iconChirho: 'L'
		},
		{
			titleChirho: 'WebAssembly Built-in',
			descriptionChirho:
				'First-class Wasm target. Compile your Haskell to run in browsers, serverless edges, and embedded environments without any extra tooling.',
			iconChirho: 'W'
		},
		{
			titleChirho: 'GHC Compatible',
			descriptionChirho:
				'91.8% compatibility with the GHC test suite. Type classes, GADTs, RankNTypes, ScopedTypeVariables, Template Haskell, and dozens more extensions.',
			iconChirho: 'G'
		},
		{
			titleChirho: 'Written in Rust',
			descriptionChirho:
				'Memory-safe compiler infrastructure with predictable performance. No runtime GC pauses in the compiler itself. Fast incremental builds.',
			iconChirho: 'R'
		},
		{
			titleChirho: 'Runtime GC',
			descriptionChirho:
				'Mark-sweep garbage collector via a Rust-based runtime system. Heap allocation tracking, automatic collection, and C ABI interop out of the box.',
			iconChirho: 'M'
		}
	];
</script>

<div class="page-chirho">
	<!-- Navigation -->
	<nav class="nav-chirho">
		<div class="nav-inner-chirho">
			<a href="/" class="logo-chirho">
				<span class="logo-mark-chirho">H</span>
				<span class="logo-text-chirho">Haskelujah</span>
			</a>
			<div class="nav-links-chirho">
				<a href="#features-chirho">Features</a>
				<a href="#performance-chirho">Performance</a>
				<a href="#get-started-chirho">Get Started</a>
				<a
					href="https://github.com/loveJesus/haskelujah-chirho"
					target="_blank"
					rel="noopener noreferrer"
					class="nav-github-chirho"
				>
					GitHub
				</a>
			</div>
		</div>
	</nav>

	<!-- Hero Section -->
	<section class="hero-chirho">
		<div class="hero-content-chirho">
			<p class="scripture-chirho">
				"For God so loved the world, that He gave His only begotten Son, that all who believe
				in Him should not perish but have everlasting life."
				<span class="scripture-ref-chirho">-- John 3:16</span>
			</p>

			<h1 class="hero-title-chirho">
				<span class="hero-accent-chirho">Haskelujah</span> ☧
			</h1>
			<p class="hero-subtitle-chirho">A Haskell compiler written in Rust</p>
			<p class="hero-description-chirho">
				Cranelift default backend. No LLVM required. WebAssembly built-in.
				<br />
				91.8% GHC test-suite compatibility. Faster than GHC.
			</p>

			<div class="hero-actions-chirho">
				<a href="#get-started-chirho" class="btn-primary-chirho">Get Started</a>
				<a
					href="https://github.com/loveJesus/haskelujah-chirho"
					target="_blank"
					rel="noopener noreferrer"
					class="btn-secondary-chirho"
				>
					View on GitHub
				</a>
			</div>
		</div>

		<div class="hero-glow-chirho"></div>
	</section>

	<!-- Stats Bar -->
	<section class="stats-chirho">
		<div class="stats-inner-chirho">
			{#each statsChirho as statChirho}
				<div class="stat-card-chirho">
					<span class="stat-value-chirho">{statChirho.valueChirho}</span>
					<span class="stat-label-chirho">{statChirho.labelChirho}</span>
					<span class="stat-detail-chirho">{statChirho.detailChirho}</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Features Section -->
	<section id="features-chirho" class="features-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Why Haskelujah</h2>
			<p class="section-subtitle-chirho">
				A modern Haskell compiler designed for developer experience and performance.
			</p>
			<div class="features-grid-chirho">
				{#each featuresChirho as featureChirho}
					<div class="feature-card-chirho">
						<div class="feature-icon-chirho">{featureChirho.iconChirho}</div>
						<h3 class="feature-title-chirho">{featureChirho.titleChirho}</h3>
						<p class="feature-desc-chirho">{featureChirho.descriptionChirho}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Performance Section -->
	<section id="performance-chirho" class="performance-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Performance</h2>
			<p class="section-subtitle-chirho">fib(42) benchmark -- compiled output runtime, Apple M-series</p>

			<div class="benchmarks-chirho">
				{#each benchmarksChirho as benchChirho}
					<div class="benchmark-row-chirho">
						<div class="benchmark-label-chirho">
							<span class="benchmark-name-chirho">{benchChirho.nameChirho}</span>
							<span class="benchmark-time-chirho">{benchChirho.timeChirho}</span>
						</div>
						<div class="benchmark-bar-outer-chirho">
							<div
								class="benchmark-bar-inner-chirho"
								style="width: {benchChirho.barChirho}%; background-color: {benchChirho.colorChirho}"
							></div>
						</div>
					</div>
				{/each}
			</div>

			<p class="benchmark-note-chirho">
				Lower is better. Cranelift compiles 4x faster than LLVM with comparable runtime
				performance.
			</p>

			<h3 class="section-subtitle-chirho" style="margin-top: 2rem;">Full Benchmark Suite</h3>
			<table class="bench-table-chirho">
				<thead>
					<tr><th>Benchmark</th><th>Problem</th><th>GHC 9.14 -O2</th><th>Cranelift</th><th>LLVM</th><th>Best vs GHC</th></tr>
				</thead>
				<tbody>
					{#each benchmarkSuiteChirho as bChirho}
						<tr>
							<td>{bChirho.nameChirho}</td>
							<td>{bChirho.descChirho}</td>
							<td>{bChirho.ghcChirho}</td>
							<td>{bChirho.clChirho}</td>
							<td>{bChirho.llvmChirho}</td>
							<td>{bChirho.ratioChirho}</td>
						</tr>
					{/each}
				</tbody>
			</table>
			<p class="benchmark-note-chirho">
				Benchmarks on Apple M-series. Both Cranelift and LLVM backends support tail-call optimization (TCO).
				Cranelift beats GHC -O2 on recursive workloads; LLVM excels at list-heavy operations.
			</p>
		</div>
	</section>

	<!-- Get Started Section -->
	<section id="get-started-chirho" class="get-started-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Get Started</h2>
			<p class="section-subtitle-chirho">Up and running in seconds.</p>

			<div class="code-block-chirho">
				<div class="code-header-chirho">
					<span class="code-dot-chirho"></span>
					<span class="code-dot-chirho"></span>
					<span class="code-dot-chirho"></span>
					<span class="code-title-chirho">terminal</span>
				</div>
				<pre class="code-content-chirho"><code
						><span class="code-comment-chirho"># Install Haskelujah</span>
<span class="code-prompt-chirho">$</span> curl -fsSL https://haskelujah.org/install-chirho.sh | sh

<span class="code-comment-chirho"># Create a new project</span>
<span class="code-prompt-chirho">$</span> haskelujah init my-project
<span class="code-output-chirho">  Created my-project/
  Created my-project/Main.hs
  Created my-project/haskelujah.toml</span>

<span class="code-comment-chirho"># Build and run</span>
<span class="code-prompt-chirho">$</span> haskelujah build-run .
<span class="code-output-chirho">  Compiling Main.hs ... done (0.04s)
  Hello, world!</span>
</code></pre>
			</div>

			<div class="code-block-chirho" style="margin-top: 2rem;">
				<div class="code-header-chirho">
					<span class="code-dot-chirho"></span>
					<span class="code-dot-chirho"></span>
					<span class="code-dot-chirho"></span>
					<span class="code-title-chirho">Main.hs</span>
				</div>
				<pre class="code-content-chirho"><code
						><span class="code-keyword-chirho">module</span> Main <span class="code-keyword-chirho">where</span>

<span class="code-comment-chirho">-- Fibonacci with pattern matching</span>
<span class="code-fn-chirho">fib_chirho</span> :: <span class="code-type-chirho">Int</span> -> <span class="code-type-chirho">Int</span>
<span class="code-fn-chirho">fib_chirho</span> 0 = 0
<span class="code-fn-chirho">fib_chirho</span> 1 = 1
<span class="code-fn-chirho">fib_chirho</span> n = fib_chirho (n - 1) + fib_chirho (n - 2)

<span class="code-fn-chirho">main</span> :: <span class="code-type-chirho">IO</span> ()
<span class="code-fn-chirho">main</span> = putStrLn (<span class="code-string-chirho">"fib 42 = "</span> ++ show (fib_chirho 42))
</code></pre>
			</div>
		</div>
	</section>

	<!-- Architecture Section -->
	<section class="architecture-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Architecture</h2>
			<p class="section-subtitle-chirho">
				21 focused Rust crates. 12-phase compilation pipeline.
			</p>

			<div class="pipeline-vertical-chirho">
				{#each [
					{ phaseChirho: '1. Lex', detailChirho: 'Tokenize Haskell source' },
					{ phaseChirho: '2. Layout', detailChirho: 'Insert braces and semicolons' },
					{ phaseChirho: '3. Parse', detailChirho: 'Lossless green-tree CST' },
					{ phaseChirho: '4. Lower', detailChirho: 'Abstract syntax tree from CST' },
					{ phaseChirho: '5. Resolve', detailChirho: 'Scope resolution, qualified names' },
					{ phaseChirho: '6. Kind Infer', detailChirho: 'Kind inference with unification' },
					{ phaseChirho: '7. Type Infer', detailChirho: 'HM Algorithm W, typeclasses, GADTs' },
					{ phaseChirho: '8. Exhaustiveness', detailChirho: 'Pattern match checking' },
					{ phaseChirho: '9. Desugar/Core', detailChirho: 'System FC Core IR, dict passing' },
					{ phaseChirho: '10. Simplify', detailChirho: 'Inlining, CSE, specialization' },
					{ phaseChirho: '11. Codegen', detailChirho: 'Cranelift / LLVM / WebAssembly' },
					{ phaseChirho: '12. Link', detailChirho: 'Native executable or .wasm' },
				] as stepChirho, iChirho}
					<div class="pipeline-step-chirho">
						<div class="pipeline-num-chirho">{stepChirho.phaseChirho}</div>
						<div class="pipeline-detail-chirho">{stepChirho.detailChirho}</div>
					</div>
					{#if iChirho < 11}
						<div class="pipeline-vline-chirho"></div>
					{/if}
				{/each}
			</div>
		</div>
	</section>

	<!-- Footer -->
	<footer class="footer-chirho">
		<div class="footer-inner-chirho">
			<div class="footer-left-chirho">
				<span class="logo-mark-chirho footer-logo-chirho">H</span>
				<span>Haskelujah Chirho</span>
			</div>
			<div class="footer-center-chirho">
				<p class="footer-scripture-chirho">
					"For God so loved the world, that He gave His only begotten Son, that all who
					believe in Him should not perish but have everlasting life." -- John 3:16
				</p>
			</div>
			<div class="footer-right-chirho">
				<a
					href="https://github.com/loveJesus/haskelujah-chirho"
					target="_blank"
					rel="noopener noreferrer">GitHub</a
				>
				<a href="mailto:haskelujah@mailer-aleluya.xjes.us">Contact</a>
			</div>
		</div>
	</footer>
</div>

<style>
	/* Page Layout */
	.page-chirho {
		min-height: 100vh;
	}

	.section-inner-chirho {
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 2rem;
	}

	/* Navigation */
	.nav-chirho {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		background: rgba(10, 14, 23, 0.85);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(125, 211, 252, 0.08);
	}

	.nav-inner-chirho {
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 2rem;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.logo-chirho {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
	}

	.logo-mark-chirho {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: linear-gradient(135deg, #7dd3fc, #a78bfa);
		color: #0a0e17;
		font-family: 'Inter', serif;
		font-weight: 700;
		font-size: 1.1rem;
		border-radius: 8px;
	}

	.logo-text-chirho {
		font-weight: 600;
		font-size: 1.15rem;
		color: #e2e8f0;
	}

	.nav-links-chirho {
		display: flex;
		gap: 2rem;
		align-items: center;
	}

	.nav-links-chirho a {
		font-size: 0.9rem;
		color: #94a3b8;
		font-weight: 400;
	}

	.nav-links-chirho a:hover {
		color: #e2e8f0;
	}

	.nav-github-chirho {
		padding: 0.4rem 1rem;
		border: 1px solid rgba(125, 211, 252, 0.3);
		border-radius: 6px;
		font-weight: 500;
		color: #7dd3fc !important;
	}

	.nav-github-chirho:hover {
		background: rgba(125, 211, 252, 0.08);
	}

	/* Hero */
	.hero-chirho {
		position: relative;
		padding: 10rem 2rem 6rem;
		text-align: center;
		overflow: hidden;
	}

	.hero-content-chirho {
		position: relative;
		z-index: 1;
		max-width: 800px;
		margin: 0 auto;
	}

	.scripture-chirho {
		font-size: 0.95rem;
		font-style: italic;
		color: #64748b;
		max-width: 600px;
		margin: 0 auto 3rem;
		line-height: 1.8;
	}

	.scripture-ref-chirho {
		display: block;
		margin-top: 0.5rem;
		font-style: normal;
		font-weight: 500;
		color: #7dd3fc;
		opacity: 0.7;
	}

	.hero-title-chirho {
		font-size: clamp(2.5rem, 6vw, 4.5rem);
		font-weight: 700;
		letter-spacing: -0.03em;
		line-height: 1.1;
		margin-bottom: 1rem;
	}

	.hero-accent-chirho {
		background: linear-gradient(135deg, #7dd3fc, #a78bfa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-subtitle-chirho {
		font-size: 1.35rem;
		color: #94a3b8;
		font-weight: 400;
		margin-bottom: 1.5rem;
	}

	.hero-description-chirho {
		font-size: 1.05rem;
		color: #64748b;
		margin-bottom: 2.5rem;
		line-height: 1.8;
	}

	.hero-actions-chirho {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn-primary-chirho {
		display: inline-flex;
		align-items: center;
		padding: 0.75rem 2rem;
		background: linear-gradient(135deg, #7dd3fc, #a78bfa);
		color: #0a0e17;
		font-weight: 600;
		border-radius: 8px;
		font-size: 1rem;
		transition: opacity 0.2s;
	}

	.btn-primary-chirho:hover {
		opacity: 0.9;
		color: #0a0e17;
	}

	.btn-secondary-chirho {
		display: inline-flex;
		align-items: center;
		padding: 0.75rem 2rem;
		border: 1px solid rgba(125, 211, 252, 0.3);
		color: #7dd3fc;
		font-weight: 500;
		border-radius: 8px;
		font-size: 1rem;
		transition: background-color 0.2s;
	}

	.btn-secondary-chirho:hover {
		background: rgba(125, 211, 252, 0.08);
		color: #bae6fd;
	}

	.hero-glow-chirho {
		position: absolute;
		top: 20%;
		left: 50%;
		transform: translateX(-50%);
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, rgba(125, 211, 252, 0.06) 0%, transparent 70%);
		pointer-events: none;
	}

	/* Stats */
	.stats-chirho {
		padding: 3rem 2rem;
		background: rgba(125, 211, 252, 0.02);
		border-top: 1px solid rgba(125, 211, 252, 0.06);
		border-bottom: 1px solid rgba(125, 211, 252, 0.06);
	}

	.stats-inner-chirho {
		max-width: 1100px;
		margin: 0 auto;
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 2rem;
	}

	.stat-card-chirho {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value-chirho {
		font-size: 2.25rem;
		font-weight: 700;
		background: linear-gradient(135deg, #7dd3fc, #a78bfa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.stat-label-chirho {
		font-size: 0.95rem;
		color: #e2e8f0;
		font-weight: 500;
	}

	.stat-detail-chirho {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Features */
	.features-chirho {
		padding: 6rem 0;
	}

	.section-title-chirho {
		font-size: 2rem;
		font-weight: 700;
		text-align: center;
		margin-bottom: 0.75rem;
		letter-spacing: -0.02em;
	}

	.section-subtitle-chirho {
		text-align: center;
		color: #64748b;
		font-size: 1.1rem;
		margin-bottom: 3.5rem;
	}

	.features-grid-chirho {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1.5rem;
	}

	.feature-card-chirho {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(125, 211, 252, 0.08);
		border-radius: 12px;
		padding: 2rem;
		transition: border-color 0.2s;
	}

	.feature-card-chirho:hover {
		border-color: rgba(125, 211, 252, 0.2);
	}

	.feature-icon-chirho {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(125, 211, 252, 0.08);
		color: #7dd3fc;
		border-radius: 8px;
		font-weight: 700;
		font-size: 1rem;
		margin-bottom: 1rem;
	}

	.feature-title-chirho {
		font-size: 1.1rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
	}

	.feature-desc-chirho {
		font-size: 0.9rem;
		color: #94a3b8;
		line-height: 1.7;
	}

	/* Performance */
	.performance-chirho {
		padding: 6rem 0;
		background: rgba(125, 211, 252, 0.02);
		border-top: 1px solid rgba(125, 211, 252, 0.06);
		border-bottom: 1px solid rgba(125, 211, 252, 0.06);
	}

	.benchmarks-chirho {
		max-width: 700px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.benchmark-row-chirho {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.benchmark-label-chirho {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
	}

	.benchmark-name-chirho {
		font-weight: 500;
		font-size: 0.95rem;
	}

	.benchmark-time-chirho {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 600;
		font-size: 1rem;
		color: #7dd3fc;
	}

	.benchmark-bar-outer-chirho {
		height: 12px;
		background: rgba(255, 255, 255, 0.04);
		border-radius: 6px;
		overflow: hidden;
	}

	.benchmark-bar-inner-chirho {
		height: 100%;
		border-radius: 6px;
		transition: width 0.6s ease;
	}

	.benchmark-note-chirho {
		text-align: center;
		font-size: 0.85rem;
		color: #64748b;
		margin-top: 2rem;
	}

	.bench-table-chirho {
		width: 100%;
		max-width: 800px;
		margin: 1.5rem auto;
		border-collapse: collapse;
		font-size: 0.9rem;
	}
	.bench-table-chirho th {
		background: rgba(125, 211, 252, 0.08);
		color: #7dd3fc;
		padding: 0.75rem 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(125, 211, 252, 0.2);
	}
	.bench-table-chirho td {
		padding: 0.6rem 1rem;
		border-bottom: 1px solid rgba(125, 211, 252, 0.06);
		color: #cbd5e1;
	}
	.bench-table-chirho tr:hover td {
		background: rgba(125, 211, 252, 0.04);
	}

	/* Get Started */
	.get-started-chirho {
		padding: 6rem 0;
	}

	.code-block-chirho {
		max-width: 700px;
		margin: 0 auto;
		background: #0f1521;
		border: 1px solid rgba(125, 211, 252, 0.1);
		border-radius: 12px;
		overflow: hidden;
	}

	.code-header-chirho {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.02);
		border-bottom: 1px solid rgba(125, 211, 252, 0.06);
	}

	.code-dot-chirho {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.1);
	}

	.code-dot-chirho:nth-child(1) {
		background: #ef4444;
		opacity: 0.6;
	}

	.code-dot-chirho:nth-child(2) {
		background: #eab308;
		opacity: 0.6;
	}

	.code-dot-chirho:nth-child(3) {
		background: #22c55e;
		opacity: 0.6;
	}

	.code-title-chirho {
		margin-left: 0.5rem;
		font-size: 0.8rem;
		color: #64748b;
		font-family: 'JetBrains Mono', monospace;
	}

	.code-content-chirho {
		padding: 1.5rem;
		overflow-x: auto;
		font-size: 0.85rem;
		line-height: 1.8;
	}

	.code-content-chirho code {
		color: #e2e8f0;
	}

	:global(.code-comment-chirho) {
		color: #4a5568;
	}

	:global(.code-prompt-chirho) {
		color: #7dd3fc;
		font-weight: 600;
	}

	:global(.code-output-chirho) {
		color: #64748b;
	}

	:global(.code-keyword-chirho) {
		color: #a78bfa;
		font-weight: 600;
	}

	:global(.code-type-chirho) {
		color: #7dd3fc;
	}

	:global(.code-fn-chirho) {
		color: #34d399;
	}

	:global(.code-string-chirho) {
		color: #fbbf24;
	}

	/* Architecture */
	.architecture-chirho {
		padding: 6rem 0;
		background: rgba(125, 211, 252, 0.02);
		border-top: 1px solid rgba(125, 211, 252, 0.06);
	}

	.pipeline-chirho {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-items: center;
	}

	.pipeline-vertical-chirho {
		max-width: 500px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.pipeline-step-chirho {
		display: flex;
		align-items: center;
		gap: 1rem;
		width: 100%;
		padding: 0.6rem 1.5rem;
		background: rgba(125, 211, 252, 0.04);
		border: 1px solid rgba(125, 211, 252, 0.1);
		border-radius: 8px;
	}
	.pipeline-num-chirho {
		font-weight: 600;
		color: #7dd3fc;
		font-size: 0.85rem;
		min-width: 120px;
		font-family: 'JetBrains Mono', monospace;
	}
	.pipeline-detail-chirho {
		font-size: 0.85rem;
		color: #94a3b8;
	}
	.pipeline-vline-chirho {
		width: 2px;
		height: 8px;
		background: rgba(125, 211, 252, 0.15);
	}

	/* Footer */
	.footer-chirho {
		padding: 3rem 2rem;
		border-top: 1px solid rgba(125, 211, 252, 0.06);
	}

	.footer-inner-chirho {
		max-width: 1100px;
		margin: 0 auto;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 2rem;
	}

	.footer-left-chirho {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.footer-logo-chirho {
		width: 28px;
		height: 28px;
		font-size: 0.85rem;
	}

	.footer-center-chirho {
		flex: 1;
		text-align: center;
	}

	.footer-scripture-chirho {
		font-size: 0.8rem;
		font-style: italic;
		color: #475569;
		max-width: 500px;
		margin: 0 auto;
		line-height: 1.6;
	}

	.footer-right-chirho {
		display: flex;
		gap: 1.5rem;
	}

	.footer-right-chirho a {
		font-size: 0.9rem;
		color: #64748b;
	}

	.footer-right-chirho a:hover {
		color: #7dd3fc;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.stats-inner-chirho {
			grid-template-columns: repeat(2, 1fr);
		}

		.features-grid-chirho {
			grid-template-columns: 1fr;
		}

		.nav-links-chirho {
			gap: 1rem;
		}

		.footer-inner-chirho {
			flex-direction: column;
			text-align: center;
		}

		.hero-chirho {
			padding: 8rem 1.5rem 4rem;
		}
	}

	@media (max-width: 480px) {
		.nav-links-chirho a:not(.nav-github-chirho) {
			display: none;
		}
	}
</style>
