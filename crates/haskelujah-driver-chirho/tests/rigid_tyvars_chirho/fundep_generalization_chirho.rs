// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit 519 controls: functional dependencies at generalization and at the
//! module-end check. Every verdict below was measured on GHC 9.14.1 (and on
//! a frozen held CLI, 4cfb9584 or its pure move 44fe3bcc) before it was
//! written down: the probes, raw
//! outputs and GHC diagnostics are retained under
//! `tmp-chirho/fundep-generalization-1017-chirho/` of claude2_chirho's
//! worktree. The rejected programs are GHC-39999 "No instance for"; T18398
//! itself stays in the corpus, so only reductions of it are embedded here.
//! tc126 is the one corpus file read in whole, by `include_str!`, beside its
//! reductions.
//! workflow: language-features-chirho/fundep-generalization-chirho

use super::{assert_output_chirho, check_chirho};

const HEADER_CHIRHO: &str = "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies, FlexibleContexts, FlexibleInstances, ExistentialQuantification, GADTSyntax #-}";

/// `class C a b | a -> b` and the existential T18398 matches on.
const C_EX_CHIRHO: &str =
    "class C a b | a -> b where\n  meth :: a -> b -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n";

/// The fixed-range class of gpt_chirho's control: `| a -> b` with a free
/// leading parameter.
const FIXED_RANGE_CHIRHO: &str = "class CChirho pChirho aChirho bChirho | aChirho -> bChirho where\n  methChirho :: pChirho -> aChirho -> bChirho -> String\n\ndata ExChirho = ExChirho\n";

/// `class C4 a b c | b -> c`: a non-leading determinant.
const C4_CHIRHO: &str = "class C4Chirho a b c | b -> c where\n  meth4Chirho :: a -> b -> c -> ()\n\ndata ExChirho = ExChirho\n";

/// `class C3 a b c | a b -> c` with an instance that repeats one variable.
const C3_CHIRHO: &str = "class C3Chirho a b c | a b -> c where\n  meth3Chirho :: a -> b -> c -> ()\n\ndata ExChirho = ExChirho\n\ninstance C3Chirho x x Int where\n  meth3Chirho _ _ _ = ()\n";

/// `class Shape a r | a -> r` with one instance whose head applies a
/// variable, `Shape (c a) (c a)`: it matches `[Int]`, `(Int, Char)` and
/// `Bool -> Bool` by binding `c`, as tc126's `Bug f (c a) (c r)` does.
const SHAPE_CHIRHO: &str = "class ShapeChirho aChirho rChirho | aChirho -> rChirho where\n  shapeChirho :: aChirho -> rChirho\n\ninstance ShapeChirho (cChirho aChirho) (cChirho aChirho) where\n  shapeChirho xChirho = xChirho\n";

/// GHC's should_compile tc126, byte for byte: `Bug (Int -> Int) [Int] r`
/// is solved by `instance Bug f (c a) (c r)` with `c` bound to `[]`.
const TC126_CHIRHO: &str =
    include_str!("../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc126.hs");

/// A type family fixes the determinant only after a later use solves its
/// argument: `KeyChirho [e]` reduces to `ExChirho` once `xsChirho` is known
/// to be a list, and the reduction drops `e`. TypeFamilies implies
/// MonoLocalBinds, so this program carries its own extensions.
const FAMILY_DETERMINANT_CHIRHO: &str = "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies, FlexibleContexts, TypeFamilies #-}\nmodule RangeR11 where\n\nclass CChirho aChirho bChirho | aChirho -> bChirho where\n  methChirho :: aChirho -> bChirho -> String\n\ndata ExChirho = ExChirho\n\ntype family KeyChirho kChirho\ntype instance KeyChirho [eChirho] = ExChirho\n\nkeyChirho :: kChirho -> KeyChirho kChirho\nkeyChirho _ = undefined\n\nfChirho xsChirho yChirho = (methChirho (keyChirho xsChirho) yChirho, xsChirho ++ [])\n";

