<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Workflow — Website showcase build (Illuminated Compiler)

How the 2026-07-27 site-chirho rebuild flowed, as a repeatable DAG. Companion prose guide ships
on the site itself at `/haskelujah-website-guide-chirho.md`
(source: `site-chirho/static/haskelujah-website-guide-chirho.md`).

```mermaid
flowchart TD
    A[L.J. direction #8080-#8082] --> B[Claim lane in room\nzero-cargo discipline]
    B --> C[Recon: live deploy root,\nold-site drift audit]
    C --> D1[Research agent:\nSOTA exemplars + palette]
    C --> D2[Asset-API agent:\nRunware/OpenAI recipes verified]
    C --> E[Truth pass: measurement artifacts,\nCLI source, stats script]
    E --> F[Brick 1: concept + stack locked\nIlluminated Compiler / SvelteKit+three]
    D1 --> F
    F --> G1[Design tokens + fonts\nself-hosted OFL]
    F --> G2[Data layer: compat-chirho.ts\nbuild-time artifact parser]
    G1 --> H[Components: hero/plaque/story/\ncanon/scriptorium/begin/confessio/\nroadmap/colophon]
    G2 --> H
    H --> I[Rose-window WebGL module\nkindle + dolly + guardrails]
    I --> J[Gate 1: svelte-check --fail-on-warnings\n+ production build]
    J --> K1[Iteration pass 1\nPlaywright comb: layout, console,\nsections, mobile, hover]
    K1 --> K2[Iteration pass 2\nkindle sequence, seals, glow cap,\noverflow fix, reduced-motion +\nkeyboard verification]
    K2 --> K3[Iteration pass 3\nprod build sweep: payload, og,\nfinal screenshots]
    K3 --> L[Commit site-chirho paths only]
    L --> M[Deploy: wrangler pages\n+ live-curl verification]
    M --> N[Room announcement + AICEO report\n+ progress DB close]
    D2 -.->|asset recipes held for\nfuture art passes| K2
```

Key invariants (see the on-site guide for the full method): numbers parse from committed
artifacts at build time and the build fails on drift; no claims that don't trace to repo
source; ≥3 fine-toothed iteration passes before "ok"; zero cargo while another agent owns
the builder; commit only explicitly named paths.
