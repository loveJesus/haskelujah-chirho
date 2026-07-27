// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// Structured page content. Provenance notes:
// - toolsChirho: the 12 subcommands verified against crates/haskelujah-cli-chirho/src/main.rs
// - statsStripChirho: bash spec-chirho/stats-chirho.sh, run 2026-07-27 (grep/find based)
// - phasesChirho: the 12-phase pipeline as documented in the repo
// - compat numbers live in compat-chirho.ts (parsed from committed measurement artifacts)

export interface PhaseChirho {
	numeralChirho: string;
	nameChirho: string;
	detailChirho: string;
	hueChirho: string;
}

export const phasesChirho: PhaseChirho[] = [
	{ numeralChirho: 'I', nameChirho: 'Lex', detailChirho: 'Source becomes tokens', hueChirho: '#3e63c4' },
	{ numeralChirho: 'II', nameChirho: 'Layout', detailChirho: 'The offside rule; braces and semicolons appear', hueChirho: '#2e8fb8' },
	{ numeralChirho: 'III', nameChirho: 'Parse', detailChirho: 'A lossless green-tree CST', hueChirho: '#2fa089' },
	{ numeralChirho: 'IV', nameChirho: 'Lower', detailChirho: 'Concrete tree to abstract syntax', hueChirho: '#3e7c64' },
	{ numeralChirho: 'V', nameChirho: 'Resolve', detailChirho: 'Names, scopes, qualified imports', hueChirho: '#6fa34a' },
	{ numeralChirho: 'VI', nameChirho: 'Kind Infer', detailChirho: 'Kinds by unification', hueChirho: '#b8a032' },
	{ numeralChirho: 'VII', nameChirho: 'Type Infer', detailChirho: 'Algorithm W, typeclasses, GADTs', hueChirho: '#e3c363' },
	{ numeralChirho: 'VIII', nameChirho: 'Exhaustiveness', detailChirho: 'Pattern-match coverage', hueChirho: '#d9822b' },
	{ numeralChirho: 'IX', nameChirho: 'Core', detailChirho: 'Desugar to System FC; dictionaries made explicit', hueChirho: '#c23b22' },
	{ numeralChirho: 'X', nameChirho: 'Simplify', detailChirho: 'Inlining, CSE, specialisation', hueChirho: '#a93a63' },
	{ numeralChirho: 'XI', nameChirho: 'Codegen', detailChirho: 'LLVM · WebAssembly · Cranelift', hueChirho: '#7c4fa8' },
	{ numeralChirho: 'XII', nameChirho: 'Link', detailChirho: 'A native executable, or .wasm', hueChirho: '#4a5fb0' }
];

export interface PipelineStageChirho {
	titleChirho: string;
	phaseIdxChirho: [number, number];
	codeChirho: string;
	captionChirho: string;
}

// The same tiny program, shown honestly transforming through the pipeline.
// Dumps are illustrative (hand-written in the shape of each IR), and labeled so on the page.
export const sampleSourceChirho = `module Main where

double :: Int -> Int
double n = n + n

main :: IO ()
main = print (double 21)`;

