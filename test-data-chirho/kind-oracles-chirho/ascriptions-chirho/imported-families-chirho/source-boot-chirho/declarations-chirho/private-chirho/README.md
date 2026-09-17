<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Private boot declarations and original export names

Claude independently ran the unchanged runners against GHC9.14.1 on2026-09-17.
Each JSONL record carries complete source files, predictions, command, exit and
diagnostic output. The original runners and observations are preserved together;
the temporary directories in their records are provenance, not dependencies.

The nineteen private-declaration predictions include one refutation: privacy
exempts an absent declaration from existence, not a present declaration from
agreement. Boot-exported names must be exported by the implementation. Private
declarations remain unavailable to importers and must be well formed. A private
type mentioned by an exported signature still affects signature agreement.

The driver integration module `boot_private_chirho.rs` exercises the same source
bytes through file check and file compile. Before this repair12/19 tests passed;
afterwards19/19 passed. Three formerly valid cases were rejected, and four
negative controls previously failed their reason assertion. No assertion was
weakened to obtain the result.

The separate seven re-export references establish that identity includes the
original defining module, not merely export spelling. Same-origin re-exports
need no local declaration; replacing them with a same-spelled local or foreign
type is rejected by GHC-91999. These observations are **not a completed candidate
implementation**. Required defining-module provenance on interface export entries
is the planned owner; spans and importing aliases are not substitutes. Package
identity and ambiguous same-spelled exports remain separate work.
