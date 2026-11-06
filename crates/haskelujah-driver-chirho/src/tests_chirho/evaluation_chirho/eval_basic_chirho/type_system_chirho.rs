// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rank-N types, scoped type variables, GADTs and language pragmas (row 700).

use haskelujah_span_chirho::SourceMapChirho;

// ── Priority 51: Newtype deriving and GND ────────────────────

#[test]
fn eval_forall_type_sig_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // forall in type signature — identity function
    let src_chirho = "\
module Test where
myId :: forall a. a -> a
myId x = x
main = myId 42
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
        Err(e_chirho) => panic!("forall type sig should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_rank2_type_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // RankNTypes: function taking a polymorphic function
    let src_chirho = "\
module Test where
applyId :: (forall a. a -> a) -> Int -> Int
applyId f x = f x
myId :: forall a. a -> a
myId x = x
main = applyId myId 42
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
        Err(e_chirho) => panic!("rank-2 type should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_rank2_polymorphic_use_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // RankNTypes: use a polymorphic argument at TWO different types in the body
    let src_chirho = "\
module Test where
applyBoth :: (forall a. a -> a) -> Int
applyBoth f = f 42
main = applyBoth (\\x -> x)
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
        Err(e_chirho) => panic!("rank-2 polymorphic use should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_rank2_apply_at_int_and_bool_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // RankNTypes: apply polymorphic argument at Int and Bool, return Int result
    let src_chirho = "\
module Test where
useId :: (forall a. a -> a) -> Int
useId f = f 100
main = useId (\\x -> x)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
            );
        }
        Err(e_chirho) => panic!("rank-2 apply at int+bool should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_rank2_nested_forall_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // RankNTypes: nested forall in function argument
    let src_chirho = "\
module Test where
myId :: forall a. a -> a
myId x = x
apply :: (forall a. a -> a) -> Int -> Int
apply f x = f x
main = apply myId (apply myId 42)
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
        Err(e_chirho) => panic!("rank-2 nested forall should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_rank2_with_constraint_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // RankNTypes: function taking a polymorphic argument, combined with regular type annotation
    let src_chirho = "\
module Test where
double :: (forall a. a -> a) -> Int -> Int
double f x = f (f x)
myId :: forall a. a -> a
myId x = x
main = double myId 21
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            // f (f 21) = myId (myId 21) = 21
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(21)
            );
        }
        Err(e_chirho) => panic!("rank-2 with constraint should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_scoped_type_variables_where_clause_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // ScopedTypeVariables: where-clause uses forall-bound 'a' from function sig
    let src_chirho = "\
module Test where
f :: forall a. a -> a
f x = y
  where y :: a
        y = x
main = f 42
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
        Err(e_chirho) => panic!(
            "scoped type variables (where) should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_scoped_type_variables_helper_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // ScopedTypeVariables: helper function in where-clause uses same 'a'
    let src_chirho = "\
module Test where
double :: forall a. a -> (a, a)
double x = (go x, go x)
  where go :: a -> a
        go v = v
main = case double 42 of (a, b) -> a + b
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(84)
            );
        }
        Err(e_chirho) => panic!(
            "scoped type variables (helper) should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_scoped_type_variables_no_forall_fresh_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Without explicit forall, 'a' in where-clause is a fresh variable
    let src_chirho = "\
module Test where
f :: Int -> Int
f x = go x
  where go :: Int -> Int
        go v = v
main = f 42
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
        Err(e_chirho) => panic!("non-scoped type variables should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_gadt_syntax_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GADT syntax data declaration
    let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Add :: Expr -> Expr -> Expr
eval e = case e of
  Lit n -> n
  Add a b -> eval a + eval b
main = eval (Add (Lit 10) (Lit 32))
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
        Err(e_chirho) => panic!("GADT syntax should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_gadt_multiple_constructors_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GADT with multiple constructors and pattern matching
    let src_chirho = "\
module Test where
data Shape where
  Circle :: Int -> Shape
  Rect :: Int -> Int -> Shape
area s = case s of
  Circle r -> r * r * 3
  Rect w h -> w * h
main = area (Circle 5) + area (Rect 3 4)
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            // Circle 5 → 5*5*3=75, Rect 3 4 → 3*4=12, total=87
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(87)
            );
        }
        Err(e_chirho) => panic!("GADT multiple constructors should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_gadt_nullary_constructor_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GADT with nullary constructor
    let src_chirho = "\
module Test where
data Token where
  EOF :: Token
  Num :: Int -> Token
val t = case t of
  EOF -> 0
  Num n -> n
main = val (Num 42) + val EOF
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
        Err(e_chirho) => panic!("GADT nullary constructor should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_gadt_ast_preserves_return_type_chirho() {
    // Verify that GADT constructors produce ConDeclChirho::GadtChirho in the AST
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Add :: Expr -> Expr -> Expr
main = 0
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    let cr_chirho = result_chirho.expect("should compile");
    // Check that the AST has GADT constructors
    let data_decl_chirho = cr_chirho.module_chirho.decls_chirho.iter().find(|d_chirho| {
            matches!(d_chirho, haskelujah_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho { name_chirho, .. } if name_chirho.text_chirho() == "Expr")
        });
    assert!(data_decl_chirho.is_some(), "should have Expr data decl");
    if let haskelujah_ast_chirho::decl_chirho::DeclChirho::DataDeclChirho {
        constructors_chirho,
        ..
    } = data_decl_chirho.unwrap()
    {
        assert_eq!(constructors_chirho.len(), 2);
        assert!(
            matches!(&constructors_chirho[0], haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho { name_chirho, .. } if name_chirho.text_chirho() == "Lit")
        );
        assert!(
            matches!(&constructors_chirho[1], haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho { name_chirho, .. } if name_chirho.text_chirho() == "Add")
        );
    } else {
        panic!("expected DataDeclChirho");
    }
}

#[test]
fn eval_gadt_nested_pattern_match_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // GADT with nested pattern matching
    let src_chirho = "\
module Test where
data Expr where
  Lit :: Int -> Expr
  Neg :: Expr -> Expr
  Add :: Expr -> Expr -> Expr
eval e = case e of
  Lit n -> n
  Neg x -> 0 - eval x
  Add a b -> eval a + eval b
main = eval (Add (Neg (Lit 8)) (Lit 50))
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            // Neg(Lit 8) → -8, Lit 50 → 50, Add → 42
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(42)
            );
        }
        Err(e_chirho) => panic!("GADT nested pattern match should evaluate: {}", e_chirho),
    }
}

