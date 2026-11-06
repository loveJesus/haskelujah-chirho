// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class dictionaries, instances and method dispatch (row 700).

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn from_integer_explicit_call_chirho() {
    // Test that `fromInteger 42` works through the dict pass.
    // fromInteger is a Num class method; for Int it's the identity.
    use crate::compile_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = fromInteger 42
";
    let compile_result_chirho =
        compile_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs")
            .expect("should compile");

    let (result_chirho, _) = crate::stg_lower_chirho::lower_and_run_chirho(
        &compile_result_chirho.core_chirho,
        None,
        compile_result_chirho.newtype_cons_chirho,
    )
    .expect("should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(42),
        "fromInteger 42 should evaluate to 42"
    );
}

#[test]
fn callsite_dict_arg_insertion_chirho() {
    // A user-defined constrained function `add` that calls a class
    // method `+` internally. When `main` calls `add 3 4`, the dict
    // pass must insert the Num dictionary at the call site:
    //   add = \$dNum -> \x -> \y -> ($sel_Num_+ $dNum) x y
    //   main = add $fNumInt (fromInteger 3) (fromInteger 4)
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
add x y = x + y
main = add 3 4
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("call-site dict insertion should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(7),
        "add 3 4 should be 7"
    );
}

#[test]
fn callsite_dict_passthrough_chirho() {
    // Dict pass-through: a constrained function `add` is called from
    // another constrained function `addTwice`, which is then called
    // from monomorphic `main`. The dict must flow through two levels:
    //   addTwice = \$dNum -> \x -> \y -> add $dNum (add $dNum x y) y
    //   main = addTwice $fNumInt 3 4
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
add x y = x + y
addTwice x y = add (add x y) y
main = addTwice 3 4
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("dict pass-through should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(11),
        "addTwice 3 4 = add (add 3 4) 4 = add 7 4 = 11"
    );
}

#[test]
fn superclass_dict_extraction_chirho() {
    // Tests context reduction + superclass dict extraction:
    // `f` uses both `==` (Eq) and `+` (Num). Context reduction removes
    // the redundant `Eq a` pred since `Num` has `Eq` as a superclass.
    // The dict pass generates:
    //   f = \$dNum -> let $dEq = $sel_Num_super_Eq $dNum in
    //                   if ($sel_Eq_== $dEq) x y then ($sel_Num_+ $dNum) x y else ...
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 3
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("superclass dict extraction should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(6),
        "f 3 3: 3 == 3 is true, so 3 + 3 = 6"
    );
}

#[test]
fn superclass_dict_extraction_else_branch_chirho() {
    // Same as above but hits the else branch: 3 /= 4 so x - y = -1
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
f x y = if x == y then x + y else x - y
main = f 3 4
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("superclass dict extraction else branch should evaluate");
    assert_eq!(
        result_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(-1),
        "f 3 4: 3 /= 4, so 3 - 4 = -1"
    );
}

#[test]
fn eval_eq_list_double_chirho() {
    // [1.0, 2.0] == [1.0, 2.0] should be True (via conditional Eq [a] instance)
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 2.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "equal\n",
                "[1.0, 2.0] == [1.0, 2.0] should be True"
            );
        }
        Err(e_chirho) => {
            panic!("Eq [Double] test failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_eq_list_double_neq_chirho() {
    // [1.0, 2.0] == [1.0, 3.0] should be False
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = if [1.0, 2.0] == [1.0, 3.0] then putStrLn \"equal\" else putStrLn \"not equal\"
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho, "not equal\n",
                "[1.0, 2.0] == [1.0, 3.0] should be False"
            );
        }
        Err(e_chirho) => {
            panic!("Eq [Double] neq test failed: {}", e_chirho);
        }
    }
}

#[test]
fn eval_user_instance_show_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Instance method with a single-equation case instead of multi-equation
    let src_chirho = "\
module Test where
data Color = Red | Green | Blue
instance Show Color where
  show c = case c of
    Red -> \"Red\"
    Green -> \"Green\"
    Blue -> \"Blue\"
main = putStrLn (show Red)
";
    let (_, machine_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None)
            .expect("user Show instance should evaluate");
    assert_eq!(machine_chirho.io_output_chirho, "Red\n");
}

#[test]
fn eval_class_default_method_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Class with default method; instance provides only isEqual,
    // isNotEqual should use the default.
    let src_chirho = "\
module Test where
class MyEq a where
  isEqual :: a -> a -> Bool
  isNotEqual :: a -> a -> Int
  isNotEqual x y = if isEqual x y then 0 else 1
instance MyEq Int where
  isEqual x y = x == y
