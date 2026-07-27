// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// A small asciinema .cast (v2) replayer: JSONL events appended into a <pre> with an
// SGR-color subset (what our recording actually uses). Zero dependencies, fully themeable.

interface CastEventChirho {
	atChirho: number;
	textChirho: string;
}

export interface CastPlayerOptsChirho {
	idleLimitChirho?: number;
	onEndChirho?: () => void;
}

const SGR_FG_CHIRHO: Record<string, string> = {
	'30': 'cast-fg-black-chirho',
	'31': 'cast-fg-red-chirho',
	'32': 'cast-fg-green-chirho',
	'33': 'cast-fg-gold-chirho',
	'34': 'cast-fg-blue-chirho',
	'35': 'cast-fg-magenta-chirho',
	'36': 'cast-fg-cyan-chirho',
	'37': 'cast-fg-white-chirho',
	'90': 'cast-fg-gray-chirho',
	'96': 'cast-fg-cyan-chirho'
};

export class CastPlayerChirho {
	private targetChirho: HTMLElement;
	private eventsChirho: CastEventChirho[] = [];
	private optsChirho: CastPlayerOptsChirho;

	private boldChirho = false;
	private fgChirho = '';
	private currentLineChirho: HTMLElement | null = null;
	private pendingCrChirho = false;

	private playingChirho = false;
	private speedChirho = 1;
	private cursorChirho = 0;
	private playedMsChirho = 0;
	private lastTickChirho = 0;
	private rafChirho = 0;

	constructor(targetChirho: HTMLElement, optsChirho: CastPlayerOptsChirho = {}) {
		this.targetChirho = targetChirho;
		this.optsChirho = optsChirho;
	}

	async loadChirho(urlChirho: string): Promise<void> {
		const resChirho = await fetch(urlChirho);
		if (!resChirho.ok) {
			throw new Error(`cast fetch failed: ${resChirho.status}`);
		}
		const bodyChirho = await resChirho.text();
		const linesChirho = bodyChirho.split('\n').filter((lChirho) => lChirho.trim().length > 0);
		let idleLimitChirho = this.optsChirho.idleLimitChirho ?? 2.5;
		const rawChirho: CastEventChirho[] = [];
		for (const lineChirho of linesChirho) {
			if (lineChirho.startsWith('{')) {
				try {
					const headChirho = JSON.parse(lineChirho) as { idle_time_limit?: number };
					if (typeof headChirho.idle_time_limit === 'number') {
						idleLimitChirho = headChirho.idle_time_limit;
					}
				} catch {
					// tolerate a malformed header; defaults stand
				}
				continue;
			}
			try {
				const evChirho = JSON.parse(lineChirho) as [number, string, string];
				if (evChirho[1] === 'o') {
					rawChirho.push({ atChirho: evChirho[0], textChirho: evChirho[2] });
				}
			} catch {
				// skip malformed event lines
			}
		}
		if (rawChirho.length === 0) {
			throw new Error('cast parsed to zero events — wrong content served?');
		}
		// compress idle gaps so replays never stall
		let shiftChirho = 0;
		let prevChirho = 0;
		this.eventsChirho = rawChirho.map((evChirho) => {
			const gapChirho = evChirho.atChirho - prevChirho;
			if (gapChirho > idleLimitChirho) {
				shiftChirho += gapChirho - idleLimitChirho;
			}
			prevChirho = evChirho.atChirho;
			return { atChirho: (evChirho.atChirho - shiftChirho) * 1000, textChirho: evChirho.textChirho };
		});
	}

	get durationMsChirho(): number {
		return this.eventsChirho.length
			? this.eventsChirho[this.eventsChirho.length - 1].atChirho
			: 0;
	}

	playChirho(): void {
		if (this.playingChirho || this.eventsChirho.length === 0) return;
		if (this.cursorChirho >= this.eventsChirho.length) this.restartChirho();
		this.playingChirho = true;
		this.lastTickChirho = performance.now();
		this.rafChirho = requestAnimationFrame(this.tickChirho);
	}

	pauseChirho(): void {
		this.playingChirho = false;
		cancelAnimationFrame(this.rafChirho);
	}

	restartChirho(): void {
		this.pauseChirho();
		this.cursorChirho = 0;
		this.playedMsChirho = 0;
		this.boldChirho = false;
		this.fgChirho = '';
		this.pendingCrChirho = false;
		this.currentLineChirho = null;
		this.targetChirho.textContent = '';
	}

