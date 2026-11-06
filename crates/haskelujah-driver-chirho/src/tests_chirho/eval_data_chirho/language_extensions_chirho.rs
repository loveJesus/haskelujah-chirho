// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Language extension evaluation tests, from KindSignatures to associated type families (row 700).

use crate::{eval_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

// ── KindSignatures (§E.41) ──────────────────────────────────────────

#[test]
fn kind_sig_data_star_chirho() {
    // data Proxy (a :: *) = MkProxy
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Proxy (a :: *) = MkProxy
main = case MkProxy of
  MkProxy -> 42
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("kind sig data star should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
    );
}

#[test]
fn kind_sig_data_with_field_chirho() {
    // data Wrapper (a :: *) = MkWrapper a — kind-annotated var used in constructor
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Wrapper (a :: *) = MkWrapper a
unwrap (MkWrapper x) = x
main = unwrap (MkWrapper 99)
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("kind sig data with field should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
    );
}

#[test]
fn kind_sig_newtype_chirho() {
    // newtype Id (a :: *) = MkId a
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
newtype Id (a :: *) = MkId a
getId (MkId x) = x
main = getId (MkId 77)
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("kind sig newtype should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(77)
    );
}

#[test]
fn kind_sig_mixed_annotated_unannotated_chirho() {
    // data Pair (a :: *) b = MkPair a b — mix of annotated and plain
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Pair (a :: *) b = MkPair a b
fst2 (MkPair x y) = x
main = fst2 (MkPair 55 100)
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("kind sig mixed should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(55)
    );
}

#[test]
fn kind_sig_arrow_kind_chirho() {
    // data HKD (f :: * -> *) = MkHKD — higher-kinded type variable
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data HKD (f :: * -> *) = MkHKD
main = case MkHKD of
  MkHKD -> 123
";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
        .expect("kind sig arrow kind should work");
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(123)
    );
}

// ── MultiWayIf tests ────────────────────────────────────────────────

#[test]
fn multi_way_if_basic_chirho() {
    // Basic multi-way if with otherwise, hitting last branch
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
classify x = if | x > 100   -> 1
                | x > 10    -> 2
                | otherwise  -> 3
main = print (classify 5)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MultiWayIf basic failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "3\n");
}

#[test]
fn multi_way_if_first_branch_chirho() {
    // Multi-way if hitting the first branch
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
f x = if | x > 100  -> 1
         | x > 10   -> 2
         | otherwise -> 3
main = print (f 200)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MultiWayIf first branch failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "1\n");
}

#[test]
fn multi_way_if_middle_branch_chirho() {
    // Multi-way if hitting the middle branch
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE MultiWayIf #-}
module Test where
f x = if | x > 100  -> 1
         | x > 10   -> 2
         | otherwise -> 3
main = print (f 50)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("MultiWayIf middle branch failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "2\n");
}

// ── NumericUnderscores tests ─────────────────────────────────────────

#[test]
fn numeric_underscores_int_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print 1_000_000
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NumericUnderscores int failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "1000000\n");
}

#[test]
fn numeric_underscores_hex_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print 0xFF_FF
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NumericUnderscores hex failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "65535\n");
}

#[test]
fn numeric_underscores_arithmetic_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NumericUnderscores #-}
module Test where
main = print (1_000 + 2_000)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NumericUnderscores arith failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "3000\n");
}

// ── Enum succ/pred tests ────────────────────────────────────────────

#[test]
fn enum_succ_derived_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (succ Red)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Enum succ failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "Green\n");
}

#[test]
fn enum_pred_derived_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (pred Blue)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Enum pred failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "Green\n");
}

#[test]
fn enum_from_enum_derived_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
module Test where
data Color = Red | Green | Blue deriving (Show, Eq, Enum)
main = print (fromEnum Blue)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Enum fromEnum failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "2\n");
}

// ── TupleSections tests ─────────────────────────────────────────────

#[test]
fn tuple_section_left_gap_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print ((,1) 42)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TupleSections left gap failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "(42,1)\n");
}