fn program_chirho(module_chirho: &str, declarations_chirho: &[&str]) -> String {
    format!(
        "{HEADER_CHIRHO}\nmodule {module_chirho} where\n\n{}\n",
        declarations_chirho.join("\n")
    )
}

/// GHC rejects each: a variable only the fundep fixes is not quantified, and
/// no instance supplies the closed determinant.
const REJECTED_CHIRHO: &[(&str, &[&str])] = &[
    (
        "FundepN01Chirho",
        &[
            C_EX_CHIRHO,
            "f x = ((\\y -> (case x of MkEx _ -> meth x y)), (\\z -> (case x of MkEx _ -> meth x z)))",
        ],
    ),
    (
        "FundepN02Chirho",
        &[
            C_EX_CHIRHO,
            "f x = (\\y -> case x of { MkEx _ -> meth x y }, \\z -> case x of { MkEx _ -> meth x z })",
        ],
    ),
    (
        "FundepN03Chirho",
        &[C_EX_CHIRHO, "f x = \\y -> case x of MkEx _ -> meth x y"],
    ),
    (
        "FundepF04Chirho",
        &[
            "class C2 a b | b -> a where\n  meth2 :: a -> b -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "f x = \\y -> case x of MkEx _ -> meth2 y x",
        ],
    ),
    (
        "FundepF05Chirho",
        &[
            "class C4 a b c | b -> c where\n  meth4 :: a -> b -> c -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "f x = \\s t -> case x of MkEx _ -> meth4 s x t",
        ],
    ),
    (
        "FundepF06Chirho",
        &[
            "class C3 a b c | a b -> c where\n  meth3 :: a -> b -> c -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "f x = \\t -> case x of MkEx _ -> meth3 x x t",
        ],
    ),
    (
        "FundepF09Chirho",
        &[
            C_EX_CHIRHO,
            "outerChirho :: Ex -> ()\nouterChirho x = let f = \\y -> case x of MkEx _ -> meth x y in ()",
        ],
    ),
    (
        "FundepF09bChirho",
        &[
            C_EX_CHIRHO,
            "outerChirho :: Ex -> ()\nouterChirho x = let f y = case x of MkEx _ -> meth x y in ()",
        ],
    ),
    (
        "FundepF10Chirho",
        &[
            C_EX_CHIRHO,
            "f :: Ex -> b -> ()\nf x y = case x of MkEx _ -> meth x y",
        ],
    ),
    (
        "FundepF12Chirho",
        &[
            "class C a b | a -> b where\n  meth :: a -> b -> ()\n\ndata Plain = Plain\n",
            "f x = \\y -> meth (x :: Plain) y",
        ],
    ),
    (
        "FundepF16Chirho",
        &[
            C3_CHIRHO,
            "fChirho = \\tChirho -> meth3Chirho ExChirho True tChirho",
        ],
    ),
    (
        "RangeR04Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "fChirho pChirho = \\bChirho -> methChirho pChirho ExChirho bChirho",
        ],
    ),
    (
        "RangeR06Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "type ExAliasChirho = ExChirho\n\nfChirho pChirho = \\bChirho -> methChirho pChirho (ExChirho :: ExAliasChirho) bChirho",
        ],
    ),
    (
        "RangeR09Chirho",
        &[
            C4_CHIRHO,
            "outerChirho :: Int -> ()\nouterChirho n = let gChirho s t = meth4Chirho s ExChirho t in gChirho n 'c'",
        ],
    ),
    (
        "HigherKindedH04Chirho",
        &[
            SHAPE_CHIRHO,
            "data ExChirho = ExChirho\n\nnullaryChirho = shapeChirho ExChirho",
        ],
    ),
];

/// GHC rejects each too; they need extensions the shared header lacks.
const REJECTED_SOURCES_CHIRHO: &[(&str, &str)] = &[("RangeR11Chirho", FAMILY_DETERMINANT_CHIRHO)];