	setSpeedChirho(speedChirho: number): void {
		this.speedChirho = speedChirho;
	}

	get playingNowChirho(): boolean {
		return this.playingChirho;
	}

	disposeChirho(): void {
		this.pauseChirho();
	}

	private tickChirho = (): void => {
		if (!this.playingChirho) return;
		const nowChirho = performance.now();
		this.playedMsChirho += (nowChirho - this.lastTickChirho) * this.speedChirho;
		this.lastTickChirho = nowChirho;
		while (
			this.cursorChirho < this.eventsChirho.length &&
			this.eventsChirho[this.cursorChirho].atChirho <= this.playedMsChirho
		) {
			this.writeChirho(this.eventsChirho[this.cursorChirho].textChirho);
			this.cursorChirho++;
		}
		this.targetChirho.scrollTop = this.targetChirho.scrollHeight;
		if (this.cursorChirho >= this.eventsChirho.length) {
			this.playingChirho = false;
			this.optsChirho.onEndChirho?.();
			return;
		}
		this.rafChirho = requestAnimationFrame(this.tickChirho);
	};

	private ensureLineChirho(): HTMLElement {
		if (!this.currentLineChirho) {
			const lineChirho = document.createElement('span');
			lineChirho.className = 'cast-line-chirho';
			this.targetChirho.appendChild(lineChirho);
			this.currentLineChirho = lineChirho;
		}
		return this.currentLineChirho;
	}

	private newlineChirho(): void {
		this.ensureLineChirho();
		this.targetChirho.appendChild(document.createTextNode('\n'));
		this.currentLineChirho = null;
		this.pendingCrChirho = false;
	}

	private writeChirho(chunkChirho: string): void {
		// split off escape sequences, keep them as tokens
		const partsChirho = chunkChirho.split(/(\u001b\[[0-9;]*[A-Za-z])/);
		for (const partChirho of partsChirho) {
			if (partChirho.length === 0) continue;
			if (partChirho.startsWith('\u001b[')) {
				const finalChirho = partChirho[partChirho.length - 1];
				const argsChirho = partChirho.slice(2, -1);
				if (finalChirho === 'm') {
					this.applySgrChirho(argsChirho);
				} else if (finalChirho === 'K') {
					if (this.currentLineChirho) this.currentLineChirho.textContent = '';
				} else if (finalChirho === 'J') {
					this.targetChirho.textContent = '';
					this.currentLineChirho = null;
				}
				// other CSI sequences are ignored on purpose
				continue;
			}
			this.writeTextChirho(partChirho);
		}
	}

	private applySgrChirho(argsChirho: string): void {
		const codesChirho = argsChirho.length ? argsChirho.split(';') : ['0'];
		for (const codeChirho of codesChirho) {
			if (codeChirho === '0' || codeChirho === '') {
				this.boldChirho = false;
				this.fgChirho = '';
			} else if (codeChirho === '1') {
				this.boldChirho = true;
			} else if (SGR_FG_CHIRHO[codeChirho]) {
				this.fgChirho = SGR_FG_CHIRHO[codeChirho];
			}
		}
	}

	private writeTextChirho(textChirho: string): void {
		const piecesChirho = textChirho.split(/(\r\n|\n|\r)/);
		for (const pieceChirho of piecesChirho) {
			if (pieceChirho === '') continue;
			if (pieceChirho === '\r\n' || pieceChirho === '\n') {
				this.newlineChirho();
				continue;
			}
			if (pieceChirho === '\r') {
				this.pendingCrChirho = true;
				continue;
			}
			if (this.pendingCrChirho && this.currentLineChirho) {
				this.currentLineChirho.textContent = '';
				this.pendingCrChirho = false;
			}
			const lineChirho = this.ensureLineChirho();
			const spanChirho = document.createElement('span');
			let clsChirho = '';
			if (this.fgChirho) clsChirho += this.fgChirho;
			if (this.boldChirho) clsChirho += (clsChirho ? ' ' : '') + 'cast-bold-chirho';
			if (clsChirho) spanChirho.className = clsChirho;
			spanChirho.textContent = pieceChirho;
			lineChirho.appendChild(spanChirho);
		}
	}
}