#[test]
fn tuple_section_right_gap_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print ((1,) 99)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TupleSections right gap failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "(1,99)\n");
}

#[test]
fn tuple_section_map_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE TupleSections #-}
module Test where
main = print (map (,True) [1,2,3])
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("TupleSections map failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "[(1,True),(2,True),(3,True)]\n");
}

// ── StandaloneDeriving tests ────────────────────────────────────────

#[test]
fn standalone_deriving_show_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Show Color
main = print Green
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Show failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "Green\n");
}

#[test]
fn standalone_deriving_eq_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Eq Color
deriving instance Show Color
main = print (Red == Red)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Eq failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "True\n");
}

#[test]
fn standalone_deriving_ord_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE StandaloneDeriving #-}
module Test where
data Color = Red | Green | Blue
deriving instance Eq Color
deriving instance Ord Color
deriving instance Show Color
main = print (compare Red Blue)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("StandaloneDeriving Ord failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "LT\n");
}

// ── DeriveAnyClass test ─────────────────────────────────────────────

#[test]
fn derive_anyclass_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // DeriveAnyClass generates an empty instance relying on default methods.
    // Use with a class that has a default: class MyClass a where myMethod :: a -> Int; myMethod _ = 42
    // For now just verify the extension doesn't cause a parse/compile error
    // and that standard classes still work alongside it.
    let src_chirho = r#"
{-# LANGUAGE DeriveAnyClass #-}
module Test where
data Color = Red | Green | Blue deriving (Show, Eq)
main = print (Red == Green)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DeriveAnyClass failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "False\n");
}

// ── UnicodeSyntax tests ─────────────────────────────────────────────

#[test]
fn unicode_syntax_arrows_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "{-# LANGUAGE UnicodeSyntax #-}\nmodule Test where\nid2 \u{2237} a \u{2192} a\nid2 x = x\nmain = print (id2 42)\n";
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax arrows failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn unicode_syntax_lambda_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = print ((\u{03BB}x -> x + 1) 41)\n";
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax lambda failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn unicode_syntax_fat_arrow_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Test ⇒ for class constraints
    let src_chirho = "{-# LANGUAGE UnicodeSyntax #-}\nmodule Test where\nshowIt \u{2237} Show a \u{21D2} a \u{2192} String\nshowIt x = show x\nmain = putStrLn (showIt 42)\n";
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("UnicodeSyntax fat arrow failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

// ── DerivingStrategies tests ────────────────────────────────────────

#[test]
fn deriving_strategies_stock_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingStrategies #-}
module Test where
data Color = Red | Green | Blue
  deriving stock (Show, Eq)
main = print (Red == Green)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DerivingStrategies stock failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "False\n");
}

#[test]
fn deriving_strategies_stock_newtype_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DerivingStrategies #-}
module Test where
newtype Age = MkAge Int
  deriving stock (Show, Eq)
main = print (MkAge 42)
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| {
                panic!("DerivingStrategies stock newtype failed: {}", e_chirho)
            });
    // Stock-derived Show preserves the constructor; newtype-derived Show erases it.
    assert_eq!(m_chirho.io_output_chirho, "MkAge 42\n");
}

// ── RoleAnnotations test ────────────────────────────────────────────

/// A type without parameters takes no role, so GHC refuses this annotation by the count rule
/// root measured as GHC-54298 (`RoleCountHoldChirho`, row 694 #31844); this exact source is
/// not measured. It used to run because lowering rewrote an annotation it could not read.
#[test]
fn a_role_for_a_type_without_parameters_is_refused_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE RoleAnnotations #-}
module Test where
data MyTag = TagA | TagB deriving Show
type role MyTag nominal
main = print TagA
"#;
    let message_chirho =
        match eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None) {
            Ok(_) => panic!("GHC refuses a role for a type without parameters; it ran"),
            Err(error_chirho) => error_chirho.to_string(),
        };
    assert!(
        message_chirho.contains("Wrong number of roles"),
        "refused, but not for its role count: {message_chirho}"
    );
}

