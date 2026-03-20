<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16 -->

<script lang="ts">
	// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. John 3:16

	const statsChirho = [
		{ labelChirho: 'GHC Compatibility', valueChirho: '91.8%', detailChirho: '861 / 938 tests' },
		{ labelChirho: 'Rust Code', valueChirho: '150K+', detailChirho: '21 crates, 132 source files' },
		{ labelChirho: 'Passing Tests', valueChirho: '2,131+', detailChirho: '50/50 curated, 57 LLVM round-trips' },
		{ labelChirho: 'Compiler Targets', valueChirho: '3', detailChirho: 'Cranelift, LLVM, Wasm' }
	];

	const heroPillsChirho = [
		'cargo install haskelujah',
		'Cranelift default backend',
		'Single binary workflow',
		'Cabal + Hackage direction'
	];

	const benchmarksChirho = [
		{ nameChirho: 'Cranelift (default)', timeChirho: '0.91s', barChirho: 72, colorChirho: '#7dd3fc' },
		{ nameChirho: 'GHC 9.14 -O2', timeChirho: '1.26s', barChirho: 100, colorChirho: '#64748b' }
	];

	const benchmarkSuiteChirho = [
		{ nameChirho: 'fib(42)', descChirho: 'Recursive Fibonacci', ghcChirho: '1.26s', clChirho: '0.91s', llvmChirho: '0.91s', ratioChirho: '1.38x faster' },
		{ nameChirho: 'ack(3,11)', descChirho: 'Ackermann function', ghcChirho: '0.26s', clChirho: '0.62s', llvmChirho: '1.03s', ratioChirho: '0.42x' },
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
				"For God so loved the world that he gave his only begotten Son, that whoever
				believes in him should not perish but have eternal life."
				<span class="scripture-ref-chirho">-- John 3:16</span>
			</p>

			<h1 class="hero-title-chirho">
				<span class="hero-accent-chirho">Haskelujah</span> ☧
			</h1>
			<p class="hero-subtitle-chirho">A self-contained Haskell compiler written in Rust</p>
			<p class="hero-description-chirho">
				Install one executable. Compile with Cranelift by default, switch to LLVM when you
				want it, and keep moving without dragging a toolchain behind you.
			</p>

			<div class="hero-pill-row-chirho">
				{#each heroPillsChirho as heroPillChirho}
					<span class="hero-pill-chirho">{heroPillChirho}</span>
				{/each}
			</div>

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

			<div class="hero-proof-chirho">
				<div class="hero-proof-card-chirho">
					<span class="hero-proof-label-chirho">Latest proof</span>
					<strong>2,131+ passing tests</strong>
					<p>Compiler, driver, parser, runtimes, and backend round trips all green.</p>
				</div>
				<div class="hero-proof-card-chirho">
					<span class="hero-proof-label-chirho">Developer experience</span>
					<strong>Cranelift first, LLVM optional</strong>
					<p>No mandatory LLVM install for the default build path.</p>
				</div>
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
						<div class="feature-text-chirho">
							<h3>{featureChirho.titleChirho}</h3>
							<p>{featureChirho.descriptionChirho}</p>
						</div>
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
						><span class="code-comment-chirho"># Install the CLI</span>
<span class="code-prompt-chirho">$</span> cargo install haskelujah

<span class="code-comment-chirho"># Create a file</span>
<span class="code-prompt-chirho">$</span> cat > Main.hs
<span class="code-output-chirho">main = putStrLn "Hello from Haskelujah!"</span>

<span class="code-comment-chirho"># Compile with the default Cranelift backend</span>
<span class="code-prompt-chirho">$</span> haskelujah build Main.hs
<span class="code-output-chirho">  Compiling Main.hs ... done
  Wrote ./main</span>

<span class="code-comment-chirho"># Run it</span>
<span class="code-prompt-chirho">$</span> ./main
<span class="code-output-chirho">  Hello from Haskelujah!</span>
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

<span class="code-comment-chirho">-- Tail-call optimized loop plus ordinary recursion</span>
<span class="code-fn-chirho">euler1_chirho</span> :: <span class="code-type-chirho">Int</span> -> <span class="code-type-chirho">Int</span>
<span class="code-fn-chirho">euler1_chirho</span> limit_chirho = go_chirho 0 0
  <span class="code-keyword-chirho">where</span>
    <span class="code-fn-chirho">go_chirho</span> acc_chirho n_chirho
      | n_chirho >= limit_chirho = acc_chirho
      | n_chirho <span class="code-keyword-chirho">mod</span> 3 == 0 || n_chirho <span class="code-keyword-chirho">mod</span> 5 == 0
          = go_chirho (acc_chirho + n_chirho) (n_chirho + 1)
      | <span class="code-keyword-chirho">otherwise</span> = go_chirho acc_chirho (n_chirho + 1)

<span class="code-fn-chirho">main</span> :: <span class="code-type-chirho">IO</span> ()
<span class="code-fn-chirho">main</span> = print (euler1_chirho 1000)
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

	<!-- Ecosystem Section -->
	<section class="ecosystem-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Ecosystem</h2>
			<p class="section-subtitle-chirho">Built for compatibility with the Haskell ecosystem.</p>

			<div class="feature-grid-chirho">
				<div class="feature-card-chirho">
					<div class="feature-icon-chirho">P</div>
					<div class="feature-text-chirho">
						<h3>Cabal Support</h3>
						<p>Reads standard <code>.cabal</code> files. Multi-module projects with library and executable components compile out of the box.</p>
					</div>
				</div>
				<div class="feature-card-chirho">
					<div class="feature-icon-chirho">H</div>
					<div class="feature-text-chirho">
						<h3>Hackage Packages</h3>
						<p><code>haskelujah install aeson</code> fetches packages from Hackage, resolves dependencies, and extracts <code>.cabal</code> metadata. Auto-detects latest versions.</p>
					</div>
				</div>
				<div class="feature-card-chirho">
					<div class="feature-icon-chirho">1</div>
					<div class="feature-text-chirho">
						<h3>Single Binary</h3>
						<p>One self-contained executable. No runtime dependencies, no GHC installation, no LLVM toolchain (unless you opt in). Just <code>cargo install haskelujah</code>.</p>
					</div>
				</div>
				<div class="feature-card-chirho">
					<div class="feature-icon-chirho">R</div>
					<div class="feature-text-chirho">
						<h3>REPL</h3>
						<p><code>haskelujah repl</code> gives you an interactive environment with <code>:type</code>, <code>:info</code>, <code>:load</code>, multi-line input, and expression evaluation.</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Limitations Section -->
	<section class="limitations-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Current Limitations</h2>
			<p class="section-subtitle-chirho">Where we still fall short of GHC.</p>
			<ul class="limitation-list-chirho">
				<li><strong>Lazy evaluation:</strong> Compilation is strict-only. The STG interpreter supports laziness, but compiled backends evaluate strictly. Thunks and lazy data structures are planned.</li>
				<li><strong>Garbage collection:</strong> Mark-sweep GC exists in the RTS but is currently disabled (no root tracking). Programs leak memory on long runs.</li>
				<li><strong>Type class dictionaries at runtime:</strong> Type checking supports full typeclasses, but compiled code uses simplified dictionary elision. Complex polymorphic dispatch is partial.</li>
				<li><strong>String as [Char]:</strong> String literals are C strings internally. <code>unpack</code>/<code>pack</code> conversion works but isn't transparent like GHC's representation.</li>
				<li><strong>Template Haskell:</strong> Basic splices and <code>makeLenses</code> work; full TH (typed splices, reify) is incomplete.</li>
				<li><strong>FFI:</strong> Basic libc interop (puts, printf, malloc). Full C header parsing and foreign exports are not yet implemented.</li>
			</ul>
		</div>
	</section>

	<!-- Roadmap Section -->
	<section class="roadmap-chirho">
		<div class="section-inner-chirho">
			<h2 class="section-title-chirho">Roadmap</h2>
			<p class="section-subtitle-chirho">Where we're headed, God willing.</p>
			<div class="roadmap-grid-chirho">
				<div class="roadmap-item-chirho">
					<h3>Lazy Evaluation</h3>
					<p>Thunks, lazy data structures, and proper WHNF semantics in compiled backends.</p>
				</div>
				<div class="roadmap-item-chirho">
					<h3>GC Root Tracking</h3>
					<p>Wire GC roots through codegen so the mark-sweep collector can safely reclaim memory.</p>
				</div>
				<div class="roadmap-item-chirho">
					<h3>Integrated IDE</h3>
					<p>A Zed-like editor extensible via Haskell (like Emacs uses Lisp). LSP support with hover types, go-to-definition, and diagnostics.</p>
				</div>
				<div class="roadmap-item-chirho">
					<h3>Cross-Compilation</h3>
					<p>Target selection from CLI: compile to Linux, macOS, Windows, or embedded targets from a single machine.</p>
				</div>
				<div class="roadmap-item-chirho">
					<h3>Full Hackage</h3>
					<p>Compile real-world packages like aeson, lens, and servant. Bridge the gap from toy programs to production Haskell.</p>
				</div>
				<div class="roadmap-item-chirho">
					<h3>Profiling</h3>
					<p>Cost-center annotation, heap profiling, and time profiling for performance optimization.</p>
				</div>
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
					"For God so loved the world that he gave his only begotten Son, that whoever
					believes in him should not perish but have eternal life." -- John 3:16
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
		background:
			linear-gradient(180deg, rgba(0, 0, 0, 0) 0%, rgba(246, 190, 96, 0.03) 100%),
			transparent;
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
		background: rgba(18, 15, 11, 0.84);
		backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--line-chirho);
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
		background: linear-gradient(135deg, var(--accent-gold-chirho), var(--accent-cyan-chirho));
		color: #120f0b;
		font-family: 'Space Grotesk', serif;
		font-weight: 700;
		font-size: 1.1rem;
		border-radius: 8px;
	}

	.logo-text-chirho {
		font-weight: 600;
		font-size: 1.15rem;
		color: var(--text-main-chirho);
	}

	.nav-links-chirho {
		display: flex;
		gap: 2rem;
		align-items: center;
	}

	.nav-links-chirho a {
		font-size: 0.9rem;
		color: var(--text-muted-chirho);
		font-weight: 400;
	}

	.nav-links-chirho a:hover {
		color: var(--text-main-chirho);
	}

	.nav-github-chirho {
		padding: 0.4rem 1rem;
		border: 1px solid rgba(246, 190, 96, 0.3);
		border-radius: 6px;
		font-weight: 500;
		color: var(--accent-gold-chirho) !important;
	}

	.nav-github-chirho:hover {
		background: rgba(246, 190, 96, 0.08);
	}

	/* Hero */
	.hero-chirho {
		position: relative;
		padding: 10rem 2rem 6rem;
		text-align: center;
		overflow: hidden;
		background: url('/hero-bg-chirho.png') center center / cover no-repeat;
	}

	.hero-chirho::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			180deg,
			rgba(18, 15, 11, 0.30) 0%,
			rgba(18, 15, 11, 0.45) 40%,
			rgba(18, 15, 11, 0.75) 100%
		);
		z-index: 0;
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
		color: var(--text-muted-chirho);
		max-width: 600px;
		margin: 0 auto 3rem;
		line-height: 1.8;
		text-shadow: 0 1px 6px rgba(0, 0, 0, 0.5);
	}

	.scripture-ref-chirho {
		display: block;
		margin-top: 0.5rem;
		font-style: normal;
		font-weight: 500;
		color: var(--accent-cyan-chirho);
		opacity: 0.7;
	}

	.hero-title-chirho {
		font-size: clamp(2.5rem, 6vw, 4.5rem);
		font-weight: 700;
		letter-spacing: -0.03em;
		line-height: 1.1;
		margin-bottom: 1rem;
		text-shadow:
			0 -4px 20px rgba(246, 190, 96, 0.4),
			0 -8px 40px rgba(246, 190, 96, 0.2),
			0 2px 8px rgba(0, 0, 0, 0.5);
	}

	.hero-accent-chirho {
		background: linear-gradient(135deg, #ffd700, #f6be60, #ffeaa7, #71d7d1);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		filter: drop-shadow(0 -3px 12px rgba(255, 215, 0, 0.35));
	}

	.hero-subtitle-chirho {
		font-size: 1.35rem;
		color: var(--text-soft-chirho);
		font-weight: 400;
		margin-bottom: 1.5rem;
		text-shadow: 0 1px 8px rgba(0, 0, 0, 0.5);
	}

	.hero-description-chirho {
		font-size: 1.05rem;
		color: var(--text-muted-chirho);
		margin-bottom: 1.5rem;
		line-height: 1.8;
		text-shadow: 0 1px 6px rgba(0, 0, 0, 0.4);
	}

	.hero-pill-row-chirho {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 0.75rem;
		margin: 0 auto 2.5rem;
		max-width: 760px;
	}

	.hero-pill-chirho {
		padding: 0.5rem 0.85rem;
		border-radius: 999px;
		border: 1px solid rgba(246, 190, 96, 0.16);
		background: rgba(246, 190, 96, 0.06);
		color: var(--text-soft-chirho);
		font-size: 0.82rem;
		letter-spacing: 0.02em;
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
		background: linear-gradient(135deg, var(--accent-gold-chirho), #f0d37a);
		color: #120f0b;
		font-weight: 600;
		border-radius: 8px;
		font-size: 1rem;
		transition: opacity 0.2s;
	}

	.btn-primary-chirho:hover {
		opacity: 0.9;
		color: #120f0b;
	}

	.btn-secondary-chirho {
		display: inline-flex;
		align-items: center;
		padding: 0.75rem 2rem;
		border: 1px solid rgba(113, 215, 209, 0.3);
		color: var(--accent-cyan-chirho);
		font-weight: 500;
		border-radius: 8px;
		font-size: 1rem;
		transition: background-color 0.2s;
	}

	.btn-secondary-chirho:hover {
		background: rgba(113, 215, 209, 0.08);
		color: #b7f0ec;
	}

	.hero-proof-chirho {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
		margin-top: 2.25rem;
	}

	.hero-proof-card-chirho {
		padding: 1.15rem 1.2rem;
		text-align: left;
		border-radius: 16px;
		border: 1px solid var(--line-chirho);
		background: linear-gradient(180deg, rgba(35, 28, 18, 0.9), rgba(18, 15, 11, 0.92));
		box-shadow: var(--shadow-chirho);
	}

	.hero-proof-card-chirho strong {
		display: block;
		font-size: 1.05rem;
		margin-bottom: 0.35rem;
		color: var(--text-main-chirho);
	}

	.hero-proof-card-chirho p {
		color: var(--text-muted-chirho);
		font-size: 0.9rem;
	}

	.hero-proof-label-chirho {
		display: inline-block;
		margin-bottom: 0.6rem;
		color: var(--accent-gold-chirho);
		font-family: 'IBM Plex Mono', monospace;
		font-size: 0.73rem;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.hero-glow-chirho {
		position: absolute;
		top: 20%;
		left: 50%;
		transform: translateX(-50%);
		width: 600px;
		height: 600px;
		background: radial-gradient(circle, rgba(246, 190, 96, 0.08) 0%, transparent 70%);
		pointer-events: none;
	}

	/* Stats */
	.stats-chirho {
		padding: 3rem 2rem;
		background: rgba(246, 190, 96, 0.025);
		border-top: 1px solid var(--line-chirho);
		border-bottom: 1px solid var(--line-chirho);
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
		background: linear-gradient(135deg, var(--accent-gold-chirho), var(--accent-cyan-chirho));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.stat-label-chirho {
		font-size: 0.95rem;
		color: var(--text-main-chirho);
		font-weight: 500;
	}

	.stat-detail-chirho {
		font-size: 0.8rem;
		color: var(--text-muted-chirho);
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
		color: var(--text-muted-chirho);
		font-size: 1.1rem;
		margin-bottom: 3.5rem;
	}

	.features-grid-chirho {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1.5rem;
	}

	.feature-card-chirho {
		display: flex;
		flex-direction: row;
		align-items: flex-start;
		gap: 1.25rem;
		background: var(--panel-chirho);
		border: 1px solid var(--line-chirho);
		border-radius: 18px;
		padding: 1.5rem;
		transition:
			border-color 0.2s,
			transform 0.2s ease;
		box-shadow: var(--shadow-chirho);
	}

	.feature-card-chirho:hover {
		border-color: rgba(246, 190, 96, 0.28);
		transform: translateY(-3px);
	}

	.feature-icon-chirho {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 44px;
		height: 44px;
		background: rgba(246, 190, 96, 0.09);
		color: var(--accent-gold-chirho);
		border-radius: 10px;
		font-weight: 700;
		font-size: 1.1rem;
	}

	.feature-text-chirho h3 {
		margin: 0 0 0.35rem;
		font-size: 1.05rem;
		color: var(--text-main-chirho);
	}

	.feature-text-chirho p {
		margin: 0;
		font-size: 0.9rem;
		color: var(--text-soft-chirho);
		line-height: 1.6;
	}

	/* Performance */
	.performance-chirho {
		padding: 6rem 0;
		background: rgba(113, 215, 209, 0.025);
		border-top: 1px solid var(--line-chirho);
		border-bottom: 1px solid var(--line-chirho);
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
		font-family: 'IBM Plex Mono', monospace;
		font-weight: 600;
		font-size: 1rem;
		color: var(--accent-cyan-chirho);
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
		color: var(--text-muted-chirho);
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
		background: rgba(246, 190, 96, 0.08);
		color: var(--accent-gold-chirho);
		padding: 0.75rem 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(125, 211, 252, 0.2);
	}
	.bench-table-chirho td {
		padding: 0.6rem 1rem;
		border-bottom: 1px solid rgba(125, 211, 252, 0.06);
		color: var(--text-soft-chirho);
	}
	.bench-table-chirho tr:hover td {
		background: rgba(246, 190, 96, 0.04);
	}

	/* Get Started */
	.get-started-chirho {
		padding: 6rem 0;
	}

	.code-block-chirho {
		max-width: 700px;
		margin: 0 auto;
		background: #15110c;
		border: 1px solid var(--line-chirho);
		border-radius: 18px;
		overflow: hidden;
		box-shadow: var(--shadow-chirho);
	}

	.code-header-chirho {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0.75rem 1rem;
		background: rgba(246, 190, 96, 0.04);
		border-bottom: 1px solid var(--line-chirho);
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
		color: var(--text-muted-chirho);
		font-family: 'IBM Plex Mono', monospace;
	}

	.code-content-chirho {
		padding: 1.5rem;
		overflow-x: auto;
		font-size: 0.85rem;
		line-height: 1.8;
	}

	.code-content-chirho code {
		color: var(--text-main-chirho);
	}

	:global(.code-comment-chirho) {
		color: #4a5568;
	}

	:global(.code-prompt-chirho) {
		color: var(--accent-cyan-chirho);
		font-weight: 600;
	}

	:global(.code-output-chirho) {
		color: #64748b;
	}

	:global(.code-keyword-chirho) {
		color: var(--accent-gold-chirho);
		font-weight: 600;
	}

	:global(.code-type-chirho) {
		color: var(--accent-cyan-chirho);
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
		background: rgba(246, 190, 96, 0.025);
		border-top: 1px solid var(--line-chirho);
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
		background: rgba(246, 190, 96, 0.05);
		border: 1px solid var(--line-chirho);
		border-radius: 8px;
	}
	.pipeline-num-chirho {
		font-weight: 600;
		color: var(--accent-gold-chirho);
		font-size: 0.85rem;
		min-width: 120px;
		font-family: 'IBM Plex Mono', monospace;
	}
	.pipeline-detail-chirho {
		font-size: 0.85rem;
		color: var(--text-soft-chirho);
	}
	.pipeline-vline-chirho {
		width: 2px;
		height: 8px;
		background: rgba(246, 190, 96, 0.15);
	}

	/* Footer */
	.footer-chirho {
		padding: 3rem 2rem;
		border-top: 1px solid var(--line-chirho);
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
		color: var(--text-soft-chirho);
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
		color: var(--text-muted-chirho);
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
		color: var(--text-muted-chirho);
	}

	.footer-right-chirho a:hover {
		color: var(--accent-cyan-chirho);
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

	/* Ecosystem Section */
	.ecosystem-chirho {
		padding: 5rem 2rem;
		background: rgba(18, 15, 11, 0.42);
	}

	/* Limitations Section */
	.limitations-chirho {
		padding: 5rem 2rem;
		background: rgba(25, 20, 14, 0.86);
	}

	.limitation-list-chirho {
		list-style: none;
		padding: 0;
		max-width: 800px;
		margin: 2rem auto;
	}

	.limitation-list-chirho li {
		padding: 0.75rem 0;
		border-bottom: 1px solid var(--line-chirho);
		color: var(--text-soft-chirho);
		line-height: 1.6;
	}

	.limitation-list-chirho li strong {
		color: var(--text-main-chirho);
	}

	/* Roadmap Section */
	.roadmap-chirho {
		padding: 5rem 2rem;
		background: rgba(18, 15, 11, 0.42);
	}

	.roadmap-grid-chirho {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.roadmap-item-chirho {
		background: var(--panel-strong-chirho);
		border: 1px solid var(--line-chirho);
		border-radius: 12px;
		padding: 1.5rem;
	}

	.roadmap-item-chirho h3 {
		color: var(--accent-gold-chirho);
		margin: 0 0 0.5rem;
		font-size: 1.1rem;
	}

	.roadmap-item-chirho p {
		color: var(--text-soft-chirho);
		margin: 0;
		line-height: 1.5;
		font-size: 0.95rem;
	}

	@media (max-width: 768px) {
		.hero-proof-chirho {
			grid-template-columns: 1fr;
		}
	}
</style>
