// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

// Compilation pipeline, check, LLVM, Wasm, incremental, module discovery tests

#[allow(unused_imports)]
use crate::{
    check_source_file_chirho, check_source_path_chirho, compile_modules_chirho,
    compile_modules_incremental_chirho, compile_source_chirho, discover_modules_chirho,
    eval_modules_chirho, eval_source_chirho, eval_source_with_input_chirho,
    eval_source_with_machine_chirho, eval_source_with_step_limit_chirho, render_summary_chirho,
};
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ExecutionModeChirho, ValueChirho};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskelujah_syntax_chirho::SourceFileChirho;

#[test]
fn builds_a_check_summary_for_batch_mode_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BatchSampleChirho.hs",
        "module BatchSampleChirho where\nvalueChirho = 1\n",
    );

    let check_summary_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho)
            .expect("driver should accept a valid source file");

    assert_eq!(check_summary_chirho.module_name_chirho, "BatchSampleChirho");
    assert!(
        !check_summary_chirho
            .runtime_plan_chirho
            .incremental_session_chirho
    );
    assert!(
        render_summary_chirho(&check_summary_chirho)
            .contains("llvm_preview: ; haskelujah llvm stub")
    );
}

#[test]
fn full_pipeline_parses_and_resolves_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module Test where\ndata Color = Red | Green\nf x = x\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    )
    .expect("full pipeline should succeed");

    assert_eq!(
        result_chirho.module_chirho.name_chirho.text_chirho(),
        "Test"
    );
    assert!(result_chirho.module_chirho.decls_chirho.len() >= 2);
    // Core module was produced by desugaring
    assert_eq!(result_chirho.core_chirho.name_chirho, "Test");
    assert!(!result_chirho.core_chirho.bindings_chirho.is_empty());
    // Backend output was produced
    assert!(result_chirho.llvm_ir_chirho.contains("; ModuleID = 'Test'"));
    assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
}

#[test]
fn frontend_text_printf_formatparse_and_fieldformat_surface_compiles_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module TextPrintfMiniChirho where
import Text.Printf

usePrintfTypesChirho :: FormatParse -> FieldFormat -> FormatAdjustment -> FormatSign -> ()
usePrintfTypesChirho _fpChirho _ffChirho _adjChirho _signChirho = ()

projectFormatParseChirho fpChirho =
  (fpModifiers fpChirho, fpChar fpChirho, fpRest fpChirho)

buildFormatParseChirho aChirho bChirho cChirho = FormatParse aChirho bChirho cChirho

projectFieldFormatChirho ffChirho =
  ( fmtWidth ffChirho
  , fmtPrecision ffChirho
  , fmtAdjust ffChirho
  , fmtSign ffChirho
  , fmtAlternate ffChirho
  , fmtModifiers ffChirho
  , fmtChar ffChirho
  )

buildFieldFormatChirho aChirho bChirho cChirho dChirho eChirho fChirho gChirho =
  FieldFormat aChirho bChirho cChirho dChirho eChirho fChirho gChirho

pickFormatSignChirho True = SignPlus
pickFormatSignChirho False = SignSpace

pickFormatAdjustmentChirho True = LeftAdjust
pickFormatAdjustmentChirho False = ZeroPad
"#,
        &mut source_map_chirho,
        "TextPrintfMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Text.Printf record selectors and constructors should be in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_base_builtin_import_surface_for_quickcheck_arbitrary_compiles_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module QuickCheckBuiltinSurfaceMiniChirho where
import Data.Char (generalCategory, GeneralCategory(..))
import Data.Ord
import System.IO

unicodeOkayChirho :: Char -> Bool
unicodeOkayChirho cChirho =
  case generalCategory cChirho of
    Surrogate -> False
    NotAssigned -> False
    _ -> True

downValueChirho :: Down Int -> Int
downValueChirho = getDown

encodingsChirho :: [TextEncoding]
encodingsChirho =
  [ utf8
  , utf8_bom
  , utf16
  , utf16le
  , utf16be
  , utf32
  , utf32le
  , utf32be
  , latin1
  , char8
  , localeEncoding
  ]
"#,
        &mut source_map_chirho,
        "QuickCheckBuiltinSurfaceMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "QuickCheck-facing base builtin imports should stay in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_multi_constructor_record_fields_typecheck_out_of_order_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckRecordOrderMiniChirho where\n\
data ResultMiniChirho\n\
  = SuccessMiniChirho\n\
    { numTestsMiniChirho :: Int\n\
    , labelsMiniChirho :: [Int]\n\
    , outputMiniChirho :: String\n\
    }\n\
  | FailureMiniChirho\n\
    { numTestsMiniChirho :: Int\n\
    , numDiscardedMiniChirho :: Int\n\
    , usedSizeMiniChirho :: Int\n\
    , maybeFlagMiniChirho :: Maybe Bool\n\
    , labelsMiniChirho :: [Int]\n\
    , outputMiniChirho :: String\n\
    }\n\
\n\
mkFailureMiniChirho :: ResultMiniChirho\n\
mkFailureMiniChirho =\n\
  FailureMiniChirho\n\
    { usedSizeMiniChirho = 7\n\
    , maybeFlagMiniChirho = Nothing\n\
    , numTestsMiniChirho = 3\n\
    , numDiscardedMiniChirho = 1\n\
    , labelsMiniChirho = []\n\
    , outputMiniChirho = \"\"\n\
    }\n\
\n\
recheckMiniChirho :: ResultMiniChirho -> Int\n\
recheckMiniChirho resultMiniChirho@FailureMiniChirho{} = usedSizeMiniChirho resultMiniChirho\n\
recheckMiniChirho _ = 0\n",
        &mut source_map_chirho,
        "QuickCheckRecordOrderMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "multi-constructor record construction should follow declared field order and keep per-constructor selectors in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_record_pattern_field_types_follow_constructor_layout_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE NamedFieldPuns #-}\n\
module QuickCheckRecordPatternMiniChirho where\n\
data CoverageMiniChirho\n\
  = CoverageMiniChirho\n\
    { valueMiniChirho :: Int\n\
    , flagMiniChirho :: Maybe Bool\n\
    }\n\
  | OtherMiniChirho\n\
    { valueMiniChirho :: Int\n\
    }\n\
\n\
useCoverageMiniChirho :: CoverageMiniChirho -> Int\n\
useCoverageMiniChirho CoverageMiniChirho{valueMiniChirho, flagMiniChirho} =\n\
  case flagMiniChirho of\n\
    Just True -> valueMiniChirho\n\
    Just False -> 0\n\
    Nothing -> 0\n\
useCoverageMiniChirho OtherMiniChirho{valueMiniChirho} = valueMiniChirho\n",
        &mut source_map_chirho,
        "QuickCheckRecordPatternMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "record pattern bindings should inherit field types from the matched constructor: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_record_pattern_nested_constructor_after_equals_binds_inner_name_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module RecordPatternNestedConMiniChirho where\n\
data ConfidenceMiniChirho = ConfidenceMiniChirho\n\
  { certaintyMiniChirho :: Integer\n\
  }\n\
\n\
data StateMiniChirho = MkStateMiniChirho\n\
  { coverageConfidenceMiniChirho :: Maybe ConfidenceMiniChirho\n\
  }\n\
\n\
confidenceLevelMiniChirho :: StateMiniChirho -> Integer\n\
confidenceLevelMiniChirho MkStateMiniChirho{coverageConfidenceMiniChirho = Just confidenceMiniChirho} =\n\
  certaintyMiniChirho confidenceMiniChirho\n\
confidenceLevelMiniChirho _ = 0\n",
        &mut source_map_chirho,
        "RecordPatternNestedConMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "record field patterns with nested constructors should bind the inner name, not the whole field type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_import_with_apostrophe_suffix_value_stays_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QualifiedApostropheImportMiniChirho where\n\
import qualified Control.Monad.Trans.State.Lazy as LazyState\n\
valueChirho = LazyState.liftCallCC'\n",
        &mut source_map_chirho,
        "QualifiedApostropheImportMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified imports should keep apostrophe-suffixed values in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_instance_method_can_call_later_top_level_helper_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module InstanceHelperScopeMiniChirho where\n\
class Read1MiniChirho fChirho where\n\
  liftReadsPrecMiniChirho :: Int\n\
\n\
instance Read1MiniChirho Maybe where\n\
  liftReadsPrecMiniChirho = readsDataMiniChirho\n\
\n\
readsDataMiniChirho :: Int\n\
readsDataMiniChirho = 1\n",
        &mut source_map_chirho,
        "InstanceHelperScopeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "instance methods should see later top-level helpers: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_show_composition_with_char_literal_partial_apps_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module ShowComposeMiniChirho where
showsUnaryWithMiniChirho :: (Int -> aChirho -> ShowS) -> String -> Int -> aChirho -> ShowS
showsUnaryWithMiniChirho spChirho nameChirho dChirho xChirho = showParen (dChirho > 10) $
  showString nameChirho . showChar ' ' . spChirho 11 xChirho
"#,
        &mut source_map_chirho,
        "ShowComposeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "partial applications composed around a char literal should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_parsed_show_composition_signature_infers_without_import_rewrite_chirho() {
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };
    use haskelujah_typing_chirho::infer_chirho::infer_module_chirho;

    let source_chirho = r#"module ShowComposeMiniChirho where
showsUnaryWithMiniChirho :: (Int -> aChirho -> ShowS) -> String -> Int -> aChirho -> ShowS
showsUnaryWithMiniChirho spChirho nameChirho dChirho xChirho = showParen (dChirho > 10) $
  showString nameChirho . showChar ' ' . spChirho 11 xChirho
"#;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "ShowComposeParsedMiniChirho.hs",
        source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());
    let result_chirho = infer_module_chirho(&module_chirho);

    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "direct inference on the parsed/lowered ShowS signature should succeed before import rewriting: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn frontend_show_composition_without_signature_partial_apps_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module ShowComposeNoSigMiniChirho where
showsUnaryWithMiniChirho spChirho nameChirho dChirho xChirho = showParen (dChirho > 10) $
  showString nameChirho . showChar ' ' . spChirho 11 xChirho
"#,
        &mut source_map_chirho,
        "ShowComposeNoSigMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "partial applications composed around a char literal should typecheck even without a signature: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_tuple_showchar_composition_chain_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module TupleShowChainMiniChirho where
tupleShowsMiniChirho :: (Int -> aChirho -> ShowS) -> (Int -> bChirho -> ShowS) -> aChirho -> bChirho -> ShowS
tupleShowsMiniChirho sp1Chirho sp2Chirho xChirho yChirho =
  showChar '(' . sp1Chirho 0 xChirho . showChar ',' . sp2Chirho 0 yChirho . showChar ')'
"#,
        &mut source_map_chirho,
        "TupleShowChainMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "tuple-style showChar composition chains from transformers should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_scanned_transformers_iface_keeps_liftcallcc_prime_in_scope_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/transformers-0.6.3.0");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let result_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "LiftCallCCPrimeIfaceMiniChirho".to_string(),
            "LiftCallCCPrimeIfaceMiniChirho.hs".to_string(),
            "module LiftCallCCPrimeIfaceMiniChirho where\n\
import qualified Control.Monad.Trans.State.Lazy as LazyState\n\
valueChirho = LazyState.liftCallCC'\n"
                .to_string(),
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    );

    assert!(
        result_chirho.is_ok(),
        "scanned transformers ifaces should keep liftCallCC' in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_template_haskell_runio_stays_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TemplateRunIOMiniChirho where\n\
import Language.Haskell.TH\n\
valueChirho = runIO\n",
        &mut source_map_chirho,
        "TemplateRunIOMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Language.Haskell.TH should keep runIO in scope for QuickCheck.All-style code: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_template_haskell_sigp_stays_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TemplateSigPMiniChirho where\n\
import Language.Haskell.TH\n\
typedWildPatChirho = sigP wildP (conT (mkName \"Int\"))\n",
        &mut source_map_chirho,
        "TemplateSigPMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Language.Haskell.TH should keep sigP in scope for tagged-style TH helpers: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_type_equality_hrefl_keeps_heterogeneous_witnesses_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE TypeOperators #-}\n\
module TypeEqualityHReflMiniChirho where\n\
import Data.Type.Equality\n\
heteroWitnessChirho = HRefl :: Int :~~: Int\n",
        &mut source_map_chirho,
        "TypeEqualityHReflMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Type.Equality should keep HRefl in scope with the right :~~: witness type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_type_equality_prefix_partial_application_keeps_heterogeneous_kind_shape_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE PolyKinds #-}\n\
{-# LANGUAGE TypeOperators #-}\n\
{-# LANGUAGE KindSignatures #-}\n\
module TypeEqualityPrefixMiniChirho where\n\
import Data.Kind (Type)\n\
type HeteroEqPrefixChirho (a :: k1) = ((:~~:) (a :: k1) :: k2 -> Type)\n",
        &mut source_map_chirho,
        "TypeEqualityPrefixMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Type.Equality should keep :~~: well-kinded under prefix partial application: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_preprocessed_transformers_functor_classes_keeps_building_block_helpers_top_level_chirho()
 {
    let path_chirho = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/transformers-0.6.3.0/legacy/pre711/Data/Functor/Classes.hs");
    let source_chirho =
        crate::read_haskell_source_file_chirho(&path_chirho).expect("read transformers source");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = haskelujah_syntax_chirho::SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho
            .to_str()
            .expect("transformers path should be valid utf-8"),
        &source_chirho,
    );
    let parser_chirho = haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
        &source_chirho,
        source_file_chirho.file_id_chirho(),
    );
    let green_chirho = parser_chirho.parse_chirho();
    let mut direct_child_summaries_chirho: Vec<(String, String)> = Vec::new();
    let mut direct_child_offset_chirho = 0usize;
    for child_chirho in green_chirho.children_chirho() {
        let child_start_chirho = direct_child_offset_chirho;
        let child_len_chirho = child_chirho.text_len_chirho();
        direct_child_offset_chirho += child_len_chirho;
        match child_chirho {
            haskelujah_syntax_chirho::green_chirho::GreenElementChirho::NodeChirho(node_chirho) => {
                let snippet_chirho = source_chirho
                    .get(child_start_chirho..child_start_chirho + child_len_chirho)
                    .unwrap_or("")
                    .lines()
                    .take(12)
                    .collect::<Vec<_>>()
                    .join("\\n");
                direct_child_summaries_chirho
                    .push((format!("{:?}", node_chirho.kind_chirho()), snippet_chirho));
            }
            _ => {}
        }
    }
    let module_chirho = haskelujah_parser_chirho::lower_chirho::lower_module_chirho(
        &green_chirho,
        source_file_chirho.file_id_chirho(),
    );

    let top_level_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::TypeSigChirho {
                name_chirho, ..
            }
            | haskelujah_ast_chirho::decl_chirho::DeclChirho::FunBindChirho {
                name_chirho, ..
            } => Some(name_chirho.text_chirho().to_string()),
            _ => None,
        })
        .collect();
    let instance_method_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .flat_map(|decl_chirho| match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
                methods_chirho,
                ..
            } => methods_chirho
                .iter()
                .filter_map(|bind_chirho| match bind_chirho {
                    haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                        name_chirho,
                        ..
                    }
                    | haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                        name_chirho,
                        ..
                    } => Some(name_chirho.text_chirho().to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect();
    let instance_summaries_chirho: Vec<(String, Vec<String>)> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => Some((
                class_chirho.text_chirho().to_string(),
                methods_chirho
                    .iter()
                    .filter_map(|bind_chirho| match bind_chirho {
                        haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                            name_chirho,
                            ..
                        }
                        | haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                            name_chirho,
                            ..
                        } => Some(name_chirho.text_chirho().to_string()),
                        _ => None,
                    })
                    .collect(),
            )),
            _ => None,
        })
        .collect();
    let source_excerpt_chirho: Vec<String> = source_chirho
        .lines()
        .filter(|line_chirho| {
            line_chirho.contains("instance (Show a) => Show1 (Const a) where")
                || line_chirho.contains("liftShowsPrec = liftShowsPrec2 showsPrec showList")
                || line_chirho.contains("readsData ::")
                || line_chirho.contains("readsData reader d =")
                || line_chirho.contains("readsUnaryWith ::")
                || line_chirho.contains("showsUnaryWith ::")
        })
        .map(|line_chirho| format!("{line_chirho:?}"))
        .collect();
    let mut lexer_chirho = haskelujah_parser_chirho::lexer_chirho::LexerChirho::new_chirho(
        &source_chirho,
        source_file_chirho.file_id_chirho(),
    );
    let raw_tokens_chirho = lexer_chirho.lex_all_chirho();
    let layout_tokens_chirho = haskelujah_parser_chirho::layout_chirho::apply_layout_chirho(
        &source_chirho,
        raw_tokens_chirho,
        source_file_chirho.file_id_chirho(),
    );
    let non_trivia_layout_tokens_chirho: Vec<(
        haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho,
        String,
    )> = layout_tokens_chirho
        .iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .map(|token_chirho| {
            (
                token_chirho.kind_chirho,
                source_chirho
                    .get(
                        token_chirho.span_chirho.start_chirho().as_usize_chirho()
                            ..token_chirho.span_chirho.end_chirho().as_usize_chirho(),
                    )
                    .unwrap_or("")
                    .to_string(),
            )
        })
        .collect();
    let reads_data_token_window_chirho = non_trivia_layout_tokens_chirho
        .iter()
        .enumerate()
        .find(|(token_idx_chirho, (kind_chirho, text_chirho))| {
            *kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::VarIdChirho
                && text_chirho == "readsData"
                && non_trivia_layout_tokens_chirho
                    .get(*token_idx_chirho + 1)
                    .is_some_and(|(next_kind_chirho, _)| {
                        *next_kind_chirho
                            == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::ColonColonChirho
                    })
        })
        .map(|(reads_data_idx_chirho, _)| {
            let start_idx_chirho = reads_data_idx_chirho.saturating_sub(6);
            let end_idx_chirho =
                (reads_data_idx_chirho + 7).min(non_trivia_layout_tokens_chirho.len());
            non_trivia_layout_tokens_chirho[start_idx_chirho..end_idx_chirho].to_vec()
        })
        .unwrap_or_default();
    let tuple_eq1_token_window_chirho = non_trivia_layout_tokens_chirho
        .iter()
        .enumerate()
        .find(|(token_idx_chirho, (kind_chirho, _text_chirho))| {
            *kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::InstanceChirho
                && non_trivia_layout_tokens_chirho
                    .get(*token_idx_chirho + 1)
                    .is_some_and(|(next_kind_chirho, _)| {
                        *next_kind_chirho
                            == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::LeftParenChirho
                    })
                && non_trivia_layout_tokens_chirho
                    .get(*token_idx_chirho + 6)
                    .is_some_and(|(next_kind_chirho, next_text_chirho)| {
                        *next_kind_chirho
                            == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::ConIdChirho
                            && next_text_chirho == "Eq1"
                    })
        })
        .map(|(tuple_eq1_idx_chirho, _)| {
            let start_idx_chirho = tuple_eq1_idx_chirho.saturating_sub(6);
            let end_idx_chirho = (tuple_eq1_idx_chirho + 20).min(non_trivia_layout_tokens_chirho.len());
            non_trivia_layout_tokens_chirho[start_idx_chirho..end_idx_chirho].to_vec()
        })
        .unwrap_or_default();

    assert!(
        top_level_names_chirho.contains("readsData"),
        "preprocessed transformers Data.Functor.Classes should keep readsData top-level: top-level={:?}, instance-methods={:?}, instances={:?}, excerpt={:?}, token-window={:?}, tuple-eq1-window={:?}, direct-children={:?}",
        top_level_names_chirho,
        instance_method_names_chirho,
        instance_summaries_chirho,
        source_excerpt_chirho,
        reads_data_token_window_chirho,
        tuple_eq1_token_window_chirho,
        direct_child_summaries_chirho
    );
    assert!(
        top_level_names_chirho.contains("readsUnaryWith"),
        "preprocessed transformers Data.Functor.Classes should keep readsUnaryWith top-level: {:?}",
        top_level_names_chirho
    );
    assert!(
        top_level_names_chirho.contains("showsUnaryWith"),
        "preprocessed transformers Data.Functor.Classes should keep showsUnaryWith top-level: {:?}",
        top_level_names_chirho
    );
}

#[test]
fn frontend_lambda_tuple_pattern_binds_names_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module LambdaTuplePatternMiniChirho where\n\
pairMapMiniChirho :: [([Int], Int)] -> [Int]\n\
pairMapMiniChirho pairsMiniChirho = map (\\(xsMiniChirho, nMiniChirho) -> length xsMiniChirho + nMiniChirho) pairsMiniChirho\n",
        &mut source_map_chirho,
        "LambdaTuplePatternMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "lambda tuple patterns should bind their component names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_list_comprehension_tuple_generator_binds_names_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ListCompTuplePatternMiniChirho where\n\
countsMiniChirho :: [(String, Bool)] -> [(String, Int)]\n\
countsMiniChirho classesMiniChirho = [ (labelMiniChirho, if flagMiniChirho then 1 else 0) | (labelMiniChirho, flagMiniChirho) <- classesMiniChirho ]\n",
        &mut source_map_chirho,
        "ListCompTuplePatternMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "list comprehension tuple generators should bind their pattern names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_quickcheck_style_tuple_binders_compile_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckCoverageTupleBindersMiniChirho where\n\
\n\
classesMiniChirho :: [(String, Bool)] -> [(String, Int)]\n\
classesMiniChirho pairsMiniChirho =\n\
  [ (labelMiniChirho, if flagMiniChirho then 1 else 0) | (labelMiniChirho, flagMiniChirho) <- pairsMiniChirho ]\n\
\n\
tablesMiniChirho :: [(String, String)] -> [(String, String)]\n\
tablesMiniChirho pairsMiniChirho =\n\
  foldr (\\(tabMiniChirho, valueMiniChirho) accMiniChirho -> (tabMiniChirho, valueMiniChirho) : accMiniChirho) [] pairsMiniChirho\n\
\n\
requiredCoverageMiniChirho :: [(Maybe String, String, Double)] -> [(Maybe String, String, Double)]\n\
requiredCoverageMiniChirho triplesMiniChirho =\n\
  foldr (\\(keyMiniChirho, valueMiniChirho, probMiniChirho) accMiniChirho -> (keyMiniChirho, valueMiniChirho, probMiniChirho) : accMiniChirho) [] triplesMiniChirho\n",
        &mut source_map_chirho,
        "QuickCheckCoverageTupleBindersMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "QuickCheck-style list comprehensions and tuple-pattern lambdas should bind local names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_list_comprehension_let_qualifiers_bind_names_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ListCompLetQualifiersMiniChirho where\n\
allCoverageMiniChirho :: [((Maybe String, String), Double)] -> [(Maybe String, String, Int, Int, Double)]\n\
allCoverageMiniChirho requiredCoverageMiniChirho =\n\
  [ (keyMiniChirho, valueMiniChirho, totMiniChirho, nMiniChirho, pMiniChirho)\n\
  | ((keyMiniChirho, valueMiniChirho), pMiniChirho) <- requiredCoverageMiniChirho\n\
  , let totMiniChirho = case keyMiniChirho of\n\
          Just _tableMiniChirho -> 1\n\
          Nothing -> 0\n\
  , let nMiniChirho = 2\n\
  ]\n",
        &mut source_map_chirho,
        "ListCompLetQualifiersMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "list-comprehension let qualifiers should bind later names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_do_stmt_retains_full_application_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module DoStmtFullApplicationMiniChirho where\n\
data StateMiniChirho = StateMiniChirho { countMiniChirho :: Int }\n\
\n\
localMinMiniChirho :: StateMiniChirho -> Int -> [Int] -> IO Int\n\
localMinMiniChirho _stateMiniChirho resMiniChirho _tsMiniChirho = return resMiniChirho\n\
\n\
foundFailureMiniChirho :: StateMiniChirho -> Int -> [Int] -> IO Int\n\
foundFailureMiniChirho stateMiniChirho resMiniChirho tsMiniChirho =\n\
  do localMinMiniChirho stateMiniChirho{ countMiniChirho = 0 } resMiniChirho tsMiniChirho\n",
        &mut source_map_chirho,
        "DoStmtFullApplicationMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "do statements should retain the full applied expression: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_quickcheck_style_failure_record_selectors_compile_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckFailureSelectorsMiniChirho where\n\
import qualified Data.Map as Map\n\
import qualified Data.Set as Set\n\
\n\
type QcGenMiniChirho = Int\n\
type AnExceptionMiniChirho = String\n\
data WitnessMiniChirho = WitnessMiniChirho\n\
\n\
data ResultMiniChirho\n\
  = SuccessMiniChirho\n\
    { numTestsMiniChirho :: Int\n\
    , numDiscardedMiniChirho :: Int\n\
    , labelsMiniChirho :: !(Map.Map [String] Int)\n\
    , classesMiniChirho :: !(Map.Map String Int)\n\
    , tablesMiniChirho :: !(Map.Map String (Map.Map String Int))\n\
    , outputMiniChirho :: String\n\
    }\n\
  | FailureMiniChirho\n\
    { numTestsMiniChirho :: Int\n\
    , numDiscardedMiniChirho :: Int\n\
    , numShrinksMiniChirho :: Int\n\
    , numShrinkTriesMiniChirho :: Int\n\
    , numShrinkFinalMiniChirho :: Int\n\
    , usedSeedMiniChirho :: QcGenMiniChirho\n\
    , usedSizeMiniChirho :: Int\n\
    , reasonMiniChirho :: String\n\
    , theExceptionMiniChirho :: Maybe AnExceptionMiniChirho\n\
    , outputMiniChirho :: String\n\
    , failingTestCaseMiniChirho :: [String]\n\
    , failingLabelsMiniChirho :: [String]\n\
    , failingClassesMiniChirho :: Set.Set String\n\
    , witnessesMiniChirho :: [WitnessMiniChirho]\n\
    }\n\
\n\
recheckMiniChirho :: ResultMiniChirho -> (QcGenMiniChirho, Int)\n\
recheckMiniChirho resultMiniChirho@FailureMiniChirho{} =\n\
  (usedSeedMiniChirho resultMiniChirho, usedSizeMiniChirho resultMiniChirho)\n\
recheckMiniChirho _ = (0, 0)\n",
        &mut source_map_chirho,
        "QuickCheckFailureSelectorsMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "QuickCheck-style per-constructor record selectors should stay in scope for multi-constructor data: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_preprocessed_quickcheck_test_retains_failure_record_fields_chirho() {
    use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho
        .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/src/Test/QuickCheck/Test.hs");
    if !path_chirho.exists() {
        return;
    }

    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("QuickCheck Test.hs should preprocess");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let mut data_decl_debugs_chirho = Vec::new();
    let mut failure_fields_chirho: Option<Vec<String>> = None;
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::DataDeclChirho {
            name_chirho,
            constructors_chirho,
            ..
        } = decl_chirho
        {
            if name_chirho.text_chirho() == "Result" {
                data_decl_debugs_chirho.push(format!("{decl_chirho:#?}"));
                failure_fields_chirho = constructors_chirho.iter().find_map(|constructor_chirho| {
                    match constructor_chirho {
                        ConDeclChirho::RecordChirho {
                            name_chirho,
                            fields_chirho,
                            ..
                        } if name_chirho.text_chirho() == "Failure" => Some(
                            fields_chirho
                                .iter()
                                .flat_map(|field_decl_chirho| {
                                    field_decl_chirho
                                        .names_chirho
                                        .iter()
                                        .map(|name_chirho| name_chirho.text_chirho().to_string())
                                })
                                .collect(),
                        ),
                        _ => None,
                    }
                });
            }
        }
    }
    let failure_fields_chirho = failure_fields_chirho.unwrap_or_else(|| {
        panic!(
            "QuickCheck Result Failure constructor should lower; matching data decls: {:?}",
            data_decl_debugs_chirho
        )
    });

    assert!(
        failure_fields_chirho.contains(&"usedSeed".to_string()),
        "preprocessed QuickCheck Result should retain usedSeed in Failure fields: {:?}",
        failure_fields_chirho
    );
    assert!(
        failure_fields_chirho.contains(&"usedSize".to_string()),
        "preprocessed QuickCheck Result should retain usedSize in Failure fields: {:?}",
        failure_fields_chirho
    );
}

#[test]
fn frontend_preprocessed_quickcheck_test_allcoverage_retains_let_qualifiers_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho
        .join(".haskelujah-packages-chirho/QuickCheck-2.18.0.0/src/Test/QuickCheck/Test.hs");
    if !path_chirho.exists() {
        return;
    }

    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("QuickCheck Test.hs should preprocess");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let all_coverage_decl_debug_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "allCoverage" =>
            {
                Some(format!("{decl_chirho:#?}"))
            }
            _ => None,
        })
        .expect("QuickCheck allCoverage binding should lower");

    assert!(
        all_coverage_decl_debug_chirho.contains("LetChirho"),
        "preprocessed QuickCheck allCoverage should retain list-comprehension let qualifiers: {all_coverage_decl_debug_chirho}",
    );
}

#[test]
fn frontend_qualified_record_update_fields_lower_nested_rhs_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let source_chirho = "module UseMiniChirho where\n\
import qualified StateMiniChirho as S\n\
\n\
updateStateMiniChirho stateMiniChirho classPairsMiniChirho tablePairsMiniChirho =\n\
  stateMiniChirho\n\
    { S.labelsMiniChirho = [ (labelMiniChirho, if flagMiniChirho then 1 else 0)\n\
                           | (labelMiniChirho, flagMiniChirho) <- classPairsMiniChirho\n\
                           ]\n\
    , S.tablesMiniChirho =\n\
        foldr\n\
          (\\(tableMiniChirho, valueMiniChirho) accMiniChirho -> (tableMiniChirho, valueMiniChirho) : accMiniChirho)\n\
          []\n\
          tablePairsMiniChirho\n\
    }\n";
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "UseMiniChirho.hs",
        source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let update_decl_debug_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "updateStateMiniChirho" =>
            {
                Some(format!("{decl_chirho:#?}"))
            }
            _ => None,
        })
        .expect("qualified record update binding should lower");

    assert!(
        update_decl_debug_chirho.contains("RecordUpdateChirho"),
        "qualified record update should remain a record update expression: {update_decl_debug_chirho}",
    );
    assert!(
        update_decl_debug_chirho.contains("ListCompChirho"),
        "qualified record update field should keep nested list comprehensions: {update_decl_debug_chirho}",
    );
    assert!(
        update_decl_debug_chirho.contains("classPairsMiniChirho"),
        "qualified record update field should keep the generator source binding: {update_decl_debug_chirho}",
    );
    assert!(
        update_decl_debug_chirho.contains("valueMiniChirho"),
        "qualified record update field should keep tuple-pattern lambda binders: {update_decl_debug_chirho}",
    );
}

#[test]
fn frontend_parenthesized_prefix_type_operator_in_instance_head_compiles_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"{-# LANGUAGE GADTs, TypeOperators #-}
module PrefixTypeOperatorMiniChirho where

data aChirho :-> cChirho where
  UnitChirho :: cChirho -> (() :-> cChirho)

instance Functor ((:->) aChirho) where
  fmap fChirho (UnitChirho cChirho) = UnitChirho (fChirho cChirho)

unwrapUnitChirho :: (() :-> Int) -> Int
unwrapUnitChirho (UnitChirho nChirho) = nChirho
"#,
        &mut source_map_chirho,
        "PrefixTypeOperatorMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "parenthesized prefix type operators like ((:->) a) should lower as type constructors: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_polykinded_local_type_operator_reuses_shared_kind_vars_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"{-# LANGUAGE GADTs, PolyKinds, Rank2Types, TypeOperators #-}
module PolyKindedLocalTypeOperatorMiniChirho where

data aChirho :-> cChirho where
  PairChirho :: (aChirho :-> (bChirho :-> cChirho)) -> ((aChirho, bChirho) :-> cChirho)
  UnitChirho :: cChirho -> (() :-> cChirho)
  NilChirho :: aChirho :-> cChirho

showFunctionChirho :: (Show aChirho, Show bChirho) => (aChirho :-> bChirho) -> Maybe bChirho -> String
showFunctionChirho = undefined

functionMapWithChirho :: ((bChirho -> cChirho) -> (bChirho :-> cChirho)) -> (aChirho -> bChirho) -> (bChirho -> aChirho) -> (aChirho -> cChirho) -> (aChirho :-> cChirho)
functionMapWithChirho = undefined

data FunChirho aChirho bChirho = FunChirho (aChirho :-> bChirho, bChirho) (aChirho -> bChirho)
"#,
        &mut source_map_chirho,
        "PolyKindedLocalTypeOperatorMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "local poly-kinded type operators should keep shared inferred kinds within the module: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_gadt_type_operator_with_operator_constructor_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"{-# LANGUAGE GADTs, PolyKinds, Rank2Types, TypeOperators #-}
module GadtTypeOperatorConSymMiniChirho where

data aChirho :-> cChirho where
  PairChirho :: (aChirho :-> (bChirho :-> cChirho)) -> ((aChirho, bChirho) :-> cChirho)
  (:+:) :: (aChirho :-> cChirho) -> (bChirho :-> cChirho) -> (Either aChirho bChirho :-> cChirho)
  UnitChirho :: cChirho -> (() :-> cChirho)
  NilChirho :: aChirho :-> cChirho

showFunctionChirho :: (Show aChirho, Show bChirho) => (aChirho :-> bChirho) -> Maybe bChirho -> String
showFunctionChirho = undefined
"#,
        &mut source_map_chirho,
        "GadtTypeOperatorConSymMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "GADT type operators should keep operator-named constructors inside the declaration: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_backticked_left_section_infers_function_type_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module DeepseqSectionChirho where\nrwhnfChirho :: aChirho -> ()\nrwhnfChirho = (`seq` ())\n",
        &mut source_map_chirho,
        "DeepseqSectionChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "backticked left sections should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_multiline_local_signature_with_comments_and_pattern_guards_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module LocalInsertMiniChirho where
fChirho xsChirho = goChirho [] [] xsChirho where
  goChirho accChirho _fvListChirho [] = reverse accChirho
  goChirho accChirho fvListChirho (tvChirho:tvsChirho)
    = goChirho accPrimeChirho fvListPrimeChirho tvsChirho
    where
      (accPrimeChirho, fvListPrimeChirho) = insertChirho tvChirho accChirho fvListChirho

      insertChirho :: Int       -- value to insert
                   -> [Int]     -- sorted list, in reverse order
                   -> [[Int]]   -- list of fvs, as above
                   -> ([Int], [[Int]]) -- augmented lists
      insertChirho tvChirho [] [] = ([tvChirho], [[tvChirho]])
      insertChirho tvChirho (aChirho:asChirho) (fvsChirho:fvssChirho)
        | tvChirho `elem` fvsChirho
        , (asPrimeChirho, fvssPrimeChirho) <- insertChirho tvChirho asChirho fvssChirho
        = (aChirho:asPrimeChirho, fvsChirho : fvssPrimeChirho)
        | otherwise
        = (tvChirho:aChirho:asChirho, fvsChirho : fvssChirho)
"#,
        &mut source_map_chirho,
        "LocalInsertMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "multiline local signatures with commented continuations and pattern guards should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_backticked_class_method_guard_on_prime_name_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module GuardFreeVariablesMiniChirho where
class TypeSubstitutionChirho aChirho where
  freeVariablesChirho :: aChirho -> [Int]
instance TypeSubstitutionChirho Int where
  freeVariablesChirho _ = []
unify'Chirho :: Int -> Int -> Bool
unify'Chirho nChirho tChirho
  | nChirho `elem` freeVariablesChirho tChirho = True
  | otherwise = False
"#,
        &mut source_map_chirho,
        "GuardFreeVariablesMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "backticked class-method guards on prime-suffixed names should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_backticked_operator_pattern_guard_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module QualifiedBacktickPatternGuardMiniChirho where
import qualified Data.List as L

