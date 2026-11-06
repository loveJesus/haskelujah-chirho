// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typing and binding forms: monad transformers, qualified signatures, constraints, let and where (row 700).

use haskelujah_span_chirho::SourceMapChirho;

// ── Monad transformer infrastructure ──────────────────────────────────

#[test]
fn eval_maybe_t_just_chirho() {
    // Basic MaybeT wrapping and unwrapping via a user-defined single-parameter
    // newtype (inner monad fixed to the identity/ground level for evaluation).
    // newtype MaybeT a = MkMaybeT (Maybe a)
    // getMaybeT extracts the inner Maybe value.
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
newtype MaybeT a = MkMaybeT (Maybe a)
getMaybeT t = case t of { MkMaybeT inner -> inner }
main = case getMaybeT (MkMaybeT (Just 42)) of
         Just n  -> n
         Nothing -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
            "MaybeT (Just 42) should unwrap to 42"
        ),
        Err(e_chirho) => panic!("eval_maybe_t_just_chirho failed: {}", e_chirho),
    }
}

#[test]
fn eval_state_t_basic_chirho() {
    // Basic StateT wrapping and unwrapping via a user-defined newtype.
    // newtype StateT a = MkStateT (Int -> (a, Int))
    // runStateT unwraps and applies the state function.
    // addOne returns 99 and increments state, so result + state = 99+11 = 110.
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
newtype StateT a = MkStateT (Int -> (a, Int))
runStateT t s = case t of { MkStateT f -> f s }
addOne = MkStateT (\\s -> (99, s + 1))
main = case runStateT addOne 10 of
         (a, s) -> a + s
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(110),
            "StateT addOne 10 should give (99, 11), sum = 110"
        ),
        Err(e_chirho) => panic!("eval_state_t_basic_chirho failed: {}", e_chirho),
    }
}

#[test]
fn eval_nested_where_clause_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Nested where clause: innerFun uses a where-bound y that references pattern vars
    let src_chirho = "\
module Test where
outerFun xs = innerFun xs
  where
    innerFun (x:xs) = y
      where y = x + innerFun xs
    innerFun [] = 0
main = outerFun [1,2,3]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            // outerFun [1,2,3] = 1 + (2 + (3 + 0)) = 6
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
            );
        }
        Err(e_chirho) => panic!("nested where clause should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_qualified_type_sig_single_constraint_chirho() {
    // Type signature with a single constraint: Eq a => a -> a -> Bool
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
myEq :: Eq a => a -> a -> Bool
myEq x y = x == y
main = myEq 3 3
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::BoolChirho(true)
            );
        }
        Err(e_chirho) => panic!("qualified type sig should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_qualified_type_sig_multiple_constraints_chirho() {
    // Type signature with tuple constraint: (Eq a, Show a) => a -> String
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
describeEq :: (Eq a, Show a) => a -> a -> String
describeEq x y = if x == y then show x else \"not equal\"
main = do
  putStrLn (describeEq 42 42)
  putStrLn (describeEq 42 43)
";
    let (_, machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("multiple constraints should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "42\nnot equal\n");
}

#[test]
fn eval_equality_constraint_simple_chirho() {
    // Type equality constraint: (a ~ Int) => a -> a
    // This should type-check and evaluate successfully
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
identity :: (a ~ Int) => a -> a
identity x = x
main = identity 42
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
        Err(e_chirho) => panic!("equality constraint should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_equality_constraint_bare_chirho() {
    // Bare equality constraint without parens: a ~ Int => a -> a
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
identity :: a ~ Int => a -> a
identity x = x
main = identity 42
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
        Err(e_chirho) => panic!("bare equality constraint should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_let_case_in_single_line_chirho() {
    // let d' = case d of PAT -> EXPR in d'  (all on one line)
    // Tests that `in` correctly closes the intervening case-of layout
    // context before closing the let context.
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
convert d = let d' = case d of True -> 1 ; False -> 0 in d'
main = convert True
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
            );
        }
        Err(e_chirho) => panic!("single-line let/case/in should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_polymorphic_return_io_default_chirho() {
    // return is now Monad m => a -> m a; when m is ambiguous, defaults to IO
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = do
  let x = 42
  return x
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
            "polymorphic return with IO default should evaluate: {}",
            e_chirho
        ),
    }
}

#[test]
fn eval_monad_constraint_return_compiles_chirho() {
    // Monad m => a -> m a type signature should compile without errors
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
wrap :: Monad m => a -> m a
wrap x = return x
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "Monad m => a -> m a with return should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_monad_constraint_return_nothing_chirho() {
    // f :: Monad m => m (Maybe a) — polymorphic monad with return
    // When f is used at top level, m defaults to IO
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
data Maybe a = Nothing | Just a
f :: Monad m => m Int
f = return 42
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "Monad m => m Int with return should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_functor_applicative_monad_io_instances_chirho() {
    // Verify Functor IO, Applicative IO, Monad IO instances resolve
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = do
  return 42
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
        Err(e_chirho) => panic!("IO monad instance should resolve: {}", e_chirho),
    }
}

#[test]
fn eval_where_clause_captures_outer_var_chirho() {
    // GHC tc081: where-clause function captures outer function's parameter
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f x = (x + 1, g True, g 'c')
  where
    g y = if x > 2 then [] else [y]
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "where-clause capturing outer var should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_let_polymorphic_generalization_chirho() {
    // Classic let-polymorphism: `let f x = x in (f True, f 'c')`
    // f must be generalized to `forall a. a -> a` so it can be used at both types
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = let f x = x in (f True, f 'c')
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "let-polymorphism should generalize f: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_let_polymorphic_list_singleton_chirho() {
    // let-binding `wrap x = [x]` used at multiple types
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = let wrap x = [x]
       in (wrap True, wrap 42)
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "let-polymorphism for wrap should work: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_where_multiple_polymorphic_bindings_chirho() {
    // Multiple where-clause bindings, each should be generalized independently
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f = (g True, g 'a', h [1], h [True])
  where
    g x = [x]
    h xs = length xs
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "multiple polymorphic where-bindings: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_list_type_constructor_application_chirho() {
    // App(Con("[]"), a) must unify with List(a): the list type constructor
    // applied to a type should equal the list sugar notation [a].
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f :: [] Int -> [Int]
f xs = xs
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "[] Int should unify with [Int]: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_tuple_constructor_unification_chirho() {
    // Tuple constructor applied via type application should unify with
    // tuple sugar. This tests the unification normalization, not parsing.
    // We test via a data type that produces App(App(Con("(,)"), a), b)
    // through type family reduction or similar.
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Simpler: just verify (a, b) tuple works in unification
    let src_chirho = "\
module Test where
swap :: (a, b) -> (b, a)
swap (x, y) = (y, x)
result = swap (True, 'c')
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "tuple swap: {:?}",
        result_chirho.err()
    );
}

#[test]
fn eval_list_type_sig_bracket_sugar_chirho() {
    // [] a in a type signature should unify with [a] sugar
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
g :: [] Char -> [Char]
g xs = xs
h :: [Bool] -> [] Bool
h ys = ys
";
    let result_chirho = compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs");
    assert!(
        result_chirho.is_ok(),
        "[] sugar interop: {:?}",
        result_chirho.err()
    );
}
