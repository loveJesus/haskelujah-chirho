// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Row 694 GND method coercibility and representation dictionaries on the interpreter,
//! against root's fresh GHC 9.14.1 references (#31784, #31789, #31797, #31798, #31817). Each source is
//! byte for byte the reference's, with the sha256 its plan pins. Where Haskelujah matches GHC
//! the test asserts GHC's output or refusal; where it names an Unsupported gap the test
//! asserts that refusal and its comment says what GHC does.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use crate::{compile_source_chirho, eval_source_with_machine_chirho, typecheck_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

/// What the interpreter prints for an accepted program.
pub(super) fn interpreted_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{file_name_chirho}: interpreter: {error_chirho}"));
    machine_chirho.io_output_chirho
}

/// The compiler refuses the program, naming every reason.
pub(super) fn assert_refused_chirho(
    file_name_chirho: &str,
    source_chirho: &str,
    reasons_chirho: &[&str],
) {
    let message_chirho = match compile_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        Ok(_) => panic!("{file_name_chirho}: this program must be refused; it compiled"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    for reason_chirho in reasons_chirho {
        assert!(
            message_chirho.contains(reason_chirho),
            "{file_name_chirho}: refused, but not for `{reason_chirho}`: {message_chirho}"
        );
    }
}

/// R1: GHC 9.14.1 refuses it, GHC-18872. sha256 be22f447fd6b668b5c289a67c6ecc9cd17de2884652ec8948a2b26de15b7299a.
const NOMINAL_EXPLICIT_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho nominal
data HoldChirho aChirho = HoldChirho aChirho

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (WrapChirho)

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho (AgeChirho yearsChirho) -> print yearsChirho
"#;

#[test]
fn gnd_r1_nominal_explicit_hold_chirho() {
    assert_refused_chirho(
        "NominalExplicitHoldChirho.hs",
        NOMINAL_EXPLICIT_HOLD_CHIRHO,
        &["E0209", "whose parameter is nominal"],
    );
}

/// R2: GHC 9.14.1 refuses it, GHC-18872. sha256 ea9951f186fdb013c0fb8552cbd26df990c50467c8ea354ddf0da050513109a1.
const NOMINAL_FAMILY_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, TypeFamilies #-}
type family LabelChirho aChirho
type instance LabelChirho Int = Int

data HoldChirho aChirho = HoldChirho (LabelChirho aChirho)

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (WrapChirho)

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho _ -> putStrLn "held"
"#;

#[test]
fn gnd_r2_nominal_family_hold_chirho() {
    assert_refused_chirho(
        "NominalFamilyHoldChirho.hs",
        NOMINAL_FAMILY_HOLD_CHIRHO,
        &["E0209", "whose parameter is nominal"],
    );
}

/// H1: GHC 9.14.1 refuses it, GHC-18872: the equality context makes `HoldChirho`'s parameter
/// nominal (#31811, measured #31817). sha256 dbe48f44aebc40899c0852d953ebb625989979a70f153ca7e518b5f87fbf32d3.
const CONSTRUCTOR_CONTEXT_ROLE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- gpt #31809, 02db58e5. GHC 9.14.1 prediction (UNEXECUTED): REJECT. The equality context
-- makes HoldChirho's parameter nominal, so GND cannot coerce wrapChirho (GHC-18872, as R1/R2).
-- 02db58e5 code reading: ACCEPT, prints 8. H98 lowering drops `forall . (aChirho ~ Int) =>`,
-- role inference then reads aChirho as phantom, and the method proof and alias succeed.
-- No-context twin: root's measured R3 (RepresentationalHoldChirho), which prints 8.
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, GADTs, ExistentialQuantification #-}
data HoldChirho aChirho = forall . (aChirho ~ Int) => HoldChirho Int

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype WrapChirho

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho countChirho -> print countChirho
"#;

#[test]
fn gnd_h1_constructor_context_role_chirho() {
    assert_refused_chirho(
        "ConstructorContextRoleChirho.hs",
        CONSTRUCTOR_CONTEXT_ROLE_CHIRHO,
        &["E0209", "whose parameter is nominal"],
    );
}

/// R3: GHC 9.14.1 prints 8. sha256 4a6fdc8163db435c8e202915e83954ceaa1514b5961194984153f43e03523d41.
const REPRESENTATIONAL_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
data HoldChirho aChirho = HoldChirho aChirho

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (WrapChirho)

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho (AgeChirho yearsChirho) -> print yearsChirho
"#;

#[test]
fn gnd_r3_representational_hold_chirho() {
    assert_eq!(
        interpreted_chirho(
            "RepresentationalHoldChirho.hs",
            REPRESENTATIONAL_HOLD_CHIRHO
        ),
        "8\n"
    );
}

/// R4: GHC 9.14.1 prints 8. sha256 0f71cbcf0cb05eb4b050fdecc3805fdd4e82569c33cf1c56f6259afb384130dc.
const CONSTANT_RESULT_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho nominal
data HoldChirho aChirho = HoldChirho aChirho

class TallyChirho aChirho where
  tallyChirho :: aChirho -> HoldChirho Int

instance TallyChirho Int where
  tallyChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (TallyChirho)

main :: IO ()
main = case tallyChirho (AgeChirho 7) of
  HoldChirho countChirho -> print countChirho
"#;

#[test]
fn gnd_r4_constant_result_hold_chirho() {
    assert_eq!(
        interpreted_chirho("ConstantResultHoldChirho.hs", CONSTANT_RESULT_HOLD_CHIRHO),
        "8\n"
    );
}

/// R5: GHC 9.14.1 refuses it too, for a different reason: forwarding needs UndecidableInstances (GHC-22979). sha256 b9e6e8ce2bdcef59121ed5b2462ee324e2299ee3265d5392203bea3af14f0a80.
const ASSOCIATED_FAMILY_FORWARD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, TypeFamilies #-}
class HasChirho aChirho where
  type ItemChirho aChirho
  getChirho :: aChirho -> ItemChirho aChirho

instance HasChirho Int where
  type ItemChirho Int = Int
  getChirho valueChirho = valueChirho + 1

newtype AgeChirho = AgeChirho Int deriving newtype (HasChirho)

main :: IO ()
main = print (getChirho (AgeChirho 7))
"#;

#[test]
fn gnd_r5_associated_family_forward_chirho() {
    assert_refused_chirho(
        "AssociatedFamilyForwardChirho.hs",
        ASSOCIATED_FAMILY_FORWARD_CHIRHO,
        &["E0209", "associated family"],
    );
}

/// R5T: GHC 9.14.1 prints 8: forwarding an associated family is a named Unsupported gap. sha256 9b429c61691dd843cdb5a896bbcd4845cffad9328dd48888ddac38fb152061b5.
const ASSOCIATED_FAMILY_FORWARD_UNDECIDABLE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UndecidableInstances #-}
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, TypeFamilies #-}
class HasChirho aChirho where
  type ItemChirho aChirho
  getChirho :: aChirho -> ItemChirho aChirho

instance HasChirho Int where
  type ItemChirho Int = Int
  getChirho valueChirho = valueChirho + 1

newtype AgeChirho = AgeChirho Int deriving newtype (HasChirho)

main :: IO ()
main = print (getChirho (AgeChirho 7))
"#;

#[test]
fn gnd_r5t_associated_family_forward_undecidable_chirho() {
    assert_refused_chirho(
        "AssociatedFamilyForwardUndecidableChirho.hs",
        ASSOCIATED_FAMILY_FORWARD_UNDECIDABLE_CHIRHO,
        &["E0209", "associated family"],
    );
}

/// P1: GHC 9.14.1 prints 7 then True. sha256 a68f213fb4e799407a14dd546b694952695b3ff0dfb9fde2dbde0a255a62ba45.
const PARAMETRIC_NO_INLINE_SHOW_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/parametric_no_inline_show_chirho.hs"
);