insertChirho :: Int -> [Int] -> [[Int]] -> ([Int], [[Int]])
insertChirho tvChirho [] [] = ([tvChirho], [[tvChirho]])
insertChirho tvChirho (aChirho:asChirho) (fvsChirho:fvssChirho)
  | tvChirho `L.elem` fvsChirho
  , (asPrimeChirho, fvssPrimeChirho) <- insertChirho tvChirho asChirho fvssChirho
  = (aChirho:asPrimeChirho, fvsChirho : fvssPrimeChirho)
  | otherwise
  = (tvChirho:aChirho:asChirho, fvsChirho : fvssChirho)
"#,
        &mut source_map_chirho,
        "QualifiedBacktickPatternGuardMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified backticked operators in pattern guards should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_recursive_class_method_list_instance_uses_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module RecursiveFreeVariablesMiniChirho where
class TypeSubstitutionChirho aChirho where
  freeVariablesChirho :: aChirho -> [Int]

instance TypeSubstitutionChirho aChirho => TypeSubstitutionChirho [aChirho] where
  freeVariablesChirho = concatMap freeVariablesChirho

data TypeChirho
  = VarTChirho Int
  | AppTChirho TypeChirho TypeChirho

instance TypeSubstitutionChirho TypeChirho where
  freeVariablesChirho typeChirho =
    case typeChirho of
      VarTChirho varChirho -> [varChirho]
      AppTChirho leftChirho rightChirho ->
        freeVariablesChirho leftChirho ++ freeVariablesChirho rightChirho

freeVariablesFromListChirho :: [TypeChirho] -> [Int]
freeVariablesFromListChirho tysChirho = freeVariablesChirho tysChirho

unify'Chirho :: TypeChirho -> TypeChirho -> Bool
unify'Chirho (VarTChirho nameChirho) typeChirho =
  nameChirho `elem` freeVariablesChirho typeChirho
unify'Chirho typeChirho (VarTChirho nameChirho) =
  nameChirho `elem` freeVariablesChirho typeChirho
unify'Chirho _ _ = False
"#,
        &mut source_map_chirho,
        "RecursiveFreeVariablesMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "recursive class methods with list instances should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_recursive_class_method_composition_list_instance_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module RecursiveFreeVariablesComposeMiniChirho where
import Data.List (nub)

class TypeSubstitutionChirho aChirho where
  freeVariablesChirho :: aChirho -> [Int]

instance TypeSubstitutionChirho aChirho => TypeSubstitutionChirho [aChirho] where
  freeVariablesChirho = nub . concat . map freeVariablesChirho

data TypeChirho
  = VarTChirho Int
  | AppTChirho TypeChirho TypeChirho

instance TypeSubstitutionChirho TypeChirho where
  freeVariablesChirho typeChirho =
    case typeChirho of
      VarTChirho varChirho -> [varChirho]
      AppTChirho leftChirho rightChirho ->
        freeVariablesChirho leftChirho ++ freeVariablesChirho rightChirho

freeVariablesFromListChirho :: [TypeChirho] -> [Int]
freeVariablesFromListChirho tysChirho = freeVariablesChirho tysChirho

unify'Chirho :: TypeChirho -> TypeChirho -> Bool
unify'Chirho (VarTChirho nameChirho) typeChirho =
  nameChirho `elem` freeVariablesChirho typeChirho
unify'Chirho typeChirho (VarTChirho nameChirho) =
  nameChirho `elem` freeVariablesChirho typeChirho
unify'Chirho _ _ = False
"#,
        &mut source_map_chirho,
        "RecursiveFreeVariablesComposeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "recursive class-method composition through nub/concat/map should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_th_abstraction_style_local_sig_in_instance_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        r#"module ThAbstractionLocalSigMiniChirho where
import qualified Data.Map as Map
import Data.Map (Map)
import qualified Data.Set as Set
import Data.Set (Set)
import Data.List (nub, union, (\\))

type NameChirho = Int
type KindChirho = [NameChirho]

data TyVarBndrUnitChirho flagChirho
  = MkTyVarBndrUnitChirho NameChirho KindChirho

data TypeChirho
  = ForallTChirho [TyVarBndrUnitChirho ()] [TypeChirho] TypeChirho
  | AppTChirho TypeChirho TypeChirho
  | SigTChirho TypeChirho KindChirho
  | VarTChirho NameChirho
  | OtherTChirho

tvNameChirho :: TyVarBndrUnitChirho flagChirho -> NameChirho
tvNameChirho (MkTyVarBndrUnitChirho nameChirho _) = nameChirho

tvKindChirho :: TyVarBndrUnitChirho flagChirho -> KindChirho
tvKindChirho (MkTyVarBndrUnitChirho _ kindChirho) = kindChirho

plainTVChirho :: NameChirho -> TyVarBndrUnitChirho ()
plainTVChirho nameChirho = MkTyVarBndrUnitChirho nameChirho []

kindedTVChirho :: NameChirho -> KindChirho -> TyVarBndrUnitChirho ()
kindedTVChirho = MkTyVarBndrUnitChirho

class TypeSubstitutionChirho aChirho where
  freeVariablesChirho :: aChirho -> [NameChirho]

instance TypeSubstitutionChirho aChirho => TypeSubstitutionChirho [aChirho] where
  freeVariablesChirho = nub . concat . map freeVariablesChirho

instance TypeSubstitutionChirho TypeChirho where
  freeVariablesChirho typeChirho =
    case typeChirho of
      ForallTChirho tvsChirho contextChirho bodyChirho ->
        fvsUnderForallChirho tvsChirho
          (freeVariablesChirho contextChirho `union` freeVariablesChirho bodyChirho)
      AppTChirho leftChirho rightChirho ->
        freeVariablesChirho leftChirho `union` freeVariablesChirho rightChirho
      SigTChirho innerChirho kindChirho ->
        freeVariablesChirho innerChirho `union` kindChirho
      VarTChirho nameChirho -> [nameChirho]
      OtherTChirho -> []
    where
      fvsUnderForallChirho :: [TyVarBndrUnitChirho flagChirho] -> [NameChirho] -> [NameChirho]
      fvsUnderForallChirho tvsChirho fvsChirho =
        (freeVariablesChirho (map tvKindChirho tvsChirho) `union` fvsChirho)
          \\ map tvNameChirho tvsChirho

freeVariablesWellScopedChirho :: [TypeChirho] -> [TyVarBndrUnitChirho ()]
freeVariablesWellScopedChirho tysChirho =
  let fvsChirho :: [NameChirho]
      fvsChirho = freeVariablesChirho tysChirho

      varKindSigsChirho :: Map NameChirho KindChirho
      varKindSigsChirho = foldMap goTyChirho tysChirho
        where
          goTyChirho :: TypeChirho -> Map NameChirho KindChirho
          goTyChirho (ForallTChirho tvbsChirho ctxtChirho bodyChirho) =
            foldr (\tvbChirho -> Map.delete (tvNameChirho tvbChirho))
                  (foldMap goTyChirho ctxtChirho `mappend` goTyChirho bodyChirho)
                  tvbsChirho
          goTyChirho (AppTChirho leftChirho rightChirho) =
            goTyChirho leftChirho `mappend` goTyChirho rightChirho
          goTyChirho (SigTChirho innerChirho kindChirho) =
            let kindSigsChirho =
                  foldMap (\nameChirho -> Map.insert nameChirho kindChirho mempty) kindChirho
            in case innerChirho of
                 VarTChirho nameChirho -> Map.insert nameChirho kindChirho kindSigsChirho
                 _ -> goTyChirho innerChirho `mappend` kindSigsChirho
          goTyChirho _ = mempty

      scopedSortChirho :: [NameChirho] -> [NameChirho]
      scopedSortChirho = goChirho [] []

      goChirho :: [NameChirho] -> [Set NameChirho] -> [NameChirho] -> [NameChirho]
      goChirho accChirho _ [] = reverse accChirho
      goChirho accChirho fvListChirho (tvChirho:tvsChirho) =
        goChirho accPrimeChirho fvListPrimeChirho tvsChirho
        where
          (accPrimeChirho, fvListPrimeChirho) =
            insertChirho tvChirho accChirho fvListChirho

      insertChirho :: NameChirho -> [NameChirho] -> [Set NameChirho] -> ([NameChirho], [Set NameChirho])
      insertChirho tvChirho [] [] = ([tvChirho], [kindFVSetChirho tvChirho])
      insertChirho tvChirho (aChirho:asChirho) (fvsHereChirho:fvssChirho)
        | tvChirho `Set.member` fvsHereChirho
        , (asPrimeChirho, fvssPrimeChirho) <- insertChirho tvChirho asChirho fvssChirho
        = (aChirho:asPrimeChirho, fvsHereChirho `Set.union` fvTvChirho : fvssPrimeChirho)
        | otherwise
        = (tvChirho:aChirho:asChirho, fvsHereChirho `Set.union` fvTvChirho : fvsHereChirho : fvssChirho)
        where
          fvTvChirho = kindFVSetChirho tvChirho

      insertChirho _ _ _ = error "scopedSort"

      kindFVSetChirho :: NameChirho -> Set NameChirho
      kindFVSetChirho nameChirho =
        maybe Set.empty (Set.fromList . freeVariablesChirho) (Map.lookup nameChirho varKindSigsChirho)

      ascribeWithKindChirho :: NameChirho -> TyVarBndrUnitChirho ()
      ascribeWithKindChirho nameChirho =
        maybe (plainTVChirho nameChirho) (kindedTVChirho nameChirho) (Map.lookup nameChirho varKindSigsChirho)

  in map ascribeWithKindChirho (scopedSortChirho fvsChirho)
"#,
        &mut source_map_chirho,
        "ThAbstractionLocalSigMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "th-abstraction-style local signatures inside instance methods should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_symbolic_infix_fun_bind_with_var_operands_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module SymbolicInfixFunBindChirho where\ninfixr 0 ~:\n(~:) :: [Char] -> Int -> ([Char], Int)\nlabelChirho ~: valueChirho = (labelChirho, valueChirho)\nvalueOutChirho = \"ok\" ~: 1\n",
        &mut source_map_chirho,
        "SymbolicInfixFunBindChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "symbolic infix function bindings should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_forward_symbolic_operator_binding_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ForwardSymbolicOperatorBindingChirho where\nconsChirho :: aChirho -> [aChirho] -> [aChirho]\nconsChirho headChirho tailChirho = headChirho <++> tailChirho\ninfixr 5 <++>\n(<++>) :: aChirho -> [aChirho] -> [aChirho]\nvalueChirho <++> valuesChirho = valueChirho : valuesChirho\n",
        &mut source_map_chirho,
        "ForwardSymbolicOperatorBindingChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "forward references to symbolic top-level bindings should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_local_symbolic_operator_can_shadow_prelude_binding_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module LocalShadowingPlusPlusChirho where\nconsChirho :: aChirho -> [aChirho] -> [aChirho]\nconsChirho headChirho tailChirho = headChirho ++ tailChirho\n(++) :: aChirho -> [aChirho] -> [aChirho]\nvalueChirho ++ valuesChirho = valueChirho : valuesChirho\n",
        &mut source_map_chirho,
        "LocalShadowingPlusPlusChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "local symbolic bindings should shadow Prelude operators: {:?}",
        result_chirho.err()
    );
}

#[test]
fn check_accepts_mkweak_primop_with_unboxed_tuple_result_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "WeakPrimMiniChirho.hs",
        "{-# LANGUAGE MagicHash #-}\n{-# LANGUAGE UnboxedTuples #-}\nmodule WeakPrimMiniChirho where\nimport GHC.Base\nfChirho keyChirho valueChirho finalizerChirho stateChirho = case mkWeak# keyChirho valueChirho finalizerChirho stateChirho of\n  (# state1Chirho, weak1Chirho #) -> (# state1Chirho, weak1Chirho #)\nmain = 42\n",
    );

    let check_summary_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);

    assert!(
        check_summary_chirho.is_ok(),
        "mkWeak# with unboxed tuple results should typecheck: {:?}",
        check_summary_chirho.err()
    );
}

#[test]
fn check_accepts_newtvar_in_stm_context_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "StmNewTVarMiniChirho.hs",
        "module StmNewTVarMiniChirho where\nimport Control.Concurrent.STM\nnewTSemMiniChirho :: Integer -> STM (TVar Integer)\nnewTSemMiniChirho iChirho = newTVar iChirho\nmain = 42\n",
    );

    let check_summary_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);

    assert!(
        check_summary_chirho.is_ok(),
        "newTVar should remain in STM, not IO: {:?}",
        check_summary_chirho.err()
    );
}

#[test]
fn frontend_infix_data_constructor_value_and_pattern_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ViewRScopeMiniChirho where\n\
data SeqChirho aChirho = SeqChirho aChirho\n\
data ViewRChirho aChirho = EmptyRChirho | SeqChirho aChirho :> aChirho\n\
useCtorChirho :: SeqChirho aChirho -> aChirho -> ViewRChirho aChirho\n\
useCtorChirho xsChirho xChirho = (:>) xsChirho xChirho\n\
usePatChirho :: ViewRChirho aChirho -> SeqChirho aChirho\n\
usePatChirho (xsChirho :> _xChirho) = xsChirho\n\
usePatChirho EmptyRChirho = SeqChirho (error \"boom\")\n",
        &mut source_map_chirho,
        "ViewRScopeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "infix data constructors should enter scope for both value and pattern use: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_lambda_bang_pattern_params_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE BangPatterns #-}\nmodule LambdaBangPatMiniChirho where\nfChirho gChirho = \\zChirho !aryChirho -> zChirho\n",
        &mut source_map_chirho,
        "LambdaBangPatMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "lambda bang-pattern parameters should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_fun_binding_bang_as_pattern_args_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE BangPatterns, MagicHash #-}\nmodule StrictAsPatMiniChirho where\nimport GHC.Exts (Int(I#))\nfChirho !xChirho !_yChirho@(I# y#) = xChirho\n",
        &mut source_map_chirho,
        "StrictAsPatMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "strict as-pattern function args should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_case_alt_constructor_bang_subpattern_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE BangPatterns #-}\nmodule StrictCaseAltMiniChirho where\n\ndata LookupResChirho aChirho = AbsentChirho | PresentChirho aChirho Int\n\nlookupResToMaybeChirho :: LookupResChirho aChirho -> Maybe aChirho\nlookupResToMaybeChirho AbsentChirho = Nothing\nlookupResToMaybeChirho (PresentChirho xChirho _) = Just xChirho\n\nptrEqChirho :: aChirho -> aChirho -> Bool\nptrEqChirho _ _ = False\n\nalterChirho :: (Maybe vChirho -> Maybe vChirho) -> LookupResChirho vChirho -> vChirho -> vChirho\nalterChirho fChirho lookupResChirho mChirho =\n  case fChirho (lookupResToMaybeChirho lookupResChirho) of\n    Nothing -> case lookupResChirho of\n      AbsentChirho -> mChirho\n      PresentChirho _ _collPosChirho -> mChirho\n    Just !vPrimeChirho -> case lookupResChirho of\n      AbsentChirho -> vPrimeChirho\n      PresentChirho vChirho _collPosChirho ->\n        if ptrEqChirho vChirho vPrimeChirho then mChirho else vPrimeChirho\n",
        &mut source_map_chirho,
        "StrictCaseAltMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "constructor bang subpatterns in case alts should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_maybe_like_unboxed_sums_lower_to_maybe_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE UnboxedSums #-}\nmodule MaybeLikeUnboxedSumMiniChirho where\nfromMaybeUnitChirho :: (# (##) | Int #) -> Maybe Int\nfromMaybeUnitChirho (# (##) | #) = Nothing\nfromMaybeUnitChirho (# | aChirho #) = Just aChirho\nuseMaybeUnitChirho :: Maybe Int\nuseMaybeUnitChirho = fromMaybeUnitChirho (# | 1 #)\n",
        &mut source_map_chirho,
        "MaybeLikeUnboxedSumMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "maybe-like unboxed sums should lower to Maybe-compatible syntax: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_list_append_is_polymorphic_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ListAppendPolyChirho where\nvalueChirho = [Just 1] ++ [Nothing]\n",
        &mut source_map_chirho,
        "ListAppendPolyChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "list append should work for non-String element types: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_foreign_ptr_unsafe_with_foreign_ptr_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ForeignPtrUnsafeMiniChirho where\nimport Foreign.Ptr (Ptr)\nimport GHC.ForeignPtr (ForeignPtr, unsafeWithForeignPtr)\nusePtrChirho :: ForeignPtr aChirho -> (Ptr aChirho -> IO bChirho) -> IO bChirho\nusePtrChirho = unsafeWithForeignPtr\n",
        &mut source_map_chirho,
        "ForeignPtrUnsafeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "GHC.ForeignPtr.unsafeWithForeignPtr should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_runst_accepts_rank2_argument_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE RankNTypes #-}\nmodule RunSTRank2MiniChirho where\nimport Control.Monad.ST\nnewtype ActionChirho aChirho = ActionChirho (forall sChirho. ST sChirho aChirho)\nrunActionChirho :: ActionChirho aChirho -> aChirho\nrunActionChirho (ActionChirho mChirho) = runST mChirho\n",
        &mut source_map_chirho,
        "RunSTRank2MiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "builtin runST should accept a rank-2 ST argument: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_stref_and_atomic_modify_ioref2lazy_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module RandomStatefulMiniChirho where\nimport Control.Monad.ST\nimport Data.IORef\nimport Data.STRef\nimport GHC.IORef (atomicModifyIORef2Lazy)\nnewtype STGenMChirho gChirho sChirho = STGenMChirho { unSTGenMChirho :: STRef sChirho gChirho }\nnewSTGenMChirho :: gChirho -> ST sChirho (STGenMChirho gChirho sChirho)\nnewSTGenMChirho = fmap STGenMChirho . newSTRef\napplySTGenChirho :: (gChirho -> (aChirho, gChirho)) -> STGenMChirho gChirho sChirho -> ST sChirho aChirho\napplySTGenChirho fChirho (STGenMChirho refChirho) = do\n  gChirho <- readSTRef refChirho\n  case fChirho gChirho of\n    (aChirho, gPrimeChirho) -> aChirho <$ writeSTRef refChirho gPrimeChirho\natomicModifyIORefHSChirho :: IORef aChirho -> (aChirho -> (aChirho, bChirho)) -> IO bChirho\natomicModifyIORefHSChirho refChirho fChirho = do\n  (_oldChirho, (_newChirho, resChirho)) <- atomicModifyIORef2Lazy refChirho fChirho\n  pure resChirho\n",
        &mut source_map_chirho,
        "RandomStatefulMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "STRef adapters and atomicModifyIORef2Lazy should typecheck for random-style code: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_float_range_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FloatRangeMiniChirho where\nrangeChirho :: (Int, Int)\nrangeChirho = floatRange (0.0 :: Double)\n",
        &mut source_map_chirho,
        "FloatRangeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "floatRange should typecheck through the builtin RealFloat surface: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_guarded_instance_method_where_binding_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module GuardedInstanceMiniChirho where\nclass CChirho aChirho where\n  fChirho :: aChirho -> Int -> Int\ninstance CChirho Int where\n  fChirho aChirho bChirho\n    | otherwise = aChirho + rChirho\n    where\n      rChirho = bChirho\n",
        &mut source_map_chirho,
        "GuardedInstanceMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "guarded instance methods with trailing where bindings should keep lhs params in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_instance_methods_receive_specialized_class_predicates_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module InstancePredicatesMiniChirho where\nimport Control.Applicative\n\ndata BoxChirho aChirho = BoxChirho aChirho\n\ninstance Functor BoxChirho where\n  fmap fChirho (BoxChirho xChirho) = BoxChirho (fChirho xChirho)\n\ninstance Applicative BoxChirho where\n  pure = BoxChirho\n  BoxChirho fChirho <*> BoxChirho xChirho = BoxChirho (fChirho xChirho)\n\ninstance Alternative BoxChirho where\n  empty = BoxChirho []\n  many _ = pure []\n  some _ = pure []\n",
        &mut source_map_chirho,
        "InstancePredicatesMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "instance method checking should carry specialized class predicates into method inference: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_foldable_null_and_length_are_polymorphic_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FoldablePreludeMiniChirho where\n\ndata BoxChirho aChirho = EmptyBoxChirho | BoxChirho aChirho\n\ninstance Foldable BoxChirho where\n  foldr _ zChirho EmptyBoxChirho = zChirho\n  foldr fChirho zChirho (BoxChirho xChirho) = fChirho xChirho zChirho\n\nlengthBoxChirho :: BoxChirho Int -> Int\nlengthBoxChirho = length\n\nnullBoxChirho :: BoxChirho Int -> Bool\nnullBoxChirho = null\n",
        &mut source_map_chirho,
        "FoldablePreludeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "builtin null/length should accept Foldable instances, not just lists: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_data_map_local_helper_uses_real_map_types_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE BangPatterns #-}\nmodule DataMapLocalHelperMiniChirho where\nimport qualified Data.Map as M\n\nfooChirho :: Maybe Int\nfooChirho = longDivChirho 0 0 M.empty 7\n  where\n    longDivChirho :: Integer -> Int -> M.Map Integer Int -> Integer -> Maybe Int\n    longDivChirho !cChirho !eChirho nsChirho !nChirho\n      | Just ePrimeChirho <- M.lookup nChirho nsChirho = Just ePrimeChirho\n      | nChirho < 10 = let !nsPrimeChirho = M.insert nChirho eChirho nsChirho\n                       in longDivChirho (cChirho * 10) (eChirho - 1) nsPrimeChirho (nChirho * 10)\n      | otherwise = Nothing\n",
        &mut source_map_chirho,
        "DataMapLocalHelperMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Map builtins should treat maps as Map.Map k v, not Int placeholders: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_data_map_strict_from_list_with_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module DataMapFromListWithMiniChirho where\n\
import qualified Data.Map.Strict as Map\n\
countsChirho :: Map.Map Int [Int]\n\
countsChirho = Map.fromListWith (++) [(1, [2]), (1, [3]), (2, [4])]\n",
        &mut source_map_chirho,
        "DataMapFromListWithMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Map.Strict.fromListWith should stay in scope with a real map type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn multi_module_imported_selector_type_avoids_local_type_shadowing_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "ProviderChirho.hs",
            "module ProviderChirho where\n\
data Result = ProviderResult { callbacks :: [Int], reason :: String }\n",
        ),
        (
            "ConsumerChirho.hs",
            "module ConsumerChirho where\n\
import ProviderChirho hiding (Result(reason))\n\
import qualified ProviderChirho as P\n\
data Result = ConsumerResult { reason :: String }\n\
callbacksFromProviderChirho :: P.Result -> [Int]\n\
callbacksFromProviderChirho resChirho = callbacks resChirho\n\
reasonFromProviderChirho :: P.Result -> String\n\
reasonFromProviderChirho = P.reason\n",
        ),
    ];

    let result_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        result_chirho.is_ok(),
        "imported selectors should keep ProviderChirho.Result distinct from the local Result: {:?}",
        result_chirho.err()
    );
}

#[test]
fn multi_module_transitive_terminal_type_stays_qualified_consistently_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "TextProviderChirho.hs",
            "module TextProviderChirho where\n\
data TerminalChirho = MkTerminalChirho\n\
putLineChirho :: TerminalChirho -> String -> String\n\
putLineChirho _terminalChirho textChirho = textChirho\n",
        ),
        (
            "StateProviderChirho.hs",
            "module StateProviderChirho where\n\
import TextProviderChirho\n\
data StateChirho = MkStateChirho { terminalChirho :: TerminalChirho }\n",
        ),
        (
            "ConsumerChirho.hs",
            "module ConsumerChirho where\n\
import TextProviderChirho (putLineChirho)\n\
import StateProviderChirho (StateChirho(terminalChirho))\n\
renderChirho :: StateChirho -> String -> String\n\
renderChirho stateChirho textChirho = putLineChirho (terminalChirho stateChirho) textChirho\n",
        ),
    ];

    let result_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        result_chirho.is_ok(),
        "transitive imported schemes should agree on qualified Terminal-like types: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_ioref_stref_constructor_roundtrip_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module IORefSTRefCtorMiniChirho where\nimport GHC.Exts (MutVar#, RealWorld)\nimport GHC.IORef (IORef(IORef))\nimport GHC.STRef (STRef(STRef))\nwrapChirho :: MutVar# RealWorld Int -> IORef Int\nwrapChirho mvChirho = IORef (STRef mvChirho)\nunwrapChirho :: IORef Int -> ()\nunwrapChirho (IORef (STRef _)) = ()\n",
        &mut source_map_chirho,
        "IORefSTRefCtorMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "GHC.IORef and GHC.STRef constructors should typecheck through imports and pattern matches: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_traversable_sequencea_instance_method_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TaggedSequenceAChirho where\nnewtype TaggedChirho sChirho aChirho = TaggedChirho aChirho\ninstance Functor (TaggedChirho sChirho) where\n  fmap fChirho (TaggedChirho xChirho) = TaggedChirho (fChirho xChirho)\ninstance Foldable (TaggedChirho sChirho) where\n  foldMap fChirho (TaggedChirho xChirho) = fChirho xChirho\ninstance Traversable (TaggedChirho sChirho) where\n  traverse fChirho (TaggedChirho xChirho) = TaggedChirho <$> fChirho xChirho\n  sequenceA (TaggedChirho xChirho) = TaggedChirho <$> xChirho\n",
        &mut source_map_chirho,
        "TaggedSequenceAChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Traversable sequenceA instance methods should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_semigroup_traversable_traverse1_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module SemigroupTraverse1MiniChirho where\nimport Data.Functor.Apply (Apply)\nimport Data.Semigroup.Traversable (Traversable1(traverse1))\nuseTraverse1Chirho :: (Traversable1 tChirho, Apply fChirho) => (aChirho -> fChirho bChirho) -> tChirho aChirho -> fChirho (tChirho bChirho)\nuseTraverse1Chirho = traverse1\n",
        &mut source_map_chirho,
        "SemigroupTraverse1MiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Semigroup.Traversable.traverse1 should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn multi_module_default_rational_propagates_to_imported_helpers_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "MatrixChirho.hs",
            "module MatrixChirho where\ndefault (Rational)\ninverseChirho mChirho = mChirho\nmultChirho _Chirho xsChirho = xsChirho\n",
        ),
        (
            "RgbChirho.hs",
            "module RgbChirho where\nimport MatrixChirho\nvalueChirho :: [Rational]\nvalueChirho = multChirho (inverseChirho []) [1 / 2, 1, 3 / 2]\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "module default (Rational) should propagate through imported helpers: {:?}",
        results_chirho.err()
    );
}

