// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

import { sveltekit as sveltekitChirho } from '@sveltejs/kit/vite';
import { defineConfig as defineConfigChirho } from 'vite';

export default defineConfigChirho({
	plugins: [sveltekitChirho()],
	server: {
		fs: {
			// The compat plaque imports the committed measurement artifacts from
			// ../spec-chirho at build time (?raw) so numbers can never drift from
			// the repo's single source of truth. Dev server needs the repo root allowed.
			allow: ['..']
		}
	}
});