#[test]
fn gnd_p1_parametric_no_inline_show_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ParametricNoInlineShowChirho.hs",
            PARAMETRIC_NO_INLINE_SHOW_CHIRHO
        ),
        "7\nTrue\n"
    );
}

/// P2: GHC 9.14.1 prints integer then boolean. sha256 76f2a192bcbf2dd58c0bf97e865fce5271cf12e92dff823f05df1b985ffc8fcd.
const PARAMETRIC_USER_DICTIONARY_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/parametric_user_dictionary_chirho.hs"
);

#[test]
fn gnd_p2_parametric_user_dictionary_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ParametricUserDictionaryChirho.hs",
            PARAMETRIC_USER_DICTIONARY_CHIRHO
        ),
        "integer\nboolean\n"
    );
}

/// S1 (#31851): a written `ParentChirho (BoxChirho a)` is the GND `ChildChirho (BoxChirho a)`'s
/// superclass, never `ParentChirho Int`. GHC 9.14.1 prints `written` (root's
/// gnd-superclass-r1-chirho, #31854). sha256 a2db4c2a0fbcaaf17902d6b4aa0390dac821d71708c4986dbedf9f5ce6245b22.
const PARAMETRIC_WRITTEN_SUPERCLASS_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/parametric_written_superclass_chirho.hs"
);