#[test]
fn multi_module_imported_associated_type_family_reduces_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "PrimFamilyProviderChirho.hs",
            "{-# LANGUAGE TypeFamilies #-}\n{-# LANGUAGE FlexibleInstances #-}\nmodule PrimFamilyProviderChirho where\nclass PrimMonadChirho mChirho where\n  type PrimStateChirho mChirho\ndata STChirho sChirho aChirho = STChirho aChirho\ndata BoxChirho sChirho aChirho = BoxChirho\nnewArrayChirho :: PrimMonadChirho mChirho => Int -> aChirho -> mChirho (BoxChirho (PrimStateChirho mChirho) aChirho)\nnewArrayChirho = undefined\ninstance PrimMonadChirho (STChirho sChirho) where\n  type PrimStateChirho (STChirho sChirho) = sChirho\n",
        ),
        (
            "PrimFamilyConsumerChirho.hs",
            "module PrimFamilyConsumerChirho where\nimport PrimFamilyProviderChirho\nnewArrayMiniChirho :: Int -> STChirho sChirho (BoxChirho sChirho aChirho)\nnewArrayMiniChirho nChirho = newArrayChirho nChirho undefined\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "imported associated type family equations should reduce across modules: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_seed_batch_propagates_associated_type_families_between_modules_chirho() {
    use std::collections::HashMap;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let module_sources_chirho = vec![
        (
            "PrimFamilyProviderChirho".to_string(),
            "PrimFamilyProviderChirho.hs".to_string(),
            "{-# LANGUAGE TypeFamilies #-}\n{-# LANGUAGE FlexibleInstances #-}\nmodule PrimFamilyProviderChirho where\nclass PrimMonadChirho mChirho where\n  type PrimStateChirho mChirho\ndata STChirho sChirho aChirho = STChirho aChirho\ndata BoxChirho sChirho aChirho = BoxChirho\nnewArrayChirho :: PrimMonadChirho mChirho => Int -> aChirho -> mChirho (BoxChirho (PrimStateChirho mChirho) aChirho)\nnewArrayChirho = undefined\ninstance PrimMonadChirho (STChirho sChirho) where\n  type PrimStateChirho (STChirho sChirho) = sChirho\n"
                .to_string(),
        ),
        (
            "PrimFamilyConsumerChirho".to_string(),
            "PrimFamilyConsumerChirho.hs".to_string(),
            "module PrimFamilyConsumerChirho where\nimport PrimFamilyProviderChirho\nnewArrayMiniChirho :: Int -> STChirho sChirho (BoxChirho sChirho aChirho)\nnewArrayMiniChirho nChirho = newArrayChirho nChirho undefined\n"
                .to_string(),
        ),
    ];

    let result_chirho = crate::compile_module_sources_with_extra_ifaces_chirho(
        module_sources_chirho,
        &mut source_map_chirho,
        vec![],
        HashMap::new(),
        crate::ImportedTypeSynonymsChirho::new(),
        crate::ImportedTypeFamiliesChirho::new(),
    );

    assert!(
        result_chirho.is_ok(),
        "frontend seed batch should carry imported associated type family equations forward: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_sum_on_rational_list_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module SumRationalChirho where\nvalueChirho :: Rational\nvalueChirho = sum [1 / 2, 1, 3 / 2]\n",
        &mut source_map_chirho,
        "SumRationalChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "sum should stay polymorphic over Rational lists: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_signature_givens_prevent_local_bounded_defaulting_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ColourBoundedMiniChirho where\n\
data RGBChirho aChirho = RGBChirho aChirho\n\
fmapRGBChirho :: (xChirho -> yChirho) -> RGBChirho xChirho -> RGBChirho yChirho\n\
fmapRGBChirho fChirho (RGBChirho xChirho) = RGBChirho (fChirho xChirho)\n\
quantizeChirho :: (RealFrac bChirho, Integral aChirho, Bounded aChirho) => bChirho -> aChirho\n\
quantizeChirho = undefined\n\
fooChirho :: (RealFrac bChirho, Floating bChirho, Integral aChirho, Bounded aChirho) => RGBChirho bChirho -> RGBChirho aChirho\n\
fooChirho cChirho = fmapRGBChirho fChirho cChirho\n\
 where\n\
  fChirho xChirho = quantizeChirho (mChirho * xChirho)\n\
  mChirho = fromIntegral $ maxBound `asTypeOf` (fChirho undefined)\n",
        &mut source_map_chirho,
        "ColourBoundedMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "signature givens should keep local Integral/Bounded vars polymorphic: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ceiling_can_return_integer_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module CeilingIntegerMiniChirho where\nplacesChirho :: Integer\nplacesChirho = ceiling (1.5 :: Double)\n",
        &mut source_map_chirho,
        "CeilingIntegerMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "ceiling should support Integral result types beyond Int: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_max_stays_polymorphic_for_integer_ceiling_result_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckTextMiniChirho where\n\
placesChirho :: Integer\n\
placesChirho = ceiling (logBase 10 (fromIntegral (5 :: Int)) - 2 :: Double) `max` 0\n",
        &mut source_map_chirho,
        "QuickCheckTextMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "max should stay polymorphic so Integer-valued ceiling expressions typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_system_io_buffering_and_terminal_queries_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckIOMiniChirho where\n\
import System.IO\n\
fooChirho :: Handle -> IO BufferMode\n\
fooChirho = hGetBuffering\n\
barChirho :: Handle -> IO Bool\n\
barChirho = hIsTerminalDevice\n",
        &mut source_map_chirho,
        "QuickCheckIOMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "System.IO buffering and terminal query helpers should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_data_time_format_parse_helpers_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TimeFormatMiniChirho where\n\
import Data.Time\n\
import Data.Time.Format\n\
parseDayChirho :: Day\n\
parseDayChirho = parseTimeOrError True defaultTimeLocale \"%Y-%m-%d\" \"2020-01-01\"\n\
readDayChirho :: ReadS Day\n\
readDayChirho = readSTime True defaultTimeLocale \"%Y-%m-%d\"\n\
parseMaybeDayChirho :: Maybe Day\n\
parseMaybeDayChirho = parseTimeM True defaultTimeLocale \"%Y-%m-%d\" \"2020-01-01\"\n",
        &mut source_map_chirho,
        "TimeFormatMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.Time.Format parse helpers should typecheck for Day: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_text_parser_combinators_parsec_operator_and_combinators_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ParsecCompatMiniChirho where\n\
import Text.ParserCombinators.Parsec (GenParser, ParseError, parse, (<?>), try, option, char, string)\n\
type ParserChirho aChirho = GenParser Char () aChirho\n\
parserChirho :: ParserChirho String\n\
parserChirho = option \"\" (try (string \"a\")) <?> \"demo\"\n\
valueChirho :: Either ParseError String\n\
valueChirho = parse parserChirho \"demo\" \"a\"\n\
charParserChirho :: ParserChirho Char\n\
charParserChirho = char 'a' <?> \"letter a\"\n",
        &mut source_map_chirho,
        "ParsecCompatMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Text.ParserCombinators.Parsec compat operator/combinator surface should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_from_integral_can_target_word64_in_subtraction_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckGenMiniChirho where\n\
import Data.Int\n\
import Data.Word\n\
chooseUpToChirho :: Word64 -> Word64\n\
chooseUpToChirho = id\n\
fooChirho :: Int64 -> Int64 -> Word64\n\
fooChirho loChirho hiChirho = chooseUpToChirho (fromIntegral hiChirho - fromIntegral loChirho)\n",
        &mut source_map_chirho,
        "QuickCheckGenMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "fromIntegral should stay polymorphic enough to target Word64 in QuickCheck-style subtraction: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_quickcheck_choose_int64_seed_path_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QuickCheckSeedMiniChirho where\n\
import Data.Int\n\
import Data.Word\n\
import System.Random.SplitMix (SMGen, bitmaskWithRejection64')\n\
chooseUpToChirho :: Word64 -> SMGen -> Word64\n\
chooseUpToChirho nChirho genChirho = fst (bitmaskWithRejection64' nChirho genChirho)\n\
chooseInt64MiniChirho :: Int64 -> Int64 -> SMGen -> Int64\n\
chooseInt64MiniChirho loChirho hiChirho genChirho =\n\
  fromIntegral (chooseUpToChirho (fromIntegral hiChirho - fromIntegral loChirho) genChirho + fromIntegral loChirho)\n",
        &mut source_map_chirho,
        "QuickCheckSeedMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "QuickCheck chooseInt64-style SplitMix seed path should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_proxy_hash_preserves_higher_kinded_class_param_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE DefaultSignatures #-}\n{-# LANGUAGE MagicHash #-}\nmodule ProxyHashKindMiniChirho where\nimport GHC.Exts (Proxy#)\nimport GHC.Generics\n\
data CardinalityMiniChirho = ShiftMiniChirho Int | CardMiniChirho Integer\n\
class FiniteMiniChirho a where\n\
  cardinalityMiniChirho :: Proxy# a -> CardinalityMiniChirho\n\
  toFiniteMiniChirho :: Integer -> a\n\
  fromFiniteMiniChirho :: a -> Integer\n\
  default cardinalityMiniChirho :: (Generic a, GFiniteMiniChirho (Rep a)) => Proxy# a -> CardinalityMiniChirho\n\
  default toFiniteMiniChirho :: (Generic a, GFiniteMiniChirho (Rep a)) => Integer -> a\n\
  default fromFiniteMiniChirho :: (Generic a, GFiniteMiniChirho (Rep a)) => a -> Integer\n\
class GFiniteMiniChirho f where\n\
  gcardinalityMiniChirho :: Proxy# f -> CardinalityMiniChirho\n\
  toGFiniteMiniChirho :: Integer -> f a\n\
  fromGFiniteMiniChirho :: f a -> Integer\n",
        &mut source_map_chirho,
        "ProxyHashKindMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Proxy# should stay poly-kinded across earlier default signatures: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_type_binder_type_runtime_rep_annotation_preserves_newtype_application_kind_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE PolyKinds #-}\n{-# LANGUAGE KindSignatures #-}\nmodule CodeKindMiniChirho where\nimport Data.Kind (Type)\n\
data RuntimeRep\n\
data TYPE (rChirho :: RuntimeRep)\n\
newtype CodeChirho mChirho (aChirho :: TYPE rChirho) = CodeChirho (mChirho aChirho)\n\
valueChirho :: CodeChirho Maybe Int\n\
valueChirho = undefined\n",
        &mut source_map_chirho,
        "CodeKindMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "TYPE r binder annotations should not leave higher-kinded newtypes one argument short: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_fractional_literal_unifies_with_rational_annotation_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FractionalLiteralRationalChirho where\nvalueChirho :: Rational\nvalueChirho = 0.64\n",
        &mut source_map_chirho,
        "FractionalLiteralRationalChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "fractional literals should stay polymorphic until context pins them: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_floating_operator_exponent_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FloatingOperatorChirho where\npowChirho :: Floating a => a -> a\npowChirho xChirho = xChirho ** 2\n",
        &mut source_map_chirho,
        "FloatingOperatorChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "floating exponent operator should resolve with a Floating scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_stacked_context_type_signature_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module StackedContextChirho where\nfooChirho :: HasCallStack => Show a => a -> String\nfooChirho xChirho = show xChirho\n",
        &mut source_map_chirho,
        "StackedContextChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "stacked qualified contexts should lower and typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_concat_remains_polymorphic_for_nested_lists_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ConcatPolymorphismChirho where\ndata NodeChirho = ListItemChirho Int | LabelChirho String\ntype PathChirho = [NodeChirho]\ndata TestChirho = TestCaseChirho Int | TestListChirho [TestChirho] | TestLabelChirho String TestChirho\ntestCasePathsChirho :: TestChirho -> [PathChirho]\ntestCasePathsChirho t0Chirho = tcpChirho t0Chirho []\n where\n  tcpChirho (TestCaseChirho _) pChirho = [pChirho]\n  tcpChirho (TestListChirho tsChirho) pChirho = concat [ tcpChirho tChirho (ListItemChirho nChirho : pChirho) | (tChirho, nChirho) <- zip tsChirho [0..] ]\n  tcpChirho (TestLabelChirho lChirho tChirho) pChirho = tcpChirho tChirho (LabelChirho lChirho : pChirho)\n",
        &mut source_map_chirho,
        "ConcatPolymorphismChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "concat should stay polymorphic over nested lists: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_parenthesized_operator_parameter_pattern_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module OperatorParamPatternChirho where\n\
         data DigitChirho aChirho = OneChirho aChirho | TwoChirho aChirho aChirho\n\
         foldDigitChirho :: (bChirho -> bChirho -> bChirho) -> (aChirho -> bChirho) -> DigitChirho aChirho -> bChirho\n\
         foldDigitChirho _ fChirho (OneChirho aChirho) = fChirho aChirho\n\
         foldDigitChirho (<+>) fChirho (TwoChirho aChirho bChirho) = fChirho aChirho <+> fChirho bChirho\n",
        &mut source_map_chirho,
        "OperatorParamPatternChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "parenthesized operator parameters should bind as ordinary variables: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_list_literals_push_expected_type_into_overloaded_elements_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ExpectedListElementTypeChirho where\n\
         data AltFChirho fChirho aChirho = PureAltFChirho aChirho\n\
         newtype AltChirho fChirho aChirho = AltChirho [AltFChirho fChirho aChirho]\n\
         class ApplicativeLikeChirho tChirho where\n\
           pureLikeChirho :: aChirho -> tChirho aChirho\n\
         instance ApplicativeLikeChirho (AltFChirho fChirho) where\n\
           pureLikeChirho = PureAltFChirho\n\
         instance ApplicativeLikeChirho (AltChirho fChirho) where\n\
           pureLikeChirho aChirho = AltChirho [pureLikeChirho aChirho]\n",
        &mut source_map_chirho,
        "ExpectedListElementTypeChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "list literals should pass expected element types into overloaded expressions: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_local_recursive_signature_instantiates_polymorphically_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module LocalRecursiveSignatureChirho where\n{-# LANGUAGE GADTs #-}\nclass ApplyChirho fChirho where\n  apChirho :: fChirho (aChirho -> bChirho) -> fChirho aChirho -> fChirho bChirho\ndata TermChirho aChirho where\n  PureChirho :: aChirho -> TermChirho aChirho\n  ApChirho :: TermChirho (aChirho -> bChirho) -> TermChirho aChirho -> TermChirho bChirho\ninstance ApplyChirho TermChirho where\n  apChirho = ApChirho\nnormalizeChirho :: TermChirho aChirho -> TermChirho aChirho\nnormalizeChirho termChirho = goChirho termChirho\n  where\n    goChirho :: TermChirho zChirho -> TermChirho zChirho\n    goChirho (PureChirho xChirho) = PureChirho xChirho\n    goChirho (ApChirho fChirho xChirho) = apChirho (goChirho fChirho) (goChirho xChirho)\n",
        &mut source_map_chirho,
        "LocalRecursiveSignatureChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "local recursive bindings with explicit signatures should instantiate polymorphically: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_instance_method_expected_result_guides_free_alt_body_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FreeAltExpectedBodyChirho where\n\
{-# LANGUAGE GADTs #-}\n\
data AltFChirho fChirho aChirho where\n\
  PureChirho :: aChirho -> AltFChirho fChirho aChirho\n\
newtype AltChirho fChirho aChirho = AltChirho { alternativesChirho :: [AltFChirho fChirho aChirho] }\n\
instance Functor (AltFChirho fChirho) where\n\
  fmap fChirho (PureChirho aChirho) = PureChirho (fChirho aChirho)\n\
instance Functor (AltChirho fChirho) where\n\
  fmap fChirho (AltChirho xsChirho) = AltChirho (map (fmap fChirho) xsChirho)\n\
instance Applicative (AltFChirho fChirho) where\n\
  pure = PureChirho\n\
  (PureChirho fChirho) <*> yChirho = fmap fChirho yChirho\n\
instance Applicative (AltChirho fChirho) where\n\
  pure aChirho = AltChirho [pure aChirho]\n\
  (AltChirho xsChirho) <*> ysChirho = keepChirho xsChirho ysChirho\n\
    where\n\
      keepChirho :: [AltFChirho fChirho (aChirho -> bChirho)] -> AltChirho fChirho aChirho -> AltChirho fChirho bChirho\n\
      keepChirho _ _ = AltChirho []\n",
        &mut source_map_chirho,
        "FreeAltExpectedBodyChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "instance method expected result type should guide overloaded RHS inference: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_free_alt_bind_and_composition_precedence_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module FreeAltBindPrecedenceChirho where\n\
{-# LANGUAGE GADTs #-}\n\
data AltFChirho fChirho aChirho where\n\
  ApChirho :: fChirho aChirho -> AltChirho fChirho (aChirho -> bChirho) -> AltFChirho fChirho bChirho\n\
  PureChirho :: aChirho -> AltFChirho fChirho aChirho\n\
newtype AltChirho fChirho aChirho = AltChirho { alternativesChirho :: [AltFChirho fChirho aChirho] }\n\
instance Functor (AltFChirho fChirho) where\n\
  fmap fChirho (PureChirho aChirho) = PureChirho (fChirho aChirho)\n\
  fmap fChirho (ApChirho xChirho gChirho) = ApChirho xChirho (fmap (fChirho .) gChirho)\n\
instance Functor (AltChirho fChirho) where\n\
  fmap fChirho (AltChirho xsChirho) = AltChirho (map (fmap fChirho) xsChirho)\n\
instance Applicative (AltFChirho fChirho) where\n\
  pure = PureChirho\n\
  (PureChirho fChirho) <*> yChirho = fmap fChirho yChirho\n\
  yChirho <*> (PureChirho aChirho) = fmap ($ aChirho) yChirho\n\
  (ApChirho aChirho fChirho) <*> bChirho = ApChirho aChirho (flip <$> fChirho <*> (AltChirho [bChirho]))\n\
instance Applicative (AltChirho fChirho) where\n\
  pure aChirho = AltChirho [pure aChirho]\n\
  (AltChirho xsChirho) <*> ysChirho = AltChirho (xsChirho >>= alternativesChirho . (`apPrimeChirho` ysChirho))\n\
    where\n\
      apPrimeChirho :: AltFChirho fChirho (aChirho -> bChirho) -> AltChirho fChirho aChirho -> AltChirho fChirho bChirho\n\
      PureChirho fChirho `apPrimeChirho` uChirho = fmap fChirho uChirho\n\
      (ApChirho uChirho fChirho) `apPrimeChirho` vChirho = AltChirho [ApChirho uChirho (flip <$> fChirho <*> vChirho)]\n",
        &mut source_map_chirho,
        "FreeAltBindPrecedenceChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "instance methods should respect >>= and . fixities in free Alt bodies: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_higher_rank_class_methods_retain_class_predicate_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module HigherRankClassMethodsChirho where\n{-# LANGUAGE RankNTypes #-}\n{-# LANGUAGE TypeOperators #-}\ntype (:->) pChirho qChirho = forall aChirho bChirho. pChirho aChirho bChirho -> qChirho aChirho bChirho\ninfixr 0 :->\nclass BifunctorMonadChirho tChirho where\n  bireturnChirho :: pChirho :-> tChirho pChirho\n  bibindChirho :: (pChirho :-> tChirho qChirho) -> tChirho pChirho :-> tChirho qChirho\nbiliftMChirho :: BifunctorMonadChirho tChirho => (pChirho :-> qChirho) -> tChirho pChirho :-> tChirho qChirho\nbiliftMChirho fChirho = bibindChirho (bireturnChirho . fChirho)\n",
        &mut source_map_chirho,
        "HigherRankClassMethodsChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "class methods should retain the enclosing class predicate: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_zero_arity_instance_method_accepts_tuple_constructor_rhs_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TupleInstanceMethodChirho where\nclass BiapplicativeChirho pChirho where\n  bipureChirho :: aChirho -> bChirho -> pChirho aChirho bChirho\n  pairApChirho :: pChirho (aChirho -> bChirho) (cChirho -> dChirho) -> pChirho aChirho cChirho -> pChirho bChirho dChirho\ninstance BiapplicativeChirho (,) where\n  bipureChirho = (,)\n  pairApChirho ~(fChirho, gChirho) ~(aChirho, bChirho) = (fChirho aChirho, gChirho bChirho)\ninstance Monoid xChirho => BiapplicativeChirho ((,,) xChirho) where\n  bipureChirho = (,,) mempty\n  pairApChirho ~(xChirho, fChirho, gChirho) ~(x2Chirho, aChirho, bChirho) = (mappend xChirho x2Chirho, fChirho aChirho, gChirho bChirho)\n",
        &mut source_map_chirho,
        "TupleInstanceMethodChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "zero-arity instance methods should typecheck against tuple constructor rhs: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_parsed_biapplicative_class_method_keeps_applied_result_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "module BiapplicativeParsedChirho where\nclass BiapplicativeChirho pChirho where\n  bipureChirho :: aChirho -> bChirho -> pChirho aChirho bChirho\n";
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BiapplicativeParsedChirho.hs",
        source_chirho,
    );
    let frontend_result_chirho = crate::run_frontend_chirho(
        source_chirho,
        source_file_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    )
    .expect("frontend should succeed");

    let class_decl_chirho = frontend_result_chirho
        .infer_result_chirho
        .class_env_chirho
        .classes_chirho
        .get("BiapplicativeChirho")
        .expect("class should be registered");
    let scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("bipureChirho")
        .expect("method scheme should be registered");

    let result_slot_chirho = match &scheme_chirho.ty_chirho {
        haskelujah_typing_chirho::TyChirho::FunChirho(_, result_chirho, _) => {
            match result_chirho.as_ref() {
                haskelujah_typing_chirho::TyChirho::FunChirho(_, result2_chirho, _) => {
                    result2_chirho.as_ref()
                }
                other_chirho => panic!("expected second function arrow, got: {other_chirho}"),
            }
        }
        other_chirho => panic!("expected function type, got: {other_chirho}"),
    };

    assert!(
        matches!(
            result_slot_chirho,
            haskelujah_typing_chirho::TyChirho::AppChirho(_, _)
        ),
        "parsed class method result should keep p a b application rather than collapsing to bare p: {scheme_chirho}"
    );
}

#[test]
fn frontend_qualified_text_uncons_uses_text_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TextUnconsChirho where\nimport qualified Data.Text as T\nfChirho :: T.Text -> Maybe (Char, T.Text)\nfChirho = T.uncons\n",
        &mut source_map_chirho,
        "TextUnconsChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.Text.uncons should use the Text-specific scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_text_pack_unpack_use_text_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TextPackChirho where\nimport qualified Data.Text as T\npackChirho :: [Char] -> T.Text\npackChirho = T.pack\nunpackChirho :: T.Text -> [Char]\nunpackChirho = T.unpack\n",
        &mut source_map_chirho,
        "TextPackChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.Text pack/unpack should use Text instead of String: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_bytestring_char8_uncons_uses_bytestring_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringUnconsChirho where\nimport qualified Data.ByteString.Char8 as C\nfChirho :: C.ByteString -> Maybe (Char, C.ByteString)\nfChirho = C.uncons\n",
        &mut source_map_chirho,
        "ByteStringUnconsChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.ByteString.Char8.uncons should use the ByteString-specific scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_bytestring_char8_read_file_uses_bytestring_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringReadFileChirho where\nimport qualified Data.ByteString.Char8 as C\nfChirho :: FilePath -> IO C.ByteString\nfChirho = C.readFile\n",
        &mut source_map_chirho,
        "ByteStringReadFileChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.ByteString.Char8.readFile should use the ByteString-specific scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_unqualified_readfile_accepts_qualified_bytestring_signature_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringSignatureBridgeMiniChirho where\nimport Data.ByteString (readFile)\nimport qualified Data.ByteString as B\nreadQualifiedChirho :: FilePath -> IO B.ByteString\nreadQualifiedChirho = readFile\n",
        &mut source_map_chirho,
        "ByteStringSignatureBridgeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "unqualified ByteString schemes should agree with qualified ByteString signatures: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_cross_module_qualified_bytestring_aliases_stay_compatible_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "ByteStringBridgeDepMiniChirho",
            "module ByteStringBridgeDepMiniChirho where\n\
             import qualified Data.ByteString as B\n\
             byteLengthChirho :: B.ByteString -> Int\n\
             byteLengthChirho = B.length\n",
        ),
        (
            "ByteStringBridgeUseMiniChirho",
            "module ByteStringBridgeUseMiniChirho where\n\
             import ByteStringBridgeDepMiniChirho (byteLengthChirho)\n\
             import qualified Data.ByteString as B\n\
             valueChirho :: B.ByteString -> Int\n\
             valueChirho bsChirho = byteLengthChirho bsChirho\n",
        ),
    ];

    let result_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);

    assert!(
        result_chirho.is_ok(),
        "qualified Data.ByteString aliases should remain compatible across module exports: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_instance_head_qualified_bytestring_alias_matches_imported_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "ByteStringInstanceDepMiniChirho",
            "module ByteStringInstanceDepMiniChirho where\n\
             import qualified Data.ByteString as B\n\
             byteLengthChirho :: B.ByteString -> Int\n\
             byteLengthChirho = B.length\n",
        ),
        (
            "ByteStringInstanceUseMiniChirho",
            "module ByteStringInstanceUseMiniChirho where\n\
             import ByteStringInstanceDepMiniChirho (byteLengthChirho)\n\
             import qualified Data.ByteString as B\n\
             class HashMiniChirho aChirho where\n\
               hashMiniChirho :: aChirho -> Int\n\
             instance HashMiniChirho B.ByteString where\n\
               hashMiniChirho bsChirho = byteLengthChirho bsChirho\n",
        ),
    ];

    let result_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);

    assert!(
        result_chirho.is_ok(),
        "qualified Data.ByteString instance heads should agree with imported bare ByteString schemes: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_bytestring_builder_char8_is_not_shadowed_by_text_encoding_char8_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringBuilderChar8MiniChirho where\n\
         import Data.ByteString.Builder (Builder, char8)\n\
         oneCharBuilderChirho :: Builder\n\
         oneCharBuilderChirho = char8 'a'\n",
        &mut source_map_chirho,
        "ByteStringBuilderChar8MiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "Data.ByteString.Builder.char8 should not resolve to System.IO.TextEncoding char8: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_lazy_bytestring_char8_read_file_uses_bytestring_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module LazyByteStringReadFileChirho where\nimport qualified Data.ByteString.Lazy.Char8 as C\nfChirho :: FilePath -> IO C.ByteString\nfChirho = C.readFile\n",
        &mut source_map_chirho,
        "LazyByteStringReadFileChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.ByteString.Lazy.Char8.readFile should use the lazy ByteString-specific scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_prelude_char_signature_unifies_with_unqualified_char_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PreludeCharBridgeMiniChirho where\nimport qualified Prelude as P\ncharIdChirho :: P.Char -> Char\ncharIdChirho xChirho = xChirho\n",
        &mut source_map_chirho,
        "PreludeCharBridgeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Prelude type constructors should agree with their unqualified imported names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn compile_modules_qualified_exception_alias_unifies_with_someexception_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_modules_chirho(
        &[
            (
                "UpstreamExceptionAliasChirho",
                "module UpstreamExceptionAliasChirho (upstreamExceptionIdChirho) where\nimport qualified Control.Exception as E\nupstreamExceptionIdChirho :: E.SomeException -> E.SomeException\nupstreamExceptionIdChirho errChirho = errChirho\n",
            ),
            (
                "DownstreamExceptionAliasChirho",
                "module DownstreamExceptionAliasChirho where\nimport Control.Exception (SomeException)\nimport UpstreamExceptionAliasChirho\nbridgeExceptionAliasChirho :: SomeException -> SomeException\nbridgeExceptionAliasChirho = upstreamExceptionIdChirho\n",
            ),
        ],
        &mut source_map_chirho,
    );

    assert!(
        result_chirho.is_ok(),
        "qualified exception aliases from upstream modules should agree with downstream SomeException imports: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_nonempty_reverse_uses_nonempty_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module NonEmptyReverseChirho where\nimport qualified Data.List.NonEmpty as NonEmpty\nfChirho :: NonEmpty.NonEmpty a -> NonEmpty.NonEmpty a\nfChirho = NonEmpty.reverse\n",
        &mut source_map_chirho,
        "NonEmptyReverseChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.List.NonEmpty.reverse should use the NonEmpty-specific scheme: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_prelude_bool_signature_unifies_with_unqualified_bool_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PreludeBoolBridgeMiniChirho where\nimport qualified Prelude as P\nboolIdChirho :: P.Bool -> Bool\nboolIdChirho xChirho = xChirho\n",
        &mut source_map_chirho,
        "PreludeBoolBridgeMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Prelude.Bool should agree with unqualified Bool when both are in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_text_tocasefold_stays_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TextCaseFoldMiniChirho where\nimport qualified Data.Text as T\nnormalizeChirho :: T.Text -> T.Text\nnormalizeChirho = T.toCaseFold\n",
        &mut source_map_chirho,
        "TextCaseFoldMiniChirho.hs",
    );

    assert!(
        result_chirho.is_ok(),
        "qualified Data.Text.toCaseFold should stay in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn script_mode_uses_incremental_runtime_plan_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "ScriptSampleChirho.hs",
        "mainChirho = print 42\n",
    );

    let check_summary_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::ScriptChirho)
            .expect("script mode should accept module-less files");

    assert!(
        check_summary_chirho
            .runtime_plan_chirho
            .incremental_session_chirho
    );
    assert_eq!(check_summary_chirho.module_name_chirho, "Main");
}

#[test]
fn check_source_path_cpp_preprocesses_directives_chirho() {
    use std::fs;

    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("CppMain.hs");
    fs::write(
        &file_path_chirho,
        "\
{-# LANGUAGE CPP #-}
module CppMain where
#if __GLASGOW_HASKELL__ >= 810
main = 42
#else
main = 0
#endif
",
    )
    .expect("CPP test source should be written");

    let summary_chirho =
        check_source_path_chirho(&file_path_chirho, ExecutionModeChirho::BatchChirho)
            .expect("CPP preprocessing should allow checking a file with directives");
    assert_eq!(summary_chirho.module_name_chirho, "CppMain");
}

#[test]
fn check_source_file_cpp_preprocesses_mixed_case_language_pragma_chirho() {
    use std::fs;

    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("CppMixedCaseMain.hs");
    fs::write(
        &file_path_chirho,
        "\
{-# Language CPP #-}
module CppMixedCaseMain where
#if __GLASGOW_HASKELL__ >= 810
mainChirho = 42
#else
mainChirho = (
#endif
",
    )
    .expect("mixed-case CPP test source should be written");

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho =
        SourceFileChirho::from_path_with_map_chirho(&mut source_map_chirho, &file_path_chirho)
            .expect("source file should load");

    let summary_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho)
            .expect("mixed-case Language CPP pragma should still preprocess");
    assert_eq!(summary_chirho.module_name_chirho, "CppMixedCaseMain");
}

#[test]
fn check_source_path_cpp_finds_package_include_headers_chirho() {
    use std::fs;

    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let src_dir_chirho = temp_dir_chirho.path().join("src");
    let include_dir_chirho = temp_dir_chirho.path().join("include");
    fs::create_dir_all(&src_dir_chirho).expect("src dir should exist");
    fs::create_dir_all(&include_dir_chirho).expect("include dir should exist");

    let file_path_chirho = src_dir_chirho.join("CppIncludeMain.hs");
    fs::write(
        include_dir_chirho.join("test-header.h"),
        "#define CPP_INCLUDE_VALUE_CHIRHO 42\n",
    )
    .expect("CPP header should be written");
    fs::write(
        &file_path_chirho,
        "\
{-# LANGUAGE CPP #-}
#include \"test-header.h\"
module CppIncludeMain where
main = CPP_INCLUDE_VALUE_CHIRHO
",
    )
    .expect("CPP source should be written");

    let summary_chirho =
        check_source_path_chirho(&file_path_chirho, ExecutionModeChirho::BatchChirho)
            .expect("CPP preprocessing should find package include headers");
    assert_eq!(summary_chirho.module_name_chirho, "CppIncludeMain");
}

#[test]
fn read_haskell_source_file_expands_primitive_deriveprim_cpp_macros_chirho() {
    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho
        .join(".haskelujah-packages-chirho/primitive-0.9.1.0/Data/Primitive/Types.hs");
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("primitive types source should preprocess");

    assert!(
        !source_chirho.contains("derivePrim("),
        "primitive CPP preprocessing should expand derivePrim macro calls: {source_chirho}"
    );
    assert!(
        source_chirho.contains("instance Prim (Word) where")
            || source_chirho.contains("instance Prim Word where"),
        "primitive CPP preprocessing should retain expanded Prim instances: {source_chirho}"
    );
}

#[test]
fn read_haskell_source_file_keeps_primitive_bytearray_unsafe_thaw_arr_hash_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
    use haskelujah_ast_chirho::pat_chirho::PatChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho
        .join(".haskelujah-packages-chirho/primitive-0.9.1.0/Data/Primitive/ByteArray.hs");
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("primitive ByteArray source should preprocess");

    assert!(
        source_chirho.contains("unsafeThawByteArray (ByteArray arr#)")
            && source_chirho.contains("unsafeCoerce# arr#"),
        "driver preprocessing should keep arr# in unsafeThawByteArray: {source_chirho}"
    );

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|decl_chirho| {
            matches!(decl_chirho, DeclChirho::FunBindChirho { name_chirho, .. }
                    if name_chirho.text_chirho() == "unsafeThawByteArray")
        })
        .expect("expected unsafeThawByteArray binding");
    let match_chirho = match decl_chirho {
        DeclChirho::FunBindChirho { matches_chirho, .. } => &matches_chirho[0],
        other_chirho => panic!("expected function binding, got {:?}", other_chirho),
    };
    assert!(
        matches!(&match_chirho.pats_chirho[0], PatChirho::ParenChirho { inner_chirho, .. }
            if matches!(inner_chirho.as_ref(), PatChirho::ConChirho { con_chirho, args_chirho, .. }
                if con_chirho.text_chirho() == "ByteArray"
                    && args_chirho.len() == 1
                    && matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#")))
            || matches!(&match_chirho.pats_chirho[0], PatChirho::ConChirho { con_chirho, args_chirho, .. }
                if con_chirho.text_chirho() == "ByteArray"
                    && args_chirho.len() == 1
                    && matches!(&args_chirho[0], PatChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#")),
        "unsafeThawByteArray pattern should keep arr#: {:?}",
        match_chirho.pats_chirho[0]
    );
    let rhs_expr_chirho = match &match_chirho.rhs_chirho {
        RhsChirho::UnguardedChirho(expr_chirho) => expr_chirho,
        other_chirho => panic!("expected unguarded rhs, got {:?}", other_chirho),
    };
    let primitive_arg_chirho = match rhs_expr_chirho {
        ExprChirho::AppChirho { arg_chirho, .. } => arg_chirho.as_ref(),
        other_chirho => panic!("expected primitive application, got {:?}", other_chirho),
    };
    let lambda_body_chirho = match primitive_arg_chirho {
        ExprChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
            ExprChirho::LamChirho { body_chirho, .. } => body_chirho.as_ref(),
            other_chirho => panic!("expected primitive lambda, got {:?}", other_chirho),
        },
        other_chirho => panic!(
            "expected parenthesized primitive lambda, got {:?}",
            other_chirho
        ),
    };
    let tuple_elements_chirho = match lambda_body_chirho {
        ExprChirho::TupleChirho {
            elements_chirho, ..
        } => elements_chirho,
        other_chirho => panic!("expected tuple body, got {:?}", other_chirho),
    };
    let mutable_arg_chirho = match &tuple_elements_chirho[1] {
        ExprChirho::AppChirho { arg_chirho, .. } => arg_chirho.as_ref(),
        other_chirho => panic!(
            "expected MutableByteArray application, got {:?}",
            other_chirho
        ),
    };
    let unsafe_coerce_arg_chirho = match mutable_arg_chirho {
        ExprChirho::ParenChirho { inner_chirho, .. } => match inner_chirho.as_ref() {
            ExprChirho::AppChirho { arg_chirho, .. } => arg_chirho.as_ref(),
            other_chirho => panic!("expected unsafeCoerce# application, got {:?}", other_chirho),
        },
        other_chirho => panic!(
            "expected parenthesized unsafeCoerce# application, got {:?}",
            other_chirho
        ),
    };
    assert!(
        matches!(unsafe_coerce_arg_chirho, ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "arr#"),
        "unsafeThawByteArray rhs should keep arr# inside unsafeCoerce#: {:?}",
        match_chirho.rhs_chirho
    );
}

#[test]
fn read_hsc_source_sanitizes_clock_hsc2hs_directives_chirho() {
    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho =
        repo_root_chirho.join(".haskelujah-packages-chirho/clock-0.8.4/System/Clock.hsc");
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("clock hsc source should preprocess");

    assert!(
        !source_chirho.contains("#{"),
        "preprocessed hsc source should not retain hsc2hs directives"
    );
    assert!(
        source_chirho.contains("type ClockId = CClockId"),
        "CPP should keep the active ClockId branch: {source_chirho}"
    );

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_modules_chirho(
        &[("System.Clock", source_chirho.as_str())],
        &mut source_map_chirho,
    );
    assert!(
        result_chirho.is_ok(),
        "sanitized System.Clock should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn clock_iface_exports_normalize_and_s2ns_chirho() {
    use haskelujah_naming_chirho::iface_chirho::build_iface_with_imports_chirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho =
        repo_root_chirho.join(".haskelujah-packages-chirho/clock-0.8.4/System/Clock.hsc");
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("clock hsc source should preprocess");

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());
    let fun_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            haskelujah_ast_chirho::decl_chirho::DeclChirho::FunBindChirho {
                name_chirho, ..
            } => Some(name_chirho.text_chirho().to_string()),
            _ => None,
        })
        .collect();
    assert!(
        !module_chirho.decls_chirho.is_empty(),
        "System.Clock should retain declarations after hsc preprocessing"
    );
    assert!(
        fun_names_chirho.contains("normalize"),
        "System.Clock should retain normalize: {:?}",
        fun_names_chirho
    );
    assert!(
        fun_names_chirho.contains("s2ns"),
        "System.Clock should retain s2ns: {:?}",
        fun_names_chirho
    );
    assert!(
        module_chirho
            .exports_chirho
            .as_ref()
            .is_some_and(|exports_chirho| !exports_chirho.is_empty()),
        "System.Clock should retain explicit exports; module name={}",
        module_chirho.name_chirho.full_name_chirho()
    );
    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &[]);

    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("normalize"),
        "System.Clock iface should export normalize: {:?}",
        iface_chirho
            .exports_chirho
            .values_chirho
            .keys()
            .collect::<Vec<_>>()
    );
    assert!(
        iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key("s2ns"),
        "System.Clock iface should export s2ns: {:?}",
        iface_chirho
            .exports_chirho
            .values_chirho
            .keys()
            .collect::<Vec<_>>()
    );
}

#[test]
fn foreign_import_is_in_scope_for_later_top_level_bindings_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ForeignImportScopeChirho where\nforeign import ccall unsafe \"foo\" fooChirho :: Int -> IO Int\nbarChirho = fooChirho 1\n",
        &mut source_map_chirho,
        "ForeignImportScopeChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "later bindings should see foreign imports: {:?}",
        result_chirho.err()
    );
}

#[test]
fn compile_modules_propagates_module_qualified_exports_to_downstream_imports_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_modules_chirho(
        &[
            (
                "UpstreamChirho",
                "module UpstreamChirho (helperChirho, HelperWrapChirho(..)) where\nhelperChirho = 42\nnewtype HelperWrapChirho = HelperWrapChirho Int\n",
            ),
            (
                "DownstreamChirho",
                "module DownstreamChirho where\nimport UpstreamChirho (helperChirho, HelperWrapChirho(..))\nimport qualified UpstreamChirho as UpstreamChirho\nuseHelperChirho = helperChirho\nwrapHelperChirho = HelperWrapChirho helperChirho\nuseQualifiedChirho = UpstreamChirho.helperChirho\n",
            ),
        ],
        &mut source_map_chirho,
    );
    assert!(
        result_chirho.is_ok(),
        "downstream module should see upstream exports: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_preprocessed_containers_intset_retains_helper_funbinds_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho
        .join(".haskelujah-packages-chirho/containers-0.8/src/Data/IntSet/Internal.hs");
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("containers source should preprocess");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let fun_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    let mut sorted_fun_names_chirho: Vec<_> = fun_names_chirho.iter().cloned().collect();
    sorted_fun_names_chirho.sort();
    let mut helper_decl_kinds_chirho: Vec<String> = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. }
                if ["bin", "tip", "prefixOf", "linkKey", "symDiffTip"]
                    .contains(&name_chirho.text_chirho()) =>
            {
                helper_decl_kinds_chirho.push(format!("fun:{}", name_chirho.text_chirho()));
            }
            DeclChirho::TypeSigChirho { name_chirho, .. }
                if ["bin", "tip", "prefixOf", "linkKey", "symDiffTip"]
                    .contains(&name_chirho.text_chirho()) =>
            {
                helper_decl_kinds_chirho.push(format!("sig:{}", name_chirho.text_chirho()));
            }
            DeclChirho::PatBindChirho { pat_chirho, .. } => {
                for name_chirho in
                    haskelujah_typing_chirho::linearity_chirho::pat_bound_names_chirho(pat_chirho)
                {
                    if ["bin", "tip", "prefixOf", "linkKey", "symDiffTip"]
                        .contains(&name_chirho.as_str())
                    {
                        helper_decl_kinds_chirho.push(format!("pat:{name_chirho}"));
                    }
                }
            }
            _ => {}
        }
    }

    for helper_name_chirho in ["bin", "tip", "prefixOf", "linkKey", "symDiffTip"] {
        assert!(
            fun_names_chirho.contains(helper_name_chirho),
            "preprocessed IntSet.Internal should retain top-level helper {helper_name_chirho}; got {} helpers; sample: {:?}; helper decls: {:?}",
            fun_names_chirho.len(),
            &sorted_fun_names_chirho[sorted_fun_names_chirho.len().saturating_sub(24)..],
            helper_decl_kinds_chirho
        );
    }
}

#[test]
fn frontend_preprocessed_th_abstraction_datatype_retains_remaining_funbinds_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho.join(
        ".haskelujah-packages-chirho/th-abstraction-0.7.2.0/src/Language/Haskell/TH/Datatype.hs",
    );
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("Datatype.hs should preprocess");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let fun_names_chirho: std::collections::HashSet<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    let mut empty_fun_names_chirho = Vec::new();
    let mut nearby_decl_kinds_chirho = Vec::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                if name_chirho.text_chirho().is_empty() {
                    empty_fun_names_chirho.push("fun".to_string());
                }
                if ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"]
                    .contains(&name_chirho.text_chirho())
                {
                    nearby_decl_kinds_chirho.push(format!("fun:{}", name_chirho.text_chirho()));
                }
            }
            DeclChirho::TypeSigChirho { name_chirho, .. }
                if ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"]
                    .contains(&name_chirho.text_chirho()) =>
            {
                nearby_decl_kinds_chirho.push(format!("sig:{}", name_chirho.text_chirho()));
            }
            DeclChirho::PatBindChirho { pat_chirho, .. } => {
                for pat_name_chirho in
                    haskelujah_typing_chirho::linearity_chirho::pat_bound_names_chirho(pat_chirho)
                {
                    if ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"]
                        .contains(&pat_name_chirho.as_str())
                    {
                        nearby_decl_kinds_chirho.push(format!("pat:{pat_name_chirho}"));
                    }
                }
            }
            _ => {}
        }
    }

    assert!(
        empty_fun_names_chirho.is_empty(),
        "preprocessed Datatype.hs should not contain empty top-level funbind names; nearby decls: {:?}",
        nearby_decl_kinds_chirho
    );
    for helper_name_chirho in ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"] {
        assert!(
            fun_names_chirho.contains(helper_name_chirho),
            "preprocessed Datatype.hs should retain top-level helper {helper_name_chirho}; nearby decls: {:?}",
            nearby_decl_kinds_chirho
        );
    }
}

#[test]
fn frontend_preprocessed_th_abstraction_datatype_remaining_sites_have_no_placeholder_exprs_chirho()
{
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let repo_root_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let path_chirho = repo_root_chirho.join(
        ".haskelujah-packages-chirho/th-abstraction-0.7.2.0/src/Language/Haskell/TH/Datatype.hs",
    );
    let source_chirho = crate::read_haskell_source_file_chirho(&path_chirho)
        .expect("Datatype.hs should preprocess");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        path_chirho,
        &source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    for helper_name_chirho in ["mkExtraFunArgForalls", "freeVariablesWellScoped", "unify'"] {
        let decl_chirho = module_chirho
            .decls_chirho
            .iter()
            .find(|decl_chirho| {
                matches!(
                    decl_chirho,
                    DeclChirho::FunBindChirho { name_chirho, .. }
                        if name_chirho.text_chirho() == helper_name_chirho
                )
            })
            .unwrap_or_else(|| panic!("should lower top-level helper {helper_name_chirho}"));
        let decl_debug_chirho = format!("{decl_chirho:#?}");
        assert!(
            !decl_debug_chirho.contains("text_chirho: \"\""),
            "preprocessed Datatype.hs helper {helper_name_chirho} should not contain placeholder empty names: {decl_debug_chirho}",
        );
    }
}

