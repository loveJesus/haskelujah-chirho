// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// The rose window: 12 stained-glass panes — one per compiler phase — around a λ oculus.
// Procedural WebGL2 via three.js. Mobile guardrails: DPR clamped, rAF pausable, adaptive
// degrade ladder (DPR → particles → caller-swapped poster), context-loss surfaced to caller,
// reduced-motion renders a single static frame.

import {
	AdditiveBlending,
	BufferAttribute,
	BufferGeometry,
	CircleGeometry,
	Color,
	DoubleSide,
	Group,
	Mesh,
	PerspectiveCamera,
	Points,
	Raycaster,
	RingGeometry,
	Scene,
	ShaderMaterial,
	Shape,
	ShapeGeometry,
	Timer,
	Vector2,
	Vector3,
	WebGLRenderer
} from 'three';

export interface RoseWindowOptsChirho {
	paneHuesChirho: string[];
	staticChirho: boolean;
	mobileChirho: boolean;
	onPaneHoverChirho: (idxChirho: number | null, xChirho: number, yChirho: number) => void;
	onContextLostChirho: () => void;
}

export interface RoseWindowHandleChirho {
	setRunningChirho: (runChirho: boolean) => void;
	setScrollProgressChirho: (pChirho: number) => void;
	setPointerChirho: (xChirho: number, yChirho: number) => void;
	resizeChirho: () => void;
	disposeChirho: () => void;
}

const NOISE_GLSL_CHIRHO = /* glsl */ `
	float hashChirho(vec2 p) {
		return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
	}
	float noiseChirho(vec2 p) {
		vec2 i = floor(p);
		vec2 f = fract(p);
		// quintic interpolation — the cubic version shows its bilinear grid under magnification
		vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
		return mix(
			mix(hashChirho(i), hashChirho(i + vec2(1.0, 0.0)), u.x),
			mix(hashChirho(i + vec2(0.0, 1.0)), hashChirho(i + vec2(1.0, 1.0)), u.x),
			u.y
		);
	}
`;

const PANE_VERT_CHIRHO = /* glsl */ `
	varying vec2 vPosChirho;
	void main() {
		vPosChirho = position.xy;
		gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
	}
`;

const PANE_FRAG_CHIRHO = /* glsl */ `
	precision highp float;
	varying vec2 vPosChirho;
	uniform vec3 uColorChirho;
	uniform vec2 uSunDirChirho;
	uniform float uTimeChirho;
	uniform float uHoverChirho;
	uniform float uGlowChirho;
	uniform float uSeedChirho;
	uniform float uKindleChirho;
	uniform float uR0Chirho;
	uniform float uR1Chirho;
	${NOISE_GLSL_CHIRHO}
	void main() {
		float r = length(vPosChirho);
		float ang = atan(vPosChirho.y, vPosChirho.x);
		// leaded-glass cells in a polar domain, warped by noise so no two panes match
		vec2 cell = vec2(ang * 6.0 + uSeedChirho * 13.0, (r - uR0Chirho) * 9.0);
		cell += 0.4 * vec2(
			noiseChirho(cell * 1.7 + uSeedChirho),
			noiseChirho(cell * 1.3 + 11.0 + uSeedChirho)
		);
		vec2 g = fract(cell) - 0.5;
		float lead = smoothstep(0.38, 0.5, max(abs(g.x), abs(g.y)));
		// glass mottle + slow internal shimmer; second octave keeps texture alive under zoom
		float mottle = 0.66 + 0.24 * noiseChirho(vPosChirho * 13.0 + uSeedChirho * 7.0)
			+ 0.10 * noiseChirho(vPosChirho * 34.0 + uSeedChirho * 3.0);
		float shimmer = 0.94 + 0.06 * noiseChirho(vPosChirho * 3.0 + uTimeChirho * 0.05);
		// transmission: panes facing the moving sun glow brighter
		float sun = 0.5 + 0.5 * clamp(dot(normalize(vPosChirho), uSunDirChirho), -1.0, 1.0);
		vec3 glass = uColorChirho * mottle * shimmer * (0.52 + 0.95 * sun);
		glass += uColorChirho * uHoverChirho * 0.85;
		glass *= 1.0 + min(uGlowChirho, 1.0) * 1.8;
		// the kindling: each pane wakes in pipeline order, with a brief bloom as it lights
		float kindleBloom = exp(-pow(uKindleChirho - 0.55, 2.0) * 14.0) * 0.55;
		glass *= 0.18 + 0.82 * uKindleChirho + kindleBloom;
		vec3 col = mix(glass, vec3(0.055, 0.045, 0.035), lead * 0.88);
		// gilded rims at the stone edges
		float rim = smoothstep(uR1Chirho - 0.018, uR1Chirho - 0.004, r)
			+ (1.0 - smoothstep(uR0Chirho + 0.004, uR0Chirho + 0.018, r));
		col += vec3(0.82, 0.66, 0.28) * rim * (0.3 + 0.5 * uGlowChirho + 0.4 * uHoverChirho);
		gl_FragColor = vec4(col, 1.0);
	}
`;