// ── DeriveDataTypeable test ─────────────────────────────────────────

#[test]
fn derive_data_typeable_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE DeriveDataTypeable #-}
module Test where
import Data.Typeable (Typeable)
data Color = Red | Green | Blue deriving (Show, Eq, Typeable)
main = print Green
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("DeriveDataTypeable failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "Green\n");
}

// ---------------------------------------------------------------
// Associated type families in classes
// ---------------------------------------------------------------

/// `{-# LANGUAGE TypeFamilies #-}`: class with associated type family
/// parses and compiles without error (associated type registered as type family).
#[test]
fn assoc_type_family_basic_chirho() {
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class Container f where
  type Elem f
  size :: f -> Int
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
        &mut sm_chirho,
        "TestChirho.hs",
        src_chirho,
    );
    let frontend_chirho = crate::run_frontend_chirho(
        src_chirho,
        sf_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    )
    .expect("frontend should succeed");
    // Verify the class has 1 method and 1 associated type
    let class_decl_chirho = frontend_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| {
            matches!(
                d_chirho,
                haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho { .. }
            )
        })
        .expect("should have a ClassDeclChirho");
    if let haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
        methods_chirho,
        associated_tfs_chirho,
        ..
    } = class_decl_chirho
    {
        assert_eq!(methods_chirho.len(), 1, "should have 1 method (size)");
        assert_eq!(
            associated_tfs_chirho.len(),
            1,
            "should have 1 associated type family (Elem)"
        );
        assert_eq!(associated_tfs_chirho[0].name_chirho.text_chirho(), "Elem");
    }
}

/// Associated type family appears in AST and class has the associated_tfs_chirho field.
#[test]
fn assoc_type_family_parsed_chirho() {
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class MyClass a where
  type MyFamily a :: *
  myMethod :: a -> Int
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
        &mut sm_chirho,
        "TestChirho.hs",
        src_chirho,
    );
    let frontend_chirho = crate::run_frontend_chirho(
        src_chirho,
        sf_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    )
    .expect("frontend should succeed");
    let module_chirho = &frontend_chirho.module_chirho;
    // Find the class decl and check it has an associated type family
    let class_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| {
            matches!(
                d_chirho,
                haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho { .. }
            )
        })
        .expect("should have a ClassDeclChirho");
    if let haskelujah_ast_chirho::decl_chirho::DeclChirho::ClassDeclChirho {
        associated_tfs_chirho,
        ..
    } = class_decl_chirho
    {
        assert_eq!(
            associated_tfs_chirho.len(),
            1,
            "should have 1 associated type family"
        );
        assert_eq!(
            associated_tfs_chirho[0].name_chirho.text_chirho(),
            "MyFamily"
        );
    }
}

/// Associated type family instance in instance decl is extracted.
#[test]
fn assoc_type_family_instance_parsed_chirho() {
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

class MyCol a where
  type Key a

instance MyCol Int where
  type Key Int = Bool
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let sf_chirho = crate::SourceFileChirho::from_source_map_chirho(
        &mut sm_chirho,
        "TestChirho.hs",
        src_chirho,
    );
    let frontend_chirho = crate::run_frontend_chirho(
        src_chirho,
        sf_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &std::collections::HashMap::new(),
    )
    .expect("frontend should succeed");
    let module_chirho = &frontend_chirho.module_chirho;
    let inst_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find(|d_chirho| {
            matches!(
                d_chirho,
                haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho { .. }
            )
        })
        .expect("should have an InstanceDeclChirho");
    if let haskelujah_ast_chirho::decl_chirho::DeclChirho::InstanceDeclChirho {
        assoc_tf_instances_chirho,
        ..
    } = inst_decl_chirho
    {
        assert_eq!(
            assoc_tf_instances_chirho.len(),
            1,
            "should have 1 associated type family instance"
        );
        assert_eq!(
            assoc_tf_instances_chirho[0]
                .family_name_chirho
                .text_chirho(),
            "Key"
        );
    }
}