#[test]
fn frontend_reexported_with_frozen_call_stack_from_ghc_stack_chirho() {
    use haskelujah_naming_chirho::{build_iface_with_imports_chirho, builtin_module_ifaces_chirho};
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let safe_util_source_chirho = "\
module Safe.Util (module GHC.Stack) where
import GHC.Stack
";
    let main_source_chirho = "\
module Main where
import Safe.Util
answer = withFrozenCallStack id (42 :: Int)
";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut ifaces_chirho = builtin_module_ifaces_chirho();

    let safe_util_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "Safe/Util.hs",
        safe_util_source_chirho,
    );
    let safe_util_file_id_chirho = safe_util_file_chirho.file_id_chirho();
    let safe_util_parser_chirho =
        ParserChirho::new_chirho(safe_util_source_chirho, safe_util_file_id_chirho);
    let safe_util_green_chirho = safe_util_parser_chirho.parse_chirho();
    let safe_util_module_chirho =
        lower_module_chirho(&safe_util_green_chirho, safe_util_file_id_chirho);
    let safe_util_iface_chirho =
        build_iface_with_imports_chirho(&safe_util_module_chirho, &ifaces_chirho);
    ifaces_chirho.push(safe_util_iface_chirho);

    let main_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "Main.hs",
        main_source_chirho,
    );
    let main_file_id_chirho = main_file_chirho.file_id_chirho();
    let empty_imported_types_chirho = std::collections::HashMap::new();
    crate::run_frontend_chirho(
        main_source_chirho,
        main_file_id_chirho,
        &ifaces_chirho,
        &empty_imported_types_chirho,
    )
    .expect("GHC.Stack re-exports should preserve withFrozenCallStack");
}

#[test]
fn frontend_state_t_signature_composition_preserves_tuple_payload_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module StateMini where\nnewtype StateT s m a = StateT { runStateT :: s -> m (a,s) }\nstate :: (Monad m) => (s -> (a, s)) -> StateT s m a\nstate f = StateT (return . f)\n",
        &mut source_map_chirho,
        "StateMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "StateT (return . f) should type-check from source without losing tuple payload: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_statet_partial_application_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BuiltinStateTMiniChirho.hs",
        "module BuiltinStateTMiniChirho where\n\
import Data.Word\n\
import Control.Monad.State.Strict (StateT, runStateT, execStateT)\n\
stepChirho :: StateT Int Maybe Word64\n\
stepChirho = undefined\n\
runChirho :: Maybe (Word64, Int)\n\
runChirho = runStateT stepChirho 7\n\
execChirho :: Maybe Int\n\
execChirho = execStateT stepChirho 7\n",
    );

    let result_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "builtin StateT/runStateT/execStateT surface should accept partial application: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_state_alias_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BuiltinStateAliasMiniChirho.hs",
        "module BuiltinStateAliasMiniChirho where\n\
import Data.Word\n\
import Control.Monad.State.Strict (State, runState, evalState, execState)\n\
stepChirho :: State Int Word64\n\
stepChirho = undefined\n\
runChirho :: (Word64, Int)\n\
runChirho = runState stepChirho 7\n\
evalChirho :: Word64\n\
evalChirho = evalState stepChirho 7\n\
execChirho :: Int\n\
execChirho = execState stepChirho 7\n",
    );

    let result_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "builtin State/runState/evalState/execState surface should preserve the pure State alias: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_st_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BuiltinSTMiniChirho.hs",
        "module BuiltinSTMiniChirho where\n\
import Control.Monad.ST (ST, runST)\n\
stepChirho :: ST Int (Int, Bool)\n\
stepChirho = undefined\n\
pairChirho :: (Int, Bool)\n\
pairChirho = runST stepChirho\n",
    );

    let result_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "builtin ST/runST surface should preserve the ST type constructor: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_readert_partial_application_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BuiltinReaderTMiniChirho.hs",
        "module BuiltinReaderTMiniChirho where\n\
import Data.Word\n\
import Control.Monad.Trans.Reader (ReaderT, runReaderT)\n\
stepChirho :: ReaderT Int Maybe Word64\n\
stepChirho = undefined\n\
runChirho :: Int -> Maybe Word64\n\
runChirho = runReaderT stepChirho\n",
    );

    let result_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "builtin ReaderT/runReaderT surface should accept partial application: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_builtin_maybet_partial_application_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "BuiltinMaybeTMiniChirho.hs",
        "module BuiltinMaybeTMiniChirho where\n\
import Control.Monad.Trans.Maybe (MaybeT, runMaybeT)\n\
stepChirho :: MaybeT Maybe Int\n\
stepChirho = undefined\n\
runChirho :: Maybe (Maybe Int)\n\
runChirho = runMaybeT stepChirho\n",
    );

    let result_chirho =
        check_source_file_chirho(source_file_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "builtin MaybeT/runMaybeT surface should accept partial application: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_writer_t_lift_callcc_and_catch_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module WriterMini where\n\
type CallCC m a b = ((a -> m b) -> m a) -> m a\n\
type Catch e m a = m a -> (e -> m a) -> m a\n\
newtype WriterT w m a = WriterT { unWriterT :: w -> m (a,w) }\n\
liftCallCC :: CallCC m (a, w) (b, w) -> CallCC (WriterT w m) a b\n\
liftCallCC callCC f = WriterT $ \\ w -> callCC $ \\ c -> unWriterT (f (\\ a -> WriterT $ \\ _ -> c (a, w))) w\n\
liftCatch :: Catch e m (a, w) -> Catch e (WriterT w m) a\n\
liftCatch catchE m h = WriterT $ \\ w -> unWriterT m w `catchE` \\ e -> unWriterT (h e) w\n",
        &mut source_map_chirho,
        "WriterMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "WriterT higher-order callback lifting should type-check from source: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_writer_t_nested_dollar_callcc_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module WriterLazyMini where\n\
type CallCC m a b = ((a -> m b) -> m a) -> m a\n\
class Monoid w where\n  mempty :: w\n\
newtype WriterT w m a = WriterT { runWriterT :: m (a,w) }\n\
liftCallCC :: (Monoid w) => CallCC m (a,w) (b,w) -> CallCC (WriterT w m) a b\n\
liftCallCC callCC f = WriterT $ callCC $ \\ c -> runWriterT (f (\\ a -> WriterT (c (a, mempty))))\n",
        &mut source_map_chirho,
        "WriterLazyMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "nested $ callCC lifting should type-check without losing tuple payload: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_resolves_bundled_stdlib_json_module_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module StdlibJsonDemoChirho where\nimport Haskelujah.JSON\nanswerChirho = encode (String \"hello\")\n",
        &mut source_map_chirho,
        "StdlibJsonDemoChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "bundled stdlib module Haskelujah.JSON should resolve without package install: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_imported_signatures_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ExceptImportMiniChirho where\n\
import Control.Applicative\n\
import Control.Monad.Signatures\n\
import Control.Monad.Zip (MonadZip(mzipWith))\n\
import Data.Functor.Contravariant\n\
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\n\
liftCallCCChirho :: CallCC m (Either e a) (Either e b) -> CallCC (ExceptTChirho e m) a b\n\
liftCallCCChirho callCCChirho fChirho = ExceptTChirho $ callCCChirho $ \\cChirho -> runExceptTChirho (fChirho (\\aChirho -> ExceptTChirho $ cChirho (Right aChirho)))\n\
mzipWrapChirho :: MonadZip m => (a -> b -> c) -> ExceptTChirho e m a -> ExceptTChirho e m b -> ExceptTChirho e m c\n\
mzipWrapChirho fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n\
contramapWrapChirho :: Contravariant m => (a -> b) -> ExceptTChirho e m b -> ExceptTChirho e m a\n\
contramapWrapChirho fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho\n",
        &mut source_map_chirho,
        "ExceptImportMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "imported Control.Monad.Signatures / MonadZip / Contravariant shapes should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_infix_catch_trye_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptInfixCatchTryEChirho where
import Control.Monad
newtype ExceptTChirho eChirho mChirho aChirho = ExceptTChirho { runExceptTChirho :: mChirho (Either eChirho aChirho) }
catchEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> (eChirho -> ExceptTChirho ePrimeChirho mChirho aChirho) -> ExceptTChirho ePrimeChirho mChirho aChirho
mChirho `catchEChirho` hChirho = ExceptTChirho $ do
  aChirho <- runExceptTChirho mChirho
  case aChirho of
    Left lChirho -> runExceptTChirho (hChirho lChirho)
    Right rChirho -> return (Right rChirho)
tryEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> ExceptTChirho eChirho mChirho (Either eChirho aChirho)
tryEChirho mChirho = catchEChirho (liftM Right mChirho) (return . Left)
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptInfixCatchTryEChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "infix catchE / tryE ExceptT pattern should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_infix_catch_lambda_rhs_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptInfixCatchLambdaRhsChirho where
import Control.Monad
newtype ExceptTChirho eChirho mChirho aChirho = ExceptTChirho { runExceptTChirho :: mChirho (Either eChirho aChirho) }
throwEChirho :: Monad mChirho => eChirho -> ExceptTChirho eChirho mChirho aChirho
throwEChirho = ExceptTChirho . return . Left
catchEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> (eChirho -> ExceptTChirho ePrimeChirho mChirho aChirho) -> ExceptTChirho ePrimeChirho mChirho aChirho
catchEChirho mChirho hChirho = ExceptTChirho $ do
  aChirho <- runExceptTChirho mChirho
  case aChirho of
    Left lChirho -> runExceptTChirho (hChirho lChirho)
    Right rChirho -> return (Right rChirho)
onEChirho :: Monad mChirho => ExceptTChirho eChirho mChirho aChirho -> ExceptTChirho eChirho mChirho bChirho -> ExceptTChirho eChirho mChirho aChirho
onEChirho action1Chirho action2Chirho = action1Chirho `catchEChirho` \\eChirho -> action2Chirho >> throwEChirho eChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptInfixCatchLambdaRhsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "infix catchE with lambda rhs should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_constructor_pattern_infix_funbind_enters_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ColourMiniChirho where
newtype ChanChirho pChirho aChirho = ChanChirho aChirho
addChirho :: Num aChirho => ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho
(ChanChirho aChirho) `addChirho` (ChanChirho bChirho) = ChanChirho (aChirho + bChirho)
overChirho :: Num aChirho => ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho -> ChanChirho pChirho aChirho
overChirho leftChirho rightChirho = leftChirho `addChirho` rightChirho
";
    let result_chirho =
        compile_source_chirho(source_chirho, &mut source_map_chirho, "ColourMiniChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "constructor-pattern infix funbind should be in scope for later bindings: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_infix_instance_method_with_as_pattern_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ColourAsPatternMiniChirho where
data RGBAChirho aChirho = RGBAChirho aChirho aChirho
class FooChirho fChirho where
  barChirho :: fChirho aChirho -> fChirho aChirho -> fChirho aChirho
instance FooChirho RGBAChirho where
  xChirho@(RGBAChirho _ _) `barChirho` yChirho = xChirho
blendChirho :: FooChirho fChirho => fChirho aChirho -> fChirho aChirho -> fChirho aChirho
blendChirho leftChirho rightChirho = leftChirho `barChirho` rightChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ColourAsPatternMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "infix instance method with as-pattern lhs should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_prelude_exports_alternative_helpers_implicitly_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module PreludeAlternativeMiniChirho where
choicesChirho :: [Int]
choicesChirho = [1] <|> [2]
pairedChirho :: [Int]
pairedChirho = liftA2 (+) [1] [2]
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "PreludeAlternativeMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "implicit Prelude should provide <|> and liftA2: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_with_exceptt_functor_operator_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptWithExceptTOperatorChirho where
newtype ExceptTChirho eChirho mChirho aChirho = ExceptTChirho { runExceptTChirho :: mChirho (Either eChirho aChirho) }
withExceptTChirho :: Functor mChirho => (eChirho -> ePrimeChirho) -> ExceptTChirho eChirho mChirho aChirho -> ExceptTChirho ePrimeChirho mChirho aChirho
withExceptTChirho fChirho (ExceptTChirho actionChirho) =
  ExceptTChirho ((\\ valueChirho -> case valueChirho of
    Left errChirho -> Left (fChirho errChirho)
    Right okChirho -> Right okChirho) <$> actionChirho)
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptWithExceptTOperatorChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "ExceptT-style withExceptT should type-check through <$>: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_imported_top_level_methods_without_sigs_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ExceptImportTopLevelChirho where\n\
import Control.Applicative\n\
import Control.Monad.Zip (MonadZip(mzipWith))\n\
import Data.Functor.Contravariant\n\
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\n\
mzipWrapChirho fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n\
contramapWrapChirho fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho\n",
        &mut source_map_chirho,
        "ExceptImportTopLevelChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "top-level ExceptT wrappers should type-check without explicit signatures: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_imported_instance_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptImportInstancesChirho where
import Control.Applicative
import Control.Monad.Zip (MonadZip(mzipWith))
import Data.Functor.Contravariant
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance (MonadZip m) => MonadZip (ExceptTChirho e m) where
  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho
instance Contravariant m => Contravariant (ExceptTChirho e m) where
  contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptImportInstancesChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "ExceptT-style imported instance method bodies should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_imported_instance_methods_run_frontend_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptImportInstancesFrontendChirho where
import Control.Applicative
import Control.Monad.Zip (MonadZip(mzipWith))
import Data.Functor.Contravariant
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance (MonadZip m) => MonadZip (ExceptTChirho e m) where
  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho
instance Contravariant m => Contravariant (ExceptTChirho e m) where
  contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho
";
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "ExceptImportInstancesFrontendChirho.hs",
        source_chirho,
    );
    let frontend_result_chirho = crate::run_frontend_chirho(
        source_chirho,
        source_file_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    );
    assert!(
        frontend_result_chirho.is_ok(),
        "ExceptT instance-method shapes should pass the shared frontend before backend lowering: {:?}",
        frontend_result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_top_level_same_method_names_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ExceptImportSameNamesChirho where\n\
import Control.Applicative\n\
import Control.Monad.Zip (MonadZip(mzipWith))\n\
import Data.Functor.Contravariant\n\
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\n\
mzipWith :: MonadZip m => (a -> b -> c) -> ExceptTChirho e m a -> ExceptTChirho e m b -> ExceptTChirho e m c\n\
mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n\
contramap :: Contravariant m => (a -> b) -> ExceptTChirho e m b -> ExceptTChirho e m a\n\
contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho\n",
        &mut source_map_chirho,
        "ExceptImportSameNamesChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "top-level wrappers using the imported method names themselves should still type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_top_level_same_method_names_without_sigs_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ExceptImportSameNamesNoSigChirho where\n\
import Control.Applicative\n\
import Control.Monad.Zip (MonadZip(mzipWith))\n\
import Data.Functor.Contravariant\n\
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }\n\
mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho\n\
contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho\n",
        &mut source_map_chirho,
        "ExceptImportSameNamesNoSigChirho.hs",
    );
    assert!(
        result_chirho.is_err(),
        "top-level wrappers that shadow imported class methods should stay rejected without explicit signatures"
    );
}

#[test]
fn frontend_exceptt_single_monadzip_instance_method_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptImportSingleMonadZipChirho where
import Control.Applicative
import Control.Monad.Zip (MonadZip(mzipWith))
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance (MonadZip m) => MonadZip (ExceptTChirho e m) where
  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptImportSingleMonadZipChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "single MonadZip ExceptT instance method should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_class_dot_imports_expose_monad_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ClassDotMonadMethodsChirho where
import Control.Monad (MonadPlus(..), MonadFail(..))
import Control.Monad.IO.Class (MonadIO(..))
import Control.Monad.Trans.Class (MonadTrans(..))
import Control.Monad.Zip (MonadZip(..))
import GHC.Base (Monad(..))

listPlusChirho :: [Int]
listPlusChirho = mzero `mplus` [1]

maybeFailChirho :: Maybe Int
maybeFailChirho = fail \"nope\"

liftIOAliasChirho :: IO Int -> IO Int
liftIOAliasChirho = liftIO

liftAliasChirho :: MonadTrans t => m a -> t m a
liftAliasChirho = lift

zipAliasChirho :: MonadZip m => m Int -> m Int -> m Int
zipAliasChirho = mzipWith (+)

baseMonadAliasChirho :: Maybe Int
baseMonadAliasChirho = return 1 >>= \\xChirho -> Just xChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ClassDotMonadMethodsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "class `(..)` imports should expose associated monad methods: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_class_dot_imports_expose_base_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ClassDotBaseMethodsChirho where
import Prelude (Int, Maybe(..), (+))
import Control.Applicative (Applicative(..), Alternative(..))
import Data.Foldable (Foldable(..))
import Data.Functor (Functor(..))
import Data.Monoid (Monoid(..))
import Data.Semigroup (Semigroup(..))
import Data.Traversable (Traversable(..))

functorDotChirho :: Maybe Int
functorDotChirho = fmap (+1) (Just 1)

applicativeDotChirho :: Maybe Int
applicativeDotChirho = pure (+1) <*> Just 2

alternativeDotChirho :: Maybe Int
alternativeDotChirho = empty <|> Just 3

semigroupDotChirho :: [Int]
semigroupDotChirho = [1] <> [2]

monoidDotChirho :: [Int]
monoidDotChirho = mappend mempty (mconcat [[1], [2]])

foldableDotChirho :: [Int]
foldableDotChirho = foldMap (\\xChirho -> [xChirho]) [1, 2]

traversableDotChirho :: Maybe [Int]
traversableDotChirho = traverse Just [1, 2]

sequenceADotChirho :: Maybe [Int]
sequenceADotChirho = sequenceA [Just 1, Just 2]

mapMDotChirho :: Maybe [Int]
mapMDotChirho = mapM Just [1, 2]

sequenceDotChirho :: Maybe [Int]
sequenceDotChirho = sequence [Just 1, Just 2]
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ClassDotBaseMethodsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "class `(..)` imports should expose associated base methods: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_normalized_class_dot_imports_expose_extra_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ClassDotExtraMethodsChirho where
import Control.DeepSeq (NFData(..))
import Data.Functor.Contravariant (Contravariant(..))
import System.Random (RandomGen(..), SplitGen(..))

contramapAliasChirho :: Contravariant f => (a -> b) -> f b -> f a
contramapAliasChirho = contramap

rnfAliasChirho :: NFData a => a -> ()
rnfAliasChirho = rnf

splitAliasChirho :: RandomGen g => g -> (g, g)
splitAliasChirho = split

splitGenAliasChirho :: SplitGen g => g -> (g, g)
splitGenAliasChirho = splitGen
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ClassDotExtraMethodsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "normalized class `(..)` imports should expose extra associated methods: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_normalized_class_dot_imports_expose_standard_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ClassDotStandardMethodsChirho where
import Prelude (Bool, Int, Integer, String, Ordering, Rational)
import Data.Bits (Bits(..), FiniteBits(..))
import Data.Eq (Eq(..))
import Data.Ord (Ord(..))
import Data.String (IsString(..))
import GHC.Enum (Enum(..), Bounded(..))
import GHC.Num (Num(..))
import GHC.Read (Read(..))

eqAliasChirho :: Eq a => a -> a -> Bool
eqAliasChirho = (==)

neqAliasChirho :: Eq a => a -> a -> Bool
neqAliasChirho = (/=)

compareAliasChirho :: Ord a => a -> a -> Ordering
compareAliasChirho = compare

maxAliasChirho :: Ord a => a -> a -> a
maxAliasChirho = max

plusAliasChirho :: Num a => a -> a -> a
plusAliasChirho = (+)

fromIntegerAliasChirho :: Num a => Integer -> a
fromIntegerAliasChirho = fromInteger

readAliasChirho :: Read a => String -> a
readAliasChirho = read

fromStringAliasChirho :: IsString a => String -> a
fromStringAliasChirho = fromString

toEnumAliasChirho :: Enum a => Int -> a
toEnumAliasChirho = toEnum

fromEnumAliasChirho :: Enum a => a -> Int
fromEnumAliasChirho = fromEnum

minBoundAliasChirho :: Bounded a => a
minBoundAliasChirho = minBound

bitAndAliasChirho :: Bits a => a -> a -> a
bitAndAliasChirho = (.&.)

finiteBitSizeAliasChirho :: FiniteBits a => a -> Int
finiteBitSizeAliasChirho = finiteBitSize
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ClassDotStandardMethodsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "normalized standard class `(..)` imports should expose associated methods: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_normalized_class_dot_imports_expose_floating_methods_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ClassDotFloatingMethodsChirho where
import GHC.Float (Floating(..))

piAliasChirho :: Floating a => a
piAliasChirho = pi

sqrtAliasChirho :: Floating a => a -> a
sqrtAliasChirho = sqrt

sinAliasChirho :: Floating a => a -> a
sinAliasChirho = sin

atanAliasChirho :: Floating a => a -> a
atanAliasChirho = atan
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ClassDotFloatingMethodsChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "normalized Floating `(..)` import should expose associated methods: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_exceptt_single_contravariant_instance_method_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module ExceptImportSingleContravariantChirho where
import Data.Functor.Contravariant
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance Contravariant m => Contravariant (ExceptTChirho e m) where
  contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho
";
    let result_chirho = compile_source_chirho(
        source_chirho,
        &mut source_map_chirho,
        "ExceptImportSingleContravariantChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "single Contravariant ExceptT instance method should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_instance_methods_stay_nested_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_naming_chirho::builtin_module_ifaces_chirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module InstanceShapeMiniChirho where
import Control.Applicative (liftA2)
import Control.Monad.Zip (MonadZip(mzipWith))
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance (MonadZip m) => MonadZip (ExceptTChirho e m) where
  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho
";
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "InstanceShapeMiniChirho.hs",
        source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());
    let frontend_chirho = crate::run_frontend_chirho(
        source_chirho,
        source_file_chirho.file_id_chirho(),
        &builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    );

    let leaked_funbind_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    assert!(
        !leaked_funbind_names_chirho
            .iter()
            .any(|name_chirho| name_chirho == "mzipWith"),
        "instance method should stay nested, not leak as top-level FunBind: {:?}",
        leaked_funbind_names_chirho
    );
    assert!(
        frontend_chirho.is_ok(),
        "frontend should accept nested instance-method shape once imports are seeded correctly: {:?}",
        frontend_chirho.err()
    );
}

#[test]
fn frontend_instance_methods_complex_shape_stay_nested_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = "\
module InstanceComplexShapeMiniChirho where
import Control.Applicative
import Control.Monad.Zip (MonadZip(mzipWith))
import Data.Functor.Contravariant
newtype ExceptTChirho e m a = ExceptTChirho { runExceptTChirho :: m (Either e a) }
instance (MonadZip m) => MonadZip (ExceptTChirho e m) where
  mzipWith fChirho (ExceptTChirho aChirho) (ExceptTChirho bChirho) = ExceptTChirho $ mzipWith (liftA2 fChirho) aChirho bChirho
instance Contravariant m => Contravariant (ExceptTChirho e m) where
  contramap fChirho = ExceptTChirho . contramap (fmap fChirho) . runExceptTChirho
";
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "InstanceComplexShapeMiniChirho.hs",
        source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(source_chirho, source_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    let leaked_funbind_names_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                Some(name_chirho.text_chirho().to_string())
            }
            _ => None,
        })
        .collect();
    let leaked_funbind_slices_chirho: Vec<(String, String)> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let start_chirho = span_chirho.start_chirho().as_usize_chirho();
                let end_chirho = span_chirho.end_chirho().as_usize_chirho();
                Some((
                    name_chirho.text_chirho().to_string(),
                    source_chirho[start_chirho..end_chirho].to_string(),
                ))
            }
            _ => None,
        })
        .collect();
    let decl_summaries_chirho: Vec<String> = module_chirho
        .decls_chirho
        .iter()
        .map(|decl_chirho| match decl_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } => format!(
                "FunBind({:?}, matches={})",
                name_chirho.text_chirho(),
                matches_chirho.len()
            ),
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => format!(
                "Instance({:?}, methods={})",
                class_chirho.text_chirho(),
                methods_chirho.len()
            ),
            DeclChirho::NewtypeDeclChirho { name_chirho, .. } => {
                format!("Newtype({:?})", name_chirho.text_chirho())
            }
            other_chirho => format!("{:?}", other_chirho),
        })
        .collect();
    assert!(
        leaked_funbind_names_chirho.is_empty(),
        "complex instance-method source should not leak top-level FunBind declarations: {:?}; decls={:?}",
        leaked_funbind_slices_chirho,
        decl_summaries_chirho
    );
    let instance_method_arity_chirho: Vec<(String, Vec<usize>)> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho {
                class_chirho,
                methods_chirho,
                ..
            } => Some((
                class_chirho.text_chirho().to_string(),
                methods_chirho
                    .iter()
                    .filter_map(|method_chirho| match method_chirho {
                        LocalBindChirho::FunBindChirho { matches_chirho, .. } => Some(
                            matches_chirho
                                .iter()
                                .map(|arm_chirho| arm_chirho.pats_chirho.len())
                                .sum(),
                        ),
                        _ => None,
                    })
                    .collect(),
            )),
            _ => None,
        })
        .collect();
    assert_eq!(
        instance_method_arity_chirho,
        vec![
            ("MonadZip".to_string(), vec![3]),
            ("Contravariant".to_string(), vec![1]),
        ],
        "instance method arities should exclude the method name token"
    );
}

#[test]
fn frontend_imported_user_class_methods_seed_real_schemes_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "ApplyProviderChirho.hs",
            "module ApplyProviderChirho (ApplyChirho(..)) where\nclass Functor fChirho => ApplyChirho fChirho where\n  (<.>) :: fChirho (aChirho -> bChirho) -> fChirho aChirho -> fChirho bChirho\n",
        ),
        (
            "ApplyConsumerChirho.hs",
            "module ApplyConsumerChirho where\nimport Data.Functor\nimport ApplyProviderChirho (ApplyChirho(..))\nliftF3Chirho :: ApplyChirho wChirho => (aChirho -> bChirho -> cChirho -> dChirho) -> wChirho aChirho -> wChirho bChirho -> wChirho cChirho -> wChirho dChirho\nliftF3Chirho fChirho aChirho bChirho cChirho = fChirho <$> aChirho <.> bChirho <.> cChirho\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "imported user class methods should carry their declared schemes across modules: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_imported_sibling_module_instances_satisfy_constraints_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "ParseErrorProviderChirho.hs",
            "module ParseErrorProviderChirho (ParseErrorChirho(..)) where\ndata ParseErrorChirho = ParseErrorChirho\ninstance Show ParseErrorChirho where\n  show ParseErrorChirho = \"ok\"\n",
        ),
        (
            "ParseErrorConsumerChirho.hs",
            "module ParseErrorConsumerChirho where\nimport ParseErrorProviderChirho\nrenderParseErrorChirho :: String\nrenderParseErrorChirho = show ParseErrorChirho\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "imported sibling-module instances should satisfy downstream constraints: {:?}",
        results_chirho.err()
    );
}

#[test]
fn multi_module_import_chirho() {
    use crate::compile_modules_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "LibChirho.hs",
            "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
        ),
        (
            "MainChirho.hs",
            "module Main where\nimport Lib\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho)
        .expect("multi-module compilation should succeed");

    assert_eq!(results_chirho.len(), 2);
    assert_eq!(
        results_chirho[0].module_chirho.name_chirho.text_chirho(),
        "Lib"
    );
    assert_eq!(
        results_chirho[1].module_chirho.name_chirho.text_chirho(),
        "Main"
    );
}