main = isNotEqual 3 4
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
        Err(e_chirho) => panic!("class default method should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_mptc_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Multi-parameter type class
    let src_chirho = "\
module Test where
class Addable a b where
  myAdd :: a -> b -> Int
instance Addable Int Int where
  myAdd x y = x + y
main = myAdd 10 32
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
        Err(e_chirho) => panic!("multi-param type class should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_fundep_parsed_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // MPTC with functional dependency parsed from source
    let src_chirho = "\
module Test where
class Convert a b | a -> b where
  convert :: a -> b
instance Convert Int Int where
  convert x = x + 1
main = convert 41
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
        Err(e_chirho) => panic!("fundep class should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_conditional_instance_chirho() {
    use crate::eval_source_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // User-defined class with conditional instance (simplified body)
    let src_chirho = "\
module Test where
class Describable a where
  desc :: a -> Int
instance Describable Int where
  desc x = x
instance Describable a => Describable [a] where
  desc xs = 99
main = desc [42]
";
    let result_chirho =
        eval_source_chirho(src_chirho, &mut source_map_chirho, "TestChirho.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
            );
        }
        Err(e_chirho) => panic!("conditional instance should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_fmap_dispatch_chirho() {
    use crate::eval_source_chirho;
    // fmap dispatches to the per-type Functor instance by the shape of its
    // second argument: Either (Right maps, Left passes through), Maybe, and [].
    let cases_chirho: [(&str, i64); 3] = [
        (
            "main = case fmap (+1) (Right 2 :: Either Int Int) of { Right x -> x; Left _ -> 0 }\n",
            3,
        ),
        (
            "main = case fmap (+10) (Just 5) of { Just x -> x; Nothing -> 0 }\n",
            15,
        ),
        ("main = sum (fmap (*2) [1,2,3])\n", 12),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected fmap result for: {body_chirho}"
            ),
            Err(e_chirho) => panic!("fmap should dispatch by functor type: {e_chirho}"),
        }
    }
}

// workflow: monadic-dispatch-chirho — WI-001 acceptance (prd_chirho.json)
#[test]
fn eval_monad_bind_dispatch_chirho() {
    use crate::eval_source_chirho;
    // Explicit >>= dispatches to $prim_Monad_>>=_<Head> by the first
    // argument's type: Either (Right binds, Left short-circuits), Maybe, [].
    let cases_chirho: [(&str, i64); 4] = [
        (
            "main = case ((Right 3 :: Either Int Int) >>= (\\x -> Right (x * 2))) of { Right x -> x; Left e -> e }\n",
            6,
        ),
        (
            "main = case (Just 10 >>= (\\x -> Just (x - 1))) of { Just x -> x; Nothing -> 0 }\n",
            9,
        ),
        ("main = sum ([1,2,3] >>= (\\x -> [x, x * 10]))\n", 66),
        (
            "main = case ((Left 7 :: Either Int Int) >>= (\\x -> Right (x * 2))) of { Right x -> x; Left e -> e }\n",
            7,
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected >>= result for: {body_chirho}"
            ),
            Err(e_chirho) => panic!(">>= should dispatch by monad type: {e_chirho}"),
        }
    }
}

// workflow: monadic-dispatch-chirho — WI-001 acceptance (prd_chirho.json)
#[test]
fn eval_monad_then_dispatch_chirho() {
    use crate::eval_source_chirho;
    let cases_chirho: [(&str, i64); 2] = [
        (
            "main = case (Just 1 >> Just 5) of { Just x -> x; Nothing -> 0 }\n",
            5,
        ),
        (
            "main = case ((Nothing >> Just 5) :: Maybe Int) of { Just x -> x; Nothing -> 0 }\n",
            0,
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected >> result for: {body_chirho}"
            ),
            Err(e_chirho) => panic!(">> should dispatch by monad type: {e_chirho}"),
        }
    }
}

// workflow: monadic-dispatch-chirho — WI-002 acceptance (prd_chirho.json)
#[test]
fn eval_maybe_do_dispatch_chirho() {
    use crate::eval_source_chirho;
    // do-notation now desugars to real >>= chains: Maybe do-blocks bind the
    // wrapped value (not the whole Just) and Nothing short-circuits.
    let cases_chirho: [(&str, i64); 3] = [
        (
            "maybeSum :: Maybe Int\nmaybeSum = do\n  x <- Just 1\n  y <- Just 2\n  return (x + y)\nmain = case maybeSum of { Just n -> n; Nothing -> 0 }\n",
            3,
        ),
        (
            "f :: Maybe Int\nf = do\n  x <- Just 1\n  _ <- Nothing\n  return x\nmain = case f of { Just n -> n; Nothing -> 99 }\n",
            99,
        ),
        (
            "pairSum :: Maybe Int\npairSum = do\n  (a, b) <- Just (3, 4)\n  return (a + b)\nmain = case pairSum of { Just n -> n; Nothing -> 0 }\n",
            7,
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected maybe-do result for: {body_chirho}"
            ),
            Err(e_chirho) => panic!("maybe-do should ride >>= dispatch: {e_chirho}"),
        }
    }
}

