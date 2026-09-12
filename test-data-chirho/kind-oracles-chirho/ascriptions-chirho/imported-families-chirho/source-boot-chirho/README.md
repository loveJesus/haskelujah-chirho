<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# Checked SOURCE boot contracts

Intermediate owned checkpoint after df1529fa, not a main landing or a complete boot implementation.

- observations-chirho.jsonl: metadata plus25 exact source cases, GHC9.14.1 reference and fresh candidate/frozen5f CLI observations. The first11 reference records came from the earlier reference-only run; the other14 were run with this comparison. All25 final candidate verdicts agree. The executable hashes were checked unchanged before and after the observations and focused corpus checks.
- focused-corpus-chirho.json:18 named files rejected for source cycles at5f;13 now accept. T6018a,Tc267a,Tc267b,Tc271,Tc271a still reject unsupported abstract closed-family, instance or class agreements. This is not a new corpus total.
- gates-chirho.json: changed-source manifest and nine gate records. Deleted source paths have null hashes. Test counts distinguish full targets from the11 focused driver lookup tests. Clippy exits0 but has454 warning lines, including duplicates and summaries.
- audit-before-chirho.json: review probes on the pre-audit dirty boot implementation. It wrongly accepted a missing implementation and a non-exported implementation value. These records do not claim an independently retained executable hash; final observations include both repairs and valid counterparts.
- reference-disagreement-chirho.jsonl: the first proposed positive source disagreed with GHC because its abstract binder was poly-kinded. The final positive states Type explicitly; the polymorphic mismatch remains a negative.

Commands have bounded children and do not treat nonzero crashes/timeouts as rejection evidence. File check/compile integration controls live in typing_integration_chirho/import_contracts_chirho/boot_chirho.rs; CLI records are separate from those in-process results. No upstream corpus inputs or public measurement artifacts were changed.