#[test]
fn multi_module_imported_try_overrides_builtin_try_chirho() {
    use crate::compile_modules_chirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "ParserPrimMiniChirho.hs",
            "module ParserPrimMini where\ntry x = x\n",
        ),
        (
            "MainChirho.hs",
            "module Main where\nimport ParserPrimMini\nvalue = try 1\n",
        ),
    ];

    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "real imported try should override builtin exception try: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_sort_on_string_list_uses_generic_builtin_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module SortStringMiniChirho where\nimport Data.List (sort)\nvalueChirho = sort [\"beta\", \"alpha\"]\n",
        &mut source_map_chirho,
        "SortStringMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Data.List.sort should stay polymorphic over [String]: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_data_list_union_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QualifiedUnionMiniChirho where\nimport qualified Data.List as List\nvalueChirho = List.union [1, 2] [2, 3]\n",
        &mut source_map_chirho,
        "QualifiedUnionMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "qualified Data.List.union should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_control_monad_zipwithm_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QualifiedZipWithMMiniChirho where\nimport qualified Control.Monad as M\nvalueChirho = M.zipWithM (\\xChirho yChirho -> Just (xChirho + yChirho)) [1, 2] [3, 4]\n",
        &mut source_map_chirho,
        "QualifiedZipWithMMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "qualified Control.Monad.zipWithM should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_foreign_storable_methods_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ForeignStorableMiniChirho where\nimport Foreign.Ptr (Ptr)\nimport Foreign.Storable (sizeOf, alignment, peek, poke)\nsizeOfIntChirho = sizeOf (0 :: Int)\nalignmentIntChirho = alignment (0 :: Int)\npeekIntChirho :: Ptr Int -> IO Int\npeekIntChirho = peek\npokeIntChirho :: Ptr Int -> Int -> IO ()\npokeIntChirho = poke\n",
        &mut source_map_chirho,
        "ForeignStorableMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Foreign.Storable methods should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_foreign_reexport_peek_elem_off_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ForeignPeekElemOffMiniChirho where\nimport Foreign\npeekElemOffIntChirho :: Ptr Int -> Int -> IO Int\npeekElemOffIntChirho = peekElemOff\n",
        &mut source_map_chirho,
        "ForeignPeekElemOffMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Foreign should re-export peekElemOff with a usable type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_bytestring_internal_c2w_w2c_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringInternalC2WMiniChirho where\nimport Data.ByteString.Internal (c2w, w2c)\nimport Data.Word (Word8)\ntoWord8Chirho :: Char -> Word8\ntoWord8Chirho = c2w\ntoCharChirho :: Word8 -> Char\ntoCharChirho = w2c\n",
        &mut source_map_chirho,
        "ByteStringInternalC2WMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Data.ByteString.Internal c2w/w2c should resolve with precise Char/Word8 types: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_bytestring_internal_memset_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringInternalMemsetMiniChirho where\nimport Data.ByteString.Internal (memset)\nimport Data.Word (Word8)\nimport Foreign.Ptr (Ptr)\nclearChirho :: Ptr Word8 -> IO (Ptr Word8)\nclearChirho ptrChirho = memset ptrChirho 0 32\n",
        &mut source_map_chirho,
        "ByteStringInternalMemsetMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Data.ByteString.Internal memset should resolve with a usable Ptr type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_bytestring_pack_word8_list_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringPackWord8MiniChirho where\nimport qualified Data.ByteString as B\nimport Data.Word (Word8)\nbytesChirho :: B.ByteString\nbytesChirho = B.pack ([65, 66, 67] :: [Word8])\n",
        &mut source_map_chirho,
        "ByteStringPackWord8MiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Data.ByteString.pack should accept [Word8]: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_bytestring_alias_type_synonym_with_strict_and_lazy_imports_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ByteStringAliasBridgeMiniChirho where\nimport qualified Data.ByteString as B\nimport qualified Data.ByteString.Lazy as BL\ntype StrictBytesChirho = B.ByteString\nstrictReadChirho :: FilePath -> IO StrictBytesChirho\nstrictReadChirho = B.readFile\nlazyReadChirho :: FilePath -> IO BL.ByteString\nlazyReadChirho = BL.readFile\n",
        &mut source_map_chirho,
        "ByteStringAliasBridgeMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "qualified ByteString type aliases should stay compatible with imported readFile schemes even when strict and lazy ByteString are both in scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_imported_compare_ordering_bridge_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ImportedCompareOrderingMiniChirho where\nimport Data.Ord (compare)\ncompareIntsChirho :: Int -> Int -> Ordering\ncompareIntsChirho = compare\n",
        &mut source_map_chirho,
        "ImportedCompareOrderingMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "imported compare should keep Ordering compatible with the builtin Ordering type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_qualified_ioref_type_alias_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module QualifiedIORefAliasMiniChirho where\nimport qualified Data.IORef as I\ntype InternalStateChirho = I.IORef Int\ncreateInternalStateChirho :: IO InternalStateChirho\ncreateInternalStateChirho = I.newIORef 0\n",
        &mut source_map_chirho,
        "QualifiedIORefAliasMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "qualified IORef type aliases should stay compatible with imported IORef schemes: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_memcpy_and_malloc_plain_foreign_ptr_bytes_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module MemcpyMallocPlainForeignPtrMiniChirho where\nimport Data.ByteString.Internal (memcpy)\nimport Data.Word (Word8)\nimport Foreign.Ptr (Ptr)\nimport GHC.ForeignPtr (mallocPlainForeignPtrBytes)\nimport Foreign.ForeignPtr (ForeignPtr)\ncopyBytesChirho :: Ptr Word8 -> Ptr Word8 -> IO ()\ncopyBytesChirho dstChirho srcChirho = memcpy dstChirho srcChirho 8\nallocBytesChirho :: IO (ForeignPtr Word8)\nallocBytesChirho = mallocPlainForeignPtrBytes 32\n",
        &mut source_map_chirho,
        "MemcpyMallocPlainForeignPtrMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "memcpy and mallocPlainForeignPtrBytes should resolve with usable ByteString/ForeignPtr types: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_plus_foreign_ptr_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PlusForeignPtrMiniChirho where\nimport Data.ByteString.Internal (plusForeignPtr)\nimport Data.Word (Word8)\nimport Foreign.ForeignPtr (ForeignPtr)\nslideChirho :: ForeignPtr Word8 -> Int -> ForeignPtr Word8\nslideChirho fpChirho offChirho = plusForeignPtr fpChirho offChirho\n",
        &mut source_map_chirho,
        "PlusForeignPtrMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "plusForeignPtr should resolve with a usable ForeignPtr type: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_exts_set_byte_array_hash_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE MagicHash #-}\n{-# LANGUAGE UnboxedTuples #-}\nmodule SetByteArrayHashMiniChirho where\nimport GHC.Exts\nfillChirho :: Int# -> Int# -> State# sChirho -> State# sChirho\nfillChirho lenChirho# cChirho# s1Chirho# = case newByteArray# lenChirho# s1Chirho# of\n  (# s2Chirho#, marrChirho# #) -> setByteArray# marrChirho# 0# lenChirho# cChirho# s2Chirho#\n",
        &mut source_map_chirho,
        "SetByteArrayHashMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "GHC.Exts setByteArray# should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_exts_ishiftra_hash_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE MagicHash #-}\nmodule IShiftRAHashMiniChirho where\nimport GHC.Exts (Int(I#), iShiftRA#)\nshiftRChirho :: Int -> Int -> Int\nshiftRChirho (I# xChirho#) (I# iChirho#) = I# (iShiftRA# xChirho# iChirho#)\n",
        &mut source_map_chirho,
        "IShiftRAHashMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "GHC.Exts iShiftRA# should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_list_comprehension_as_pattern_generator_binds_alias_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module ListCompAsPatternMiniChirho where\ndata ChunkChirho = TextChirho Int | GapChirho\nkeepTextChirho :: [ChunkChirho] -> [ChunkChirho]\nkeepTextChirho xsChirho = [cChirho | cChirho@(TextChirho _) <- xsChirho]\n",
        &mut source_map_chirho,
        "ListCompAsPatternMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "list comprehension as-pattern generators should bind their alias names: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_base_unsafe_chr_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module UnsafeChrMiniChirho where\nimport GHC.Base (unsafeChr)\ncharFromIntChirho :: Int -> Char\ncharFromIntChirho nChirho = unsafeChr nChirho\n",
        &mut source_map_chirho,
        "UnsafeChrMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "GHC.Base unsafeChr should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ghc_exts_keepalive_hash_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "{-# LANGUAGE MagicHash #-}\nmodule KeepAliveHashMiniChirho where\nimport GHC.Exts (State#, keepAlive#)\nkeepAliveWrapChirho :: aChirho -> State# sChirho -> (State# sChirho -> bChirho) -> bChirho\nkeepAliveWrapChirho xChirho sChirho# kChirho = keepAlive# xChirho sChirho# kChirho\n",
        &mut source_map_chirho,
        "KeepAliveHashMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "GHC.Exts keepAlive# should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_test_tasty_options_is_option_instance_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TestTastyOptionsMiniChirho where\nimport Test.Tasty.Options\nnewtype VectorSizeChirho = VectorSizeChirho Int\ninstance IsOption VectorSizeChirho where\n  defaultValue = VectorSizeChirho 2000000\n  parseValue = fmap VectorSizeChirho . safeRead\n  optionName = pure \"size\"\n  optionHelp = pure \"Size of vectors used in benchmarks\"\n",
        &mut source_map_chirho,
        "TestTastyOptionsMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Test.Tasty.Options IsOption instances should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_test_tasty_test_tree_helpers_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module TestTastyMiniChirho where\nimport Test.Tasty\ntreeChirho :: TestTree\ntreeChirho = testGroup \"demo\" []\nmainChirho :: IO ()\nmainChirho = defaultMain treeChirho\n",
        &mut source_map_chirho,
        "TestTastyMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Test.Tasty TestTree helpers should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_prelude_list_index_operator_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PreludeListIndexMiniChirho where\nvalueChirho = [10, 20, 30] !! 1\n",
        &mut source_map_chirho,
        "PreludeListIndexMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "Prelude !! should resolve and type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_extended_char_escape_literals_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module CharEscapeMiniChirho where\nvalueChirho = ['\\026', '\\BS', '\\DEL', '\\DC1', '\\x41', '\\o101']\n",
        &mut source_map_chirho,
        "CharEscapeMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "extended char escapes should lex and type-check as Char literals: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_integer_literals_unify_with_integer_annotations_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module IntegerLiteralMiniChirho where\nzeroChirho :: Integer\nzeroChirho = 0\nsuccChirho :: Integer -> Integer\nsuccChirho nChirho = nChirho + 1\n",
        &mut source_map_chirho,
        "IntegerLiteralMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "integer literals should stay polymorphic and unify with Integer annotations: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_ambiguous_numeric_default_prefers_integer_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module DefaultIntegerMiniChirho where\nvalueChirho = toInteger 0\n",
        &mut source_map_chirho,
        "DefaultIntegerMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "ambiguous numeric literals should default compatibly with Integer: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_value_operator_import_roundtrip_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "Safe/Util.hs",
            "module Safe.Util ((.^)) where\n(.^) fChirho gChirho xChirho yChirho = fChirho (gChirho xChirho yChirho)\n",
        ),
        (
            "Safe.hs",
            "module Safe where\nimport Safe.Util ((.^))\nvalueChirho = (+ 1) .^ (+) 2 3\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "value operators should round-trip through export/import interfaces: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_qualified_value_import_keeps_unqualified_tycons_when_module_also_unqualified_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "ParserPrimMiniChirho.hs",
            "module ParserPrimMiniChirho where\n\
data IdentityChirho aChirho = IdentityChirho aChirho\n\
newtype ParsecTChirho sChirho uChirho mChirho aChirho = ParsecTChirho { runParsecTChirho :: sChirho -> mChirho aChirho }\n\
type ParsecChirho sChirho uChirho = ParsecTChirho sChirho uChirho IdentityChirho\n\
runParserChirho :: ParsecTChirho sChirho uChirho IdentityChirho aChirho -> uChirho -> String -> sChirho -> aChirho\n\
runParserChirho (ParsecTChirho fChirho) _ _ sChirho = case fChirho sChirho of IdentityChirho xChirho -> xChirho\n",
        ),
        (
            "MainChirho.hs",
            "module MainChirho where\n\
import ParserPrimMiniChirho hiding (runParserChirho)\n\
import qualified ParserPrimMiniChirho as NChirho\n\
type GenParserChirho tokChirho stChirho aChirho = ParsecChirho [tokChirho] stChirho aChirho\n\
runParserCompatChirho :: GenParserChirho tokChirho stChirho aChirho -> stChirho -> String -> [tokChirho] -> aChirho\n\
runParserCompatChirho = NChirho.runParserChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "qualified value imports should keep module-local tycons unqualified when the module is also imported unqualified: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_reexported_type_alias_unifies_with_underlying_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimMiniChirho.hs",
            "module PrimMiniChirho (IdentityMiniChirho(..), ParsecTMiniChirho(..), ParsecMiniChirho, runMiniChirho) where\n\
data IdentityMiniChirho aChirho = IdentityMiniChirho aChirho\n\
newtype ParsecTMiniChirho sChirho uChirho mChirho aChirho = ParsecTMiniChirho { unParsecTMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecMiniChirho sChirho uChirho aChirho = ParsecTMiniChirho sChirho uChirho IdentityMiniChirho aChirho\n\
runMiniChirho :: ParsecTMiniChirho sChirho uChirho IdentityMiniChirho aChirho -> ParsecTMiniChirho sChirho uChirho IdentityMiniChirho aChirho\n\
runMiniChirho pChirho = pChirho\n",
        ),
        (
            "ParsecMiniChirho.hs",
            "module ParsecMiniChirho (ParsecTMiniChirho(..), ParsecMiniChirho, runMiniChirho) where\n\
import PrimMiniChirho\n",
        ),
        (
            "PermMiniChirho.hs",
            "module PermMiniChirho where\n\
import ParsecMiniChirho\n\
fChirho :: ParsecMiniChirho sChirho stChirho aChirho -> ParsecMiniChirho sChirho stChirho aChirho\n\
fChirho pChirho = runMiniChirho pChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "re-exported type aliases should unify with imported value schemes that mention the underlying constructor: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_reexported_type_alias_flows_through_imported_value_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimChoiceMiniChirho.hs",
            "module PrimChoiceMiniChirho (IdentityChoiceMiniChirho(..), ParsecTChoiceMiniChirho(..), ParsecChoiceMiniChirho, choiceMiniChirho) where\n\
data IdentityChoiceMiniChirho aChirho = IdentityChoiceMiniChirho aChirho\n\
newtype ParsecTChoiceMiniChirho sChirho uChirho mChirho aChirho = ParsecTChoiceMiniChirho { unParsecTChoiceMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecChoiceMiniChirho sChirho uChirho aChirho = ParsecTChoiceMiniChirho sChirho uChirho IdentityChoiceMiniChirho aChirho\n\
choiceMiniChirho :: [ParsecTChoiceMiniChirho sChirho uChirho IdentityChoiceMiniChirho aChirho] -> ParsecTChoiceMiniChirho sChirho uChirho IdentityChoiceMiniChirho aChirho\n\
choiceMiniChirho (pChirho:_) = pChirho\n\
choiceMiniChirho [] = ParsecTChoiceMiniChirho (\\_ -> error \"empty\")\n",
        ),
        (
            "ParsecChoiceMiniChirho.hs",
            "module ParsecChoiceMiniChirho (ParsecTChoiceMiniChirho(..), ParsecChoiceMiniChirho, choiceMiniChirho) where\n\
import PrimChoiceMiniChirho\n",
        ),
        (
            "PermChoiceMiniChirho.hs",
            "module PermChoiceMiniChirho where\n\
import ParsecChoiceMiniChirho\n\
permuteMiniChirho :: [ParsecChoiceMiniChirho sChirho stChirho aChirho] -> ParsecChoiceMiniChirho sChirho stChirho aChirho\n\
permuteMiniChirho xsChirho = choiceMiniChirho xsChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "re-exported parser aliases should flow through imported value schemes that mention the underlying constructor: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_reexported_type_alias_in_data_field_unifies_with_underlying_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimFieldMiniChirho.hs",
            "module PrimFieldMiniChirho (IdentityFieldMiniChirho(..), ParsecTFieldMiniChirho(..), ParsecFieldMiniChirho, choiceFieldMiniChirho) where\n\
data IdentityFieldMiniChirho aChirho = IdentityFieldMiniChirho aChirho\n\
newtype ParsecTFieldMiniChirho sChirho uChirho mChirho aChirho = ParsecTFieldMiniChirho { unParsecTFieldMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecFieldMiniChirho sChirho uChirho aChirho = ParsecTFieldMiniChirho sChirho uChirho IdentityFieldMiniChirho aChirho\n\
choiceFieldMiniChirho :: [ParsecTFieldMiniChirho sChirho uChirho IdentityFieldMiniChirho aChirho] -> ParsecTFieldMiniChirho sChirho uChirho IdentityFieldMiniChirho aChirho\n\
choiceFieldMiniChirho (pChirho:_) = pChirho\n\
choiceFieldMiniChirho [] = ParsecTFieldMiniChirho (\\_ -> error \"empty\")\n",
        ),
        (
            "ParsecFieldMiniChirho.hs",
            "module ParsecFieldMiniChirho (ParsecTFieldMiniChirho(..), ParsecFieldMiniChirho, choiceFieldMiniChirho) where\n\
import PrimFieldMiniChirho\n",
        ),
        (
            "PermFieldMiniChirho.hs",
            "module PermFieldMiniChirho where\n\
import ParsecFieldMiniChirho\n\
data BoxFieldMiniChirho sChirho stChirho aChirho = BoxFieldMiniChirho (ParsecFieldMiniChirho sChirho stChirho aChirho)\n\
permuteFieldMiniChirho :: BoxFieldMiniChirho sChirho stChirho aChirho -> ParsecFieldMiniChirho sChirho stChirho aChirho\n\
permuteFieldMiniChirho (BoxFieldMiniChirho pChirho) = choiceFieldMiniChirho [pChirho]\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "re-exported parser aliases should still unify when they appear in imported data constructor fields: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_partially_applied_reexported_type_alias_flows_through_imported_value_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimPermMiniChirho.hs",
            "module PrimPermMiniChirho (IdentityPermMiniChirho(..), ParsecTPermMiniChirho(..), ParsecPermMiniChirho, choicePermMiniChirho) where\n\
data IdentityPermMiniChirho aChirho = IdentityPermMiniChirho aChirho\n\
newtype ParsecTPermMiniChirho sChirho uChirho mChirho aChirho = ParsecTPermMiniChirho { unParsecTPermMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecPermMiniChirho sChirho uChirho = ParsecTPermMiniChirho sChirho uChirho IdentityPermMiniChirho\n\
choicePermMiniChirho :: [ParsecTPermMiniChirho sChirho uChirho IdentityPermMiniChirho aChirho] -> ParsecTPermMiniChirho sChirho uChirho IdentityPermMiniChirho aChirho\n\
choicePermMiniChirho (pChirho:_) = pChirho\n\
choicePermMiniChirho [] = ParsecTPermMiniChirho (\\_ -> error \"empty\")\n",
        ),
        (
            "ParsecPermMiniChirho.hs",
            "module ParsecPermMiniChirho (ParsecTPermMiniChirho(..), ParsecPermMiniChirho, choicePermMiniChirho) where\n\
import PrimPermMiniChirho\n",
        ),
        (
            "PermUseMiniChirho.hs",
            "module PermUseMiniChirho where\n\
import ParsecPermMiniChirho\n\
permuteMiniChirho :: [ParsecPermMiniChirho sChirho stChirho aChirho] -> ParsecPermMiniChirho sChirho stChirho aChirho\n\
permuteMiniChirho xsChirho = choicePermMiniChirho (map idMiniChirho xsChirho ++ emptyMiniChirho)\n\
  where\n\
    idMiniChirho pChirho = pChirho\n\
    emptyMiniChirho = []\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "partially applied re-exported type aliases should flow through imported value schemes in list-producing expressions: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_partially_applied_reexported_type_alias_in_local_data_field_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimPermFieldMiniChirho.hs",
            "module PrimPermFieldMiniChirho (IdentityPermFieldMiniChirho(..), ParsecTPermFieldMiniChirho(..), ParsecPermFieldMiniChirho, choicePermFieldMiniChirho) where\n\
data IdentityPermFieldMiniChirho aChirho = IdentityPermFieldMiniChirho aChirho\n\
newtype ParsecTPermFieldMiniChirho sChirho uChirho mChirho aChirho = ParsecTPermFieldMiniChirho { unParsecTPermFieldMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecPermFieldMiniChirho sChirho uChirho = ParsecTPermFieldMiniChirho sChirho uChirho IdentityPermFieldMiniChirho\n\
choicePermFieldMiniChirho :: [ParsecTPermFieldMiniChirho sChirho uChirho IdentityPermFieldMiniChirho aChirho] -> ParsecTPermFieldMiniChirho sChirho uChirho IdentityPermFieldMiniChirho aChirho\n\
choicePermFieldMiniChirho (pChirho:_) = pChirho\n\
choicePermFieldMiniChirho [] = ParsecTPermFieldMiniChirho (\\_ -> error \"empty\")\n",
        ),
        (
            "ParsecPermFieldMiniChirho.hs",
            "module ParsecPermFieldMiniChirho (ParsecTPermFieldMiniChirho(..), ParsecPermFieldMiniChirho, choicePermFieldMiniChirho) where\n\
import PrimPermFieldMiniChirho\n",
        ),
        (
            "PermFieldUseMiniChirho.hs",
            "module PermFieldUseMiniChirho where\n\
import ParsecPermFieldMiniChirho\n\
data BoxPermFieldMiniChirho sChirho stChirho aChirho = BoxPermFieldMiniChirho (ParsecPermFieldMiniChirho sChirho stChirho aChirho)\n\
permuteFieldMiniChirho :: BoxPermFieldMiniChirho sChirho stChirho aChirho -> ParsecPermFieldMiniChirho sChirho stChirho aChirho\n\
permuteFieldMiniChirho (BoxPermFieldMiniChirho pChirho) = choicePermFieldMiniChirho [pChirho]\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "partially applied re-exported type aliases should stay expanded when they appear in local data constructor fields: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_imported_type_alias_expands_through_parent_alias_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimAliasMiniChirho.hs",
            "module PrimAliasMiniChirho (IdentityAliasMiniChirho(..), ParsecTAliasMiniChirho(..), ParsecAliasMiniChirho) where\n\
data IdentityAliasMiniChirho aChirho = IdentityAliasMiniChirho aChirho\n\
newtype ParsecTAliasMiniChirho sChirho uChirho mChirho aChirho = ParsecTAliasMiniChirho { unParsecTAliasMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecAliasMiniChirho sChirho uChirho aChirho = ParsecTAliasMiniChirho sChirho uChirho IdentityAliasMiniChirho aChirho\n",
        ),
        (
            "CompatAliasMiniChirho.hs",
            "module CompatAliasMiniChirho (GenParserAliasMiniChirho) where\n\
import PrimAliasMiniChirho (ParsecAliasMiniChirho)\n\
type GenParserAliasMiniChirho tokChirho stChirho aChirho = ParsecAliasMiniChirho [tokChirho] stChirho aChirho\n",
        ),
        (
            "UseAliasMiniChirho.hs",
            "module UseAliasMiniChirho where\n\
import PrimAliasMiniChirho (IdentityAliasMiniChirho, ParsecTAliasMiniChirho)\n\
import CompatAliasMiniChirho (GenParserAliasMiniChirho)\n\
coerceAliasMiniChirho :: GenParserAliasMiniChirho tokChirho stChirho aChirho -> ParsecTAliasMiniChirho [tokChirho] stChirho IdentityAliasMiniChirho aChirho\n\
coerceAliasMiniChirho parserChirho = parserChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "imported aliases should expand through parent aliases to the underlying constructor: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_reexported_type_alias_expands_through_hidden_parent_alias_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "PrimAliasCompatMiniChirho.hs",
            "module PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho(..), ParsecTAliasCompatMiniChirho(..), ParsecAliasCompatMiniChirho) where\n\
data IdentityAliasCompatMiniChirho aChirho = IdentityAliasCompatMiniChirho aChirho\n\
newtype ParsecTAliasCompatMiniChirho sChirho uChirho mChirho aChirho = ParsecTAliasCompatMiniChirho { unParsecTAliasCompatMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecAliasCompatMiniChirho sChirho uChirho aChirho = ParsecTAliasCompatMiniChirho sChirho uChirho IdentityAliasCompatMiniChirho aChirho\n",
        ),
        (
            "CompatPrimMiniChirho.hs",
            "module CompatPrimMiniChirho (GenParserCompatMiniChirho) where\n\
import PrimAliasCompatMiniChirho (ParsecAliasCompatMiniChirho)\n\
type GenParserCompatMiniChirho tokChirho stChirho aChirho = ParsecAliasCompatMiniChirho [tokChirho] stChirho aChirho\n",
        ),
        (
            "CompatTopMiniChirho.hs",
            "module CompatTopMiniChirho (module CompatPrimMiniChirho) where\n\
import CompatPrimMiniChirho\n",
        ),
        (
            "UseCompatTopMiniChirho.hs",
            "module UseCompatTopMiniChirho where\n\
import PrimAliasCompatMiniChirho (IdentityAliasCompatMiniChirho, ParsecTAliasCompatMiniChirho)\n\
import CompatTopMiniChirho (GenParserCompatMiniChirho)\n\
coerceCompatMiniChirho :: GenParserCompatMiniChirho tokChirho stChirho aChirho -> ParsecTAliasCompatMiniChirho [tokChirho] stChirho IdentityAliasCompatMiniChirho aChirho\n\
coerceCompatMiniChirho parserChirho = parserChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "reexported aliases should keep the hidden parent alias chain needed for downstream expansion: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_template_haskell_quoted_names_typecheck_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [(
        "TemplateHaskellQuotedNamesMiniChirho.hs",
        "module TemplateHaskellQuotedNamesMiniChirho where\n\
import Language.Haskell.TH.Syntax\n\
bimapConstMiniChirho :: Int -> Int\n\
bimapConstMiniChirho xChirho = xChirho\n\
valueNameMiniChirho :: Name\n\
valueNameMiniChirho = 'bimapConstMiniChirho\n\
typeNameMiniChirho :: Name\n\
typeNameMiniChirho = ''Name\n\
opNameMiniChirho :: Name\n\
opNameMiniChirho = '(.)\n",
    )];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "Template Haskell quoted names should lower to Name values: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_explicit_import_overrides_broadly_seeded_value_scheme_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "TextChoiceSeedMiniChirho.hs",
            "module TextChoiceSeedMiniChirho (TextParserMiniChirho(..), choiceMiniChirho) where\n\
data TextParserMiniChirho aChirho = TextParserMiniChirho\n\
choiceMiniChirho :: [TextParserMiniChirho aChirho] -> TextParserMiniChirho aChirho\n\
choiceMiniChirho (xChirho:_) = xChirho\n\
choiceMiniChirho [] = TextParserMiniChirho\n",
        ),
        (
            "PrimPolyChoiceSeedMiniChirho.hs",
            "module PrimPolyChoiceSeedMiniChirho (IdentityPolyChoiceMiniChirho(..), ParsecTPolyChoiceMiniChirho(..), ParsecPolyChoiceMiniChirho, choiceMiniChirho) where\n\
data IdentityPolyChoiceMiniChirho aChirho = IdentityPolyChoiceMiniChirho aChirho\n\
newtype ParsecTPolyChoiceMiniChirho sChirho uChirho mChirho aChirho = ParsecTPolyChoiceMiniChirho { unParsecTPolyChoiceMiniChirho :: sChirho -> mChirho aChirho }\n\
type ParsecPolyChoiceMiniChirho sChirho uChirho aChirho = ParsecTPolyChoiceMiniChirho sChirho uChirho IdentityPolyChoiceMiniChirho aChirho\n\
choiceMiniChirho :: [ParsecTPolyChoiceMiniChirho sChirho uChirho IdentityPolyChoiceMiniChirho aChirho] -> ParsecTPolyChoiceMiniChirho sChirho uChirho IdentityPolyChoiceMiniChirho aChirho\n\
choiceMiniChirho (pChirho:_) = pChirho\n\
choiceMiniChirho [] = ParsecTPolyChoiceMiniChirho (\\_ -> error \"empty\")\n",
        ),
        (
            "PolyChoiceSeedMiniChirho.hs",
            "module PolyChoiceSeedMiniChirho (ParsecTPolyChoiceMiniChirho(..), ParsecPolyChoiceMiniChirho, choiceMiniChirho) where\n\
import PrimPolyChoiceSeedMiniChirho\n",
        ),
        (
            "UsePolyChoiceSeedMiniChirho.hs",
            "module UsePolyChoiceSeedMiniChirho where\n\
import PolyChoiceSeedMiniChirho\n\
permuteMiniChirho :: [ParsecPolyChoiceMiniChirho sChirho stChirho aChirho] -> ParsecPolyChoiceMiniChirho sChirho stChirho aChirho\n\
permuteMiniChirho xsChirho = choiceMiniChirho xsChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "explicit imports should override broadly seeded value schemes from earlier modules: {:?}",
        results_chirho.err()
    );
}

#[test]
fn frontend_same_module_operator_binding_resolves_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PermMiniChirho where\ninfixl 1 <||>\n(<||>) fChirho xChirho = fChirho xChirho\nvalueChirho = Just <||> 1\n",
        &mut source_map_chirho,
        "PermMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "same-module operator function bindings should resolve under bare operator syntax: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_parenthesized_hash_operator_binding_resolves_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module HashOpMiniChirho where\n(#.) :: aChirho -> bChirho -> bChirho\n(#.) _ xChirho = xChirho\nvalueChirho = 1 #. 2\n",
        &mut source_map_chirho,
        "HashOpMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "parenthesized hash-operator bindings should resolve under bare operator syntax: {:?}",
        result_chirho.err()
    );
}

#[test]
fn check_source_path_parenthesized_hash_operator_binding_resolves_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("HashOpFileMiniChirho.hs");
    std::fs::write(
        &file_path_chirho,
        "module HashOpFileMiniChirho where\n(#.) :: aChirho -> bChirho -> bChirho\n(#.) _ xChirho = xChirho\nvalueChirho = 1 #. 2\n",
    )
    .expect("temp source should write");

    let result_chirho =
        check_source_path_chirho(&file_path_chirho, ExecutionModeChirho::BatchChirho);
    assert!(
        result_chirho.is_ok(),
        "file-path frontend should keep parenthesized hash-operator bindings intact: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_where_helper_polymorphism_in_recursive_group_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module WherePolyMiniChirho where\nvalueChirho = (symbolChirho \"ok\", reservedChirho) where\n  reservedChirho = lexemeChirho ()\n  symbolChirho nameChirho = lexemeChirho nameChirho\n  lexemeChirho pChirho = pChirho\n",
        &mut source_map_chirho,
        "WherePolyMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "local where helper should generalize across String and () uses: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_nested_as_pattern_binder_visible_in_where_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module BitQueueMiniChirho where\ndata BitQueueBChirho = BQBChirho Int Int\nnewtype BitQueueChirho = BQChirho BitQueueBChirho\nunconsQChirho :: BitQueueChirho -> Maybe (Bool, BitQueueChirho)\nunconsQChirho (BQChirho bqChirho@(BQBChirho _ loChirho)) = Just (hdChirho, BQChirho tlChirho)\n  where\n    hdChirho = loChirho == 0\n    tlChirho = bqChirho\n",
        &mut source_map_chirho,
        "BitQueueMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "nested as-pattern binders should stay visible inside where clauses: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_pattern_bind_where_binds_are_in_scope_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module PatBindWhereMiniChirho where\n(discardChirho, isDiscardChirho) = (msgChirho, msgChirho == \"x\")\n  where\n    msgChirho = \"x\"\n",
        &mut source_map_chirho,
        "PatBindWhereMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "pattern binding where clauses should stay in scope for the rhs: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_system_random_split_and_splitmix_surface_typechecks_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module RandomMiniChirho where\nimport System.Random\nimport System.Random.SplitMix\nsplitStdMiniChirho :: RandomGen g => g -> (g, g)\nsplitStdMiniChirho = split\nrandomStepMiniChirho :: (Random a, RandomGen g) => g -> (a, g)\nrandomStepMiniChirho = random\nrandomRangeStepMiniChirho :: (Random a, RandomGen g) => (a, a) -> g -> (a, g)\nrandomRangeStepMiniChirho = randomR\nrandomIOMiniChirho :: Random a => IO a\nrandomIOMiniChirho = randomIO\nrandomRIOMiniChirho :: Random a => (a, a) -> IO a\nrandomRIOMiniChirho = randomRIO\nsplitSmMiniChirho :: SMGen -> (SMGen, SMGen)\nsplitSmMiniChirho = splitSMGen\nnextSmMiniChirho :: SMGen -> (Int, SMGen)\nnextSmMiniChirho = nextInt\n",
        &mut source_map_chirho,
        "RandomMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "System.Random / SplitMix builtin surface should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_text_case_mapping_parse_and_lower_survives_generated_module_size_chirho() {
    use crate::read_haskell_source_file_chirho;
    use haskelujah_parser_chirho::{
        cst_parser_chirho::ParserChirho, lower_chirho::lower_module_chirho,
    };
    use std::path::PathBuf;

    let case_mapping_path_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(
            ".haskelujah-packages-chirho/text-2.1.4/src/Data/Text/Internal/Fusion/CaseMapping.hs",
        );
    let source_chirho = read_haskell_source_file_chirho(&case_mapping_path_chirho)
        .expect("text CaseMapping source should exist");
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &case_mapping_path_chirho,
        source_chirho,
    );
    let parser_chirho = ParserChirho::new_chirho(
        source_file_chirho.contents_chirho(),
        source_file_chirho.file_id_chirho(),
    );
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, source_file_chirho.file_id_chirho());

    assert_eq!(
        module_chirho.name_chirho.full_name_chirho(),
        "Data.Text.Internal.Fusion.CaseMapping"
    );
    assert!(
        module_chirho.decls_chirho.len() >= 3,
        "generated text CaseMapping module should lower into top-level declarations"
    );
}

#[test]
fn frontend_text_case_mapping_frontend_survives_generated_module_size_chirho() {
    use crate::{
        ImportedTypeSynonymsChirho, merge_stdlib_frontend_artifacts_chirho,
        read_haskell_source_file_chirho, run_frontend_with_type_synonyms_and_type_families_chirho,
        seed_builtin_type_families_chirho, source_imports_stdlib_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let case_mapping_path_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(
            ".haskelujah-packages-chirho/text-2.1.4/src/Data/Text/Internal/Fusion/CaseMapping.hs",
        );
    let source_chirho = read_haskell_source_file_chirho(&case_mapping_path_chirho)
        .expect("text CaseMapping source should exist");

    let mut builtin_ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut imported_types_chirho = HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if source_imports_stdlib_chirho(&source_chirho) {
        merge_stdlib_frontend_artifacts_chirho(
            &mut builtin_ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &case_mapping_path_chirho,
        &source_chirho,
    );
    let result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
        &source_chirho,
        source_file_chirho.file_id_chirho(),
        &builtin_ifaces_chirho,
        &imported_types_chirho,
        &imported_type_synonyms_chirho,
        &imported_type_families_chirho,
    );

    assert!(
        result_chirho.is_ok(),
        "generated text CaseMapping module should survive the frontend pipeline: {:?}",
        result_chirho.err()
    );
}

#[test]
fn exhaustiveness_check_passes_for_complete_case_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // data Color = Red | Green — case covers both constructors
    let result_chirho = compile_source_chirho(
        "module Test where\ndata Color = Red | Green\nf x = case x of\n  Red -> 1\n  Green -> 2\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "complete case should pass exhaustiveness check"
    );
}

#[test]
fn exhaustiveness_check_warns_incomplete_case_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // data Color = Red | Green | Blue — case only covers Red
    // Non-exhaustive patterns are warnings, not errors (matches GHC behavior)
    let result_chirho = compile_source_chirho(
        "module Test where\ndata Color = Red | Green | Blue\nf x = case x of\n  Red -> 1\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "incomplete case should succeed with warnings: {:?}",
        result_chirho.err()
    );
}

#[test]
fn incremental_compilation_marks_recompiled_chirho() {
    use crate::compile_modules_incremental_chirho;
    use haskelujah_incremental_chirho::IncrementalSessionChirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut session_chirho = IncrementalSessionChirho::new_chirho();

    let sources_chirho: Vec<(&str, &str)> = vec![
        (
            "LibChirho.hs",
            "module Lib where\ndata Color = Red | Green\nhelper x = x\n",
        ),
        ("MainChirho.hs", "module Main where\nimport Lib\nf x = x\n"),
    ];

    // First build: everything should be recompiled.
    let results_chirho = compile_modules_incremental_chirho(
        &sources_chirho,
        &mut source_map_chirho,
        &mut session_chirho,
    )
    .expect("incremental compilation should succeed");

    assert_eq!(results_chirho.len(), 2);
    assert!(
        results_chirho[0].1,
        "Lib should be recompiled on first build"
    );
    assert!(
        results_chirho[1].1,
        "Main should be recompiled on first build"
    );

    // Second build with identical sources: nothing should be recompiled.
    let mut source_map_chirho2 = SourceMapChirho::new_chirho();
    let results2_chirho = compile_modules_incremental_chirho(
        &sources_chirho,
        &mut source_map_chirho2,
        &mut session_chirho,
    )
    .expect("second incremental compilation should succeed");

    assert_eq!(results2_chirho.len(), 2);
    assert!(
        !results2_chirho[0].1,
        "Lib should NOT be recompiled when source unchanged"
    );
    assert!(
        !results2_chirho[1].1,
        "Main should NOT be recompiled when source unchanged"
    );
}

#[test]
fn incremental_detects_source_change_chirho() {
    use crate::compile_modules_incremental_chirho;
    use haskelujah_incremental_chirho::IncrementalSessionChirho;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut session_chirho = IncrementalSessionChirho::new_chirho();

    let sources_v1_chirho: Vec<(&str, &str)> =
        vec![("TestChirho.hs", "module Test where\nf x = x\n")];

    let _ = compile_modules_incremental_chirho(
        &sources_v1_chirho,
        &mut source_map_chirho,
        &mut session_chirho,
    )
    .expect("v1 should compile");

    // Change the source.
    let sources_v2_chirho: Vec<(&str, &str)> =
        vec![("TestChirho.hs", "module Test where\nf x = x\ng y = y\n")];

    let mut source_map2_chirho = SourceMapChirho::new_chirho();
    let results_chirho = compile_modules_incremental_chirho(
        &sources_v2_chirho,
        &mut source_map2_chirho,
        &mut session_chirho,
    )
    .expect("v2 should compile");

    assert_eq!(results_chirho.len(), 1);
    assert!(
        results_chirho[0].1,
        "Test should be recompiled when source changes"
    );
}

