<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Interface file decomposition Chirho

Row 648 (gpt_chirho #30337; placement approved in #30349) is a behavior-neutral split of the
18,784-line `iface_chirho.rs`. It is the prerequisite for row 642 and carries none of row 642's
provenance semantics.

## Bounds (#30337, #30349)

- Option A: regroup the independent statements by family. Keep every module's content exactly,
  including the relative order of its copies, constructors, methods, first-wins values,
  associated types, completeness marks and builtin origin. There is no historical-order table.
- The vector's order is incidental. It becomes the family order, and the docs say that consumers
  address modules by name.
- The verifier first asserts 518 entries and 518 unique names on both sides. It then compares the
  name set and the content keyed by name, and reports the permutation.
- The tests split as proposed. All 58 names and bodies are kept, the path mapping is recorded, and
  `mk_name_chirho` gets the narrowest visibility that compiles.
- Landing order: the lint-only preparation, then the move, then this record.

## Checklist

- [x] Trace: the prologue, 438 independent push-only statements and the epilogue; the merge's
  order semantics; every reader of the merged vector; the reversed-order probe.
- [x] Placement checkpoint (#30346), approved in #30349 (option A, tests split).
- [x] Preparation, lint-only: Clippy's machine-applicable `single_element_loop` rewrite at its
  33 sites. The canonical dump is byte-identical to main's.
- [x] Move: 25 family files in four directories plus a root, each under 1,000 lines. The
  15 export-filter tests keep their path, and the 43 builtin-table tests move.
  `mk_name_chirho` is `pub(super)`. Four detached comment groups (5 lines) describing removed
  code or the old batch order are dropped.
- [x] Statement verifier:
  - all 438 statements are verbatim, and each family body is exactly its statements in order;
  - the root's moved text is verbatim, and the helpers are token-identical to the closures;
  - 56 tests are token-identical, and 2 differ only by trailing commas that rustfmt re-wraps
    after the dedent;
  - mutation check: 4 of 4 mutants refused for their own reason.
- [x] Interface verifier:
  - 518 entries and 518 unique names on both sides, equal name sets, and identical content
    keyed by name;
  - the permutation is reported: the longest run kept in main's order is 128;
  - mutation check: 9 of 9 data mutants refused for their own reason.
- [x] Replayed onto 9421c5d9 with both patch-ids unchanged.
- [x] Gate on the replayed tip, ef48e618:
  - the workspace check is warning-free;
  - naming 156, typing_integration 74 (2 ignored), and the native-symbol target 29;
  - changed-line Clippy is 0 over 18,732 lines in 34 files;
  - the driver library's 345 project, re-export and Hackage tests pass in 566 s, the same 345
    names row 621 passed;
  - the corpus sweep moves 0 of 4,811 files: 3,895 accepted and 916 rejected under both main's
    CLI and the tip's.

## Test paths, old to new

Every test was at `iface_chirho::tests_chirho::<name>` before row 648.

- `iface_chirho::tests_chirho::*` (15 tests, unchanged):
- `iface_chirho::builtin_chirho::tests_chirho::*` (27 tests):
  - `builtin_dedup_data_map_chirho`
  - `builtin_dedup_foreign_storable_chirho`
  - `builtin_foreign_reexports_peek_elem_off_chirho`
  - `builtin_control_monad_class_methods_exported_chirho`
  - `builtin_data_foldable_class_methods_exported_chirho`
  - `builtin_prelude_exports_alternative_helpers_chirho`
  - `builtin_control_applicative_const_dotdot_imports_getconst_chirho`
  - `builtin_transformer_accessors_import_with_dotdot_chirho`
  - `builtin_collection_and_arrow_helpers_exported_chirho`
  - `builtin_bytestring_internal_and_system_io_unsafe_exports_effect_helpers_chirho`
  - `builtin_foreign_ptr_exports_unsafe_with_foreign_ptr_chirho`
  - `builtin_control_monad_st_lazy_exports_conversion_helpers_chirho`
  - `builtin_control_monad_combinators_exports_core_reexports_chirho`
  - `builtin_test_tasty_exports_test_tree_helpers_chirho`
  - `builtin_test_tasty_options_exports_is_option_class_chirho`
  - `builtin_text_read_exports_parens_and_readprec_method_chirho`
  - `builtin_readp_exports_triple_plus_chirho`
  - `builtin_system_mem_weak_reexports_ghc_weak_surface_chirho`
  - `builtin_data_text_internal_exports_text_constructor_chirho`
  - `builtin_data_text_lazy_exports_foldr_chunks_chirho`
  - `builtin_warp_support_modules_export_expected_names_chirho`
  - `builtin_data_functor_compose_exports_getcompose_via_type_chirho`
  - `builtin_word_modules_export_byteswap_helpers_chirho`
  - `builtin_with_index_instances_modules_exist_chirho`
  - `builtin_semigroup_traversable_exports_traverse1_chirho`
  - `builtin_splitmix32_iface_exists_chirho`
  - `builtin_text_parser_combinators_parsec_exports_parser_values_chirho`
- `iface_chirho::builtin_chirho::ghc_chirho::tests_chirho::*` (16 tests):
  - `builtin_ghc_typelits_exports_symbol_predicates_chirho`
  - `builtin_ghc_exts_exports_aligned_byte_array_primop_chirho`
  - `builtin_ghc_exts_exports_seq_hash_primop_chirho`
  - `builtin_ghc_exts_exports_ishiftra_hash_primop_chirho`
  - `builtin_ghc_exts_exports_runrw_hash_primop_chirho`
  - `builtin_ghc_prim_exports_mutable_array_primops_chirho`
  - `builtin_ghc_base_exports_unsafe_chr_chirho`
  - `builtin_ghc_base_exports_mkweak_primop_chirho`
  - `builtin_ghc_exts_exports_extra_mvar_primops_chirho`
  - `builtin_ghc_exts_exports_mutvar_primops_chirho`
  - `builtin_ghc_integer_gmp_internals_exports_integer_constructors_chirho`
  - `builtin_ghc_base_exports_primitive_char_ops_chirho`
  - `builtin_ghc_cstring_exports_unpackers_chirho`
  - `builtin_ghc_byteorder_exports_type_and_value_chirho`
  - `builtin_ghc_ioref_exports_constructor_value_chirho`
  - `builtin_ghc_stref_exports_constructor_value_chirho`