const OCULUS_FRAG_CHIRHO = /* glsl */ `
	precision highp float;
	varying vec2 vPosChirho;
	uniform float uTimeChirho;
	uniform float uGlowChirho;
	uniform float uProgressChirho;
	uniform float uVelocityChirho;
	uniform float uRChirho;
	${NOISE_GLSL_CHIRHO}
	// returns (signed distance, normalized position along the stroke)
	vec2 capsuleChirho(vec2 p, vec2 a, vec2 b, float rad) {
		vec2 pa = p - a;
		vec2 ba = b - a;
		float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
		return vec2(length(pa - ba * h) - rad, h);
	}
	// a true 3D warp starfield: eight depth shells under perspective divide. As travel
	// grows, every shell's scale grows, so each star's screen position (cell/scale)
	// slides toward the vanishing point and shrinks — flying BACKWARDS out of the sky.
	// While the camera is actually moving, near stars stretch radially (warp streaks).
	vec3 warpStarsChirho(vec2 p, float t, float travel, float vel) {
		vec3 acc = vec3(0.0);
		for (int i = 0; i < 8; i++) {
			float fi = float(i);
			float z = fract(fi * 0.125 + travel);
			float scale = mix(3.0, 34.0, z * z);
			float fade = smoothstep(0.0, 0.18, z) * (1.0 - smoothstep(0.82, 1.0, z));
			vec2 suv = p * scale + fi * 37.7;
			vec2 cell = floor(suv);
			float h = hashChirho(cell + fi * 11.0);
			if (h < 0.55) continue;
			vec2 spos = 0.15 + 0.7 * vec2(hashChirho(cell + 3.1), hashChirho(cell + 9.7));
			vec2 off = fract(suv) - spos;
			// star's world direction — streaks run along it, away from the vanishing point
			vec2 world = (cell + spos - fi * 37.7) / scale;
			vec2 rdir = normalize(world + vec2(1e-4));
			float dR = dot(off, rdir);
			float dT = off.x * rdir.y - off.y * rdir.x;
			float near = 1.0 - z;
			float sizeBase = mix(900.0, 150.0, near * near);
			float stretch = 1.0 + vel * 26.0 * near;
			float iStar = exp(-(dT * dT * sizeBase * stretch + dR * dR * sizeBase / stretch));
			float tw = 0.75 + 0.25 * sin(t * (1.0 + h * 3.0) + h * 30.0);
			float hue = hashChirho(cell + 21.0);
			vec3 tint = mix(vec3(0.82, 0.86, 1.0), vec3(1.0, 0.85, 0.62), step(0.72, hue));
			tint = mix(tint, vec3(1.0, 0.66, 0.56), step(0.93, hue));
			acc += tint * iStar * tw * fade * (0.35 + near * 0.9);
		}
		return acc;
	}
	// one inclined two-armed spiral galaxy, dusty arms and a warm core
	vec3 galaxyChirho(vec2 p, float t, float recede) {
		vec2 g = (p - vec2(0.52, 0.4)) * recede;
		float cs = cos(0.6);
		float sn = sin(0.6);
		g = mat2(cs, -sn, sn, cs) * g;
		g.y *= 2.3;
		float r = length(g) + 1e-4;
		float th = atan(g.y, g.x);
		float arms = 0.5 + 0.5 * sin(2.0 * th - log(r) * 4.5 + t * 0.03);
		float dust = 0.65 + 0.35 * noiseChirho(g * 6.0 + 3.0);
		float disk = exp(-r * 3.2) * (0.35 + 0.65 * arms) * dust;
		float core = exp(-r * 11.0);
		return vec3(0.55, 0.62, 0.85) * disk * 0.85 + vec3(1.0, 0.85, 0.6) * core * 0.95;
	}
	void main() {
		vec2 p = vPosChirho / uRChirho; // normalized to oculus radius
		float glowCapChirho = min(uGlowChirho, 0.85);
		float t = uTimeChirho;

		// nebula ground: domain-warped noise through a lapis -> violet -> teal palette
		vec2 q = p * 2.3 + vec2(t * 0.016, -t * 0.011);
		float warp = noiseChirho(q * 1.6 + 3.7);
		float n = noiseChirho(q + warp * 1.15);
		vec3 col = mix(vec3(0.028, 0.05, 0.16), vec3(0.07, 0.06, 0.21), smoothstep(0.3, 0.75, n));
		col = mix(col, vec3(0.045, 0.13, 0.17), smoothstep(0.62, 0.95, noiseChirho(q * 1.3 + 9.0)) * 0.5);
		col *= 0.62 + glowCapChirho * 0.75;

		// the warp field: always travelling backwards — a steady inward stream at rest,
		// the scroll gesture surging it. A faint baseline stretch keeps the motion legible.
		float eased = uProgressChirho * uProgressChirho * (3.0 - 2.0 * uProgressChirho);
		float travel = eased * 0.55 + t * 0.045;
		col += warpStarsChirho(p, t, travel, uVelocityChirho + 0.035) * (0.55 + glowCapChirho * 0.7);
		// the galaxy rides the middle depth, receding with everything else
		col += galaxyChirho(p, t, 1.0 + eased * 2.0) * (0.4 + glowCapChirho * 0.5);

		// λ — vector-crisp at any zoom (fwidth AA), and molten inside: energy flowing
		// along each stroke, a white-hot centerline, slow swirl, gold dust twinkling
		float wStroke = 0.058;
		vec2 cMain = capsuleChirho(p, vec2(-0.3, 0.56), vec2(0.34, -0.58), wStroke);
		vec2 cLeg = capsuleChirho(p, vec2(-0.01, 0.03), vec2(-0.34, -0.58), wStroke);
		float d;
		float along;
		float lenScale;
		if (cMain.x < cLeg.x) {
			d = cMain.x;
			along = cMain.y;
			lenScale = 1.6;
		} else {
			d = cLeg.x;
			along = cLeg.y;
			lenScale = 0.95;
		}
		float aa = fwidth(d) * 1.4 + 1e-4;
		float body = 1.0 - smoothstep(0.0, aa, d);
		float halo = exp(-max(d, 0.0) * 9.0);
		float pulse = 0.93 + 0.07 * sin(t * 0.7);
		float depth = clamp(-d / wStroke, 0.0, 1.0);
		float flow = 0.5 + 0.32 * sin(along * lenScale * 9.0 - t * 1.7)
			+ 0.18 * sin(along * lenScale * 23.0 - t * 3.1);
		float swirl = noiseChirho(p * 7.0 + vec2(t * 0.12, -t * 0.09));
		// dark burnished rims -> white-hot centerline; flow travels as brightness waves
		float coreGlow = smoothstep(0.2, 0.92, depth);
		vec3 lam = mix(vec3(0.58, 0.38, 0.1), vec3(1.0, 0.93, 0.6), coreGlow);
		lam *= (0.68 + 0.34 * flow) * (0.9 + 0.2 * swirl) * pulse;
		// gold dust inside the glyph
		vec2 sc = floor(p * 46.0);
		float sh = hashChirho(sc + 5.0);
		if (sh > 0.86) {
			vec2 sp = 0.25 + 0.5 * vec2(hashChirho(sc + 1.3), hashChirho(sc + 8.7));
			float sd = length(fract(p * 46.0) - sp);
			float stw = 0.5 + 0.5 * sin(t * (2.0 + sh * 4.0) + sh * 50.0);
			lam += vec3(1.0, 0.95, 0.7) * exp(-sd * sd * 180.0) * stw * 0.8;
		}
		col = mix(col, lam, body);
		col += vec3(1.0, 0.9, 0.55) * halo * pulse * (0.14 + 0.4 * glowCapChirho);

		// rim + screen-space dither (kills banding in the nebula gradients)
		float r = length(vPosChirho);
		float rim = smoothstep(uRChirho - 0.02, uRChirho - 0.006, r);
		col += vec3(0.82, 0.66, 0.28) * rim * 0.5;
		col += (hashChirho(gl_FragCoord.xy * 0.7 + fract(t)) - 0.5) * 0.012;
		gl_FragColor = vec4(col, 1.0);
	}
`;