/// GHC accepts each: givens, matching instances, polymorphic or flexible
/// determinants, concrete ranges, classes without a fundep, and nested
/// bindings that quantify a constraint mentioning a quantified variable.
const ACCEPTED_CHIRHO: &[(&str, &[&str])] = &[
    (
        "FundepF01Chirho",
        &[
            C_EX_CHIRHO,
            "g :: C Ex b => Ex -> b -> ()\ng x y = case x of MkEx _ -> meth x y",
        ],
    ),
    (
        "FundepF02Chirho",
        &[
            C_EX_CHIRHO,
            "instance C Ex Int where\n  meth _ _ = ()\n\nf x = \\y -> case x of MkEx _ -> meth x y",
        ],
    ),
    ("FundepF03Chirho", &[C_EX_CHIRHO, "h x y = meth x y"]),
    (
        "FundepF07Chirho",
        &[
            "class C3 a b c | a b -> c where\n  meth3 :: a -> b -> c -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "f x = \\s t -> case x of MkEx _ -> meth3 x s t",
        ],
    ),
    (
        "FundepF08Chirho",
        &[
            "class D a b where\n  methD :: a -> b -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "f x = \\y -> case x of MkEx _ -> methD x y",
        ],
    ),
    (
        "FundepF09cChirho",
        &[
            "class D a b where\n  methD :: a -> b -> ()\n\ndata Ex where\n  MkEx :: a -> Ex\n",
            "outerChirho :: Ex -> ()\nouterChirho x = let f y = case x of MkEx _ -> methD x y in ()",
        ],
    ),
    (
        "FundepF14Chirho",
        &[
            C_EX_CHIRHO,
            "outerChirho :: C a b => a -> b -> ()\nouterChirho x z = let g y = meth x y in g z",
        ],
    ),
    (
        "FundepF15Chirho",
        &[
            "class D a b where\n  methD :: a -> b -> ()\n\ndata Plain = Plain\n",
            "f x = \\y -> methD (x :: Plain) y",
        ],
    ),
    (
        "FundepF17Chirho",
        &[
            C3_CHIRHO,
            "fChirho = \\tChirho -> meth3Chirho ExChirho ExChirho tChirho",
        ],
    ),
    (
        "RangeR00Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "fChirho pChirho = methChirho pChirho ExChirho (0 :: Int)",
        ],
    ),
    (
        "RangeR01Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "outerChirho :: Int -> Bool\nouterChirho nChirho = let gChirho pChirho = methChirho pChirho ExChirho nChirho in True",
        ],
    ),
    (
        "RangeR02Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "outerChirho :: CChirho q ExChirho b => q -> b -> String\nouterChirho qChirho bChirho = let gChirho pChirho = methChirho pChirho ExChirho bChirho in gChirho qChirho",
        ],
    ),
    (
        "RangeR03Chirho",
        &[
            FIXED_RANGE_CHIRHO,
            "instance CChirho p ExChirho Int where\n  methChirho _ _ _ = \"inst\"\n\nfChirho pChirho = \\bChirho -> methChirho pChirho ExChirho bChirho",
        ],
    ),
    (
        "RangeR07Chirho",
        &[
            "class C5Chirho pChirho aChirho bChirho | bChirho -> aChirho where\n  meth5Chirho :: pChirho -> aChirho -> bChirho -> String\n\ndata ExChirho = ExChirho\n",
            "fChirho pChirho = meth5Chirho pChirho (0 :: Int) ExChirho",
        ],
    ),
    (
        "RangeR08Chirho",
        &[
            C4_CHIRHO,
            "outerChirho :: Int -> ()\nouterChirho _ = let gChirho s t = meth4Chirho s ExChirho t in ()",
        ],
    ),
    (
        "SuperclassS00Chirho",
        &[
            "class CChirho aChirho bChirho | aChirho -> bChirho where\n  methChirho :: aChirho -> bChirho -> ()\n\ndata ExChirho = ExChirho\n\nclass CChirho aChirho bChirho => DChirho pChirho aChirho bChirho where {}\n",
            "fChirho :: DChirho pChirho ExChirho bChirho => pChirho -> bChirho -> ()\nfChirho _ bChirho = methChirho ExChirho bChirho",
        ],
    ),
    (
        "HigherKindedH01Chirho",
        &[SHAPE_CHIRHO, "listChirho = shapeChirho [1 :: Int]"],
    ),
    (
        "HigherKindedH02Chirho",
        &[SHAPE_CHIRHO, "pairChirho = shapeChirho (1 :: Int, 'x')"],
    ),
    (
        "HigherKindedH03Chirho",
        &[SHAPE_CHIRHO, "arrowChirho = shapeChirho not"],
    ),
    (
        "HigherKindedH05Chirho",
        &[
            "class WrapChirho aChirho rChirho | aChirho -> rChirho where\n  wrapChirho :: aChirho -> rChirho\n\ninstance WrapChirho (Maybe (cChirho aChirho)) (Maybe (cChirho aChirho)) where\n  wrapChirho xChirho = xChirho\n",
            "maybeChirho = wrapChirho (Just [1 :: Int])",
        ],
    ),
];