#[test]
fn gnd_s1_parametric_written_superclass_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ParametricWrittenSuperclassChirho.hs",
            PARAMETRIC_WRITTEN_SUPERCLASS_CHIRHO
        ),
        "written\n"
    );
}

/// M1 (#31864): direct `show` and `describeChirho` calls at `BoxChirho Int` and `BoxChirho Bool`,
/// through the parametric GND instances, with no helper. GHC 9.14.1 prints 7, True, integer,
/// boolean (root's role-surfaces-and-direct-gnd-r1-chirho, #31865). sha256 60e765e782f0c8d3ee13fede12f020f70c622e226bd87f786c61e5a6eb4066dd.
const DIRECT_PARAMETRIC_GND_METHODS_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/direct_parametric_gnd_methods_chirho.hs"
);

#[test]
fn gnd_m1_direct_parametric_gnd_methods_chirho() {
    assert_eq!(
        interpreted_chirho(
            "DirectParametricGndMethodsChirho.hs",
            DIRECT_PARAMETRIC_GND_METHODS_CHIRHO
        ),
        "7\nTrue\ninteger\nboolean\n"
    );
}

/// A1 and W1 (#31897, #31909): T3955's shape over its own Reader, derived through GND and
/// written. GHC 9.14.1 accepts both and prints 7 (mptc-superclass-r1-chirho, #31916). Here both
/// check like GHC and compile without a refusal, through the multi-parameter superclass routes.
/// Their output is not asserted: `>>=` at an abstract monad falls back to IO, an older gap with
/// its own repair (#31929, #31936). A1 sha256
/// cdce7e412c011dc56559ef286accbee019ed976976c54144fdf374cb442e1263, W1 sha256
/// 055f6d2c5d4f3b0183434109281bcffd028f41f3bf73028c8b95052a0f836a20.
const CLASS_ARGUMENT_SUPERCLASS_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/class_argument_superclass_chirho.hs"
);
const WRITTEN_CLASS_ARGUMENT_SUPERCLASS_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/written_class_argument_superclass_chirho.hs"
);

#[test]
fn gnd_a1_and_w1_check_and_compile_chirho() {
    for (file_name_chirho, source_chirho) in [
        (
            "ClassArgumentSuperclassChirho.hs",
            CLASS_ARGUMENT_SUPERCLASS_CHIRHO,
        ),
        (
            "WrittenClassArgumentSuperclassChirho.hs",
            WRITTEN_CLASS_ARGUMENT_SUPERCLASS_CHIRHO,
        ),
    ] {
        typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            file_name_chirho,
        )
        .unwrap_or_else(|error_chirho| {
            panic!("{file_name_chirho}: GHC accepts it; the checker refused it: {error_chirho}")
        });
        compile_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            file_name_chirho,
        )
        .unwrap_or_else(|error_chirho| {
            panic!("{file_name_chirho}: GHC accepts it; the compiler refused it: {error_chirho}")
        });
    }
}

