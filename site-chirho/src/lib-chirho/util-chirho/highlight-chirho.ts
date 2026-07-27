// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// A minimal Haskell tokenizer for display highlighting. Produces typed segments that
// components render as <span>s — no HTML strings, no injection surface.

export type SegmentKindChirho =
	| 'keyword'
	| 'type'
	| 'operator'
	| 'number'
	| 'string'
	| 'comment'
	| 'plain';

export interface SegmentChirho {
	textChirho: string;
	kindChirho: SegmentKindChirho;
}

const KEYWORDS_CHIRHO = new Set([
	'module',
	'where',
	'import',
	'data',
	'newtype',
	'type',
	'class',
	'instance',
	'let',
	'in',
	'do',
	'case',
	'of',
	'if',
	'then',
	'else',
	'deriving',
	'rec',
	'mdo'
]);

const TOKEN_RE_CHIRHO =
	/(--[^\n]*)|("(?:[^"\\]|\\.)*")|(\b\d[\d_]*\b)|([A-Z][A-Za-z0-9_']*)|([a-z_][A-Za-z0-9_']*)|(::|->|=>|<-|>>=|>>|\.\.|[=+\-*/<>$.:|\\@~]+)|(\s+)|(.)/g;

export function highlightHaskellChirho(sourceChirho: string): SegmentChirho[] {
	const segsChirho: SegmentChirho[] = [];
	let matchChirho: RegExpExecArray | null;
	TOKEN_RE_CHIRHO.lastIndex = 0;
	while ((matchChirho = TOKEN_RE_CHIRHO.exec(sourceChirho)) !== null) {
		const [
			fullChirho,
			commentChirho,
			stringChirho,
			numberChirho,
			conidChirho,
			varidChirho,
			opChirho
		] = matchChirho;
		let kindChirho: SegmentKindChirho = 'plain';
		if (commentChirho) kindChirho = 'comment';
		else if (stringChirho) kindChirho = 'string';
		else if (numberChirho) kindChirho = 'number';
		else if (conidChirho) kindChirho = 'type';
		else if (varidChirho) kindChirho = KEYWORDS_CHIRHO.has(varidChirho) ? 'keyword' : 'plain';
		else if (opChirho) kindChirho = 'operator';
		segsChirho.push({ textChirho: fullChirho, kindChirho });
	}
	return segsChirho;
}