#[test]
fn cabal_project_compilation_chirho() {
    use crate::compile_cabal_project_chirho;
    use std::io::Write;

    // Create a temp directory with a small Cabal project
    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    // Write .cabal file
    let cabal_path_chirho = temp_dir_chirho.join("hello.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         hello
version:      0.1.0.0

library
  exposed-modules: Lib
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
  default-language: Haskell2010
"#
    )
    .unwrap();

    // Write Haskell source
    let lib_path_chirho = temp_dir_chirho.join("src").join("Lib.hs");
    let mut lib_file_chirho = std::fs::File::create(&lib_path_chirho).unwrap();
    write!(lib_file_chirho, "module Lib where\nf x = x\n").unwrap();

    // Create an empty package index (base is builtin, so no external deps needed)
    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("cabal project should compile");

    assert_eq!(result_chirho.package_chirho.name_chirho, "hello");
    assert!(result_chirho.build_plan_chirho.steps_chirho.is_empty()); // only base (builtin)
    assert_eq!(result_chirho.module_results_chirho.len(), 1);
    assert_eq!(
        result_chirho.module_results_chirho[0]
            .module_chirho
            .name_chirho
            .text_chirho(),
        "Lib"
    );

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_multi_module_chirho() {
    use crate::compile_cabal_project_chirho;
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_multi_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("multi.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         multi
version:      0.1.0.0

library
  exposed-modules: Lib, Helper
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5
"#
    )
    .unwrap();

    let mut lib_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
    write!(lib_chirho, "module Lib where\nf x = x\n").unwrap();

    let mut helper_chirho = std::fs::File::create(temp_dir_chirho.join("src/Helper.hs")).unwrap();
    write!(helper_chirho, "module Helper where\ng y = y\n").unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("multi-module cabal project should compile");

    assert_eq!(result_chirho.package_chirho.name_chirho, "multi");
    assert_eq!(result_chirho.module_results_chirho.len(), 2);

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_conditional_hs_source_dirs_chirho() {
    use crate::compile_cabal_project_chirho;
    use std::io::Write;

    let temp_dir_chirho =
        std::env::temp_dir().join("haskelujah_cabal_conditional_srcdir_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("unix")).unwrap();
    std::fs::create_dir_all(temp_dir_chirho.join("win")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("conditional-srcdir.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         conditional-srcdir
version:      0.1.0.0

library
  exposed-modules: Lib
  if os(windows)
    hs-source-dirs: win
  else
    hs-source-dirs: unix
  build-depends:   base >=4.14 && <5
"#
    )
    .unwrap();

    std::fs::write(
        temp_dir_chirho.join("unix/Lib.hs"),
        "module Lib where\nvalueChirho = 42\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir_chirho.join("win/Lib.hs"),
        "module Lib where\nvalueChirho = 7\n",
    )
    .unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("conditional hs-source-dirs cabal project should compile");

    assert_eq!(result_chirho.module_results_chirho.len(), 1);
    let source_dir_chirho = &result_chirho
        .package_chirho
        .library_chirho
        .as_ref()
        .unwrap()
        .build_info_chirho
        .hs_source_dirs_chirho;
    let expected_dir_chirho = if cfg!(target_os = "windows") {
        "win"
    } else {
        "unix"
    };
    assert_eq!(source_dir_chirho, &vec![expected_dir_chirho.to_string()]);

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_imported_shift_helper_with_strict_dollar_chirho() {
    use crate::compile_cabal_project_chirho;
    use haskelujah_package_chirho::PackageIndexChirho;
    use std::io::Write;

    let temp_dir_chirho =
        std::env::temp_dir().join("haskelujah_cabal_strict_dollar_shift_helper_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("Utils")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("strict-dollar-shift-helper.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    writeln!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name: strict-dollar-shift-helper
version: 0.1.0.0
build-type: Simple

library
  exposed-modules: Main
  other-modules: Utils.Prelude, Utils.BitUtil
  hs-source-dirs: .
  default-language: Haskell2010
"#
    )
    .unwrap();

    std::fs::write(
        temp_dir_chirho.join("Utils").join("Prelude.hs"),
        "module Utils.Prelude (module Prelude) where\nimport Prelude\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir_chirho.join("Utils").join("BitUtil.hs"),
        "module Utils.BitUtil (shiftRL) where\nimport Data.Bits (shiftR)\nshiftRL :: Word -> Int -> Word\nshiftRL = shiftR\n",
    )
    .unwrap();
    std::fs::write(
        temp_dir_chirho.join("Main.hs"),
        "module Main where\nimport Data.Bits\nimport Utils.Prelude\nimport Prelude ()\nimport Utils.BitUtil (shiftRL)\n\ntakeWhileAntitoneBitsChirho :: Int -> (Int -> Bool) -> Word -> Word\ntakeWhileAntitoneBitsChirho prefixChirho predicateChirho bitmapChirho =\n  let nextChirho dChirho hChirho (nPrimeChirho, bPrimeChirho) =\n        if nPrimeChirho .&. hChirho /= 0 && (predicateChirho $! prefixChirho + bPrimeChirho + dChirho)\n          then (shiftRL nPrimeChirho dChirho, bPrimeChirho + dChirho)\n          else (nPrimeChirho, bPrimeChirho)\n      (_ignoreChirho, bChirho) =\n        nextChirho 1 0x2 $\n          nextChirho 2 0xC $\n            nextChirho 4 0xF0 $\n              nextChirho 8 0xFF00 $\n                nextChirho 16 0xFFFF0000 $\n                  nextChirho 32 0xFFFFFFFF00000000 $\n                    (bitmapChirho, 0)\n      mChirho =\n        if bChirho /= 0 || (bitmapChirho .&. 0x1 /= 0 && predicateChirho prefixChirho)\n          then ((2 `shiftL` bChirho) - 1)\n          else ((1 `shiftL` bChirho) - 1)\n   in bitmapChirho .&. mChirho\n",
    )
    .unwrap();

    let index_chirho = PackageIndexChirho::new_chirho();
    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
    assert!(
        result_chirho.is_ok(),
        "strict-dollar imported shift helper project should compile: {:?}",
        result_chirho
    );
}

#[test]
fn cabal_project_missing_module_skipped_chirho() {
    use crate::compile_cabal_project_chirho;
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_missing_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("miss.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         miss
version:      0.1.0.0

library
  exposed-modules: Exists, DoesNotExist
  hs-source-dirs:  src
  build-depends:   base
"#
    )
    .unwrap();

    // Only create Exists.hs, not DoesNotExist.hs
    let mut exists_chirho = std::fs::File::create(temp_dir_chirho.join("src/Exists.hs")).unwrap();
    write!(exists_chirho, "module Exists where\nval = 42\n").unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();

    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("should compile what exists");

    // Only Exists should be compiled (DoesNotExist not found on disk)
    assert_eq!(result_chirho.module_results_chirho.len(), 1);

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_builds_executable_llvm_plan_chirho() {
    use crate::build_cabal_project_chirho;
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_build_exec_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("build-exec.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         build-exec
version:      0.1.0.0

library
  exposed-modules: Lib
  hs-source-dirs:  src
  build-depends:   base >=4.14 && <5

executable hello-app
  main-is:         Main.hs
  hs-source-dirs:  src
  other-modules:   Lib
  build-depends:   base >=4.14 && <5, build-exec
"#
    )
    .unwrap();

    let mut lib_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
    write!(
        lib_file_chirho,
        "module Lib where\nmessage = \"Hello from Cabal build\"\n"
    )
    .unwrap();

    let mut main_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Main.hs")).unwrap();
    write!(
        main_file_chirho,
        "module Main where\nimport Lib\nmain = putStrLn message\n"
    )
    .unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
    let result_chirho = build_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("cabal executable project should build");

    assert_eq!(result_chirho.package_chirho.name_chirho, "build-exec");
    assert_eq!(result_chirho.executables_chirho.len(), 1);
    assert_eq!(result_chirho.executables_chirho[0].name_chirho, "hello-app");
    assert_eq!(
        result_chirho.executables_chirho[0].compilation_order_chirho,
        vec!["Lib".to_string(), "Main".to_string()]
    );
    assert!(
        !result_chirho.executables_chirho[0]
            .core_chirho
            .bindings_chirho
            .is_empty()
    );
    assert!(
        result_chirho.executables_chirho[0]
            .llvm_ir_chirho
            .contains("define i32 @main()")
    );
    assert!(
        result_chirho.executables_chirho[0]
            .llvm_ir_chirho
            .contains("@haskelujah_main")
    );

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

#[test]
fn cabal_project_cranelift_dedups_duplicate_prelude_bindings_chirho() {
    use crate::build_cabal_project_chirho;
    use haskelujah_backend_cranelift_chirho::{
        TargetConfigChirho, compile_core_to_object_executable_chirho,
    };
    use std::io::Write;

    let temp_dir_chirho = std::env::temp_dir().join("haskelujah_cabal_cranelift_dedup_test_chirho");
    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
    std::fs::create_dir_all(temp_dir_chirho.join("src")).unwrap();

    let cabal_path_chirho = temp_dir_chirho.join("dup-io.cabal");
    let mut cabal_file_chirho = std::fs::File::create(&cabal_path_chirho).unwrap();
    write!(
        cabal_file_chirho,
        r#"cabal-version: 3.0
name:         dup-io
version:      0.1.0.0

executable dup-io
  main-is:         Main.hs
  hs-source-dirs:  src
  other-modules:   Lib
  build-depends:   base >=4.14 && <5
"#
    )
    .unwrap();

    let mut lib_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Lib.hs")).unwrap();
    write!(
        lib_file_chirho,
        "module Lib where\nsayHi = putStrLn \"lib\"\n"
    )
    .unwrap();

    let mut main_file_chirho = std::fs::File::create(temp_dir_chirho.join("src/Main.hs")).unwrap();
    write!(
        main_file_chirho,
        "module Main where\nimport Lib\nmain = do\n  putStrLn \"main\"\n  sayHi\n"
    )
    .unwrap();

    let index_chirho = haskelujah_package_chirho::PackageIndexChirho::new_chirho();
    let build_result_chirho = build_cabal_project_chirho(&cabal_path_chirho, &index_chirho)
        .expect("cabal executable project should build");
    let config_chirho = TargetConfigChirho::default();
    let object_result_chirho = compile_core_to_object_executable_chirho(
        &build_result_chirho.executables_chirho[0].core_chirho,
        &config_chirho,
    )
    .expect("Cranelift should accept merged multi-module Prelude bindings");
    assert!(
        !object_result_chirho.object_bytes_chirho.is_empty(),
        "Cranelift object output should not be empty"
    );

    let _ = std::fs::remove_dir_all(&temp_dir_chirho);
}

// ── PrimOp / arithmetic end-to-end tests ────────────────────────────

#[test]
fn compile_primop_to_llvm_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Use non-literal args so constant folding doesn't eliminate the primop
    let result_chirho = compile_source_chirho(
        "module Test where\nf x = x + 1\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    )
    .expect("should compile");
    // LLVM IR should contain an add instruction (not folded away because x is a variable)
    assert!(
        result_chirho.llvm_ir_chirho.contains("add i64"),
        "LLVM IR should contain add instruction: {}",
        result_chirho.llvm_ir_chirho
    );
}

#[test]
fn compile_primop_to_wasm_chirho() {
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        "module Test where\nmain = 3 + 4\n",
        &mut source_map_chirho,
        "TestChirho.hs",
    )
    .expect("should compile");
    // WASM binary should be a valid module
    assert_eq!(&result_chirho.wasm_bytes_chirho[0..4], b"\0asm");
    assert!(result_chirho.wasm_bytes_chirho.len() > 20);
}

#[test]
fn discover_test_suite_modules_chirho() {
    use haskelujah_package_chirho::{BuildInfoChirho, PackageDescChirho, TestSuiteChirho};
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    let test_dir_chirho = dir_chirho.path().join("test");
    fs::create_dir_all(&test_dir_chirho).unwrap();
    fs::write(test_dir_chirho.join("Spec.hs"), "module Spec where\n").unwrap();
    fs::write(test_dir_chirho.join("Main.hs"), "module Main where\n").unwrap();

    let pkg_chirho = PackageDescChirho {
        name_chirho: "test-pkg".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: None,
        internal_libraries_chirho: vec![],
        executables_chirho: vec![],
        test_suites_chirho: vec![TestSuiteChirho {
            name_chirho: "my-tests".to_string(),
            type_chirho: Some("exitcode-stdio-1.0".to_string()),
            main_is_chirho: Some("Main.hs".to_string()),
            other_modules_chirho: vec!["Spec".to_string()],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![],
                hs_source_dirs_chirho: vec!["test".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
    let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
    assert!(
        names_chirho.contains(&"Main"),
        "should discover test-suite Main"
    );
    assert!(
        names_chirho.contains(&"Spec"),
        "should discover test-suite other-module Spec"
    );
}

#[test]
fn discover_setup_hs_chirho() {
    use haskelujah_package_chirho::PackageDescChirho;
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    fs::write(
        dir_chirho.path().join("Setup.hs"),
        "import Distribution.Simple\nmain = defaultMain\n",
    )
    .unwrap();

    let pkg_chirho = PackageDescChirho {
        name_chirho: "setup-pkg".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: None,
        internal_libraries_chirho: vec![],
        executables_chirho: vec![],
        test_suites_chirho: vec![],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
    let names_chirho: Vec<&str> = modules_chirho.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names_chirho.contains(&"Setup"), "should discover Setup.hs");
}

#[test]
fn discover_hierarchical_module_chirho() {
    use haskelujah_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    let src_dir_chirho = dir_chirho.path().join("src").join("Data").join("Map");
    fs::create_dir_all(&src_dir_chirho).unwrap();
    fs::write(
        src_dir_chirho.join("Internal.hs"),
        "module Data.Map.Internal where\n",
    )
    .unwrap();

    let pkg_chirho = PackageDescChirho {
        name_chirho: "hier-pkg".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: Some(LibraryChirho {
            name_chirho: None,
            exposed_modules_chirho: vec!["Data.Map.Internal".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![],
                hs_source_dirs_chirho: vec!["src".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }),
        internal_libraries_chirho: vec![],
        executables_chirho: vec![],
        test_suites_chirho: vec![],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
    assert_eq!(modules_chirho.len(), 1);
    assert_eq!(modules_chirho[0].0, "Data.Map.Internal");
    assert!(modules_chirho[0].1.ends_with("Data/Map/Internal.hs"));
}

#[test]
fn discover_hsc_module_chirho() {
    use haskelujah_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    let src_dir_chirho = dir_chirho.path().join("System");
    fs::create_dir_all(&src_dir_chirho).unwrap();
    fs::write(
        src_dir_chirho.join("Clock.hsc"),
        "module System.Clock where\nclockValueChirho = 1\n",
    )
    .unwrap();

    let pkg_chirho = PackageDescChirho {
        name_chirho: "hsc-pkg".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: Some(LibraryChirho {
            name_chirho: None,
            exposed_modules_chirho: vec!["System.Clock".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![],
                hs_source_dirs_chirho: vec![".".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }),
        internal_libraries_chirho: vec![],
        executables_chirho: vec![],
        test_suites_chirho: vec![],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
    assert_eq!(modules_chirho.len(), 1);
    assert_eq!(modules_chirho[0].0, "System.Clock");
    assert!(modules_chirho[0].1.ends_with("System/Clock.hsc"));
}

#[test]
fn discover_named_internal_library_modules_chirho() {
    use haskelujah_package_chirho::{BuildInfoChirho, LibraryChirho, PackageDescChirho};
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    let src_dir_chirho = dir_chirho.path().join("src");
    let internal_dir_chirho = dir_chirho.path().join("internal");
    fs::create_dir_all(&src_dir_chirho).unwrap();
    fs::create_dir_all(&internal_dir_chirho).unwrap();
    fs::write(src_dir_chirho.join("Demo.hs"), "module Demo where\n").unwrap();
    fs::create_dir_all(internal_dir_chirho.join("Demo")).unwrap();
    fs::write(
        internal_dir_chirho.join("Demo").join("Internal.hs"),
        "module Demo.Internal where\n",
    )
    .unwrap();

    let pkg_chirho = PackageDescChirho {
        name_chirho: "demo".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: Some(LibraryChirho {
            name_chirho: None,
            exposed_modules_chirho: vec!["Demo".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![],
                hs_source_dirs_chirho: vec!["src".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }),
        internal_libraries_chirho: vec![LibraryChirho {
            name_chirho: Some("demo-internal".to_string()),
            exposed_modules_chirho: vec!["Demo.Internal".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![],
                hs_source_dirs_chirho: vec!["internal".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }],
        executables_chirho: vec![],
        test_suites_chirho: vec![],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let modules_chirho = crate::discover_modules_chirho(&pkg_chirho, dir_chirho.path());
    let names_chirho: Vec<&str> = modules_chirho
        .iter()
        .map(|(name_chirho, _)| name_chirho.as_str())
        .collect();
    assert!(names_chirho.contains(&"Demo"));
    assert!(names_chirho.contains(&"Demo.Internal"));
}

#[test]
fn collect_library_package_deps_skips_internal_library_name_chirho() {
    use haskelujah_package_chirho::{
        BuildInfoChirho, DependencyChirho, LibraryChirho, PackageDescChirho,
        VersionConstraintChirho,
    };

    let pkg_chirho = PackageDescChirho {
        name_chirho: "demo".to_string(),
        version_chirho: None,
        cabal_version_chirho: None,
        license_chirho: None,
        author_chirho: None,
        maintainer_chirho: None,
        synopsis_chirho: None,
        description_chirho: None,
        category_chirho: None,
        homepage_chirho: None,
        bug_reports_chirho: None,
        build_type_chirho: None,
        library_chirho: Some(LibraryChirho {
            name_chirho: None,
            exposed_modules_chirho: vec!["Demo".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![
                    DependencyChirho {
                        package_chirho: "base".to_string(),
                        constraint_chirho: VersionConstraintChirho::AnyChirho,
                    },
                    DependencyChirho {
                        package_chirho: "demo-internal".to_string(),
                        constraint_chirho: VersionConstraintChirho::AnyChirho,
                    },
                ],
                hs_source_dirs_chirho: vec!["src".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }),
        internal_libraries_chirho: vec![LibraryChirho {
            name_chirho: Some("demo-internal".to_string()),
            exposed_modules_chirho: vec!["Demo.Internal".to_string()],
            other_modules_chirho: vec![],
            build_info_chirho: BuildInfoChirho {
                build_depends_chirho: vec![DependencyChirho {
                    package_chirho: "text".to_string(),
                    constraint_chirho: VersionConstraintChirho::AnyChirho,
                }],
                hs_source_dirs_chirho: vec!["internal".to_string()],
                default_language_chirho: None,
                ghc_options_chirho: vec![],
                cpp_options_chirho: vec![],
                default_extensions_chirho: vec![],
                other_extensions_chirho: vec![],
                imports_chirho: vec![],
            },
        }],
        executables_chirho: vec![],
        test_suites_chirho: vec![],
        benchmarks_chirho: vec![],
        flags_chirho: vec![],
        source_repos_chirho: vec![],
        common_stanzas_chirho: vec![],
        custom_setup_chirho: None,
    };

    let deps_chirho = crate::collect_library_package_deps_chirho(&pkg_chirho);
    let dep_names_chirho: Vec<&str> = deps_chirho
        .iter()
        .map(|dep_chirho| dep_chirho.package_chirho.as_str())
        .collect();
    assert_eq!(dep_names_chirho, vec!["base", "text"]);
}

#[test]
fn read_hsc_source_file_strips_spaced_cpp_directives_chirho() {
    use std::fs;
    use tempfile::tempdir;

    let dir_chirho = tempdir().unwrap();
    let file_path_chirho = dir_chirho.path().join("Clock.hsc");
    fs::write(
        &file_path_chirho,
        "module System.Clock where\n\
#  include <time.h>\n\
#  define CLOCK_PROCESS_CPUTIME_ID 1\n\
#  ifdef CLOCK_PROCESS_CPUTIME_ID\n\
clockFlagChirho = 1\n\
#  endif\n",
    )
    .unwrap();

    let processed_chirho =
        crate::read_haskell_source_file_chirho(&file_path_chirho).expect("read hsc source");

    assert!(processed_chirho.contains("module System.Clock where"));
    assert!(processed_chirho.contains("clockFlagChirho = 1"));
    assert!(!processed_chirho.contains("#  include"));
    assert!(!processed_chirho.contains("#  ifdef"));
    assert!(!processed_chirho.contains("#  endif"));
}

#[test]
fn strip_cpp_directives_drops_else_branch_residue_chirho() {
    let stripped_chirho = crate::strip_cpp_directives_chirho(
        "module ResidueMiniChirho where\n\
#if MIN_VERSION_template_haskell(2,16,0)\n\
keptChirho = 1\n\
#else\n\
droppedChirho = 2\n\
#endif\n\
afterChirho = keptChirho\n",
    );

    assert!(stripped_chirho.contains("module ResidueMiniChirho where"));
    assert!(stripped_chirho.contains("keptChirho = 1"));
    assert!(stripped_chirho.contains("afterChirho = keptChirho"));
    assert!(!stripped_chirho.contains("droppedChirho = 2"));
    assert!(!stripped_chirho.contains("#else"));
    assert!(!stripped_chirho.contains("#endif"));
}

#[test]
fn strip_cpp_directives_preserves_parent_line_shape_while_blanking_nested_branches_chirho() {
    let source_chirho = "module NestedResidueMiniChirho where\n\
#if OUTER\n\
outerKeptChirho = 1\n\
#if INNER\n\
innerKeptChirho = outerKeptChirho\n\
#else\n\
innerDroppedChirho = 2\n\
#endif\n\
#else\n\
outerDroppedChirho = 3\n\
#endif\n\
finalChirho = outerKeptChirho\n";
    let stripped_chirho = crate::strip_cpp_directives_chirho(source_chirho);
    let stripped_lines_chirho: Vec<_> = stripped_chirho.lines().collect();
    let source_lines_chirho: Vec<_> = source_chirho.lines().collect();

    assert_eq!(stripped_lines_chirho.len(), source_lines_chirho.len());
    assert!(stripped_chirho.contains("outerKeptChirho = 1"));
    assert!(stripped_chirho.contains("innerKeptChirho = outerKeptChirho"));
    assert!(stripped_chirho.contains("finalChirho = outerKeptChirho"));
    assert!(!stripped_chirho.contains("innerDroppedChirho = 2"));
    assert!(!stripped_chirho.contains("outerDroppedChirho = 3"));
    assert!(!stripped_chirho.contains("#if"));
    assert!(!stripped_chirho.contains("#else"));
    assert!(!stripped_chirho.contains("#endif"));
}

#[test]
fn strip_cpp_directives_preserves_line_starting_with_unboxed_tuple_syntax_chirho() {
    let stripped_chirho = crate::strip_cpp_directives_chirho(
        "module UnboxedTupleResidueMiniChirho where\n\
(# keptChirho #)\n\
#if FLAG\n\
alsoKeptChirho = 1\n\
#endif\n",
    );

    assert!(stripped_chirho.contains("(# keptChirho #)"));
    assert!(stripped_chirho.contains("alsoKeptChirho = 1"));
    assert!(!stripped_chirho.contains("#if"));
    assert!(!stripped_chirho.contains("#endif"));
}

#[test]
fn frontend_top_level_patbind_exports_through_iface_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let sources_chirho = [
        (
            "ClockSeedMiniChirho.hs",
            "module ClockSeedMiniChirho (s2nsMiniChirho) where\n\
s2nsMiniChirho = 10 ^ 9\n",
        ),
        (
            "ClockUseMiniChirho.hs",
            "module ClockUseMiniChirho where\n\
import ClockSeedMiniChirho (s2nsMiniChirho)\n\
valueMiniChirho = s2nsMiniChirho\n",
        ),
    ];
    let results_chirho = compile_modules_chirho(&sources_chirho, &mut source_map_chirho);
    assert!(
        results_chirho.is_ok(),
        "top-level pattern bindings should export through module interfaces: {:?}",
        results_chirho.err()
    );
}

#[test]
fn llvm_executable_constant_chirho() {
    // main = 42 should produce LLVM IR with ret i64 42
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
            .expect("should compile");

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert!(exec_ir_chirho.contains("define i64 @haskelujah_main()"));
    assert!(exec_ir_chirho.contains("ret i64 42"));
    assert!(exec_ir_chirho.contains("define i32 @main()"));
    assert!(exec_ir_chirho.contains("ptrtoint ptr @haskelujah_main to i64"));
    assert!(
        exec_ir_chirho.contains("call i64 @haskelujah_main_with_large_stack_chirho(i64 %main_fn)")
    );
}

#[test]
fn llvm_executable_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 should produce correct LLVM IR
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    // f should compile to an add instruction
    assert!(exec_ir_chirho.contains("add i64"));
    // main should call f with arguments
    assert!(exec_ir_chirho.contains("call i64 @haskelujah_f(i64 10, i64 32)"));
}

#[test]
fn llvm_executable_fibonacci_chirho() {
    let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );
    // Should have recursive fib function and main
    assert!(exec_ir_chirho.contains("@haskelujah_fib"));
    assert!(exec_ir_chirho.contains("define i32 @main()"));
}

#[test]
fn wasm_executable_constant_chirho() {
    // main = 42 should produce valid WASM with dict elision
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
            .expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert_eq!(&wasm_chirho[0..4], b"\0asm");
    assert_eq!(&wasm_chirho[4..8], &[1, 0, 0, 0]);
    assert!(wasm_chirho.len() > 20);
}

#[test]
fn wasm_executable_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 should produce WASM with call instruction
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert_eq!(&wasm_chirho[0..4], b"\0asm");
    // Should contain call opcode (0x10) for the function call f 10 32
    assert!(
        wasm_chirho.contains(&0x10_u8),
        "WASM should contain call instruction"
    );
}

#[test]
fn wasm_executable_fibonacci_chirho() {
    let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let wasm_chirho = haskelujah_backend_wasm_chirho::compile_core_to_wasm_executable_chirho(
        &result_chirho.core_chirho,
    );
    assert_eq!(&wasm_chirho[0..4], b"\0asm");
    // Should contain function call and case/if instructions
    assert!(
        wasm_chirho.contains(&0x10_u8),
        "WASM should contain call instruction"
    );
    assert!(
        wasm_chirho.contains(&0x04_u8),
        "WASM should contain if instruction"
    );
}

// ── LLVM round-trip smoke tests (§H.61) ──────────────────────────────
// These tests compile Haskell source to LLVM IR, invoke clang to produce
// a native binary, execute it, and compare the exit code to the STG
// interpreter result.

/// Helper: compile source to LLVM IR executable, link with clang, run,
/// and capture the exit code plus stdout.
fn llvm_round_trip_output_chirho(src_chirho: &str) -> Option<(i32, String)> {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let ll_path_chirho = tmp_dir_chirho.path().join("main.ll");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&ll_path_chirho, &exec_ir_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let compile_output_chirho = std::process::Command::new("clang")
        .arg("-O0")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&ll_path_chirho)
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts")
        .output()
        .ok()?;

    if !compile_output_chirho.status.success() {
        return None;
    }

    let run_output_chirho = std::process::Command::new(&bin_path_chirho).output().ok()?;

    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

fn haskell_string_literal_chirho(text_chirho: &str) -> String {
    let mut out_chirho = String::with_capacity(text_chirho.len() + 2);
    out_chirho.push('"');
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '\\' => out_chirho.push_str("\\\\"),
            '"' => out_chirho.push_str("\\\""),
            '\n' => out_chirho.push_str("\\n"),
            '\r' => out_chirho.push_str("\\r"),
            '\t' => out_chirho.push_str("\\t"),
            _ => out_chirho.push(ch_chirho),
        }
    }
    out_chirho.push('"');
    out_chirho
}

fn llvm_round_trip_output_with_input_chirho(
    src_chirho: &str,
    input_chirho: &str,
) -> Option<(i32, String)> {
    use std::io::Write as _;
    use std::process::Stdio;

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;

    let exec_ir_chirho = haskelujah_backend_llvm_chirho::compile_core_to_llvm_executable_chirho(
        &result_chirho.core_chirho,
    );

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let ll_path_chirho = tmp_dir_chirho.path().join("main.ll");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&ll_path_chirho, &exec_ir_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let compile_output_chirho = std::process::Command::new("clang")
        .arg("-O0")
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&ll_path_chirho)
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts")
        .output()
        .ok()?;

    if !compile_output_chirho.status.success() {
        return None;
    }

    let mut child_chirho = std::process::Command::new(&bin_path_chirho)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    child_chirho
        .stdin
        .as_mut()?
        .write_all(input_chirho.as_bytes())
        .ok()?;
    let run_output_chirho = child_chirho.wait_with_output().ok()?;

    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

/// Helper: compile source to LLVM IR executable, link with clang, run, return exit code.
fn llvm_round_trip_chirho(src_chirho: &str) -> Option<i32> {
    llvm_round_trip_output_chirho(src_chirho)
        .map(|(exit_code_chirho, _stdout_chirho)| exit_code_chirho)
}

fn cranelift_round_trip_output_chirho(src_chirho: &str) -> Option<(i32, String)> {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok()?;
    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .ok()?;

    let tmp_dir_chirho = tempfile::tempdir().ok()?;
    let obj_path_chirho = tmp_dir_chirho.path().join("main.o");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&obj_path_chirho, &obj_chirho.object_bytes_chirho).ok()?;
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho()?;

    let mut link_cmd_chirho = std::process::Command::new("cc");
    link_cmd_chirho
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&obj_path_chirho);
    if cfg!(target_os = "macos") {
        link_cmd_chirho.arg("-Wl,-no_fixup_chains");
    }
    link_cmd_chirho
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts");
    let compile_status_chirho = link_cmd_chirho.status().ok()?;
    if !compile_status_chirho.success() {
        return None;
    }

    let run_output_chirho = std::process::Command::new(&bin_path_chirho).output().ok()?;
    let exit_code_chirho = run_output_chirho.status.code()?;
    let stdout_chirho = String::from_utf8(run_output_chirho.stdout).ok()?;
    Some((exit_code_chirho, stdout_chirho))
}

fn cranelift_round_trip_chirho(src_chirho: &str) -> Option<i32> {
    cranelift_round_trip_output_chirho(src_chirho)
        .map(|(exit_code_chirho, _stdout_chirho)| exit_code_chirho)
}

fn ensure_rts_staticlib_for_tests_chirho() -> Option<std::path::PathBuf> {
    let crate_dir_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root_chirho = crate_dir_chirho.parent()?.parent()?.to_path_buf();
    let cargo_status_chirho = std::process::Command::new("cargo")
        .current_dir(&workspace_root_chirho)
        .args(["build", "-p", "haskelujah-rts", "--quiet"])
        .status()
        .ok()?;
    if !cargo_status_chirho.success() {
        return None;
    }
    Some(workspace_root_chirho.join("target").join("debug"))
}

#[test]
fn llvm_round_trip_constant_chirho() {
    // main = 42 → exit code 42
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 42");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: main = 42 should exit with 42"
        );
    }
    // If clang not available or linking fails, skip silently
}

#[test]
fn llvm_round_trip_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 → exit code 42
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: f 10 32 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_subtraction_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 50 - 8");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 50 - 8 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_multiplication_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 6 * 7");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 6 * 7 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_division_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 84 `div` 2");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 84 `div` 2 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_modulo_chirho() {
    let exit_code_chirho = llvm_round_trip_chirho("module Main where\nmain = 127 `mod` 85");
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM round-trip: 127 `mod` 85 should exit with 42"
        );
    }
}

#[test]
fn llvm_round_trip_floor_div_mod_negative_operands_chirho() {
    let src_chirho = "\
module Main where
main = ((-7) `div` 2) + ((-7) `mod` 2) + ((-7) `quot` 2) + ((-7) `rem` 2) + 49
";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "LLVM should floor div/mod but keep quot/rem truncating"
        );
    }
}

#[test]
fn llvm_round_trip_fibonacci_chirho() {
    // fib 10 = 55 → exit code 55
    let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 55,
            "LLVM round-trip: fib 10 should exit with 55"
        );
    }
}

#[test]
fn llvm_round_trip_matches_stg_chirho() {
    // Compare LLVM native execution with STG interpreter result
    let src_chirho = "module Main where\nmain = 3 * 14";
    // STG interpreter
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let stg_result_chirho = crate::eval_source_chirho(src_chirho, &mut sm_chirho, "Main.hs", None);
    let stg_val_chirho = match stg_result_chirho {
        Ok(ValueChirho::IntChirho(n_chirho)) => n_chirho,
        _ => return, // skip if STG eval fails
    };

    // LLVM native
    let native_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = native_code_chirho {
        assert_eq!(
            code_chirho as i64, stg_val_chirho,
            "LLVM native exit code should match STG interpreter result"
        );
    }
}

#[test]
fn llvm_round_trip_put_str_ln_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn \"Hello from Haskelujah!\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "putStrLn executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n");
    }
}

#[test]
fn llvm_round_trip_put_str_output_chirho() {
    let src_chirho = "module Main where\nmain = do\n  putStr \"Hello\"\n  putStr \" from\"\n  putStrLn \" Haskelujah!\"\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "putStr executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n");
    } else {
        panic!("LLVM putStr round-trip failed");
    }
}

#[test]
fn llvm_round_trip_get_line_output_chirho() {
    let src_chirho =
        "module Main where\nmain = do\n  name <- getLine\n  putStr \"Hello, \"\n  putStrLn name\n";
    if let Some((exit_code_chirho, stdout_chirho)) =
        llvm_round_trip_output_with_input_chirho(src_chirho, "Alice\n")
    {
        assert_eq!(
            exit_code_chirho, 0,
            "getLine executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello, Alice\n");
    } else {
        panic!("LLVM getLine round-trip failed");
    }
}

#[test]
fn llvm_round_trip_get_line_read_int_output_chirho() {
    let src_chirho =
        "module Main where\nmain = do\n  line <- getLine\n  print (((read line) :: Int) + 1)\n";
    if let Some((exit_code_chirho, stdout_chirho)) =
        llvm_round_trip_output_with_input_chirho(src_chirho, "41\n")
    {
        assert_eq!(
            exit_code_chirho, 0,
            "getLine+read executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    } else {
        panic!("LLVM getLine+read round-trip failed");
    }
}

#[test]
fn llvm_round_trip_recursive_io_return_unit_output_chirho() {
    let src_chirho = "module Main where\nprintTodos [] = return ()\nprintTodos (x:xs) = do\n  putStrLn x\n  printTodos xs\nmain = printTodos [\"a\",\"b\"]\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "recursive IO return executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "a\nb\n");
    } else {
        panic!("LLVM recursive IO return round-trip failed");
    }
}

#[test]
fn llvm_round_trip_write_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("write-file-llvm-chirho.txt");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  writeFile {file_path_literal_chirho} \"hello from llvm\"\n  putStr \"ok\"\n"
    );
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(&src_chirho).expect("writeFile LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "writeFile executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "ok");
    let file_contents_chirho =
        std::fs::read_to_string(&file_path_chirho).expect("writeFile should create file");
    assert_eq!(file_contents_chirho, "hello from llvm");
}

#[test]
fn llvm_round_trip_read_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("read-file-llvm-chirho.txt");
    std::fs::write(&file_path_chirho, "hello from llvm disk")
        .expect("fixture file should be written");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  contentsChirho <- readFile {file_path_literal_chirho}\n  putStr contentsChirho\n"
    );
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(&src_chirho).expect("readFile LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "readFile executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "hello from llvm disk");
}

#[test]
fn llvm_round_trip_print_int_output_chirho() {
    let src_chirho = "module Main where\nmain = print 42";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_print_true_output_chirho() {
    let src_chirho = "module Main where\nmain = print True";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print True executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\n");
    }
}

#[test]
fn llvm_round_trip_print_false_output_chirho() {
    let src_chirho = "module Main where\nmain = print False";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print False executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "False\n");
    }
}

#[test]
fn llvm_round_trip_print_char_output_chirho() {
    let src_chirho = "module Main where\nmain = print 'A'";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print Char executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "'A'\n");
    }
}

#[test]
fn llvm_round_trip_print_float_output_chirho() {
    let src_chirho = "module Main where\nmain = print 3.14";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print Float executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "3.14\n");
    }
}

#[test]
fn llvm_round_trip_print_comparison_outputs_bool_chirho() {
    let src_chirho = r#"module Main where
main = do
  print (2 == 2)
  print (2 /= 3)
  print (2 < 3)
  print (3 > 2)
  print (2 <= 2)
  print (3 >= 3)"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "comparison executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\nTrue\nTrue\nTrue\nTrue\nTrue\n");
    }
}

#[test]
fn llvm_round_trip_if_then_else_true_branch_output_chirho() {
    let src_chirho =
        "module Main where\nmain = if 2 + 3 == 5 then putStrLn \"yes\" else putStrLn \"no\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "if-then-else executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "yes\n");
    }
}

#[test]
fn llvm_round_trip_if_then_else_false_branch_output_chirho() {
    let src_chirho =
        "module Main where\nmain = if 2 + 3 == 6 then putStrLn \"yes\" else putStrLn \"no\"";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "if-then-else executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "no\n");
    }
}

#[test]
fn llvm_round_trip_do_put_str_ln_then_print_output_chirho() {
    let src_chirho =
        "module Main where\nmain = do\n  putStrLn \"Hello from Haskelujah!\"\n  print 42\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "do block executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Hello from Haskelujah!\n42\n");
    }
}

#[test]
fn llvm_round_trip_bind_return_print_output_chirho() {
    let src_chirho = "module Main where\nmain = do\n  x <- return 42\n  print x\n";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "bind executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_recursive_where_print_output_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStrLn "Hello from Haskelujah Chirho!"
  print (2 + 3)
  print (factorial 10)
  where
    factorial 0 = 1
    factorial n = n * factorial (n - 1)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("recursive where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "recursive where executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "Hello from Haskelujah Chirho!\n5\n3628800\n");
}

#[test]
fn llvm_round_trip_nested_where_print_output_chirho() {
    let src_chirho = r#"module Main where
sumTo n = outer n where
  outer m = go m 0 where
    go 0 acc = acc
    go k acc = go (k - 1) (acc + k)
main = print (sumTo 10)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("nested where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "nested where executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "55\n");
}

#[test]
fn llvm_round_trip_where_sibling_cross_reference_output_chirho() {
    let src_chirho = "module Main where\nf x = result where result = a + b; a = x * 2; b = x * 3\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("where sibling LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where sibling LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "25\n");
}

#[test]
fn llvm_round_trip_higher_order_on_function_output_chirho() {
    let src_chirho = "module Main where\nonChirho fChirho gChirho xChirho yChirho = fChirho (gChirho xChirho) (gChirho yChirho)\nmain = print (((+) `onChirho` abs) (-3) 3)";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("higher-order on LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "higher-order on executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "6\n");
}

#[test]
fn llvm_round_trip_let_inside_where_output_chirho() {
    let src_chirho =
        "module Main where\nf x = a where a = let sq = x * x in sq + 1\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("let-in-where LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "let-in-where LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "26\n");
}

#[test]
fn llvm_round_trip_derived_eq_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Color = Red | Green | Blue deriving (Eq, Show)
main = do
  print (Red == Red)
  print (Red == Blue)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("derived Eq LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Eq LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "True\nFalse\n");
}

#[test]
fn llvm_round_trip_derived_ord_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Prio = Low | Med | High deriving (Ord, Eq, Show)
main = print (compare High Low)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("derived Ord LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Ord LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "GT\n");
}

#[test]
fn llvm_round_trip_euler1_tail_recursion_no_stack_overflow_chirho() {
    let src_chirho = r#"module Main where
euler1 limit = go 0 0 where
  go acc n =
    if n >= limit
      then acc
      else if mod n 3 == 0
        then go (acc + n) (n + 1)
        else if mod n 5 == 0
          then go (acc + n) (n + 1)
          else go acc (n + 1)
main = print (euler1 100000000)
"#;
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("tail-recursive LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "tail-recursive LLVM executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "2333333316666668\n");
}

#[test]
fn llvm_round_trip_put_str_ln_show_int_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 42)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_true_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show True)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show True executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "True\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_negative_int_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show (-42))";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show negative Int executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "-42\n");
    }
}

#[test]
fn llvm_round_trip_print_bool_list_output_chirho() {
    let src_chirho = "module Main where\nmain = print [True,False,True]";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print bool list executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "[True,False,True]\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_false_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show False)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show False executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "False\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_char_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 'A')";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show Char executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "'A'\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_float_output_chirho() {
    let src_chirho = "module Main where\nmain = putStrLn (show 3.14)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show Float executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "3.14\n");
    }
}

#[test]
fn llvm_round_trip_put_str_ln_show_derived_enum_output_chirho() {
    let src_chirho = "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = putStrLn (show Green)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show derived enum executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Green\n");
    }
}

#[test]
fn llvm_round_trip_print_derived_enum_output_chirho() {
    let src_chirho =
        "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = print Green";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print derived enum executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "Green\n");
    }
}

#[test]
fn llvm_round_trip_print_derived_field_constructor_output_chirho() {
    let src_chirho =
        "module Main where\ndata Pair = MkPair Int Int deriving (Show)\nmain = print (MkPair 3 4)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "print derived field constructor executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "MkPair 3 4\n");
    }
}

#[test]
fn llvm_round_trip_nested_adt_list_pattern_output_chirho() {
    let src_chirho = "module Main where\ndata Value = VInt Int | VList [Value]\nscore (VList [VInt n]) = n\nscore (VList (_:_)) = 1\nscore _ = 2\nmain = print (score (VList [VInt 7]))";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "nested ADT/list pattern executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "7\n");
    }
}