export const pipelineStagesChirho: PipelineStageChirho[] = [
	{
		titleChirho: 'Tokens, then structure',
		phaseIdxChirho: [0, 1],
		codeChirho: `{ module Main where
  { double :: Int -> Int
  ; double n = n + n
  ; main :: IO ()
  ; main = print (double 21)
  } }`,
		captionChirho:
			'The lexer streams tokens; the layout algorithm reads indentation and writes the braces and semicolons you never typed.'
	},
	{
		titleChirho: 'A tree that forgets nothing',
		phaseIdxChirho: [2, 3],
		codeChirho: `Module "Main"
├─ TypeSig  double :: Int -> Int
├─ FunBind  double
│   └─ Match [VarPat n]
│       └─ InfixApp (Var n) (+) (Var n)
└─ FunBind  main
    └─ App (Var print)
           (App (Var double) (Lit 21))`,
		captionChirho:
			'A lossless concrete syntax tree — every comment and space survives — then lowered to abstract syntax.'
	},
	{
		titleChirho: 'Every name finds its home',
		phaseIdxChirho: [4, 5],
		codeChirho: `double ↦ Main.double
(+)    ↦ GHC.Num.+        Num :: Type -> Constraint
print  ↦ System.IO.print  IO  :: Type -> Type
Int    ↦ GHC.Types.Int    Int :: Type`,
		captionChirho:
			'Scopes are resolved, imports are followed, and every type constructor is assigned a kind by unification.'
	},
	{
		titleChirho: 'The types agree',
		phaseIdxChirho: [6, 7],
		codeChirho: `double :: Int -> Int   ✓ inferred ≡ declared
main   :: IO ()        ✓
Num Int                ✓ solved by instance
double n               ✓ patterns exhaustive`,
		captionChirho:
			'Hindley–Milner inference meets the declared signatures; class constraints are solved; every match is checked for coverage.'
	},
	{
		titleChirho: 'Sugar gone, dictionaries visible',
		phaseIdxChirho: [8, 9],
		codeChirho: `double = λ(n :: Int) → (+) @Int $fNumInt n n
main   = print @Int $fShowInt (double (I# 21#))

-- after simplify: inline, then fold
main   = print @Int $fShowInt (I# 42#)`,
		captionChirho:
			'System FC Core: typeclasses become dictionary arguments. The simplifier inlines double and folds 21 + 21 where it can.'
	},
	{
		titleChirho: 'And then, machine',
		phaseIdxChirho: [10, 11],
		codeChirho: `_main:                      ; arm64
    mov  x0, #42
    bl   _hlj_print_int
    ret

(func $main                 ;; or wasm
  (call $print_i64 (i64.const 42)))`,
		captionChirho:
			'Three backends share one Core — LLVM, WebAssembly, Cranelift. Their maturity is tracked honestly in the ledger below.'
	}
];

export interface ToolChirho {
	commandChirho: string;
	blurbChirho: string;
}

export const toolsChirho: ToolChirho[] = [
	{ commandChirho: 'init', blurbChirho: 'Scaffold a new project' },
	{ commandChirho: 'build', blurbChirho: 'Compile .cabal projects to native binaries' },
	{ commandChirho: 'run', blurbChirho: 'Execute — or shebang a script with #!/usr/bin/env haskelujah' },
	{ commandChirho: 'check', blurbChirho: 'Type-check without codegen' },
	{ commandChirho: 'repl', blurbChirho: 'Interactive :type, :info, :load, evaluation' },
	{ commandChirho: 'test', blurbChirho: 'Compile and run test suites' },
	{ commandChirho: 'install', blurbChirho: 'Fetch from Hackage; resolve dependencies' },
	{ commandChirho: 'fmt', blurbChirho: 'Format Haskell source' },
	{ commandChirho: 'lsp', blurbChirho: 'Diagnostics and hover for VS Code, Neovim, Zed, Helix' },
	{ commandChirho: 'mcp', blurbChirho: 'Model Context Protocol server for AI assistants' },
	{ commandChirho: 'edit', blurbChirho: 'Built-in editor, GUI or terminal' },
	{ commandChirho: 'clean', blurbChirho: 'Remove build artifacts' }
];

export interface StatChirho {
	valueChirho: string;
	labelChirho: string;
}

// bash spec-chirho/stats-chirho.sh, 2026-07-27
export const statsStripChirho: StatChirho[] = [
	{ valueChirho: '224,961', labelChirho: 'lines of Rust' },
	{ valueChirho: '25', labelChirho: 'focused crates' },
	{ valueChirho: '499', labelChirho: 'module interfaces' },
	{ valueChirho: '411', labelChirho: 'Hackage package snapshots in the corpus' }
];

export const statsProvenanceChirho =
	'Counted by spec-chirho/stats-chirho.sh on 2026-07-27.';

export interface LimitationChirho {
	titleChirho: string;
	bodyChirho: string;
	/** repo-relative path of the committed bug writeup, when one exists */
	writeupPathChirho?: string;
}