/// GHC accepts each too; they need extensions the shared header lacks.
const ACCEPTED_SOURCES_CHIRHO: &[(&str, &str)] = &[("Tc126Chirho", TC126_CHIRHO)];

/// How one program misses its GHC verdict, if it does.
fn miss_chirho(
    expect_rejected_chirho: bool,
    module_chirho: &str,
    source_chirho: &str,
) -> Option<String> {
    match (
        expect_rejected_chirho,
        check_chirho(&format!("{module_chirho}.hs"), source_chirho),
    ) {
        (true, Ok(())) => Some(format!("{module_chirho}: accepted")),
        (true, Err(err_chirho)) if !err_chirho.contains("no instance for") => Some(format!(
            "{module_chirho}: rejected for another reason: {err_chirho}"
        )),
        (false, Err(err_chirho)) => Some(format!("{module_chirho}: rejected: {err_chirho}")),
        _ => None,
    }
}

/// Every program that misses its GHC verdict, by name and with the reason,
/// so a failing run (a mutant included) shows exactly which programs it
/// reached rather than only the first.
fn misses_chirho(
    expect_rejected_chirho: bool,
    programs_chirho: &[(&str, &[&str])],
    sources_chirho: &[(&str, &str)],
) -> Vec<String> {
    let built_chirho = programs_chirho
        .iter()
        .filter_map(|(module_chirho, declarations_chirho)| {
            miss_chirho(
                expect_rejected_chirho,
                module_chirho,
                &program_chirho(module_chirho, declarations_chirho),
            )
        });
    let whole_chirho = sources_chirho
        .iter()
        .filter_map(|(module_chirho, source_chirho)| {
            miss_chirho(expect_rejected_chirho, module_chirho, source_chirho)
        });
    built_chirho.chain(whole_chirho).collect()
}

#[test]
fn a_variable_only_a_fundep_fixes_is_not_quantified_away_chirho() {
    let missed_chirho = misses_chirho(true, REJECTED_CHIRHO, REJECTED_SOURCES_CHIRHO);
    assert!(
        missed_chirho.is_empty(),
        "GHC rejects each of these with `No instance for`: {missed_chirho:#?}"
    );
}

#[test]
fn valid_fundep_programs_stay_accepted_chirho() {
    let missed_chirho = misses_chirho(false, ACCEPTED_CHIRHO, ACCEPTED_SOURCES_CHIRHO);
    assert!(
        missed_chirho.is_empty(),
        "GHC accepts each of these: {missed_chirho:#?}"
    );
}

#[test]
fn a_concrete_state_monad_still_runs_chirho() {
    // mtl's `MonadState s m | m -> s` with `m` fixed by `execState`.
    assert_output_chirho(
        "FundepM01Chirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\nmodule Main where\nimport Control.Monad.State\n\nbumpChirho :: Int -> Int\nbumpChirho k = execState (modify (+ k)) 0\n\nmain :: IO ()\nmain = print (bumpChirho 3)\n",
        "3\n",
    );
}