// workflow: monadic-dispatch-chirho — Higher-order non-monadic functions must
// not be rewritten through the higher-kinded method dispatch fallback.
#[test]
fn eval_higher_order_on_function_dispatch_chirho() {
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
on f g x y = f (g x) (g y)
main = ((+) `on` abs) (-3) 3
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(6)
        ),
        Err(e_chirho) => panic!("higher-order on function should evaluate: {e_chirho}"),
    }
}

// workflow: monadic-dispatch-chirho — The CLI probe's `on` shape goes through
// `print`, so it must select a body-backed Product Applicative path rather than
// a loud missing-method placeholder.
#[test]
fn eval_higher_order_on_function_print_output_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
main = print (((+) `on` abs) (-3) 3)
  where on f g x y = f (g x) (g y)
";
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "6\n")
        }
        Err(e_chirho) => panic!("higher-order on print should evaluate: {e_chirho}"),
    }
}

// workflow: monadic-dispatch-chirho — WI-003 annotation-driven return/pure
// dispatch. Result-position annotations can prove the monad head for direct
// return/pure expressions; function-shaped annotations and embedded IO returns
// must keep the IO primop fallback.
#[test]
fn eval_annotated_pure_return_dispatch_chirho() {
    use crate::{eval_source_chirho, eval_source_with_machine_chirho};
    let value_cases_chirho: [(&str, i64); 6] = [
        (
            "main = case (pure 7 :: Maybe Int) of { Just x -> x; Nothing -> 0 }\n",
            7,
        ),
        (
            "main = case (pure 7 :: Either String Int) of { Right x -> x; Left _ -> 0 }\n",
            7,
        ),
        (
            "f :: Int -> Maybe Int\nf x = return (x + 1)\nmain = case f 4 of { Just x -> x; Nothing -> 0 }\n",
            5,
        ),
        (
            "f :: Int -> Maybe Int\nf = return\nmain = case f 8 of { Just x -> x; Nothing -> 0 }\n",
            8,
        ),
        (
            "g :: Int -> Either String Int\ng = pure\nmain = case g 9 of { Right x -> x; Left _ -> 0 }\n",
            9,
        ),
        (
            "main = case ((return 1 :: Maybe Int) >>= (\\x -> Just (x + 1))) of { Just x -> x; Nothing -> 0 }\n",
            2,
        ),
    ];
    for (body_chirho, want_chirho) in value_cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected annotated return/pure result for: {body_chirho}"
            ),
            Err(e_chirho) => {
                panic!("annotated return/pure should dispatch for {body_chirho}: {e_chirho}")
            }
        }
    }

    let stdout_cases_chirho: [(&str, &str); 5] = [
        ("main = (return :: Int -> IO Int) 5 >>= print\n", "5\n"),
        (
            "f :: Int -> IO Int\nf = return\nmain = f 4 >>= print\n",
            "4\n",
        ),
        ("say = putStrLn\nmain = say \"aliased\"\n", "aliased\n"),
        (
            "main = (return 5 :: IO Int) >>= (\\x -> print (case (pure x :: Maybe Int) of { Just y -> y; Nothing -> 0 }))\n",
            "5\n",
        ),
        (
            "acts :: [IO Int]\nacts = [return 1, return 2]\nmain = head acts >>= print\n",
            "1\n",
        ),
    ];
    for (body_chirho, want_chirho) in stdout_cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho =
            eval_source_with_machine_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok((_, machine_chirho)) => assert_eq!(
                machine_chirho.io_output_chirho, want_chirho,
                "annotated IO fallback changed for: {body_chirho}"
            ),
            Err(e_chirho) => {
                panic!("annotated IO fallback should evaluate for {body_chirho}: {e_chirho}")
            }
        }
    }
}

#[test]
fn eval_applicative_either_dispatch_chirho() {
    use crate::eval_source_chirho;
    let cases_chirho: [(&str, i64); 4] = [
        (
            "main = case (Right (+1) <*> Right 2 :: Either Int Int) of { Right x -> x; Left e -> e }\n",
            3,
        ),
        (
            "main = case (Left 9 <*> Right 2 :: Either Int Int) of { Right x -> x; Left e -> e }\n",
            9,
        ),
        (
            "main = case (Right (+1) <*> Left 8 :: Either Int Int) of { Right x -> x; Left e -> e }\n",
            8,
        ),
        (
            "main = case (Just (+1) <*> Just 2) of { Just x -> x; Nothing -> 0 }\n",
            3,
        ),
    ];
    for (body_chirho, want_chirho) in cases_chirho {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let src_chirho = format!("module Test where\n{body_chirho}");
        let result_chirho = eval_source_chirho(&src_chirho, &mut sm_chirho, "TestChirho.hs", None);
        match result_chirho {
            Ok(val_chirho) => assert_eq!(
                val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(want_chirho),
                "unexpected Applicative result for: {body_chirho}"
            ),
            Err(e_chirho) => panic!("Either Applicative should dispatch: {e_chirho}"),
        }
    }
}