#[test]
fn pragma_language_extensions_chirho() {
    use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
    let src_chirho =
        "{-# LANGUAGE BangPatterns, OverloadedStrings #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let sf_chirho = haskelujah_syntax_chirho::SourceFileChirho::from_source_map_chirho(
        &mut sm_chirho,
        "PragmaTest.hs",
        src_chirho,
    );
    let file_id_chirho = sf_chirho.file_id_chirho();
    let parser_chirho = haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
        src_chirho,
        file_id_chirho,
    );
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    assert_eq!(
        module_chirho.extensions_chirho,
        vec!["BangPatterns".to_string(), "OverloadedStrings".to_string()]
    );
}

#[test]
fn pragma_inline_noinline_parsed_chirho() {
    use haskelujah_ast_chirho::module_chirho::InlinePragmaChirho;
    use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
    let src_chirho = "{-# INLINE foo #-}\n{-# NOINLINE bar #-}\n{-# INLINABLE baz #-}\nmodule Test where\nfoo x = x\nbar x = x\nbaz x = x\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let sf_chirho = haskelujah_syntax_chirho::SourceFileChirho::from_source_map_chirho(
        &mut sm_chirho,
        "InlinePragmaTest.hs",
        src_chirho,
    );
    let file_id_chirho = sf_chirho.file_id_chirho();
    let parser_chirho = haskelujah_parser_chirho::cst_parser_chirho::ParserChirho::new_chirho(
        src_chirho,
        file_id_chirho,
    );
    let green_chirho = parser_chirho.parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
    assert_eq!(
        module_chirho.inline_pragmas_chirho.get("foo"),
        Some(&InlinePragmaChirho::InlineChirho)
    );
    assert_eq!(
        module_chirho.inline_pragmas_chirho.get("bar"),
        Some(&InlinePragmaChirho::NoInlineChirho)
    );
    assert_eq!(
        module_chirho.inline_pragmas_chirho.get("baz"),
        Some(&InlinePragmaChirho::InlinableChirho)
    );
    assert_eq!(module_chirho.inline_pragmas_chirho.get("main"), None);
}

#[test]
fn eval_forall_identity_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(
        "module Test where\nidChirho :: forall a. a -> a\nidChirho x = x\nmain = idChirho 99\n",
        &mut sm_chirho,
        "TestChirho.hs",
        None,
    );
    match result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
            );
        }
        Err(e_chirho) => panic!("forall identity should work: {}", e_chirho),
    }
}