const SHAFT_FRAG_CHIRHO = /* glsl */ `
	precision highp float;
	varying vec2 vUvChirho;
	uniform float uTimeChirho;
	uniform float uGlowChirho;
	uniform float uSeedChirho;
	${NOISE_GLSL_CHIRHO}
	void main() {
		// a soft beam: bright at the top (window), fading down; edges feathered
		float across = smoothstep(0.0, 0.32, vUvChirho.x) * (1.0 - smoothstep(0.68, 1.0, vUvChirho.x));
		float along = pow(1.0 - vUvChirho.y, 1.6);
		float sway = 0.9 + 0.1 * sin(uTimeChirho * 0.21 + uSeedChirho * 6.0);
		// dither kills banding in the long gradient
		float dith = (hashChirho(vUvChirho * 913.0 + uTimeChirho) - 0.5) * 0.012;
		float a = across * along * sway * (0.05 + 0.11 * uGlowChirho) + dith * across * along;
		gl_FragColor = vec4(vec3(0.86, 0.72, 0.38), max(a, 0.0));
	}
`;

const SHAFT_VERT_CHIRHO = /* glsl */ `
	varying vec2 vUvChirho;
	void main() {
		vUvChirho = uv;
		gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
	}
`;

const DUST_VERT_CHIRHO = /* glsl */ `
	attribute float aPhaseChirho;
	uniform float uTimeChirho;
	uniform float uSizeChirho;
	varying float vTwinkleChirho;
	void main() {
		vec3 p = position;
		p.x += sin(uTimeChirho * 0.11 + aPhaseChirho * 6.28) * 0.09;
		p.y += sin(uTimeChirho * 0.07 + aPhaseChirho * 12.6) * 0.07;
		vTwinkleChirho = 0.55 + 0.45 * sin(uTimeChirho * 0.9 + aPhaseChirho * 31.4);
		vec4 mv = modelViewMatrix * vec4(p, 1.0);
		gl_PointSize = uSizeChirho * (1.0 + aPhaseChirho) / max(-mv.z, 0.1);
		gl_Position = projectionMatrix * mv;
	}
`;

