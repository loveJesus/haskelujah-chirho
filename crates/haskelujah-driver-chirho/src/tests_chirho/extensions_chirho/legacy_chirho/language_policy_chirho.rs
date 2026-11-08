// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing extension controls, grouped by topic.

use crate::{
    FrontendOptionsChirho, compile_source_chirho, eval_source_chirho,
    eval_source_with_options_chirho, frontend_warnings_chirho,
    frontend_warnings_with_options_chirho,
};
use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn mono_local_binds_chirho() {
    // MonoLocalBinds restricts generalization in where/let — we accept the pragma
    let src_chirho = "\
{-# LANGUAGE MonoLocalBinds #-}
module Test where
main = let x = 42 in x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MonoLocal.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn no_monomorphism_restriction_chirho() {
    // Disable monomorphism restriction
    let src_chirho = "\
{-# LANGUAGE NoMonomorphismRestriction #-}
module Test where
f = (+)
main = f 20 22
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoMono.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

fn defer_typed_holes_chirho() -> FrontendOptionsChirho {
    FrontendOptionsChirho::from_ghc_options_chirho(["-fdefer-typed-holes"])
}

#[test]
fn typed_hole_compiles_chirho() {
    // `_` in expression position compiles as a deferred typed hole warning
    // We just check that it doesn't error out at the type-checking stage
    let src_chirho = "\
module Test where
f :: Int -> Int
f = _
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // This should succeed — the hole function is defined but never called
    let result_chirho = eval_source_with_options_chirho(
        src_chirho,
        &mut sm_chirho,
        "TypedHole.hs",
        None,
        &defer_typed_holes_chirho(),
    )
    .map(|(value_chirho, _machine_chirho)| value_chirho);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn typed_hole_named_compiles_chirho() {
    // Named holes like `_foo` defer the same way
    let src_chirho = "\
module Test where
g :: Int -> Int -> Int
g x y = _result
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_with_options_chirho(
        src_chirho,
        &mut sm_chirho,
        "NamedHole.hs",
        None,
        &defer_typed_holes_chirho(),
    )
    .map(|(value_chirho, _machine_chirho)| value_chirho);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn typed_hole_warning_emitted_chirho() {
    // A deferred typed hole produces a warning
    let src_chirho = "\
module Test where
f :: Int -> Int
f = _
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho = frontend_warnings_with_options_chirho(
        src_chirho,
        &mut sm_chirho,
        "HoleWarn.hs",
        &defer_typed_holes_chirho(),
    );
    let has_hole_warning_chirho = warnings_chirho.iter().any(|w_chirho| {
        let msg_chirho = format!("{:?}", w_chirho);
        msg_chirho.contains("hole") || msg_chirho.contains("4200")
    });
    assert!(has_hole_warning_chirho, "should emit typed hole warning");
}

#[test]
fn haskell2010_pragma_chirho() {
    // {-# LANGUAGE Haskell2010 #-} should be accepted
    let src_chirho = "\
{-# LANGUAGE Haskell2010 #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Haskell2010.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn instance_sigs_chirho() {
    let src_chirho = "\
{-# LANGUAGE InstanceSigs #-}
module Test where
class MyEq a where
  myEq :: a -> a -> Bool
instance MyEq Int where
  myEq :: Int -> Int -> Bool
  myEq x y = x == y
main = if myEq (42 :: Int) (42 :: Int) then 1 else 0
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "InstanceSigs.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(1));
}

#[test]
fn type_synonym_instances_chirho() {
    // TypeSynonymInstances pragma accepted; type synonym instance compiles
    let src_chirho = "\
{-# LANGUAGE TypeSynonymInstances #-}
{-# LANGUAGE FlexibleInstances #-}
module Test where
type Name = [Char]
class Greet a where
  greetLen :: a -> Int
instance Greet Name where
  greetLen xs = length xs
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeSynInst.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn polykinds_pragma_accepted_chirho() {
    // PolyKinds pragma should be accepted and program compiles
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
module Test where
data Proxy a = MkProxy
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds1.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn unboxed_tuples_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE UnboxedTuples #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UnboxedTuples.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn unboxed_sums_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE UnboxedSums #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UnboxedSums.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn implicit_params_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE ImplicitParams #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "ImplicitParams.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn multi_param_type_classes_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE MultiParamTypeClasses #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MPTC.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn functional_dependencies_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE FunctionalDependencies #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "FunDeps.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn type_abstractions_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE TypeAbstractions #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeAbstractions.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn overloaded_record_dot_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE OverloadedRecordDot #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "RecordDot.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn no_field_selectors_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE NoFieldSelectors #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoFieldSel.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn duplicate_record_fields_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE DuplicateRecordFields #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DupRecFields.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn no_star_is_type_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE NoStarIsType #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoStarIsType.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn partial_type_sig_basic_chirho() {
    // Basic PartialTypeSignatures: `_` in return type position. The compiler
    // should infer the wildcard as Int and evaluate correctly.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
f :: _ -> Int
f x = x + 1
main = f 41
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PartialTypeSig.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn partial_type_sig_multiple_wildcards_chirho() {
    // Multiple wildcards in one type signature — each `_` becomes an
    // independent fresh unification variable.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
add :: _ -> _ -> Int
add x y = x + y
main = add 20 22
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "PartialTypeSigMulti.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn partial_type_sig_warning_emitted_chirho() {
    // The compiler should emit warning W4201 for each wildcard found in a
    // type signature when PartialTypeSignatures is active.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
f :: _ -> Int
f x = x + 1
main = f 41
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "PartialTypeSigWarn.hs");
    let has_wildcard_warning_chirho = warnings_chirho.iter().any(|w_chirho| {
        let msg_chirho = format!("{:?}", w_chirho);
        msg_chirho.contains("4201") || msg_chirho.contains("wildcard")
    });
    assert!(
        has_wildcard_warning_chirho,
        "expected W4201 wildcard warning for `_` in type signature"
    );
}

#[test]
fn strict_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Strict #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Strict.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn strict_data_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE StrictData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "StrictData.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn applicative_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE ApplicativeDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "ApplicativeDo.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn generalized_newtype_deriving_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE GeneralizedNewtypeDeriving #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "GND.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn derive_lift_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE DeriveLift #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "DeriveLift.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn safe_haskell_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Safe #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Safe.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn trustworthy_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Trustworthy #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Trustworthy.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE CPP #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPP.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_min_version_transformers_chirho() {
    let src_chirho = "\
{-# LANGUAGE CPP #-}
module Test where
#if MIN_VERSION_transformers(0,6,0)
main = 42
#else
main = 0
#endif
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPPTransformers.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_min_version_deepseq_and_hashable_chirho() {
    let src_chirho = "\
{-# LANGUAGE CPP #-}
module Test where
#if MIN_VERSION_deepseq(1,4,3) && MIN_VERSION_hashable(1,5,0)
main = 42
#else
main = 0
#endif
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPPDepsChirho.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_min_version_random_chirho() {
    let src_chirho = "\
{-# LANGUAGE CPP #-}
module Test where
#if MIN_VERSION_random(1,3,0)
main = 42
#else
main = 0
#endif
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPPRandomChirho.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_min_version_time_chirho() {
    let src_chirho = "\
{-# LANGUAGE CPP #-}
module Test where
#if MIN_VERSION_time(1,9,0)
main = 42
#else
main = 0
#endif
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPPTimeChirho.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_min_version_template_haskell_chirho() {
    let src_chirho = "\
{-# LANGUAGE CPP #-}
module Test where
#if MIN_VERSION_template_haskell(2,16,0) && !MIN_VERSION_template_haskell(2,17,0)
main = 42
#else
main = 0
#endif
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(
            src_chirho,
            &mut sm_chirho,
            "CPPTemplateHaskellChirho.hs",
            None
        )
        .unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn template_haskell_quotes_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TemplateHaskellQuotes #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "THQuotes.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn template_haskell_kind_helpers_appk_arrowk_chirho() {
    let src_chirho = "\
{-# LANGUAGE TemplateHaskell #-}
module THKindHelpersChirho where
import Language.Haskell.TH
import Language.Haskell.TH.Lib (appK, arrowK, appKindT, conT, starK)

kindValueChirho = arrowK `appK` starK `appK` starK
typeValueChirho = AppKindT (ConT (mkName \"Maybe\")) starK
typeValue2Chirho = appKindT (conT (mkName \"Maybe\")) (return starK)
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "THKindHelpersChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "Template Haskell kind helpers should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn qualified_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE QualifiedDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "QualifiedDo.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn overloaded_record_update_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE OverloadedRecordUpdate #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "RecordUpdate.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn type_data_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TypeData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "TypeData.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn ghc2021_meta_extension_chirho() {
    // GHC2021 should expand to all its constituent extensions
    let src_chirho = "\
{-# LANGUAGE GHC2021 #-}
module Test where
data MyList a = Nil | Cons a (MyList a)
main = case Cons 42 Nil of
  Cons x _ -> x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GHC2021.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn ghc2024_meta_extension_chirho() {
    let src_chirho = "{-# LANGUAGE GHC2024 #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "GHC2024.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn options_ghc_x_extension_chirho() {
    // {-# OPTIONS_GHC -XRecursiveDo #-} should enable RecursiveDo extension
    let src_chirho = "{-# OPTIONS_GHC -XNoImplicitPrelude #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "OptsGHC.hs");
    assert!(
        result_chirho.is_ok(),
        "OPTIONS_GHC -X should work: {:?}",
        result_chirho.err()
    );
}
