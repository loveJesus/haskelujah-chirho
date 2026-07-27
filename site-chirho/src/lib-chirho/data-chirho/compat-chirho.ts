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
	percentChirho: string;
	measuredDateChirho: string;
	measuredCommitChirho: string;
}

function parseMeasuredLineChirho(rawChirho: string, artifactChirho: string): {
	dateChirho: string;
	commitChirho: string;
} {
	const measuredChirho = rawChirho.match(/# measured: (\d{4}-\d{2}-\d{2}) @ ([0-9a-f]{6,40})/);
	if (!measuredChirho) {
		throw new Error(`compat-chirho: no "# measured:" line in ${artifactChirho}`);
	}
	return { dateChirho: measuredChirho[1], commitChirho: measuredChirho[2] };
}

function parseShouldCompileChirho(rawChirho: string): CompatAxisChirho {
	const resultChirho = rawChirho.match(/# RESULT: PASS=(\d+) FAIL=\d+ TOTAL=(\d+) \((\d+\.?\d*)%\)/);
	if (!resultChirho) {
		throw new Error('compat-chirho: RESULT line missing in should_compile artifact');
	}
	const metaChirho = parseMeasuredLineChirho(rawChirho, 'should_compile');
	return {
		passChirho: Number(resultChirho[1]),
		totalChirho: Number(resultChirho[2]),
		percentChirho: `${resultChirho[3]}%`,
		measuredDateChirho: metaChirho.dateChirho,
		measuredCommitChirho: metaChirho.commitChirho.slice(0, 8)
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
	return {
		passChirho: Number(resultChirho[1]),
		totalChirho: Number(resultChirho[2]),
		percentChirho: `${resultChirho[3]}%`,
		measuredDateChirho: metaChirho.dateChirho,
		measuredCommitChirho: metaChirho.commitChirho.slice(0, 8)
	};
}

export const shouldCompileChirho: CompatAxisChirho = parseShouldCompileChirho(shouldCompileRawChirho);
export const shouldFailChirho: CompatAxisChirho = parseShouldFailChirho(shouldFailRawChirho);

export const artifactRepoPathsChirho = {
	shouldCompileChirho: 'spec-chirho/ghc-should-compile-measurement-chirho.txt',
	shouldFailChirho: 'spec-chirho/ghc-should-fail-measurement-chirho.txt'
} as const;