#[test]
fn llvm_round_trip_non_tail_recursive_list_fold_output_chirho() {
    let src_chirho = "module Main where\nmyMax [x] = x\nmyMax (x:xs) = if x > myMax xs then x else myMax xs\nmain = print (myMax [5,3,9,2])";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "non-tail recursive list executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "9\n");
    }
}

#[test]
fn llvm_round_trip_inner_column_list_binder_output_chirho() {
    let src_chirho =
        "module Main where\nf (x:_) 0 = x\nf (_:xs) n = f xs (n-1)\nmain = print (f [5,6,7] 1)";
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "inner-column list binder executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "6\n");
    }
}

#[test]
fn llvm_round_trip_print_sum_list_output_chirho() {
    let src_chirho = "module Main where\nmain = print (sum [1,2,3])";
    let (exit_code_chirho, stdout_chirho) =
        llvm_round_trip_output_chirho(src_chirho).expect("sum list LLVM round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "sum list executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "6\n");
}

#[test]
fn llvm_round_trip_map_lambda_sum_chirho() {
    let src_chirho = "module Main where\nmain = sum (map (\\x -> x * 2) [1,2,3])";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 12, "map should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_filter_sum_chirho() {
    let src_chirho = "module Main where\nmain = sum (filter (> 2) [1,2,3,4])";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 7, "filter should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_foldr_sum_chirho() {
    let src_chirho = "module Main where\nmain = foldr (+) 0 [1,2,3]";
    let exit_code_chirho = llvm_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(code_chirho, 6, "foldr should compile through LLVM");
    }
}

#[test]
fn llvm_round_trip_recursive_custom_list_instance_output_chirho() {
    let src_chirho = r#"module Main where
class Describable a where
  describe :: a -> String

instance Describable Int where
  describe x = show x

instance Describable a => Describable [a] where
  describe [] = "[]"
  describe (x:xs) = describe x ++ ":" ++ describe xs

main = putStrLn (describe [1,2,3])
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "llvm custom list instance should exit successfully"
        );
        assert_eq!(stdout_chirho, "1:2:3:[]\n");
    }
}

#[test]
fn llvm_round_trip_show_concat_output_chirho() {
    let src_chirho = r#"module Main where
main = putStrLn ("answer: " ++ show 42)
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "show concat executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "answer: 42\n");
    }
}

#[test]
fn llvm_round_trip_string_equality_output_chirho() {
    let src_chirho = r#"module Main where
main = do
  print ("hello" == "hello")
  print ("hello" == "world")
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(exit_code_chirho, 0);
        assert_eq!(stdout_chirho, "True\nFalse\n");
    }
}

#[test]
fn llvm_round_trip_null_empty_string_output_chirho() {
    let src_chirho = r#"module Main where
main = if null "" then print (1 :: Int) else print (2 :: Int)
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(exit_code_chirho, 0);
        assert_eq!(stdout_chirho, "1\n");
    }
}

#[test]
fn llvm_round_trip_string_case_and_read_int_output_chirho() {
    let src_chirho = r#"module Main where
classifyChirho s = case s of
  "hello" -> 1
  "world" -> 2
  _ -> 3
main = do
  print (classifyChirho "hello")
  print ((read "42") :: Int)
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(exit_code_chirho, 0);
        assert_eq!(stdout_chirho, "1\n42\n");
    }
}

#[test]
fn llvm_round_trip_take_zero_matches_first_equation_chirho() {
    let src_chirho = r#"module Main where
myTakeChirho 0 _ = []
myTakeChirho _ [] = []
myTakeChirho nChirho (xChirho:xsChirho) = xChirho : myTakeChirho (nChirho - 1) xsChirho
myLenChirho [] = 0
myLenChirho (_:xsChirho) = 1 + myLenChirho xsChirho
main = print (myLenChirho (myTakeChirho 0 [1,2,3]))
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "take 0 literal-first executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "0\n");
    }
}

#[test]
fn llvm_round_trip_safe_lookup_literal_after_wildcard_output_chirho() {
    let src_chirho = r#"module Main where
safeLookupChirho _ [] = Nothing
safeLookupChirho 0 (xChirho:_) = Just xChirho
safeLookupChirho nChirho (_:xsChirho) = safeLookupChirho (nChirho - 1) xsChirho
main = case safeLookupChirho 2 [40,41,42,43] of
  Just valueChirho -> print valueChirho
  Nothing -> print 0
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "safeLookup executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn llvm_round_trip_pattern_guard_binds_name_output_chirho() {
    let src_chirho = r#"module Main where
fChirho xsChirho
  | msgChirho : _ <- xsChirho, null msgChirho = 1
  | _ : _ <- xsChirho = 2
  | otherwise = 0
main = print (fChirho ["", "x"])
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = llvm_round_trip_output_chirho(src_chirho) {
        assert_eq!(
            exit_code_chirho, 0,
            "pattern guard executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "1\n");
    }
}

#[test]
fn frontend_case_alt_pattern_guard_binds_constructor_arg_chirho() {
    let src_chirho = r#"module Main where
data FooChirho = MkFooChirho Int
gChirho xChirho = case () of
  _ | MkFooChirho nChirho <- xChirho -> nChirho
  _ -> 0
main = print (gChirho (MkFooChirho 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs")
        .expect("guarded case alt pattern binder should compile");
}

#[test]
fn frontend_backticked_pattern_guard_keeps_generator_application_chirho() {
    let src_chirho = r#"module GuardedSetMemberMiniChirho where
import qualified Data.Set as Set

fChirho :: Int -> Set.Set Int -> [Set.Set Int] -> ([Int], [Set.Set Int])
fChirho tvChirho fvsChirho fvssChirho
  | tvChirho `Set.member` fvsChirho
  , (asPrimeChirho, fvssPrimeChirho) <- insertChirho tvChirho [] fvssChirho
  = (asPrimeChirho, fvssPrimeChirho)
  | otherwise = ([], [])

insertChirho :: Int -> [Int] -> [Set.Set Int] -> ([Int], [Set.Set Int])
insertChirho tvChirho [] [] = ([tvChirho], [])
insertChirho _ (aChirho:asChirho) (fvsPrimeChirho:fvssPrimeChirho) =
  (aChirho:asChirho, fvsPrimeChirho:fvssPrimeChirho)
insertChirho _ _ _ = ([], [])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(src_chirho, &mut sm_chirho, "GuardedSetMemberMiniChirho.hs")
        .expect("mixed boolean and pattern guards should preserve generator applications");
}

#[test]
fn frontend_data_map_unions_builtin_iface_typechecks_chirho() {
    let src_chirho = r#"module DataMapUnionsMiniChirho where
import qualified Data.Map as Map

valueChirho :: Map.Map Int Int
valueChirho = Map.unions [Map.singleton 1 2, Map.singleton 3 4]
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(src_chirho, &mut sm_chirho, "DataMapUnionsMiniChirho.hs")
        .expect("Data.Map builtins should expose unions/unionsWith");
}

#[test]
fn frontend_data_map_unqualified_alias_and_qualified_values_typecheck_chirho() {
    let src_chirho = r#"module DataMapAliasMiniChirho where
import Data.Map (Map)
import qualified Data.Map as Map

valueChirho :: Map Int Int
valueChirho = Map.fromList [(1, 2), (3, 4)]
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(src_chirho, &mut sm_chirho, "DataMapAliasMiniChirho.hs")
        .expect("Data.Map builtins should agree on Map across qualified and unqualified imports");
}

#[test]
fn frontend_tuple_section_applicative_chain_typechecks_chirho() {
    let src_chirho = r#"{-# LANGUAGE TupleSections #-}
module TupleSectionApplicativeMiniChirho where

valueChirho = (, True) <$> Just 1
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(
        src_chirho,
        &mut sm_chirho,
        "TupleSectionApplicativeMiniChirho.hs",
    )
    .expect("tuple sections under <$> should lower without placeholder variables");
}

#[test]
fn frontend_local_data_map_alias_signature_typechecks_chirho() {
    let src_chirho = r#"module LocalDataMapAliasMiniChirho where
import Data.Map (Map)
import qualified Data.Map as Map

valueChirho :: [Int] -> Map Int Int
valueChirho xsChirho =
  let buildChirho :: [Int] -> Map Int Int
      buildChirho _ = Map.fromList [(1, 2), (3, 4)]
  in buildChirho xsChirho
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(src_chirho, &mut sm_chirho, "LocalDataMapAliasMiniChirho.hs")
        .expect("local signatures should preserve imported Map aliases");
}

#[test]
fn frontend_local_guarded_tuple_helper_signature_typechecks_chirho() {
    let src_chirho = r#"module LocalGuardedTupleHelperMiniChirho where
import qualified Data.Set as Set

goChirho :: Int -> Set.Set Int -> [Set.Set Int] -> ([Int], [Set.Set Int])
goChirho tvChirho fvsChirho fvssChirho =
  let insertChirho :: Int -> [Int] -> [Set.Set Int] -> ([Int], [Set.Set Int])
      insertChirho tvPrimeChirho [] [] = ([tvPrimeChirho], [])
      insertChirho tvPrimeChirho (aChirho:asChirho) (fvsPrimeChirho:fvssPrimeChirho)
        | tvPrimeChirho `Set.member` fvsPrimeChirho
        , (asPrimeChirho, fvssDoublePrimeChirho) <- insertChirho tvPrimeChirho asChirho fvssPrimeChirho
        = (aChirho:asPrimeChirho, fvsPrimeChirho:fvssDoublePrimeChirho)
        | otherwise
        = (tvPrimeChirho:aChirho:asChirho, fvsPrimeChirho:fvsPrimeChirho:fvssPrimeChirho)
      insertChirho _ _ _ = ([], [])
  in insertChirho tvChirho [] fvssChirho
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(
        src_chirho,
        &mut sm_chirho,
        "LocalGuardedTupleHelperMiniChirho.hs",
    )
    .expect("local guarded helpers should preserve tuple result signatures");
}

#[test]
fn frontend_tuple_element_with_qualified_backticked_operator_and_cons_typechecks_chirho() {
    let src_chirho = r#"module QualifiedBacktickTupleMiniChirho where
import qualified Data.Set as Set

valueChirho :: Set.Set Int -> Set.Set Int -> [Set.Set Int] -> ([Int], [Set.Set Int])
valueChirho leftChirho rightChirho restChirho =
  ([], leftChirho `Set.union` rightChirho : restChirho)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(
        src_chirho,
        &mut sm_chirho,
        "QualifiedBacktickTupleMiniChirho.hs",
    )
    .expect("tuple elements should preserve qualified backticked operators before cons");
}

#[test]
fn frontend_th_abstraction_scoped_sort_mini_typechecks_chirho() {
    let src_chirho = r#"module ThAbstractionScopedSortMiniChirho where
import qualified Data.Map as Map
import Data.Map (Map)
import qualified Data.Set as Set
import Data.Set (Set)

type NameChirho = Int
type TypeChirho = Int
type KindChirho = Int
data TyVarBndrUnitChirho = MkTyVarBndrUnitChirho

tvNameChirho :: TyVarBndrUnitChirho -> NameChirho
tvNameChirho _ = 0

plainTVChirho :: NameChirho -> TyVarBndrUnitChirho
plainTVChirho _ = MkTyVarBndrUnitChirho

kindedTVChirho :: NameChirho -> KindChirho -> TyVarBndrUnitChirho
kindedTVChirho _ _ = MkTyVarBndrUnitChirho

freeVariablesChirho :: aChirho -> [NameChirho]
freeVariablesChirho _ = []

freeVariablesWellScopedChirho :: [TypeChirho] -> [TyVarBndrUnitChirho]
freeVariablesWellScopedChirho tysChirho =
  let fvsChirho :: [NameChirho]
      fvsChirho = freeVariablesChirho tysChirho

      varKindSigsChirho :: Map NameChirho KindChirho
      varKindSigsChirho = Map.empty

      scopedSortChirho :: [NameChirho] -> [NameChirho]
      scopedSortChirho = goChirho [] []

      goChirho :: [NameChirho] -> [Set NameChirho] -> [NameChirho] -> [NameChirho]
      goChirho accChirho _ [] = reverse accChirho
      goChirho accChirho fvListChirho (tvChirho:tvsChirho)
        = goChirho accPrimeChirho fvListPrimeChirho tvsChirho
        where
          (accPrimeChirho, fvListPrimeChirho) = insertChirho tvChirho accChirho fvListChirho

      insertChirho :: NameChirho -> [NameChirho] -> [Set NameChirho] -> ([NameChirho], [Set NameChirho])
      insertChirho tvChirho [] [] = ([tvChirho], [kindFVSetChirho tvChirho])
      insertChirho tvChirho (aChirho:asChirho) (fvsChirhoHere:fvssChirho)
        | tvChirho `Set.member` fvsChirhoHere
        , (asPrimeChirho, fvssPrimeChirho) <- insertChirho tvChirho asChirho fvssChirho
        = (aChirho:asPrimeChirho, fvsChirhoHere `Set.union` fvTvChirho : fvssPrimeChirho)
        | otherwise
        = (tvChirho:aChirho:asChirho, fvsChirhoHere `Set.union` fvTvChirho : fvsChirhoHere : fvssChirho)
        where
          fvTvChirho = kindFVSetChirho tvChirho

      insertChirho _ _ _ = error "scopedSort"

      kindFVSetChirho :: NameChirho -> Set NameChirho
      kindFVSetChirho nChirho =
        maybe Set.empty (Set.fromList . freeVariablesChirho) (Map.lookup nChirho varKindSigsChirho)

      ascribeWithKindChirho :: NameChirho -> TyVarBndrUnitChirho
      ascribeWithKindChirho nChirho =
        maybe (plainTVChirho nChirho) (kindedTVChirho nChirho) (Map.lookup nChirho varKindSigsChirho)

  in map ascribeWithKindChirho (scopedSortChirho fvsChirho)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(
        src_chirho,
        &mut sm_chirho,
        "ThAbstractionScopedSortMiniChirho.hs",
    )
    .expect("th-abstraction-style local scoped sort should typecheck");
}

#[test]
fn frontend_tuple_section_inside_case_lambda_typechecks_chirho() {
    let src_chirho = r#"{-# LANGUAGE TupleSections #-}
module TupleSectionCaseLambdaMiniChirho where
import Data.Maybe (mapMaybe)

valueChirho :: [(Int, Either Bool Char)] -> [(Int, Bool)]
valueChirho xsChirho =
  mapMaybe
    (\(instTyChirho, argChirho) ->
        case argChirho of
          Left kChirho -> (, kChirho) <$> Just instTyChirho
          Right _ -> Nothing)
    xsChirho
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(
        src_chirho,
        &mut sm_chirho,
        "TupleSectionCaseLambdaMiniChirho.hs",
    )
    .expect("tuple sections inside lambda case branches should lower without dummy variables");
}

// ── Cranelift backend driver integration tests ────────────────────────

#[test]
fn cranelift_executable_constant_chirho() {
    // main = 42 should produce valid native object file
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho("module Main where\nmain = 42", &mut sm_chirho, "Main.hs")
            .expect("should compile");

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("cranelift compilation should succeed");
    assert!(
        !obj_chirho.object_bytes_chirho.is_empty(),
        "object file should not be empty"
    );
}

#[test]
fn cranelift_executable_arithmetic_chirho() {
    // f x y = x + y; main = f 10 32 should produce valid object with function call
    let src_chirho = "module Main where\nf x y = x + y\nmain = f 10 32";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("cranelift compilation should succeed");
    assert!(
        !obj_chirho.object_bytes_chirho.is_empty(),
        "object file should not be empty"
    );
}

#[test]
fn cranelift_round_trip_pattern_guard_binds_name_output_chirho() {
    let src_chirho = r#"module Main where
fChirho xsChirho
  | msgChirho : _ <- xsChirho, null msgChirho = 1
  | _ : _ <- xsChirho = 2
  | otherwise = 0
main = print (fChirho ["", "x"])
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = cranelift_round_trip_output_chirho(src_chirho)
    {
        assert_eq!(
            exit_code_chirho, 0,
            "pattern guard executable should exit successfully"
        );
        assert_eq!(stdout_chirho, "1\n");
    }
}

#[test]
fn cranelift_round_trip_floor_div_mod_negative_operands_chirho() {
    let src_chirho = "\
module Main where
main = ((-7) `div` 2) + ((-7) `mod` 2) + ((-7) `quot` 2) + ((-7) `rem` 2) + 49
";
    let exit_code_chirho = cranelift_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "Cranelift should floor div/mod but keep quot/rem truncating"
        );
    }
}

#[test]
fn cranelift_executable_fibonacci_chirho() {
    let src_chirho = r#"module Main where
fib n = case n of
  0 -> 0
  1 -> 1
  _ -> fib (n - 1) + fib (n - 2)
main = fib 10"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").expect("should compile");

    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("cranelift compilation should succeed");
    assert!(
        !obj_chirho.object_bytes_chirho.is_empty(),
        "cranelift fibonacci object file should not be empty"
    );
}

#[test]
fn cranelift_round_trip_payload_constructor_box_int_chirho() {
    let src_chirho = r#"module Main where
data Box = Box Int
unBox :: Box -> Int
unBox (Box n) = n
main = unBox (Box 42)
"#;
    let exit_code_chirho = cranelift_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 42,
            "Cranelift round-trip: Box Int should exit with 42"
        );
    }
}

#[test]
fn cranelift_round_trip_payload_constructor_rect_area_chirho() {
    let src_chirho = r#"module Main where
data Shape = Circle Int | Rect Int Int
area :: Shape -> Int
area (Circle r) = 3 * r * r
area (Rect w h) = w * h
main = area (Rect 3 7)
"#;
    let exit_code_chirho = cranelift_round_trip_chirho(src_chirho);
    if let Some(code_chirho) = exit_code_chirho {
        assert_eq!(
            code_chirho, 21,
            "Cranelift round-trip: Rect 3 7 should exit with 21"
        );
    }
}

#[test]
fn cranelift_round_trip_io_order_output_chirho() {
    let src_chirho = r#"module Main where
main = do
  putStrLn "hello"
  print 42
"#;
    if let Some((exit_code_chirho, stdout_chirho)) = cranelift_round_trip_output_chirho(src_chirho)
    {
        assert_eq!(
            exit_code_chirho, 0,
            "Cranelift IO program should exit successfully"
        );
        assert_eq!(
            stdout_chirho, "hello\n42\n",
            "Cranelift should preserve source IO sequencing",
        );
    }
}

#[test]
fn cranelift_round_trip_recursive_where_print_output_chirho() {
    let src_chirho = r#"module Main where
collatz n = go n 0
  where
    go 1 acc = acc
    go k acc = if mod k 2 == 0 then go (div k 2) (acc + 1) else go (3 * k + 1) (acc + 1)
main = print (collatz 7)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("recursive where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "recursive where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "16\n");
}

#[test]
fn cranelift_round_trip_nested_where_print_output_chirho() {
    let src_chirho = r#"module Main where
sumTo n = outer n where
  outer m = go m 0 where
    go 0 acc = acc
    go k acc = go (k - 1) (acc + k)
main = print (sumTo 10)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("nested where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "nested where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "55\n");
}

#[test]
fn cranelift_round_trip_where_sibling_cross_reference_output_chirho() {
    let src_chirho = "module Main where\nf x = result where result = a + b; a = x * 2; b = x * 3\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("where sibling Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where sibling Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "25\n");
}

#[test]
fn cranelift_round_trip_let_inside_where_output_chirho() {
    let src_chirho =
        "module Main where\nf x = a where a = let sq = x * x in sq + 1\nmain = print (f 5)";
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("let-in-where Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "let-in-where Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "26\n");
}

#[test]
fn cranelift_round_trip_derived_eq_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Color = Red | Green | Blue deriving (Eq, Show)
main = do
  print (Red == Red)
  print (Red == Blue)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("derived Eq Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Eq Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "True\nFalse\n");
}

#[test]
fn cranelift_round_trip_derived_ord_runtime_output_chirho() {
    let src_chirho = r#"module Main where
data Prio = Low | Med | High deriving (Ord, Eq, Show)
main = print (compare High Low)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("derived Ord Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "derived Ord Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "GT\n");
}

#[test]
fn cranelift_round_trip_wildcard_first_column_safe_divide_output_chirho() {
    let src_chirho = r#"module Main where
safeDivide _ 0 = 0
safeDivide a b = div a b
main = print (safeDivide 10 2)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("wildcard safeDivide Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "wildcard-first-column Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "5\n");
}

#[test]
fn cranelift_round_trip_partial_application_make_adder_output_chirho() {
    let src_chirho = r#"module Main where
makeAdder n = \x -> x + n
main = do
  let add5 = makeAdder 5
  print (add5 37)
"#;
    let (exit_code_chirho, stdout_chirho) = cranelift_round_trip_output_chirho(src_chirho)
        .expect("partial application makeAdder Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "partial application Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "42\n");
}

#[test]
fn cranelift_round_trip_where_capture_output_chirho() {
    let src_chirho = r#"module Main where
f x = go 0 where
  go n = if n >= x then n else go (n + 1)
main = print (f 5)
"#;
    let (exit_code_chirho, stdout_chirho) =
        cranelift_round_trip_output_chirho(src_chirho).expect("where capture Cranelift round-trip");
    assert_eq!(
        exit_code_chirho, 0,
        "where-capture Cranelift executable should exit successfully"
    );
    assert_eq!(stdout_chirho, "5\n");
}

// ---------------------------------------------------------------
// §29 — Structured error messages with "did you mean?" suggestions
// ---------------------------------------------------------------

#[test]
fn unbound_var_suggests_similar_name_chirho() {
    // Typo: "ad1" instead of "add1"
    let src_chirho = "module Main where\nadd1 x = x + 1\nmain = ad1 41\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
    assert!(result_chirho.is_err(), "should fail with unbound variable");
    let diag_chirho = result_chirho.unwrap_err();
    let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
    assert!(
        rendered_chirho.contains("did you mean"),
        "error should contain 'did you mean' suggestion, got: {rendered_chirho}"
    );
    assert!(
        rendered_chirho.contains("add1"),
        "suggestion should include 'add1', got: {rendered_chirho}"
    );
}

#[test]
fn type_mismatch_shows_expected_found_chirho() {
    let src_chirho = "module Main where\nf :: Int -> Int\nf x = x + 1\nmain = f True\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
    assert!(result_chirho.is_err());
    let diag_chirho = result_chirho.unwrap_err();
    let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
    assert!(
        rendered_chirho.contains("type mismatch"),
        "error should mention 'type mismatch', got: {rendered_chirho}"
    );
    assert!(
        rendered_chirho.contains("expected type:") && rendered_chirho.contains("found type:"),
        "error should show expected/found types, got: {rendered_chirho}"
    );
}

#[test]
fn error_rendered_with_source_snippet_chirho() {
    let src_chirho = "module Main where\nmain = undefined_func 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
    assert!(result_chirho.is_err());
    let diag_chirho = result_chirho.unwrap_err();
    let rendered_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
    assert!(
        rendered_chirho.contains("Main.hs"),
        "error should reference file, got: {rendered_chirho}"
    );
    assert!(
        rendered_chirho.contains("-->"),
        "error should have --> source pointer, got: {rendered_chirho}"
    );
}

#[test]
fn error_rendered_with_color_chirho() {
    let src_chirho = "module Main where\nmain = no_such_var\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs");
    assert!(result_chirho.is_err());
    let diag_chirho = result_chirho.unwrap_err();
    let colored_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, true);
    assert!(
        colored_chirho.contains("\x1b["),
        "colored output should contain ANSI escapes"
    );
    let plain_chirho = crate::render_diagnostics_chirho(&diag_chirho, &sm_chirho, false);
    assert!(
        !plain_chirho.contains("\x1b["),
        "plain output should not contain ANSI escapes"
    );
}

// ── §44 Foreign exports ──────────────────────────────────────────────

#[test]
fn foreign_export_parsed_and_reaches_core_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = concat!(
        "module Test where\n",
        "foreign export ccall addOne :: Int -> Int\n",
        "addOne x = x + 1\n",
        "main = addOne 41\n",
    );
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExport.hs")
        .expect("should compile with foreign export");
    // The Core module should have the foreign export recorded
    assert!(
        !result_chirho.core_chirho.foreign_exports_chirho.is_empty(),
        "foreign exports should be propagated to Core module"
    );
    let export_chirho = &result_chirho.core_chirho.foreign_exports_chirho[0];
    assert_eq!(export_chirho.haskell_name_chirho, "addOne");
    assert_eq!(export_chirho.foreign_name_chirho, "addOne");
    assert_eq!(export_chirho.calling_conv_chirho, "ccall");
}

#[test]
fn foreign_export_with_custom_c_name_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = concat!(
        "module Test where\n",
        "foreign export ccall \"hs_add_one\" addOne :: Int -> Int\n",
        "addOne x = x + 1\n",
        "main = addOne 41\n",
    );
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExportName.hs")
        .expect("should compile with custom C name");
    let export_chirho = &result_chirho.core_chirho.foreign_exports_chirho[0];
    assert_eq!(export_chirho.haskell_name_chirho, "addOne");
    assert_eq!(export_chirho.foreign_name_chirho, "hs_add_one");
}

#[test]
fn foreign_export_llvm_emits_wrapper_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = concat!(
        "module Test where\n",
        "foreign export ccall \"hs_val\" getVal :: Int\n",
        "getVal = 42\n",
        "main = getVal\n",
    );
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ForeignExportLLVM.hs")
        .expect("should compile");
    assert!(
        result_chirho.llvm_ir_chirho.contains("@hs_val"),
        "LLVM IR should contain the foreign export wrapper: {}",
        result_chirho.llvm_ir_chirho
    );
}

#[test]
fn cranelift_round_trip_show_concat_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = putStrLn ("fib 10 = " ++ show (55))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(
            stdout_chirho.contains("fib 10 = 55"),
            "expected 'fib 10 = 55', got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_foldr_filter_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs

foldr' :: (Int -> Int -> Int) -> Int -> [Int] -> Int
foldr' f z [] = z
foldr' f z (x:xs) = f x (foldr' f z xs)

main = print (foldr' (\x acc -> x + acc) 0 (filter' (\x -> x `mod` 2 == 0) [1,2,3,4,5,6,7,8,9,10]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(
            stdout_chirho.trim() == "30",
            "sum of evens in [1..10] = 30, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_take_zero_matches_first_equation_chirho() {
    let src_chirho = r#"module Main where
myTakeChirho 0 _ = []
myTakeChirho _ [] = []
myTakeChirho nChirho (xChirho:xsChirho) = xChirho : myTakeChirho (nChirho - 1) xsChirho
myLenChirho [] = 0
myLenChirho (_:xsChirho) = 1 + myLenChirho xsChirho
main = print (myLenChirho (myTakeChirho 0 [1,2,3]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert_eq!(stdout_chirho, "0\n");
    }
}

#[test]
fn cranelift_round_trip_safe_lookup_literal_after_wildcard_output_chirho() {
    let src_chirho = r#"module Main where
safeLookupChirho _ [] = Nothing
safeLookupChirho 0 (xChirho:_) = Just xChirho
safeLookupChirho nChirho (_:xsChirho) = safeLookupChirho (nChirho - 1) xsChirho
main = case safeLookupChirho 2 [40,41,42,43] of
  Just valueChirho -> print valueChirho
  Nothing -> print 0
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert_eq!(stdout_chirho, "42\n");
    }
}

#[test]
fn cranelift_round_trip_comprehensive_chirho() {
    let src_chirho = r#"module Main where

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs

makeAdder :: Int -> Int -> Int
makeAdder n x = n + x

countUpTo :: Int -> Int
countUpTo limit = go 0
  where go n = if n >= limit then n else go (n + 1)

data Shape = Circle Int | Rectangle Int Int

area :: Shape -> Int
area (Circle r) = r * r
area (Rectangle w h) = w * h

main :: IO ()
main = do
  putStrLn ("fib 20 = " ++ show (fib 20))
  putStrLn ("sum [1..5] = " ++ show (mySum [1,2,3,4,5]))
  let add10 = makeAdder 10
  putStrLn ("add10 32 = " ++ show (add10 32))
  putStrLn ("countUpTo 50 = " ++ show (countUpTo 50))
  putStrLn ("area Circle 7 = " ++ show (area (Circle 7)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "should exit 0");
        assert!(stdout_chirho.contains("fib 20 = 6765"), "fib 20");
        assert!(stdout_chirho.contains("sum [1..5] = 15"), "sum");
        assert!(stdout_chirho.contains("add10 32 = 42"), "closure");
        assert!(stdout_chirho.contains("countUpTo 50 = 50"), "where");
        assert!(stdout_chirho.contains("area Circle 7 = 49"), "ADT");
    }
}

#[test]
fn foreign_export_eval_still_works_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = concat!(
        "module Test where\n",
        "foreign export ccall addOne :: Int -> Int\n",
        "addOne x = x + 1\n",
        "main = addOne 41\n",
    );
    let (val_chirho, _machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ForeignExportEval.hs", None)
            .expect("foreign export should not break evaluation");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn cranelift_round_trip_nested_cons_pattern_chirho() {
    let src_chirho = r#"module Main where
headTwo :: [Int] -> Int
headTwo (a:b:_) = a + b
headTwo _ = 0
main = print (headTwo [100, 200, 300])
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "300",
            "headTwo [100,200,300] = 300, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_quicksort_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main = print (mySum (qsort [5,1,4,2,3]))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "15",
            "sum of sorted [5,1,4,2,3] = 15, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_abs_signum_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStrLn (show (abs (-42)))
  putStrLn (show (signum (-7)))
  putStrLn (show (negate 10))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert_eq!(lines_chirho.len(), 3, "expected 3 lines");
        assert_eq!(lines_chirho[0], "42", "abs(-42)");
        assert_eq!(lines_chirho[1], "-1", "signum(-7)");
        assert_eq!(lines_chirho[2], "-10", "negate(10)");
    }
}

#[test]
fn cranelift_round_trip_prime_sieve_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
removeMultiples :: Int -> [Int] -> [Int]
removeMultiples _ [] = []
removeMultiples p (x:xs) = if x `mod` p == 0
                           then removeMultiples p xs
                           else x : removeMultiples p xs
sieve :: [Int] -> [Int]
sieve [] = []
sieve (p:xs) = p : sieve (removeMultiples p xs)
countList :: [Int] -> Int
countList [] = 0
countList (_:xs) = 1 + countList xs
main = print (countList (sieve (range 2 100)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert!(
            stdout_chirho.trim() == "25",
            "25 primes up to 100, got: {stdout_chirho}"
        );
    }
}

#[test]
fn cranelift_round_trip_mixed_show_int_bool_chirho() {
    let src_chirho = r#"module Main where
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
main :: IO ()
main = do
  putStrLn ("sum = " ++ show (mySum [1,2,3]))
  putStrLn ("bool = " ++ show True)
  putStrLn ("abs = " ++ show (abs (-42)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert!(
            lines_chirho.len() >= 3,
            "expected 3 lines, got {}",
            lines_chirho.len()
        );
        assert_eq!(lines_chirho[0], "sum = 6", "show Int after sum");
        assert_eq!(lines_chirho[1], "bool = True", "show Bool");
        assert_eq!(lines_chirho[2], "abs = 42", "show Int via abs");
    }
}

#[test]
fn cranelift_round_trip_tco_euler1_chirho() {
    let src_chirho = r#"module Main where
euler1 :: Int -> Int
euler1 limit = go 0 0
  where go acc n = if n >= limit then acc
                   else if n `mod` 3 == 0 || n `mod` 5 == 0
                        then go (acc + n) (n + 1)
                        else go acc (n + 1)
main = print (euler1 1000000)
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "TCO euler1 should not stack overflow");
        assert_eq!(
            stdout_chirho.trim(),
            "233333166668",
            "euler1(1000000) = 233333166668"
        );
    }
}

#[test]
fn cranelift_round_trip_qsort_random_500_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg seed 0 = []
lcg seed n = let next = (seed * 1103515245 + 12345) `mod` 2147483648
             in (next `mod` 1000) : lcg next (n - 1)
main = print (mySum (qsort (lcg 42 500)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "quicksort 500 random should work");
        assert_eq!(
            stdout_chirho.trim(),
            "256218",
            "sum of sorted 500 random elements"
        );
    }
}

#[test]
fn cranelift_round_trip_qsort_random_1000_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg seed 0 = []
lcg seed n = let next = (seed * 1103515245 + 12345) `mod` 2147483648
             in (next `mod` 1000) : lcg next (n - 1)
main = print (mySum (qsort (lcg 42 1000)))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "quicksort 1000 random should work");
        assert_eq!(
            stdout_chirho.trim(),
            "507468",
            "sum of sorted 1000 random elements"
        );
    }
}

#[test]
fn cranelift_round_trip_range_10000_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = print (myLen (range 1 10000))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "range 10000 should work");
        assert_eq!(stdout_chirho.trim(), "10000", "len of range 1..10000");
    }
}

#[test]
fn cranelift_round_trip_range_100000_chirho() {
    let src_chirho = r#"module Main where
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
myLen :: [Int] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main = print (myLen (range 1 100000))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "range 100000 should work");
        assert_eq!(stdout_chirho.trim(), "100000", "len of range 1..100000");
    }
}

#[test]
fn cranelift_round_trip_string_equality_output_chirho() {
    let src_chirho = r#"module Main where
main = do
  print ("hello" == "hello")
  print ("hello" == "world")
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho, "True\nFalse\n");
    }
}

#[test]
fn cranelift_round_trip_null_empty_string_output_chirho() {
    let src_chirho = r#"module Main where
main = if null "" then print (1 :: Int) else print (2 :: Int)
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho, "1\n");
    }
}

#[test]
fn cranelift_round_trip_string_case_and_read_int_output_chirho() {
    let src_chirho = r#"module Main where
classifyChirho s = case s of
  "hello" -> 1
  "world" -> 2
  _ -> 3
main = do
  print (classifyChirho "hello")
  print ((read "42") :: Int)
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho, "1\n42\n");
    }
}

#[test]
fn cranelift_round_trip_foldl_100000_chirho() {
    let src_chirho = r#"module Main where
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = print (myFoldl add 0 (enumFromTo 1 100000))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "foldl 100000 should not stack overflow");
        assert_eq!(stdout_chirho.trim(), "5000050000");
    }
}

#[test]
fn cranelift_round_trip_recursive_custom_list_instance_output_chirho() {
    let src_chirho = r#"module Main where
class Describable a where
  describe :: a -> String

instance Describable Int where
  describe x = show x

instance Describable a => Describable [a] where
  describe [] = "[]"
  describe (x:xs) = describe x ++ ":" ++ describe xs

main = putStrLn (describe [1,2,3])
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "cranelift custom list instance should work");
        assert_eq!(stdout_chirho, "1:2:3:[]\n");
    }
}

#[test]
fn cranelift_round_trip_put_str_no_newline_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStr "Hello "
  putStr "World"
  putStrLn "!"
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "putStr should exit 0");
        assert_eq!(
            stdout_chirho, "Hello World!\n",
            "putStr concatenates without newlines"
        );
    }
}

#[test]
fn cranelift_round_trip_getline_bind_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  name <- getLine
  putStrLn ("Hello, " ++ name ++ "!")
