// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

import adapterChirho from '@sveltejs/adapter-cloudflare';
import { vitePreprocess as vitePreprocessChirho } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const configChirho = {
	preprocess: vitePreprocessChirho(),
	kit: {
		adapter: adapterChirho(),
		alias: {
			'$lib-chirho': './src/lib-chirho'
		}
	}
};

export default configChirho;