const DUST_FRAG_CHIRHO = /* glsl */ `
	precision mediump float;
	varying float vTwinkleChirho;
	void main() {
		vec2 c = gl_PointCoord - 0.5;
		float d = length(c);
		float a = smoothstep(0.5, 0.05, d) * vTwinkleChirho * 0.35;
		gl_FragColor = vec4(vec3(0.9, 0.78, 0.45), a);
	}
`;

const R_OUTER_CHIRHO = 1.0;
const R_INNER_CHIRHO = 0.44;
const GAP_RAD_CHIRHO = 0.028;
// the window hangs high in the nave; the hero title sits beneath it
const WINDOW_Y_CHIRHO = 0.86;
const CAMERA_Z_CHIRHO = 5.75;
const CAMERA_Y_CHIRHO = 0.3;

function buildPaneShapeChirho(a0Chirho: number, a1Chirho: number): Shape {
	const shapeChirho = new Shape();
	shapeChirho.absarc(0, 0, R_OUTER_CHIRHO - 0.03, a0Chirho, a1Chirho, false);
	shapeChirho.absarc(0, 0, R_INNER_CHIRHO + 0.03, a1Chirho, a0Chirho, true);
	shapeChirho.closePath();
	return shapeChirho;
}

export function createRoseWindowChirho(
	canvasChirho: HTMLCanvasElement,
	optsChirho: RoseWindowOptsChirho
): RoseWindowHandleChirho {
	const rendererChirho = new WebGLRenderer({
		canvas: canvasChirho,
		antialias: true,
		alpha: true,
		powerPreference: 'high-performance'
	});
	const dprCapChirho = optsChirho.mobileChirho ? 1.0 : 1.5;
	rendererChirho.setPixelRatio(Math.min(window.devicePixelRatio, dprCapChirho));
	rendererChirho.setClearColor(0x000000, 0);

	const sceneChirho = new Scene();
	const cameraChirho = new PerspectiveCamera(38, 1, 0.05, 30);
	cameraChirho.position.set(0, CAMERA_Y_CHIRHO, CAMERA_Z_CHIRHO);

	const windowGroupChirho = new Group();
	windowGroupChirho.position.y = WINDOW_Y_CHIRHO;
	windowGroupChirho.scale.setScalar(0.86);
	sceneChirho.add(windowGroupChirho);

	// --- panes ---------------------------------------------------------------
	const paneMeshesChirho: Mesh[] = [];
	const paneMaterialsChirho: ShaderMaterial[] = [];
	const paneHoverTargetsChirho: number[] = new Array(12).fill(0);
	for (let iChirho = 0; iChirho < 12; iChirho++) {
		// clockwise from twelve o'clock, matching the craft section's miniature
		const a0Chirho = Math.PI / 2 - ((iChirho + 1) / 12) * Math.PI * 2 + GAP_RAD_CHIRHO;
		const a1Chirho = Math.PI / 2 - (iChirho / 12) * Math.PI * 2 - GAP_RAD_CHIRHO;
		const geoChirho = new ShapeGeometry(buildPaneShapeChirho(a0Chirho, a1Chirho), 28);
		const matChirho = new ShaderMaterial({
			vertexShader: PANE_VERT_CHIRHO,
			fragmentShader: PANE_FRAG_CHIRHO,
			uniforms: {
				uColorChirho: { value: new Color(optsChirho.paneHuesChirho[iChirho] ?? '#3e63c4') },
				uSunDirChirho: { value: new Vector2(0, 1) },
				uTimeChirho: { value: 0 },
				uHoverChirho: { value: 0 },
				uGlowChirho: { value: 0 },
				uKindleChirho: { value: 1 },
				uSeedChirho: { value: iChirho * 0.618 },
				uR0Chirho: { value: R_INNER_CHIRHO + 0.03 },
				uR1Chirho: { value: R_OUTER_CHIRHO - 0.03 }
			}
		});
		const meshChirho = new Mesh(geoChirho, matChirho);
		meshChirho.userData.paneIndexChirho = iChirho;
		windowGroupChirho.add(meshChirho);
		paneMeshesChirho.push(meshChirho);
		paneMaterialsChirho.push(matChirho);
	}

	// --- oculus (λ) ----------------------------------------------------------
	const oculusRChirho = R_INNER_CHIRHO - 0.02;
	const oculusGeoChirho = new CircleGeometry(oculusRChirho, 64);
	const oculusMatChirho = new ShaderMaterial({
		vertexShader: PANE_VERT_CHIRHO,
		fragmentShader: OCULUS_FRAG_CHIRHO,
		uniforms: {
			uTimeChirho: { value: 0 },
			uGlowChirho: { value: 0 },
			uProgressChirho: { value: 0 },
			uVelocityChirho: { value: 0 },
			uRChirho: { value: oculusRChirho }
		}
	});
	const oculusMeshChirho = new Mesh(oculusGeoChirho, oculusMatChirho);
	windowGroupChirho.add(oculusMeshChirho);

	// --- stone tracery: rings + spokes (flat dark, gold-limned by pane rims) --
	const stoneMatChirho = new ShaderMaterial({
		vertexShader: PANE_VERT_CHIRHO,
		fragmentShader: /* glsl */ `
			precision mediump float;
			varying vec2 vPosChirho;
			uniform float uGlowChirho;
			void main() {
				float r = length(vPosChirho);
				float shade = 0.5 + 0.5 * smoothstep(0.3, 1.05, r);
				vec3 col = mix(vec3(0.10, 0.085, 0.06), vec3(0.16, 0.13, 0.08), shade);
				col *= 1.0 + uGlowChirho * 0.8;
				gl_FragColor = vec4(col, 1.0);
			}
		`,
		uniforms: { uGlowChirho: { value: 0 } },
		side: DoubleSide
	});
	const outerRingChirho = new Mesh(new RingGeometry(R_OUTER_CHIRHO - 0.035, R_OUTER_CHIRHO + 0.05, 96, 1), stoneMatChirho);
	const innerRingChirho = new Mesh(new RingGeometry(R_INNER_CHIRHO - 0.028, R_INNER_CHIRHO + 0.035, 96, 1), stoneMatChirho);
	outerRingChirho.position.z = 0.004;
	innerRingChirho.position.z = 0.004;
	windowGroupChirho.add(outerRingChirho, innerRingChirho);
	for (let iChirho = 0; iChirho < 12; iChirho++) {
		const angChirho = (iChirho / 12) * Math.PI * 2 + Math.PI / 2;
		const spokeGeoChirho = new BufferGeometry();
		const halfWChirho = 0.016;
		const r0Chirho = R_INNER_CHIRHO;
		const r1Chirho = R_OUTER_CHIRHO;
		const dxChirho = Math.cos(angChirho);
		const dyChirho = Math.sin(angChirho);
		const pxChirho = -dyChirho * halfWChirho;
		const pyChirho = dxChirho * halfWChirho;
		const vertsChirho = new Float32Array([
			dxChirho * r0Chirho + pxChirho, dyChirho * r0Chirho + pyChirho, 0.004,
			dxChirho * r1Chirho + pxChirho, dyChirho * r1Chirho + pyChirho, 0.004,
			dxChirho * r1Chirho - pxChirho, dyChirho * r1Chirho - pyChirho, 0.004,
			dxChirho * r0Chirho + pxChirho, dyChirho * r0Chirho + pyChirho, 0.004,
			dxChirho * r1Chirho - pxChirho, dyChirho * r1Chirho - pyChirho, 0.004,
			dxChirho * r0Chirho - pxChirho, dyChirho * r0Chirho - pyChirho, 0.004
		]);
		spokeGeoChirho.setAttribute('position', new BufferAttribute(vertsChirho, 3));
		windowGroupChirho.add(new Mesh(spokeGeoChirho, stoneMatChirho));
	}

	// --- light shafts ---------------------------------------------------------
	const shaftMatsChirho: ShaderMaterial[] = [];
	const shaftAnglesChirho = [-0.42, 0, 0.42];
	for (let iChirho = 0; iChirho < shaftAnglesChirho.length; iChirho++) {
		const shaftGeoChirho = new BufferGeometry();
		const wTopChirho = 0.5;
		const wBotChirho = 1.7;
		const lenChirho = 4.6;
		const vChirho = new Float32Array([
			-wTopChirho, 0, 0, wTopChirho, 0, 0, wBotChirho, -lenChirho, 0,
			-wTopChirho, 0, 0, wBotChirho, -lenChirho, 0, -wBotChirho, -lenChirho, 0
		]);
		const uvChirho = new Float32Array([0.3, 0, 0.7, 0, 1, 1, 0.3, 0, 1, 1, 0, 1]);
		shaftGeoChirho.setAttribute('position', new BufferAttribute(vChirho, 3));
		shaftGeoChirho.setAttribute('uv', new BufferAttribute(uvChirho, 2));
		const shaftMatChirho = new ShaderMaterial({
			vertexShader: SHAFT_VERT_CHIRHO,
			fragmentShader: SHAFT_FRAG_CHIRHO,
			uniforms: {
				uTimeChirho: { value: 0 },
				uGlowChirho: { value: 0 },
				uSeedChirho: { value: iChirho * 1.71 }
			},
			transparent: true,
			blending: AdditiveBlending,
			depthWrite: false
		});
		const shaftChirho = new Mesh(shaftGeoChirho, shaftMatChirho);
		shaftChirho.position.set(0, WINDOW_Y_CHIRHO - 0.12, 0.35);
		shaftChirho.rotation.z = shaftAnglesChirho[iChirho];
		shaftChirho.rotation.x = -0.45;
		sceneChirho.add(shaftChirho);
		shaftMatsChirho.push(shaftMatChirho);
	}

	// --- gold dust ------------------------------------------------------------
	const dustCountChirho = optsChirho.mobileChirho ? 140 : 320;
	const dustGeoChirho = new BufferGeometry();
	const dustPosChirho = new Float32Array(dustCountChirho * 3);
	const dustPhaseChirho = new Float32Array(dustCountChirho);
	for (let iChirho = 0; iChirho < dustCountChirho; iChirho++) {
		const tChirho = iChirho / dustCountChirho;
		dustPosChirho[iChirho * 3] = (hash01Chirho(iChirho * 3.1) - 0.5) * 3.4;
		dustPosChirho[iChirho * 3 + 1] = (hash01Chirho(iChirho * 7.7) - 0.55) * 2.6 + WINDOW_Y_CHIRHO;
		dustPosChirho[iChirho * 3 + 2] = 0.2 + hash01Chirho(iChirho * 13.3) * 2.2;
		dustPhaseChirho[iChirho] = tChirho;
	}
	dustGeoChirho.setAttribute('position', new BufferAttribute(dustPosChirho, 3));
	dustGeoChirho.setAttribute('aPhaseChirho', new BufferAttribute(dustPhaseChirho, 1));
	const dustMatChirho = new ShaderMaterial({
		vertexShader: DUST_VERT_CHIRHO,
		fragmentShader: DUST_FRAG_CHIRHO,
		uniforms: {
			uTimeChirho: { value: 0 },
			uSizeChirho: { value: optsChirho.mobileChirho ? 5.5 : 7.5 }
		},
		transparent: true,
		blending: AdditiveBlending,
		depthWrite: false
	});
	const dustChirho = new Points(dustGeoChirho, dustMatChirho);
	sceneChirho.add(dustChirho);

	// --- state + loop ---------------------------------------------------------
	const timerChirho = new Timer();
	const raycasterChirho = new Raycaster();
	const pointerNdcChirho = new Vector2(10, 10);
	let pointerActiveChirho = false;
	let runningChirho = false;
	let disposedChirho = false;
	let scrollProgressChirho = 0;
	let lastProgressChirho = 0;
	let smoothVelocityChirho = 0;
	let hoveredChirho: number | null = null;
	let rafIdChirho = 0;
	// adaptive ladder: 0 = full, 1 = lower dpr, 2 = dust off
	let ladderChirho = 0;
	let slowFramesChirho = 0;

	function hash01Chirho(nChirho: number): number {
		const sChirho = Math.sin(nChirho * 12.9898) * 43758.5453;
		return sChirho - Math.floor(sChirho);
	}

	function resizeChirho(): void {
		const wChirho = canvasChirho.clientWidth;
		const hChirho = canvasChirho.clientHeight;
		if (wChirho === 0 || hChirho === 0) return;
		rendererChirho.setSize(wChirho, hChirho, false);
		cameraChirho.aspect = wChirho / hChirho;
		cameraChirho.updateProjectionMatrix();
	}

	function frameChirho(): void {
		if (disposedChirho || !runningChirho) return;
		rafIdChirho = requestAnimationFrame(frameChirho);
		timerChirho.update();
		const dtChirho = timerChirho.getDelta();
		const tChirho = timerChirho.getElapsed();

		// adaptive degrade ladder — sustained slow frames step it down
		if (dtChirho > 0.022 && dtChirho < 0.5) {
			slowFramesChirho++;
			if (slowFramesChirho > 90 && ladderChirho === 0) {
				ladderChirho = 1;
				rendererChirho.setPixelRatio(1);
				slowFramesChirho = 0;
			} else if (slowFramesChirho > 90 && ladderChirho === 1) {
				ladderChirho = 2;
				dustChirho.visible = false;
				slowFramesChirho = 0;
			}
		} else if (dtChirho <= 0.022) {
			slowFramesChirho = Math.max(0, slowFramesChirho - 2);
		}

		// smoothed dolly velocity — drives the warp-streak length
		const instVelChirho = Math.min(
			Math.abs(scrollProgressChirho - lastProgressChirho) / Math.max(dtChirho, 1e-3),
			2.5
		);
		lastProgressChirho = scrollProgressChirho;
		smoothVelocityChirho += (instVelChirho - smoothVelocityChirho) * 0.12;

		updateUniformsChirho(tChirho);

		// pointer parallax
		const targetRxChirho = pointerActiveChirho ? pointerNdcChirho.y * 0.05 : 0;
		const targetRyChirho = pointerActiveChirho ? pointerNdcChirho.x * 0.07 : 0;
		windowGroupChirho.rotation.x += (targetRxChirho - windowGroupChirho.rotation.x) * 0.04;
		windowGroupChirho.rotation.y += (targetRyChirho - windowGroupChirho.rotation.y) * 0.04;

		// dolly through the oculus as the visitor scrolls
		const pChirho = scrollProgressChirho;
		const easedChirho = pChirho * pChirho * (3 - 2 * pChirho);
		cameraChirho.position.z = CAMERA_Z_CHIRHO - easedChirho * (CAMERA_Z_CHIRHO - 0.32);
		cameraChirho.position.y =
			CAMERA_Y_CHIRHO + easedChirho * (WINDOW_Y_CHIRHO - CAMERA_Y_CHIRHO);
		cameraChirho.lookAt(0, CAMERA_Y_CHIRHO + easedChirho * (WINDOW_Y_CHIRHO - CAMERA_Y_CHIRHO), 0);

		// hover raycast (skip while dollying — panes fly past the cursor)
		if (pointerActiveChirho && pChirho < 0.12) {
			raycasterChirho.setFromCamera(pointerNdcChirho, cameraChirho);
			const hitsChirho = raycasterChirho.intersectObjects(paneMeshesChirho, false);
			const idxChirho = hitsChirho.length
				? (hitsChirho[0].object.userData.paneIndexChirho as number)
				: null;
			if (idxChirho !== hoveredChirho) {
				hoveredChirho = idxChirho;
				emitHoverChirho();
			}
		} else if (hoveredChirho !== null) {
			hoveredChirho = null;
			emitHoverChirho();
		}
		for (let iChirho = 0; iChirho < 12; iChirho++) {
			paneHoverTargetsChirho[iChirho] = hoveredChirho === iChirho ? 1 : 0;
			const uChirho = paneMaterialsChirho[iChirho].uniforms.uHoverChirho;
			uChirho.value += (paneHoverTargetsChirho[iChirho] - uChirho.value) * 0.08;
		}

		rendererChirho.render(sceneChirho, cameraChirho);
	}

	function emitHoverChirho(): void {
		if (hoveredChirho === null) {
			optsChirho.onPaneHoverChirho(null, 0, 0);
			return;
		}
		const angChirho = Math.PI / 2 - ((hoveredChirho + 0.5) / 12) * Math.PI * 2;
		const midRChirho = (R_INNER_CHIRHO + R_OUTER_CHIRHO) / 2;
		const worldChirho = new Vector3(Math.cos(angChirho) * midRChirho, Math.sin(angChirho) * midRChirho, 0);
		worldChirho.applyMatrix4(windowGroupChirho.matrixWorld);
		worldChirho.project(cameraChirho);
		const xChirho = (worldChirho.x * 0.5 + 0.5) * canvasChirho.clientWidth;
		const yChirho = (-worldChirho.y * 0.5 + 0.5) * canvasChirho.clientHeight;
		optsChirho.onPaneHoverChirho(hoveredChirho, xChirho, yChirho);
	}

	function updateUniformsChirho(tChirho: number): void {
		const sunAngChirho = Math.PI * 0.5 + Math.sin(tChirho * 0.05) * 0.75;
		const sunChirho = new Vector2(Math.cos(sunAngChirho), Math.sin(sunAngChirho));
		const glowChirho = Math.min(scrollProgressChirho * 1.9, 1.25);
		for (let iChirho = 0; iChirho < paneMaterialsChirho.length; iChirho++) {
			const matChirho = paneMaterialsChirho[iChirho];
			matChirho.uniforms.uTimeChirho.value = tChirho;
			matChirho.uniforms.uSunDirChirho.value.copy(sunChirho);
			matChirho.uniforms.uGlowChirho.value = glowChirho;
			// kindle in pipeline order over the first moments
			matChirho.uniforms.uKindleChirho.value = optsChirho.staticChirho
				? 1
				: Math.min(Math.max((tChirho - 0.35 - iChirho * 0.16) / 1.1, 0), 1);
		}
		oculusMatChirho.uniforms.uTimeChirho.value = tChirho;
		oculusMatChirho.uniforms.uGlowChirho.value = glowChirho;
		oculusMatChirho.uniforms.uProgressChirho.value = scrollProgressChirho;
		oculusMatChirho.uniforms.uVelocityChirho.value = smoothVelocityChirho;
		stoneMatChirho.uniforms.uGlowChirho.value = glowChirho;
		for (const matChirho of shaftMatsChirho) {
			matChirho.uniforms.uTimeChirho.value = tChirho;
			matChirho.uniforms.uGlowChirho.value = 1 - scrollProgressChirho * 0.8;
		}
		dustMatChirho.uniforms.uTimeChirho.value = tChirho;
	}

	function renderStaticChirho(): void {
		resizeChirho();
		updateUniformsChirho(2.4);
		rendererChirho.render(sceneChirho, cameraChirho);
	}

	function onContextLostChirho(evChirho: Event): void {
		evChirho.preventDefault();
		optsChirho.onContextLostChirho();
	}
	canvasChirho.addEventListener('webglcontextlost', onContextLostChirho);

	resizeChirho();
	if (optsChirho.staticChirho) {
		renderStaticChirho();
	}

	return {
		setRunningChirho(runChirho: boolean): void {
			if (disposedChirho || optsChirho.staticChirho) return;
			if (runChirho && !runningChirho) {
				runningChirho = true;
				timerChirho.update();
				rafIdChirho = requestAnimationFrame(frameChirho);
			} else if (!runChirho) {
				runningChirho = false;
				cancelAnimationFrame(rafIdChirho);
			}
		},
		setScrollProgressChirho(pChirho: number): void {
			scrollProgressChirho = Math.min(Math.max(pChirho, 0), 1);
			if (optsChirho.staticChirho) renderStaticChirho();
		},
		setPointerChirho(xChirho: number, yChirho: number): void {
			pointerActiveChirho = xChirho > -2;
			pointerNdcChirho.set(xChirho, yChirho);
		},
		resizeChirho,
		disposeChirho(): void {
			disposedChirho = true;
			runningChirho = false;
			cancelAnimationFrame(rafIdChirho);
			canvasChirho.removeEventListener('webglcontextlost', onContextLostChirho);
			for (const mChirho of paneMeshesChirho) mChirho.geometry.dispose();
			for (const mChirho of paneMaterialsChirho) mChirho.dispose();
			oculusGeoChirho.dispose();
			oculusMatChirho.dispose();
			stoneMatChirho.dispose();
			for (const mChirho of shaftMatsChirho) mChirho.dispose();
			dustGeoChirho.dispose();
			dustMatChirho.dispose();
			rendererChirho.dispose();
		}
	};
}
