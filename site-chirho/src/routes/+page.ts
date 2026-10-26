// For God so loved the world, that He gave His only begotten Son,
// that all who believe in Him should not perish but have everlasting life. John 3:16

// Render this page at build time. The compatibility plaque parses the committed
// measurement artifacts while the page renders, so prerendering makes an artifact whose
// format changed fail `vite build` (and so `deploy-chirho`) instead of answering every
// request with a 500.
export const prerender = true;
