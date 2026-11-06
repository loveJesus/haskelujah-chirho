// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Record syntax, RecordWildCards and NamedFieldPuns evaluation tests (row 700).

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

// ── Priority 52: Record syntax field access ────────────────────

// ── Priority 52: Record syntax field access ────────────────────
#[test]
fn eval_record_construction_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Record construction with field names
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 3, yCoord = 4 } of { MkPoint x y -> x + y }
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(7)
            );
        }
        Err(e_chirho) => panic!("record construction should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_record_field_accessor_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Field accessor function: xCoord and yCoord used as functions
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
main = xCoord p + yCoord p
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("record field accessor should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_newtype_record_field_accessor_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Newtype record field accessor: getAge used as a function
    let src_chirho = "\
module Test where
newtype Age = MkAge { getAge :: Int }
myAge = MkAge 25
main = getAge myAge
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(25)
            );
        }
        Err(e_chirho) => panic!(
            "newtype record field accessor should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_newtype_runidentity_pattern_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Pattern: newtype Identity a = Identity { runIdentity :: a }
    let src_chirho = "\
module Test where
newtype Identity a = Identity { runIdentity :: a }
wrapped = Identity 42
main = runIdentity wrapped
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("newtype runIdentity accessor should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_record_pattern_match_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Record pattern: match using { field = var } syntax
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
getSum p = case p of
  MkPoint { xCoord = x, yCoord = y } -> x + y
main = getSum (MkPoint { xCoord = 12, yCoord = 30 })
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("record pattern match should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_record_update_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Record update: p { xCoord = 99 }
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
p = MkPoint { xCoord = 10, yCoord = 20 }
q = p { xCoord = 99 }
main = xCoord q + yCoord q
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(119)
            );
        }
        Err(e_chirho) => panic!("record update should evaluate: {}", e_chirho),
    }
}

// ── RecordWildCards ──────────────────────────────────────────────

#[test]
fn eval_record_wildcards_pattern_chirho() {
    // RecordWildCards in pattern: Foo{..} binds all fields as variables
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 10, yCoord = 20 } of
  MkPoint{..} -> xCoord + yCoord
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("RecordWildCards pattern should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_record_wildcards_expr_chirho() {
    // RecordWildCards in expression: Con{..} fills missing fields from let-bound scope
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = let xCoord = 10
           yCoord = 20
       in case MkPoint{..} of
            MkPoint { xCoord = a, yCoord = b } -> a + b
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!("RecordWildCards expression should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_record_wildcards_partial_pattern_chirho() {
    // RecordWildCards with some explicit fields: MkPoint{xCoord = a, ..}
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = case MkPoint { xCoord = 10, yCoord = 20 } of
  MkPoint{xCoord = a, ..} -> a + yCoord
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(30)
            );
        }
        Err(e_chirho) => panic!(
            "RecordWildCards partial pattern should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_record_wildcards_partial_expr_chirho() {
    // RecordWildCards in expression with some explicit fields: MkPoint{xCoord = 10, ..}
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
main = let yCoord = 32
       in case MkPoint{xCoord = 10, ..} of
            MkPoint{xCoord = a, yCoord = b} -> a + b
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!(
            "RecordWildCards partial expression should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn debug_record_wildcard_expr_ast_chirho() {
    // Debug test: verify MkPoint{..} parses as RecordConChirho with has_wildcard_chirho=true
    let src_chirho = "\
module Test where
data Point = MkPoint { xCoord :: Int, yCoord :: Int }
f = MkPoint{..}
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let fid_chirho = sm_chirho.add_file_chirho("test.hs", src_chirho);
    let cst_chirho =
        haskelujah_parser_chirho::cst_parser_chirho::parse_to_cst_chirho(src_chirho, fid_chirho);
    let module_chirho =
        haskelujah_parser_chirho::lower_chirho::lower_module_chirho(&cst_chirho, fid_chirho);
    // Find the f declaration and check its RHS
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
    for decl_chirho in &module_chirho.decls_chirho {
        eprintln!("DECL: {:#?}", decl_chirho);
    }
    let found_wildcard_chirho = module_chirho.decls_chirho.iter().any(|decl_chirho| {
        // Check FunBindChirho
        if let DeclChirho::FunBindChirho {
            name_chirho,
            matches_chirho,
            ..
        } = decl_chirho
            && name_chirho.text_chirho() == "f"
        {
            for m_chirho in matches_chirho {
                if let RhsChirho::UnguardedChirho(ExprChirho::RecordConChirho {
                    has_wildcard_chirho,
                    ..
                }) = &m_chirho.rhs_chirho
                {
                    return *has_wildcard_chirho;
                }
            }
        }
        // Check PatBindChirho
        if let DeclChirho::PatBindChirho {
            rhs_chirho:
                RhsChirho::UnguardedChirho(ExprChirho::RecordConChirho {
                    has_wildcard_chirho,
                    ..
                }),
            ..
        } = decl_chirho
        {
            return *has_wildcard_chirho;
        }
        false
    });
    assert!(
        found_wildcard_chirho,
        "MkPoint{{..}} should parse as RecordConChirho with has_wildcard_chirho=true"
    );
}

// ── NamedFieldPuns tests ──────────────────────────────────────────

#[test]
fn named_field_puns_pattern_chirho() {
    // NamedFieldPuns in pattern position: `MkPoint{x, y}` = `MkPoint{x=x, y=y}`
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Point = MkPoint { x :: Int, y :: Int }
sumPoint (MkPoint{x, y}) = x + y
main = print (sumPoint (MkPoint{x = 10, y = 32}))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns pattern failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}

#[test]
fn named_field_puns_expr_chirho() {
    // NamedFieldPuns in expression position: `MkPoint{x, y}` = `MkPoint{x=x, y=y}`
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Point = MkPoint { x :: Int, y :: Int }
getX (MkPoint{x = val}) = val
main = let x = 10
           y = 32
       in print (getX (MkPoint{x, y}))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns expr failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "10\n");
}

#[test]
fn named_field_puns_mixed_chirho() {
    // Mix of punned and explicit fields
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE NamedFieldPuns #-}
module Test where
data Pair = MkPair { fst :: Int, snd :: Int }
addPair (MkPair{fst, snd = b}) = fst + b
main = print (addPair (MkPair{fst = 20, snd = 22}))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("NamedFieldPuns mixed failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "42\n");
}