/// S2 (#31909): the written instance's `Show b` field is `Show Bool`'s, its class's template at
/// the whole head. GHC 9.14.1 prints `True` (mptc-superclass-r1-chirho, #31916). sha256
/// e25a7df38fa334dbb454270977dad3ba5174e4f3260278ef729330a08af71ff9.
const SECOND_ARGUMENT_SUPERCLASS_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/second_argument_superclass_chirho.hs"
);

#[test]
fn mptc_s2_second_argument_superclass_chirho() {
    assert_eq!(
        interpreted_chirho(
            "SecondArgumentSuperclassChirho.hs",
            SECOND_ARGUMENT_SUPERCLASS_CHIRHO
        ),
        "True\n"
    );
}

/// G1 (#31928): the GND `ShowLastChirho Bool AgeChirho` holds the newtype's own stock `Show`,
/// never the representation's `Show Int`. GHC 9.14.1 prints `AgeChirho 5`
/// (mptc-superclass-fallback-r1-chirho, #31936). sha256
/// d824d8061ba8e711ca2920aa443882b5637db546904bde00f31a98cfea5f719b.
const STOCK_SUPERCLASS_GROUND_NEWTYPE_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/stock_superclass_ground_newtype_chirho.hs"
);

#[test]
fn mptc_g1_ground_newtype_stock_superclass_chirho() {
    assert_eq!(
        interpreted_chirho(
            "StockSuperclassGroundNewtypeChirho.hs",
            STOCK_SUPERCLASS_GROUND_NEWTYPE_CHIRHO
        ),
        "AgeChirho 5\n"
    );
}

/// G3 (#31928): the factory's `Show (BoxChirho a)` field is the newtype's own GND factory applied
/// to the representation dictionary's superclass field. GHC 9.14.1 prints `5`
/// (mptc-superclass-fallback-r1-chirho, #31936). sha256
/// a075b01c9225219352697b4d1ac651178e1c2396c69b7e5991119deb099503ec.
const NEWTYPE_SUPERCLASS_PARAMETRIC_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/newtype_superclass_parametric_chirho.hs"
);

#[test]
fn mptc_g3_parametric_newtype_own_superclass_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NewtypeSuperclassParametricChirho.hs",
            NEWTYPE_SUPERCLASS_PARAMETRIC_CHIRHO
        ),
        "5\n"
    );
}

/// R2 (#31909): a class-argument GND whose superclass constrains the class's first parameter,
/// not the newtype's. No route builds that dictionary yet, so the module is refused by name.
/// GHC 9.14.1 is predicted to accept it (unmeasured); this asserts only the refusal.
const OFF_NEWTYPE_SUPERCLASS_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, MultiParamTypeClasses, FlexibleInstances, FlexibleContexts #-}
module Main where

class (Show r, Eq m) => FirstSuperChirho r m

instance FirstSuperChirho Bool Int

newtype AgeChirho = AgeChirho Int
  deriving (Eq, FirstSuperChirho Bool)

main :: IO ()
main = putStrLn "ok"
"#;

#[test]
fn gnd_class_argument_superclass_off_the_newtype_is_refused_by_name_chirho() {
    assert_refused_chirho(
        "OffNewtypeSuperclassChirho.hs",
        OFF_NEWTYPE_SUPERCLASS_CHIRHO,
        &[
            "deriving FirstSuperChirho for AgeChirho through the newtype strategy",
            "constrains another position than the newtype's",
        ],
    );
}

/// R3 (#31948): a written multi-parameter instance whose superclass only an instance with a
/// context serves (`Show [Bool]` through `Show a => Show [a]`). This pass builds no such field,
/// so the module is refused by name when it compiles, never left with an unbuilt dictionary.
/// GHC 9.14.1 is predicted to print `[True,False]` (unmeasured): a valid-source gap.
const CONTEXTUAL_SUPERCLASS_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances #-}
module Main where

