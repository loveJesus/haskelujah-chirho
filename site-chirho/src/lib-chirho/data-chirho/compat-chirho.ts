// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// TRUTH SOURCE: both GHC-compatibility numbers are parsed at BUILD TIME from the
// committed measurement artifacts in spec-chirho/. Nothing on the page is hand-typed,
// so the site cannot drift from the repo's published measurement. If an artifact's
// format changes, the build fails loudly rather than shipping a stale or wrong number.

import shouldCompileRawChirho from '../../../../spec-chirho/ghc-should-compile-measurement-chirho.txt?raw';
import shouldFailRawChirho from '../../../../spec-chirho/ghc-should-fail-measurement-chirho.txt?raw';

export interface CompatAxisChirho {
	passChirho: number;
	totalChirho: number;
	/** raw percent string as written in the artifact — parsed for validation, NOT displayed */
	percentChirho: string;
	/**
	 * What the page shows. The type checker currently has a measured run-to-run
	 * nondeterminism (±1 file observed on the should_compile corpus), so a third
	 * significant figure is precision the single-sweep method cannot support.
	 * Preference order: the artifact's own `# QUOTE-AS:` line verbatim (the artifact
	 * owner's honest display string), else FLOOR of the committed counts — floored,
	 * never rounded, so an uncertain compatibility claim can never sit above its
	 * own range (room ruling #8378).
	 */
	displayPercentChirho: string;
	measuredDateChirho: string;
	measuredCommitChirho: string;
	/**
	 * The artifact's `# STABILITY:` line VERBATIM, when present. The two axes carry
	 * different stability confidence (e.g. "DETERMINISTIC-FAIL=88 UNSTABLE=1" vs
	 * "NO-VARIANCE-OBSERVED-IN-2-PASSES UNSTABLE=unknown") and rendering the raw line
	 * is the only presentation that cannot flatten one into the other (#8392).
	 */
	stabilityChirho: string | null;
}

function approxPercentChirho(passChirho: number, totalChirho: number): string {
	return `≈ ${Math.floor((passChirho / totalChirho) * 100)}%`;
}

function parseQuoteAsChirho(rawChirho: string): string | null {
	const quoteAsChirho = rawChirho.match(/^# QUOTE-AS: (.+)$/m);
	if (!quoteAsChirho) return null;
	// the em-dash tail is guidance for humans quoting the number, not display copy
	return quoteAsChirho[1].split(' — ')[0].trim();
}

function parseMeasuredLineChirho(rawChirho: string, artifactChirho: string): {
	dateChirho: string;
	commitChirho: string;
} {
	const dateChirho = rawChirho.match(/^# measured: (\d{4}-\d{2}-\d{2})/m);
	if (!dateChirho) {
		throw new Error(`compat-chirho: no "# measured:" date in ${artifactChirho}`);
	}
	// current artifacts carry "# code measured: <hash>"; older ones inlined "@ <hash>"
	const commitChirho =
		rawChirho.match(/^# code measured: ([0-9a-f]{6,40})/m) ??
		rawChirho.match(/^# measured: \d{4}-\d{2}-\d{2} @ ([0-9a-f]{6,40})/m);
	if (!commitChirho) {
		throw new Error(`compat-chirho: no measured-commit line in ${artifactChirho}`);
	}
	return { dateChirho: dateChirho[1], commitChirho: commitChirho[1] };
}

function parseStabilityChirho(rawChirho: string): string | null {
	const stabilityChirho = rawChirho.match(/^# STABILITY: (.+)$/m);
	return stabilityChirho ? stabilityChirho[1].trim() : null;
}

function parseShouldCompileChirho(rawChirho: string): CompatAxisChirho {
	const resultChirho = rawChirho.match(/# RESULT: PASS=(\d+) FAIL=\d+ TOTAL=(\d+) \((\d+\.?\d*)%\)/);
	if (!resultChirho) {
		throw new Error('compat-chirho: RESULT line missing in should_compile artifact');
	}
	const metaChirho = parseMeasuredLineChirho(rawChirho, 'should_compile');
	const passChirho = Number(resultChirho[1]);
	const totalChirho = Number(resultChirho[2]);
	return {
		passChirho,
		totalChirho,
		percentChirho: `${resultChirho[3]}%`,
		displayPercentChirho:
			parseQuoteAsChirho(rawChirho) ?? approxPercentChirho(passChirho, totalChirho),
		measuredDateChirho: metaChirho.dateChirho,
		measuredCommitChirho: metaChirho.commitChirho.slice(0, 8),
		stabilityChirho: parseStabilityChirho(rawChirho)
	};
}

function parseShouldFailChirho(rawChirho: string): CompatAxisChirho {
	const resultChirho = rawChirho.match(
		/# RESULT: CORRECTLY-REJECTED=(\d+) WRONGLY-ACCEPTED=\d+ TOTAL=(\d+) \((\d+\.?\d*)%/
	);
	if (!resultChirho) {
		throw new Error('compat-chirho: RESULT line missing in should_fail artifact');
	}
	const metaChirho = parseMeasuredLineChirho(rawChirho, 'should_fail');
	const passChirho = Number(resultChirho[1]);
	const totalChirho = Number(resultChirho[2]);
	return {
		passChirho,
		totalChirho,
		percentChirho: `${resultChirho[3]}%`,
		displayPercentChirho:
			parseQuoteAsChirho(rawChirho) ?? approxPercentChirho(passChirho, totalChirho),
		measuredDateChirho: metaChirho.dateChirho,
		measuredCommitChirho: metaChirho.commitChirho.slice(0, 8),
		stabilityChirho: parseStabilityChirho(rawChirho)
	};
}

export const shouldCompileChirho: CompatAxisChirho = parseShouldCompileChirho(shouldCompileRawChirho);
export const shouldFailChirho: CompatAxisChirho = parseShouldFailChirho(shouldFailRawChirho);

export const artifactRepoPathsChirho = {
	shouldCompileChirho: 'spec-chirho/ghc-should-compile-measurement-chirho.txt',
	shouldFailChirho: 'spec-chirho/ghc-should-fail-measurement-chirho.txt'
} as const;