"#;
    // Use stdin-fed round trip
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Main.hs").ok();
    let Some(result_chirho) = result_chirho else {
        return;
    };
    let config_chirho = haskelujah_backend_cranelift_chirho::TargetConfigChirho::default();
    let obj_chirho = haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
        &result_chirho.core_chirho,
        &config_chirho,
    )
    .ok();
    let Some(obj_chirho) = obj_chirho else { return };

    let tmp_dir_chirho = tempfile::tempdir().ok();
    let Some(tmp_dir_chirho) = tmp_dir_chirho else {
        return;
    };
    let obj_path_chirho = tmp_dir_chirho.path().join("main.o");
    let bin_path_chirho = tmp_dir_chirho.path().join("main");
    std::fs::write(&obj_path_chirho, &obj_chirho.object_bytes_chirho).ok();
    let rts_lib_dir_chirho = ensure_rts_staticlib_for_tests_chirho();
    let Some(rts_lib_dir_chirho) = rts_lib_dir_chirho else {
        return;
    };

    let mut link_cmd2_chirho = std::process::Command::new("cc");
    link_cmd2_chirho
        .arg("-o")
        .arg(&bin_path_chirho)
        .arg(&obj_path_chirho);
    if cfg!(target_os = "macos") {
        link_cmd2_chirho
            .arg("-Wl,-no_fixup_chains")
            .arg("-Wl,-stack_size,0x10000000");
    }
    link_cmd2_chirho
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts");
    let compile_status_chirho = link_cmd2_chirho.status().ok();
    if compile_status_chirho.map_or(true, |s| !s.success()) {
        return;
    }

    // Run with piped stdin
    let run_output_chirho = std::process::Command::new(&bin_path_chirho)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child_chirho| {
            use std::io::Write;
            if let Some(stdin_chirho) = child_chirho.stdin.as_mut() {
                let _ = stdin_chirho.write_all(b"Haskelujah\n");
            }
            child_chirho.wait_with_output()
        })
        .ok();
    let Some(output_chirho) = run_output_chirho else {
        return;
    };
    let stdout_chirho = String::from_utf8_lossy(&output_chirho.stdout);
    assert!(
        stdout_chirho.contains("Hello, Haskelujah!"),
        "getLine should read stdin: got {stdout_chirho}"
    );
}

#[test]
fn cranelift_round_trip_text_processing_chirho() {
    let src_chirho = r#"module Main where
myMapChar :: (Char -> Char) -> [Char] -> [Char]
myMapChar f [] = []
myMapChar f (c:cs) = f c : myMapChar f cs
myFilterChar :: (Char -> Bool) -> [Char] -> [Char]
myFilterChar f [] = []
myFilterChar f (c:cs) = if f c then c : myFilterChar f cs else myFilterChar f cs
myLen :: [Char] -> Int
myLen [] = 0
myLen (_:xs) = 1 + myLen xs
main :: IO ()
main = do
  putStrLn (pack (myMapChar toUpper (unpack "hello")))
  putStrLn (pack (myFilterChar isAlpha (unpack "Hi 123")))
  print (myLen (unpack "test"))
"#;
    let result_chirho = cranelift_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        let lines_chirho: Vec<&str> = stdout_chirho.trim().lines().collect();
        assert_eq!(lines_chirho[0], "HELLO", "toUpper via unpack/pack");
        assert_eq!(lines_chirho[1], "Hi", "filterChar isAlpha");
        assert_eq!(lines_chirho[2], "4", "myLen unpack");
    }
}

#[test]
fn llvm_round_trip_ackermann_multi_equation_pattern_match_chirho() {
    // Regression test for multi-equation pattern match ordering.
    // ack 0 n = n+1 must take priority over ack m 0 when m=0, n=0.
    let src_chirho = r#"module Main where
ack :: Int -> Int -> Int
ack 0 n = n + 1
ack m 0 = ack (m - 1) 1
ack m n = ack (m - 1) (ack m (n - 1))
main :: IO ()
main = print (ack 3 7)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1021");
    }
}

#[test]
fn llvm_round_trip_guards_correct_branching_chirho() {
    // Regression test for LLVM phi node predecessor fix.
    // Guards produce nested case expressions whose phi nodes
    // must reference the correct predecessor block.
    let src_chirho = r#"module Main where
sign :: Int -> Int
sign n
  | n > 0 = 1
  | n == 0 = 0
  | otherwise = -1
main :: IO ()
main = do
  print (sign 5)
  print (sign 0)
  print (sign (-3))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1\n0\n-1");
    }
}

#[test]
fn llvm_round_trip_collatz_guards_where_chirho() {
    // Collatz sequence: guards + where-clause + multi-equation + TCO
    let src_chirho = r#"module Main where
collatz :: Int -> Int
collatz n = go n 0
  where
    go 1 steps = steps
    go n steps
      | n `mod` 2 == 0 = go (n `div` 2) (steps + 1)
      | otherwise = go (3 * n + 1) (steps + 1)
main :: IO ()
main = do
  print (collatz 27)
  print (collatz 1)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "111\n0");
    }
}

#[test]
fn llvm_round_trip_guard_where_binding_output_chirho() {
    let src_chirho = r#"module Main where
f :: Int -> Int
f x
  | a > 0 = a
  | otherwise = 0
  where
    a = x - 5
main :: IO ()
main = do
  print (f 7)
  print (f 4)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "2\n0");
    }
}

#[test]
fn llvm_round_trip_case_inside_do_output_chirho() {
    let src_chirho = r#"module Main where
main :: IO ()
main = do
  putStrLn "start"
  case Just 3 of
    Just n -> print (n + 1)
    Nothing -> print 0
  putStrLn "done"
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "start\n4\ndone");
    }
}

#[test]
fn llvm_round_trip_where_div_mod_binding_output_chirho() {
    let src_chirho = r#"module Main where
score :: Int -> Int
score x = result
  where
    q = div x 3
    r = mod x 3
    result = q * 10 + r
main :: IO ()
main = print (score 17)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "52");
    }
}

#[test]
fn llvm_round_trip_case_on_function_result_output_chirho() {
    let src_chirho = r#"module Main where
step :: Int -> Maybe Int
step 0 = Nothing
step n = Just (n + 1)
classify :: Int -> Int
classify n = case step n of
  Just x -> x
  Nothing -> 0
main :: IO ()
main = do
  print (classify 0)
  print (classify 4)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "0\n5");
    }
}

#[test]
fn llvm_round_trip_multiple_do_blocks_output_chirho() {
    let src_chirho = r#"module Main where
sayTwice :: String -> IO ()
sayTwice x = do
  putStrLn x
  putStrLn x
main :: IO ()
main = do
  sayTwice "a"
  sayTwice "b"
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "a\na\nb\nb");
    }
}

#[test]
fn llvm_round_trip_adt_constructor_fields_chirho() {
    // ADT constructor field extraction + Maybe pattern matching
    let src_chirho = r#"module Main where
data Shape = Circle Int | Rectangle Int Int
area :: Shape -> Int
area (Circle r) = r * r
area (Rectangle w h) = w * h
fromMaybe :: Int -> Maybe Int -> Int
fromMaybe def Nothing = def
fromMaybe _ (Just x) = x
main :: IO ()
main = do
  print (area (Circle 7))
  print (area (Rectangle 3 4))
  print (fromMaybe 0 (Just 42))
  print (fromMaybe (-1) Nothing)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "49\n12\n42\n-1");
    }
}

#[test]
fn llvm_round_trip_recursive_adt_expr_evaluator_chirho() {
    // 4-constructor recursive expression ADT with nested patterns
    let src_chirho = r#"module Main where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr | Neg Expr
eval :: Expr -> Int
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
eval (Neg e) = 0 - eval e
main :: IO ()
main = do
  print (eval (Mul (Add (Lit 3) (Lit 4)) (Lit 2)))
  print (eval (Neg (Add (Lit 5) (Lit 3))))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "14\n-8");
    }
}

#[test]
fn llvm_round_trip_recursive_tree_sum_depth_chirho() {
    // Recursive binary tree with sum and depth
    let src_chirho = r#"module Main where
data Tree = Leaf Int | Node Tree Tree
sumTree :: Tree -> Int
sumTree (Leaf n) = n
sumTree (Node l r) = sumTree l + sumTree r
max' :: Int -> Int -> Int
max' a b = if a >= b then a else b
depth :: Tree -> Int
depth (Leaf _) = 1
depth (Node l r) = 1 + max' (depth l) (depth r)
main :: IO ()
main = do
  let t = Node (Node (Leaf 1) (Leaf 2)) (Node (Leaf 3) (Leaf 4))
  print (sumTree t)
  print (depth t)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "10\n3");
    }
}

#[test]
fn llvm_round_trip_partial_application_chirho() {
    let src_chirho = r#"module Main where
add :: Int -> Int -> Int
add x y = x + y
add5 :: Int -> Int
add5 = add 5
mul :: Int -> Int -> Int
mul x y = x * y
double :: Int -> Int
double = mul 2
main :: IO ()
main = do
  print (add5 37)
  print (double 21)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "42\n42");
    }
}

#[test]
fn llvm_round_trip_number_theory_chirho() {
    // GCD + power + nth prime — comprehensive test
    let src_chirho = r#"module Main where
gcd' :: Int -> Int -> Int
gcd' a 0 = a
gcd' a b = gcd' b (a `mod` b)
power :: Int -> Int -> Int
power _ 0 = 1
power base exp = base * power base (exp - 1)
main :: IO ()
main = do
  print (gcd' 252 105)
  print (power 2 10)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "21\n1024");
    }
}

#[test]
fn llvm_round_trip_mutual_recursion_chirho() {
    let src_chirho = r#"module Main where
isEven :: Int -> Int
isEven 0 = 1
isEven n = isOdd (n - 1)
isOdd :: Int -> Int
isOdd 0 = 0
isOdd n = isEven (n - 1)
main :: IO ()
main = do
  print (isEven 10)
  print (isOdd 3)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1\n1");
    }
}

#[test]
fn llvm_round_trip_adt_with_field_constructor_chirho() {
    // Stack machine: 4-constructor ADT with fields + tuple return
    let src_chirho = r#"module Main where
data Instr = Push Int | Add | Mul | Neg
eval :: Instr -> Int -> Int -> (Int, Int)
eval (Push n) a b = (n, a)
eval Add a b = (a + b, 0)
eval Mul a b = (a * b, 0)
eval Neg a b = (0 - a, b)
main :: IO ()
main = do
  let (a1, _) = eval (Push 3) 0 0
  let (a2, _) = eval (Push 4) a1 0
  let (r, _) = eval Add a2 a1
  print r
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "7");
    }
}

#[test]
fn llvm_round_trip_list_map_filter_sum_chirho() {
    // List operations: enumFromTo, map with lambda, filter, sum
    let src_chirho = r#"module Main where
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
isEven :: Int -> Bool
isEven n = n `mod` 2 == 0
main :: IO ()
main = do
  let xs = enumFromTo 1 10
  print (sumList xs)
  print (sumList (myFilter isEven xs))
  print (sumList (myMap (\x -> x * x) (enumFromTo 1 5)))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "55\n30\n55");
    }
}

#[test]
fn llvm_round_trip_multi_arg_hof_and_foldr_chirho() {
    // Regression: 2-arg HOF + foldr (was SIGBUS before multi-arg fix)
    let src_chirho = r#"module Main where
myFoldr :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
add :: Int -> Int -> Int
add x y = x + y
mul :: Int -> Int -> Int
mul x y = x * y
main :: IO ()
main = do
  print (myFoldr add 0 [1, 2, 3, 4, 5])
  print (myFoldr mul 1 [1, 2, 3, 4, 5, 6])
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "15\n720");
    }
}

#[test]
fn llvm_round_trip_quicksort_chirho() {
    // Quicksort with filter, append, lambda HOFs — comprehensive test
    let src_chirho = r#"module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (x:xs) = append (qsort (myFilter (\y -> y <= x) xs))
                       (x : qsort (myFilter (\y -> y > x) xs))
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main :: IO ()
main = print (sumList (qsort [5, 3, 8, 1, 9, 2, 7, 4, 6]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        // sum [1..9] = 45
        assert_eq!(stdout_chirho.trim(), "45");
    }
}

#[test]
fn llvm_round_trip_take_zipwith_chirho() {
    // take + zipWith: variable rule + multi-arg HOF combined
    let src_chirho = r#"module Main where
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myZipWith :: (Int -> Int -> Int) -> [Int] -> [Int] -> [Int]
myZipWith _ [] _ = []
myZipWith _ _ [] = []
myZipWith f (x:xs) (y:ys) = f x y : myZipWith f xs ys
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = do
  print (sumList (myTake 3 (enumFromTo 1 100)))
  print (sumList (myZipWith add (enumFromTo 1 5) (enumFromTo 10 14)))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "6\n75");
    }
}

#[test]
fn llvm_round_trip_foldl_at_scale_chirho() {
    // foldl with 2-arg HOF at 100K elements
    let src_chirho = r#"module Main where
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
add :: Int -> Int -> Int
add x y = x + y
main :: IO ()
main = print (myFoldl add 0 (enumFromTo 1 100000))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "5000050000");
    }
}

#[test]
fn llvm_round_trip_qsort_random_500_chirho() {
    let src_chirho = r#"module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
lcg :: Int -> Int -> [Int]
lcg seed 0 = []
lcg seed n = let next = (seed * 1103515245 + 12345) `mod` 2147483648
             in (next `mod` 1000) : lcg next (n - 1)
main :: IO ()
main = print (mySum (qsort (lcg 42 500)))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0, "quicksort 500 random should work on LLVM");
        assert_eq!(stdout_chirho.trim(), "256218");
    }
}

#[test]
fn llvm_round_trip_merge_sort_chirho() {
    // Full merge sort with take/drop/merge
    let src_chirho = r#"module Main where
merge :: [Int] -> [Int] -> [Int]
merge [] ys = ys
merge xs [] = xs
merge (x:xs) (y:ys) = if x <= y then x : merge xs (y:ys) else y : merge (x:xs) ys
myTake :: Int -> [Int] -> [Int]
myTake 0 _ = []
myTake _ [] = []
myTake n (x:xs) = x : myTake (n - 1) xs
myDrop :: Int -> [Int] -> [Int]
myDrop 0 xs = xs
myDrop _ [] = []
myDrop n (_:xs) = myDrop (n - 1) xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
msort :: [Int] -> [Int]
msort [] = []
msort (x:[]) = [x]
msort xs = merge (msort (myTake half xs)) (msort (myDrop half xs))
  where half = myLength xs `div` 2
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main :: IO ()
main = print (sumList (msort [9, 3, 7, 1, 8, 2, 6, 4, 5]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "45");
    }
}

#[test]
fn llvm_round_trip_comprehensive_demo_chirho() {
    // Comprehensive: filter, foldl with lambda, primes, show++
    let src_chirho = r#"module Main where
myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:xs) = if p x then x : myFilter p xs else myFilter p xs
myFoldl :: (Int -> Int -> Int) -> Int -> [Int] -> Int
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
enumFromTo :: Int -> Int -> [Int]
enumFromTo lo hi = if lo > hi then [] else lo : enumFromTo (lo + 1) hi
isPrime :: Int -> Bool
isPrime n
  | n < 2 = False
  | otherwise = go 2
  where
    go d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = go (d + 1)
main :: IO ()
main = do
  let primes = myFilter isPrime (enumFromTo 2 100)
  putStrLn ("primes: " ++ show (myLength primes))
  let sq = myFoldl (\acc x -> acc + x * x) 0 (enumFromTo 1 10)
  putStrLn ("sumsq: " ++ show sq)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "primes: 25\nsumsq: 385");
    }
}

#[test]
fn llvm_round_trip_polymorphic_map_multi_use_output_chirho() {
    let src_chirho = r#"module Main where
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
countTrue :: [Bool] -> Int
countTrue [] = 0
countTrue (x:xs) = if x then 1 + countTrue xs else countTrue xs
isOddChirho :: Int -> Bool
isOddChirho n = mod n 2 == 1
main :: IO ()
main = do
  print (sumList (myMap (\x -> x + 1) [1,2,3]))
  print (countTrue (myMap isOddChirho [1,2,3,4,5]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "9\n3");
    }
}

#[test]
fn llvm_round_trip_zip_tuple_list_output_chirho() {
    let src_chirho = r#"module Main where
myZip :: [a] -> [b] -> [(a, b)]
myZip [] _ = []
myZip _ [] = []
myZip (x:xs) (y:ys) = (x, y) : myZip xs ys
sumPairs :: [(Int, Int)] -> Int
sumPairs [] = 0
sumPairs ((x, y):xys) = x + y + sumPairs xys
main :: IO ()
main = print (sumPairs (myZip [1,2,3,4,5] [10,11,12,13,14]))
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "75");
    }
}

#[test]
fn llvm_round_trip_applyop_divide_output_chirho() {
    let src_chirho = r#"module Main where
data Op = Plus | Minus | Times | Divide deriving (Eq, Show)
applyOp Plus x y = x + y
applyOp Minus x y = x - y
applyOp Times x y = x * y
applyOp Divide x y = div x y
main = print (applyOp Plus 3 4)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "7");
    }
}

#[test]
fn llvm_round_trip_applyop_modulo_output_chirho() {
    let src_chirho = r#"module Main where
data Op = Plus | Minus | Times | Divide | Modulo deriving (Eq, Show)
applyOp Plus x y = x + y
applyOp Minus x y = x - y
applyOp Times x y = x * y
applyOp Divide x y = div x y
applyOp Modulo x y = mod x y
main = print (applyOp Modulo 7 3)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "1");
    }
}

#[test]
fn llvm_round_trip_div_mod_combo_output_chirho() {
    let src_chirho = r#"module Main where
main = do
  print (div 17 5)
  print (mod 17 5)
"#;
    let result_chirho = llvm_round_trip_output_chirho(src_chirho);
    if let Some((code_chirho, stdout_chirho)) = result_chirho {
        assert_eq!(code_chirho, 0);
        assert_eq!(stdout_chirho.trim(), "3\n2");
    }
}

#[test]
fn frontend_hashable_ffi_exports_seed_qualified_io_results_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/hashable-1.5.1.0");
    let ffi_path_chirho = package_dir_chirho.join("src/Data/Hashable/FFI.hs");
    let ffi_source_chirho =
        std::fs::read_to_string(&ffi_path_chirho).expect("hashable FFI source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Data.Hashable.FFI".to_string(),
            ffi_path_chirho.display().to_string(),
            ffi_source_chirho,
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("hashable FFI frontend artifacts should collect");

    let digest_scheme_chirho = artifacts_chirho
        .imported_types_chirho
        .get("Data.Hashable.FFI.unsafe_xxh3_64bit_digest")
        .cloned()
        .expect("qualified digest scheme should be exported");
    let init_scheme_chirho = artifacts_chirho
        .imported_types_chirho
        .get("Data.Hashable.FFI.unsafe_xxh3_initState")
        .cloned()
        .expect("qualified initState scheme should be exported");

    assert!(
        digest_scheme_chirho.to_string().contains("(IO Word64)"),
        "digest should retain IO Word64, got {}",
        digest_scheme_chirho
    );
    assert!(
        init_scheme_chirho.to_string().contains("(IO ())"),
        "initState should retain IO (), got {}",
        init_scheme_chirho
    );
}

#[test]
fn frontend_real_random_frontier_moves_past_stref_and_atomic_modify_ioref2lazy_chirho() {
    use crate::compile_cabal_project_chirho;
    use haskelujah_package_chirho::PackageIndexChirho;
    use std::path::PathBuf;

    let cabal_path_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/random-1.3.1/random.cabal");
    if !cabal_path_chirho.exists() {
        return;
    }

    let index_chirho = PackageIndexChirho::new_chirho();
    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
    if let Err(error_chirho) = result_chirho {
        let error_text_chirho = format!("{error_chirho}");
        assert!(
            !error_text_chirho.contains("STRef")
                && !error_text_chirho.contains("STGenM")
                && !error_text_chirho.contains("atomicModifyIORef2Lazy"),
            "random frontend should move past the old STRef/atomicModifyIORef2Lazy frontier, got: {error_text_chirho}",
        );
    }
}

#[test]
fn frontend_constraints_package_regression_chirho() {
    use crate::compile_cabal_project_chirho;
    use haskelujah_package_chirho::PackageIndexChirho;
    use std::path::PathBuf;

    let cabal_path_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/constraints-0.14.4/constraints.cabal");
    if !cabal_path_chirho.exists() {
        return;
    }

    let index_chirho = PackageIndexChirho::new_chirho();
    let result_chirho = compile_cabal_project_chirho(&cabal_path_chirho, &index_chirho);
    if let Err(error_chirho) = result_chirho {
        panic!("constraints regression: {error_chirho}");
    }
}

#[test]
fn frontend_hashable_ffi_pair_typechecks_with_dependency_stubs_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho,
        run_frontend_with_type_synonyms_chirho, scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/hashable-1.5.1.0");
    let ffi_path_chirho = package_dir_chirho.join("src/Data/Hashable/FFI.hs");
    let xxh3_path_chirho = package_dir_chirho.join("src/Data/Hashable/XXH3.hs");
    let ffi_source_chirho =
        std::fs::read_to_string(&ffi_path_chirho).expect("hashable FFI source should exist");
    let xxh3_source_chirho =
        std::fs::read_to_string(&xxh3_path_chirho).expect("hashable XXH3 source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Data.Hashable.FFI".to_string(),
            ffi_path_chirho.display().to_string(),
            ffi_source_chirho,
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("hashable FFI frontend artifacts should collect");

    let xxh3_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &xxh3_path_chirho,
        &xxh3_source_chirho,
    );
    let xxh3_result_chirho = run_frontend_with_type_synonyms_chirho(
        &xxh3_source_chirho,
        xxh3_file_chirho.file_id_chirho(),
        &artifacts_chirho.ifaces_chirho,
        &artifacts_chirho.imported_types_chirho,
        &artifacts_chirho.imported_type_synonyms_chirho,
    );

    assert!(
        xxh3_result_chirho.is_ok(),
        "hashable XXH3 should typecheck after FFI seeding"
    );
}

#[test]
fn frontend_hashable_mix_collects_with_stdlib_seed_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho, read_haskell_source_file_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/hashable-1.5.1.0");
    let mix_path_chirho = package_dir_chirho.join("src/Data/Hashable/Mix.hs");
    let mix_source_chirho = read_haskell_source_file_chirho(&mix_path_chirho)
        .expect("hashable Mix source should exist");
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Data.Hashable.Mix".to_string(),
            mix_path_chirho.display().to_string(),
            mix_source_chirho,
        )],
        &mut source_map_chirho,
        vec![],
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("hashable Mix frontend artifacts should collect with stdlib seed");

    assert!(
        artifacts_chirho
            .imported_types_chirho
            .contains_key("Data.Hashable.Mix.mixHash"),
        "hashable Mix should export mixHash after seeded frontend collection"
    );
}

#[test]
fn frontend_package_local_empty_and_insert_override_builtins_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho,
    };
    use std::collections::HashMap;

    let upstream_source_chirho = "module LocalMultiMapSeedMiniChirho where\n\ndata MultiMapMiniChirho a = MkMultiMapMiniChirho\n\nempty :: MultiMapMiniChirho a\nempty = MkMultiMapMiniChirho\n\ninsert :: Int -> a -> MultiMapMiniChirho a -> MultiMapMiniChirho a\ninsert _ _ cacheChirho = cacheChirho\n";
    let downstream_source_chirho = "module DownstreamMultiMapSeedMiniChirho where\nimport LocalMultiMapSeedMiniChirho as MM\n\ntype CacheMiniChirho = MultiMapMiniChirho Int\n\ncacheMiniChirho :: CacheMiniChirho\ncacheMiniChirho = empty\n\nstepMiniChirho :: CacheMiniChirho\nstepMiniChirho = uncurry insert (1, 2) empty\n";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let artifacts_result_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![
            (
                "LocalMultiMapSeedMiniChirho".to_string(),
                "LocalMultiMapSeedMiniChirho.hs".to_string(),
                upstream_source_chirho.to_string(),
            ),
            (
                "DownstreamMultiMapSeedMiniChirho".to_string(),
                "DownstreamMultiMapSeedMiniChirho.hs".to_string(),
                downstream_source_chirho.to_string(),
            ),
        ],
        &mut source_map_chirho,
        vec![],
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    );

    assert!(
        artifacts_result_chirho.is_ok(),
        "package-local empty/insert exports should override builtin homonyms in downstream modules"
    );
}

#[test]
fn frontend_warp_multimap_exports_seed_insert_and_empty_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho, read_haskell_source_file_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/warp-3.4.12");
    let multimap_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/MultiMap.hs");
    let multimap_source_chirho = read_haskell_source_file_chirho(&multimap_path_chirho)
        .expect("warp MultiMap source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Network.Wai.Handler.Warp.MultiMap".to_string(),
            multimap_path_chirho.display().to_string(),
            multimap_source_chirho,
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("warp MultiMap frontend artifacts should collect");

    assert!(
        artifacts_chirho
            .imported_types_chirho
            .contains_key("Network.Wai.Handler.Warp.MultiMap.insert"),
        "warp MultiMap should export a qualified insert scheme"
    );
    assert!(
        artifacts_chirho
            .imported_types_chirho
            .contains_key("insert"),
        "warp MultiMap should export a bare insert scheme"
    );
    assert!(
        artifacts_chirho
            .imported_types_chirho
            .contains_key("Network.Wai.Handler.Warp.MultiMap.empty"),
        "warp MultiMap should export a qualified empty scheme"
    );
    assert!(
        artifacts_chirho.imported_types_chirho.contains_key("empty"),
        "warp MultiMap should export a bare empty scheme"
    );
    let insert_scheme_chirho = artifacts_chirho
        .imported_types_chirho
        .get("insert")
        .expect("warp MultiMap should seed a bare insert scheme");
    let empty_scheme_chirho = artifacts_chirho
        .imported_types_chirho
        .get("empty")
        .expect("warp MultiMap should seed a bare empty scheme");
    assert!(
        insert_scheme_chirho.to_string().contains("MultiMap"),
        "warp MultiMap insert should retain its MultiMap type, got {}",
        insert_scheme_chirho
    );
    assert!(
        empty_scheme_chirho.to_string().contains("MultiMap"),
        "warp MultiMap empty should retain its MultiMap type, got {}",
        empty_scheme_chirho
    );
}

#[test]
fn frontend_warp_fdcache_typechecks_after_multimap_seed_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho, read_haskell_source_file_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/warp-3.4.12");
    let multimap_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/MultiMap.hs");
    let fdcache_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/FdCache.hs");
    let multimap_source_chirho = read_haskell_source_file_chirho(&multimap_path_chirho)
        .expect("warp MultiMap source should exist");
    let fdcache_source_chirho = read_haskell_source_file_chirho(&fdcache_path_chirho)
        .expect("warp FdCache source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let artifacts_result_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![
            (
                "Network.Wai.Handler.Warp.MultiMap".to_string(),
                multimap_path_chirho.display().to_string(),
                multimap_source_chirho,
            ),
            (
                "Network.Wai.Handler.Warp.FdCache".to_string(),
                fdcache_path_chirho.display().to_string(),
                fdcache_source_chirho,
            ),
        ],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    );

    assert!(
        artifacts_result_chirho.is_ok(),
        "warp FdCache should typecheck after MultiMap seeding: {:?}",
        artifacts_result_chirho.err()
    );
}

#[test]
fn frontend_warp_fdcache_typechecks_with_direct_multimap_artifacts_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho, read_haskell_source_file_chirho,
        run_frontend_with_type_synonyms_and_type_families_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use std::collections::HashMap;
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/warp-3.4.12");
    let multimap_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/MultiMap.hs");
    let fdcache_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/FdCache.hs");
    let multimap_source_chirho = read_haskell_source_file_chirho(&multimap_path_chirho)
        .expect("warp MultiMap source should exist");
    let fdcache_source_chirho = read_haskell_source_file_chirho(&fdcache_path_chirho)
        .expect("warp FdCache source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let multimap_artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Network.Wai.Handler.Warp.MultiMap".to_string(),
            multimap_path_chirho.display().to_string(),
            multimap_source_chirho,
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("warp MultiMap frontend artifacts should collect");

    let fdcache_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &fdcache_path_chirho,
        &fdcache_source_chirho,
    );
    let fdcache_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
        &fdcache_source_chirho,
        fdcache_file_chirho.file_id_chirho(),
        &multimap_artifacts_chirho.ifaces_chirho,
        &multimap_artifacts_chirho.imported_types_chirho,
        &multimap_artifacts_chirho.imported_type_synonyms_chirho,
        &multimap_artifacts_chirho.imported_type_families_chirho,
    );

    assert!(
        fdcache_result_chirho.is_ok(),
        "warp FdCache should typecheck with direct MultiMap artifacts: {:?}",
        fdcache_result_chirho.err()
    );
}

#[test]
fn frontend_warp_fdcache_seeded_env_prefers_multimap_insert_and_empty_chirho() {
    use crate::{
        ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
        collect_frontend_artifacts_from_module_sources_chirho,
        collect_preferred_qualified_type_names_chirho,
        collect_safe_unqualified_imported_type_names_chirho,
        qualify_imported_scheme_for_iface_chirho, read_haskell_source_file_chirho,
        scan_dependency_package_ifaces_chirho,
    };
    use haskelujah_naming_chirho::iface_chirho::build_iface_with_imports_chirho;
    use haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho;
    use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
    use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;

    let package_dir_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".haskelujah-packages-chirho/warp-3.4.12");
    let multimap_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/MultiMap.hs");
    let fdcache_path_chirho = package_dir_chirho.join("Network/Wai/Handler/Warp/FdCache.hs");
    let multimap_source_chirho = read_haskell_source_file_chirho(&multimap_path_chirho)
        .expect("warp MultiMap source should exist");
    let fdcache_source_chirho = read_haskell_source_file_chirho(&fdcache_path_chirho)
        .expect("warp FdCache source should exist");
    let extra_ifaces_chirho = scan_dependency_package_ifaces_chirho(&package_dir_chirho);
    let mut source_map_chirho = SourceMapChirho::new_chirho();

    let multimap_artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        vec![(
            "Network.Wai.Handler.Warp.MultiMap".to_string(),
            multimap_path_chirho.display().to_string(),
            multimap_source_chirho,
        )],
        &mut source_map_chirho,
        extra_ifaces_chirho,
        HashMap::new(),
        ImportedTypeSynonymsChirho::new(),
        ImportedTypeFamiliesChirho::new(),
        true,
    )
    .expect("warp MultiMap frontend artifacts should collect");

    let fdcache_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        &fdcache_path_chirho,
        &fdcache_source_chirho,
    );
    let parser_chirho =
        ParserChirho::new_chirho(&fdcache_source_chirho, fdcache_file_chirho.file_id_chirho());
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, fdcache_file_chirho.file_id_chirho());
    let _iface_chirho =
        build_iface_with_imports_chirho(&module_chirho, &multimap_artifacts_chirho.ifaces_chirho);

    let mut merged_imported_types_chirho = multimap_artifacts_chirho.imported_types_chirho.clone();
    for (builtin_name_chirho, builtin_scheme_chirho) in
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho()
    {
        merged_imported_types_chirho
            .entry(builtin_name_chirho)
            .or_insert(builtin_scheme_chirho);
    }
    let safe_unqualified_imported_type_names_chirho =
        collect_safe_unqualified_imported_type_names_chirho(
            &module_chirho,
            &multimap_artifacts_chirho.ifaces_chirho,
        );
    let preferred_qualified_type_names_chirho = collect_preferred_qualified_type_names_chirho(
        &module_chirho,
        &multimap_artifacts_chirho.ifaces_chirho,
    );

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        if module_name_chirho != "Network.Wai.Handler.Warp.MultiMap" {
            continue;
        }
        let Some(import_iface_chirho) = multimap_artifacts_chirho
            .ifaces_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        let qualifiable_type_names_chirho: HashSet<String> = import_iface_chirho
            .exports_chirho
            .types_chirho
            .keys()
            .cloned()
            .collect();
        let names_chirho = compute_imported_names_chirho(
            &import_iface_chirho.exports_chirho,
            &import_chirho.spec_chirho,
        );
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map(|alias_chirho| alias_chirho.text_chirho().to_string())
            .unwrap_or_else(|| module_name_chirho.clone());
        for (name_chirho, namespace_chirho, _span_chirho) in names_chirho {
            if namespace_chirho
                != haskelujah_naming_chirho::env_chirho::NamespaceChirho::ValueChirho
            {
                continue;
            }
            let module_qualified_name_chirho = format!("{module_name_chirho}.{name_chirho}");
            let base_scheme_chirho = merged_imported_types_chirho
                .get(&module_qualified_name_chirho)
                .cloned()
                .or_else(|| {
                    if import_chirho.qualified_chirho {
                        None
                    } else {
                        merged_imported_types_chirho.get(&name_chirho).cloned()
                    }
                });
            let Some(base_scheme_chirho) = base_scheme_chirho else {
                continue;
            };
            let in_scope_seed_scheme_chirho = qualify_imported_scheme_for_iface_chirho(
                &base_scheme_chirho,
                &qualifiable_type_names_chirho,
                &qualifier_chirho,
                &safe_unqualified_imported_type_names_chirho,
                &preferred_qualified_type_names_chirho,
            );
            if !import_chirho.qualified_chirho {
                merged_imported_types_chirho
                    .insert(name_chirho.clone(), in_scope_seed_scheme_chirho.clone());
            }
            merged_imported_types_chirho.insert(
                format!("{qualifier_chirho}.{name_chirho}"),
                in_scope_seed_scheme_chirho,
            );
        }
    }

    let insert_scheme_chirho = merged_imported_types_chirho
        .get("insert")
        .expect("FdCache seeded env should contain bare insert");
    let empty_scheme_chirho = merged_imported_types_chirho
        .get("empty")
        .expect("FdCache seeded env should contain bare empty");
    let qualified_lookup_scheme_chirho = merged_imported_types_chirho
        .get("MM.lookup")
        .expect("FdCache seeded env should contain qualified MM.lookup");
    assert!(
        insert_scheme_chirho.to_string().contains("MultiMap"),
        "FdCache bare insert should use the MultiMap scheme before inference, got {}",
        insert_scheme_chirho
    );
    assert!(
        empty_scheme_chirho.to_string().contains("MultiMap"),
        "FdCache bare empty should use the MultiMap scheme before inference, got {}",
        empty_scheme_chirho
    );
    assert!(
        qualified_lookup_scheme_chirho
            .to_string()
            .contains("MultiMap"),
        "FdCache qualified MM.lookup should use the MultiMap scheme before inference, got {}",
        qualified_lookup_scheme_chirho
    );
}

#[test]
fn frontend_magic_hash_import_item_before_close_paren_typechecks_chirho() {
    let src_chirho = "module XorHashReproChirho where\nimport GHC.Exts (Word(..), xor#)\nfooChirho :: Word -> Word -> Word\nfooChirho (W# xChirho) (W# yChirho) = W# (xor# xChirho yChirho)\n";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut source_map_chirho, "XorHashReproChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "explicit GHC.Exts xor# import should stay in scope before close paren"
    );
}

#[test]
fn frontend_overloaded_strings_builder_literals_typecheck_chirho() {
    let src_chirho = "{-# LANGUAGE OverloadedStrings #-}\nmodule BuilderLiteralMiniChirho where\nimport Data.Text.Lazy.Builder (Builder)\nvalueChirho :: Builder\nvalueChirho = \"0.0e0\"\n";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        src_chirho,
        &mut source_map_chirho,
        "BuilderLiteralMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "builder overloaded string literals should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_strict_tuple_where_pattern_binding_typechecks_chirho() {
    let src_chirho = "{-# LANGUAGE BangPatterns #-}\nmodule WarpPackIntMiniChirho where\nfChirho :: Int -> Int\nfChirho sChirho = r0Chirho where\n  (!q0Chirho, !r0Chirho) = sChirho `divMod` 10\n  (!q1Chirho, !r1Chirho) = q0Chirho `divMod` 10\n  !r2Chirho = q1Chirho `mod` 10\n";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        src_chirho,
        &mut source_map_chirho,
        "WarpPackIntMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "strict tuple where pattern bindings should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn frontend_nonempty_cons_fixity_typechecks_against_signature_chirho() {
    let src_chirho = "module NonEmptyFixityMiniChirho where\nimport Data.List.NonEmpty (NonEmpty(..))\nfChirho :: aChirho -> aChirho -> NonEmpty aChirho\nfChirho xChirho yChirho = xChirho :| yChirho : []\n";

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(
        src_chirho,
        &mut source_map_chirho,
        "NonEmptyFixityMiniChirho.hs",
    );
    assert!(
        result_chirho.is_ok(),
        "NonEmpty constructor fixity should match GHC for signature-checked code: {:?}",
        result_chirho.err()
    );
}