export const limitationsChirho: LimitationChirho[] = [
	{
		titleChirho: 'We are too permissive',
		bodyChirho:
			'GHC rejects 767 programs in the should_fail corpus; today we correctly reject only a fraction of them (the second number above). A soundness-first effort is underway to close this honestly rather than quietly.'
	},
	{
		titleChirho: 'The type checker is not deterministic',
		bodyChirho:
			'At least one corpus file has been observed to flip between accepted and rejected across runs of the same binary on the same source — and the rest of the corpus has not been tested per-file for stability. The cause is partially identified: hash-iteration order reaching inference state. Seven order-dependent sites are fixed, and the defect still reproduces. Until it is gone, builds are not reproducible; the percentages above are floored, and the accept figure is a range.',
		writeupPathChirho: 'spec-chirho/bug-nondeterministic-typecheck-chirho.md'
	},
	{
		titleChirho: 'Running is not yet computing',
		bodyChirho:
			'A known miscompile exists: a boolean test inside a list comprehension’s recursive worker can take the wrong branch and return a silently wrong answer. The canonical lazy primes sieve hits it. This is on the interpreter path used by haskelujah run. The symptom is reproduced and isolated; the mechanism is still being traced, and no fix ships on a hypothesis.',
		writeupPathChirho: 'spec-chirho/bug-comprehension-letrec-capture-chirho.md'
	},
	{
		titleChirho: 'Compiled output is not yet trustworthy for structured values',
		bodyChirho:
			'When compiled to native code, printing a value that is not syntactically obvious at the call site can emit an internal representation instead of the value: a heap address, a raw constructor tag, or an internal name such as $tuple2. Lists, tuples, and derived Show instances are all affected once the value passes through any function; simple cases (integers, a literal list, a directly-applied constructor) are correct. The interpreter is unaffected — use haskelujah run for anything whose output you depend on.',
		writeupPathChirho: 'spec-chirho/bug-native-print-list-pointer-chirho.md'
	},
	{
		titleChirho: 'Rank-N inference gaps',
		bodyChirho:
			'Most rank-2 types work; some deep higher-rank patterns (lens-style LensLike′) still defeat inference.'
	},
	{
		titleChirho: 'Quantified constraints',
		bodyChirho:
			'forall a. C a ⇒ D (f a) in superclass positions is not yet supported. Blocks deepseq.'
	},
	{
		titleChirho: 'Associated type families',
		bodyChirho:
			'Token s / Tokens s normalisation is incomplete. Blocks the second stage of megaparsec.'
	},
	{
		titleChirho: 'Template Haskell',
		bodyChirho:
			'Basic splices and makeLenses work; typed splices, quasi-quoters, and full reify are incomplete.'
	},
	{
		titleChirho: 'FFI',
		bodyChirho:
			'Basic libc interop (puts, printf, malloc). Full C-header parsing and foreign exports are not yet implemented.'
	}
];

export interface RoadmapItemChirho {
	titleChirho: string;
	bodyChirho: string;
}

export const roadmapChirho: RoadmapItemChirho[] = [
	{
		titleChirho: 'Soundness first',
		bodyChirho: 'Reject what GHC rejects. The should_fail number on this page is the scoreboard.'
	},
	{
		titleChirho: 'The compiler, in your browser',
		bodyChirho:
			'Compile Haskelujah itself to WebAssembly so this page can run the real type checker, live. No fake playgrounds before then — this page shows only what is real.'
	},
	{
		titleChirho: 'Backend maturity',
		bodyChirho: 'Closures, thunks, and GC wired into compiled code on every backend.'
	},
	{
		titleChirho: 'Self-hosting',
		bodyChirho: 'Compile a non-trivial Haskell frontend for Haskelujah with Haskelujah.'
	},
	{
		titleChirho: 'Template Haskell, complete',
		bodyChirho: 'Typed splices, quasi-quoters, reify for every declaration form.'
	},
	{
		titleChirho: 'Cross-compilation & profiling',
		bodyChirho: 'Target selection from the CLI; cost centres, heap and time profiles.'
	}
];

export const linksChirho = {
	githubChirho: 'https://github.com/loveJesus/haskelujah-chirho',
	docsBaseChirho: 'https://github.com/loveJesus/haskelujah-chirho/blob/main_chirho/docs-chirho',
	donateChirho: 'https://kingdominvest.ing',
	newsChirho: 'https://news.kingdominvest.ing',
	contactChirho: 'mailto:haskelujah@mailer-aleluya.xjes.us',
	guideChirho: '/haskelujah-website-guide-chirho.md'
} as const;

export const installCommandChirho = 'cargo install haskelujah';
export const installScriptCommandChirho = 'curl -fsSL https://haskelujah.org/install-chirho.sh | sh';
export const tenSecondRunChirho = 'cargo install haskelujah && haskelujah repl';

export const scriptureChirho = {
	textChirho:
		'For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.',
	refChirho: 'John 3:16'
} as const;