class Show b => ShowSecondChirho a b

instance ShowSecondChirho Int [Bool]

describeChirho :: ShowSecondChirho a b => a -> b -> String
describeChirho _ secondChirho = show secondChirho

main :: IO ()
main = putStrLn (describeChirho (5 :: Int) [True, False])
"#;

#[test]
fn mptc_contextual_superclass_is_refused_by_name_chirho() {
    assert_refused_chirho(
        "ContextualSuperclassChirho.hs",
        CONTEXTUAL_SUPERCLASS_CHIRHO,
        &[
            "the `Show` superclass dictionary of the instance `ShowSecondChirho Int [Bool]`",
            "has a context, whose dictionaries this pass does not build here",
        ],
    );
}

/// Q1: GHC 9.14.1 prints (True,False). sha256 151287147a71c3efc462a709bb33a9f929a23e9033982d455a4e3dbf6ff7911c.
const QUALIFIED_STOCK_WITH_SHADOWED_INSTANCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies #-}
import Prelude hiding (Eq(..))
import qualified Prelude as P
class Eq aChirho where
  sameChirho :: aChirho -> aChirho -> Bool
instance Eq Int where
  sameChirho _ _ = False
data ColorChirho = ColorChirho Int deriving stock (P.Eq)
main :: IO ()
main = print ((ColorChirho 7 P.== ColorChirho 7, sameChirho (7 :: Int) (7 :: Int)))
"#;

#[test]
fn gnd_q1_qualified_stock_with_shadowed_instance_chirho() {
    assert_eq!(
        interpreted_chirho(
            "QualifiedStockWithShadowedInstanceChirho.hs",
            QUALIFIED_STOCK_WITH_SHADOWED_INSTANCE_CHIRHO
        ),
        "(True,False)\n"
    );
}

/// C1: GHC 9.14.1 prints 78. sha256 c7eae33b2c35720f26260d9cb232fc3b677305536bef471afcae2ad3b41f43c5.
const CONTEXTUAL_REPRESENTATION_DEFAULT_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/contextual_representation_default_chirho.hs"
);

#[test]
fn gnd_c1_contextual_representation_default_chirho() {
    assert_eq!(
        interpreted_chirho(
            "ContextualRepresentationDefaultChirho.hs",
            CONTEXTUAL_REPRESENTATION_DEFAULT_CHIRHO
        ),
        "78\n"
    );
}

/// C2: GHC 9.14.1 prints 78. sha256 b46b09a3c2898eb221e8ba3357228239e11f598178412e7070d79740008a0481.
const NESTED_GND_CONTEXT_DEPENDENCY_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/gnd_methods_chirho/nested_gnd_context_dependency_chirho.hs"
);

#[test]
fn gnd_c2_nested_gnd_context_dependency_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NestedGndContextDependencyChirho.hs",
            NESTED_GND_CONTEXT_DEPENDENCY_CHIRHO
        ),
        "78\n"
    );
}

/// D1: GHC 9.14.1 refuses it, GHC-59692. sha256 5069fbab0f6add25879e22005f6a0b05ed1fbfb2ebefbd98086f5a58c42ffdef.
const QUALIFIED_DUPLICATE_GND_INSTANCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
import Prelude
import qualified Prelude as P
newtype AgeChirho = AgeChirho Int deriving newtype (P.Show)
instance Show AgeChirho where
  show _ = "written"
main :: IO ()
main = print (AgeChirho 7)
"#;

#[test]
fn gnd_d1_qualified_duplicate_gnd_instance_chirho() {
    assert_refused_chirho(
        "QualifiedDuplicateGndInstanceChirho.hs",
        QUALIFIED_DUPLICATE_GND_INSTANCE_CHIRHO,
        &["E0209", "Duplicate instance declarations"],
    );
}
