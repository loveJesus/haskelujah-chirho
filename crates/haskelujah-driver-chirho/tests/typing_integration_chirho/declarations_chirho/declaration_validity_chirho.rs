// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 683 declaration-validity controls.
//!
//! The negatives run the whole frontend and must fail with E0209 for GHC's
//! reason. The corpus positives are should_compile files that check clean
//! today and must keep doing so. The direct validator tests read the AST that
//! lowering really produces, so a hold cannot pass merely because an earlier
//! phase stopped.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{FileIdChirho, SourceMapChirho};
use haskelujah_typing_chirho::check_module_declaration_validity_chirho;

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

// tcfail038 (GHC-10498): each method is bound twice, apart.
const CONFLICT_SOURCE_CHIRHO: &str = r#"module ConflictChirho where
data NumberChirho = OneChirho | TwoChirho
instance Eq NumberChirho where
  leftChirho == rightChirho = True
  leftChirho /= rightChirho = False
  leftChirho == rightChirho = False
  leftChirho /= rightChirho = True
"#;

// MissingDefaultMethodBinding (GHC-59794).
const MISSING_DEFAULT_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DefaultSignatures #-}
module DefaultChirho where
class SizedChirho a where
  sizeChirho :: a -> Int
  default sizeChirho :: Show a => a -> Int
"#;

// A newtype error and an unknown type: naming reports first.
const ORDER_SOURCE_CHIRHO: &str = r#"module OrderChirho where
newtype BothChirho = BothChirho Int Int
valueChirho :: NoSuchTypeOrderChirho
valueChirho = undefined
"#;

// DatatypeContexts writes the context on the head; the CST eats the head up to
// `=`, so the constructor keeps its one field (#30825).
const DATATYPE_CONTEXTS_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DatatypeContexts #-}
module ContextsChirho where
newtype Eq a => SameChirho a = SameChirho a
newtype (Show a, Eq a) => ListedChirho a = ListedChirho [a]
newtype () => BarChirho = BarChirho Int
"#;

// No constructor context: vanilla, reordered forall, rank-N field, and the
// GADT record form of T18023.
const GADT_NEWTYPES_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE GADTs, RankNTypes #-}
module GadtChirho where
newtype PlainChirho a where
  MkPlainChirho :: a -> PlainChirho a
newtype SwappedChirho a b where
  MkSwappedChirho :: forall b a. Int -> SwappedChirho a b
newtype RankChirho where
  MkRankChirho :: (forall a. Show a => a -> String) -> RankChirho
newtype RecordChirho a b where
  MkRecordChirho :: forall b a. { unRecordChirho :: Either a b } -> RecordChirho a b
"#;

// Prefix and infix equations of one method kept together, and an instance
// signature beside its equations.
const EQUATIONS_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE InstanceSigs #-}
module EquationsChirho where
data ShapeChirho = CircleChirho | SquareChirho
instance Eq ShapeChirho where
  (==) CircleChirho CircleChirho = True
  SquareChirho == SquareChirho = True
  _ == _ = False
instance Show ShapeChirho where
  show :: ShapeChirho -> String
  show CircleChirho = "circle"
  show SquareChirho = "square"
  showsPrec _ shapeChirho restChirho = show shapeChirho ++ restChirho
"#;

// A binding before its default signature, and an operator's default signature,
// which lowering does not extract.
const DEFAULT_ORDER_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DefaultSignatures #-}
module DefaultsChirho where
class ZeroChirho a where
  zeroChirho :: a
  zeroChirho = undefined
  default zeroChirho :: Num a => a
class JoinChirho a where
  (<+>) :: a -> a -> a
  default (<+>) :: Semigroup a => a -> a -> a
  leftChirho <+> rightChirho = leftChirho <> rightChirho
"#;

// #30829's boundary: a default signature, then an associated type or data
// declaration, then the default binding, which must stay attached.
const ASSOCIATED_BETWEEN_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DefaultSignatures, TypeFamilies #-}
module AssociatedChirho where
class SizedChirho a where
  sizeChirho :: a -> Int
  default sizeChirho :: Show a => a -> Int
  type UnitChirho a
  sizeChirho valueChirho = length (show valueChirho)
class CountedChirho a where
  countChirho :: a -> Int
  default countChirho :: Enum a => a -> Int
  data TagChirho a
  countChirho valueChirho = fromEnum valueChirho
"#;

// Row 685's counterexample (#30830): a fixity between a default signature and its
// binding. GHC accepts it; the walk used to end at the fixity and drop the binding.
const FIXITY_BETWEEN_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DefaultSignatures #-}
module FixityBetweenChirho where
class SizedChirho a where
  sizeChirho :: a -> Int
  default sizeChirho :: Show a => a -> Int
  infixl 6 <+>
  (<+>) :: a -> a -> a
  sizeChirho valueChirho = length (show valueChirho)
"#;

// A fixity for the class's own operator, a function and an instance written after
// the class stay top-level siblings.
const SIBLINGS_SOURCE_CHIRHO: &str = r#"module SiblingsChirho where
class JoinChirho a where
  (<+>) :: a -> a -> a
  emptyChirho :: a

-- | Joins to the left.
infixl 6 <+>

-- | A helper.
helperChirho :: Int
helperChirho = 1

instance JoinChirho Bool where
  leftChirho <+> _ = leftChirho
  emptyChirho = False
"#;

// The default body after the fixity runs: Bool inherits it, Int writes its own.
// GHC 9.14.1 prints 4, 42, 5 and 6.
const DEFAULT_BODIES_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE DefaultSignatures #-}
module Main where
class SizedChirho a where
  sizeChirho :: a -> Int
  default sizeChirho :: Show a => a -> Int
  infixl 6 <+>
  (<+>) :: a -> a -> a
  sizeChirho valueChirho = length (show valueChirho)
instance SizedChirho Bool where
  leftChirho <+> _ = leftChirho
instance SizedChirho Int where
  sizeChirho _ = 42
  leftChirho <+> rightChirho = leftChirho + rightChirho
main :: IO ()
main = do
  print (sizeChirho True)
  print (sizeChirho (7 :: Int))
  print (sizeChirho (False <+> True))
  print ((1 :: Int) <+> 2 <+> 3)
"#;

/// GHC accepts the program, so the checker must too.
fn checks_like_ghc_chirho(file_name_chirho: &str, source_chirho: &str) {
    if let Err(error_chirho) = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        panic!("{file_name_chirho}: GHC accepts this program; we rejected it: {error_chirho}");
    }
}

/// The module exactly as lowering writes it, before any later phase runs.
fn lowered_chirho(source_chirho: &str) -> ModuleChirho {
    let green_chirho =
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
    lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
}

/// Each Haskell 98 newtype constructor as lowered: its name and field count.
fn newtype_constructors_chirho(module_chirho: &ModuleChirho) -> Vec<(String, usize)> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho:
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    },
                ..
            } => Some((name_chirho.text_chirho().to_string(), fields_chirho.len())),
            DeclChirho::NewtypeDeclChirho {
                constructor_chirho:
                    ConDeclChirho::RecordChirho {
                        name_chirho,
                        fields_chirho,
                        ..
                    },
                ..
            } => Some((
                name_chirho.text_chirho().to_string(),
                fields_chirho
                    .iter()
                    .map(|field_chirho| field_chirho.names_chirho.len())
                    .sum::<usize>(),
            )),
            _ => None,
        })
        .collect()
}

/// Each class method as lowered: its name, and whether a default binding and a
/// default signature are attached.
fn class_method_defaults_chirho(module_chirho: &ModuleChirho) -> Vec<(String, bool, bool)> {
    module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho { methods_chirho, .. } => Some(methods_chirho),
            _ => None,
        })
        .flatten()
        .map(|method_chirho| {
            (
                method_chirho.name_chirho.text_chirho().to_string(),
                method_chirho.default_chirho.is_some(),
                method_chirho.default_sig_chirho.is_some(),
            )
        })
        .collect()
}

/// The validator finds nothing in the module as lowered.
fn validator_is_silent_chirho(label_chirho: &str, module_chirho: &ModuleChirho) {
    let errors_chirho = check_module_declaration_validity_chirho(module_chirho);
    assert!(
        errors_chirho.is_empty(),
        "{label_chirho}: {errors_chirho:?}"
    );
}

#[test]
fn a_newtype_constructor_with_two_fields_is_rejected_chirho() {
    // T23308's `newtype B1 = B1 C1 C1` (GHC-23517).
    rejected_like_ghc_chirho(
        "module TwoFieldsChirho where\ndata PartChirho = PartChirho Int\nnewtype BothChirho = BothChirho PartChirho PartChirho\n",
        "TwoFieldsChirho.hs",
        &["E0209", "exactly one field", "`BothChirho` has two"],
    );
}

#[test]
fn a_newtype_constructor_with_no_field_is_rejected_chirho() {
    // T15523's `newtype Duration = Foo` (GHC-23517).
    rejected_like_ghc_chirho(
        "module NoFieldChirho where\nnewtype DurationChirho = FooChirho\n",
        "NoFieldChirho.hs",
        &["E0209", "exactly one field", "`FooChirho` has none"],
    );
}

#[test]
fn a_newtype_record_with_two_fields_is_rejected_chirho() {
    rejected_like_ghc_chirho(
        "module TwoNamesChirho where\nnewtype PairChirho = PairChirho { leftChirho, rightChirho :: Int }\n",
        "TwoNamesChirho.hs",
        &["E0209", "`PairChirho` has two"],
    );
}

#[test]
fn a_gadt_newtype_constructor_context_is_rejected_chirho() {
    // T15796's `MkN :: Show a => a -> N a` (GHC-17440), with and without a forall.
    for (file_chirho, signature_chirho) in [
        ("ContextChirho.hs", "Show a => a -> ShownChirho a"),
        (
            "ForallContextChirho.hs",
            "forall a. Show a => a -> ShownChirho a",
        ),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE GADTs #-}}\nmodule ContextChirho where\nnewtype ShownChirho a where\n  MkShownChirho :: {signature_chirho}\n"
        );
        rejected_like_ghc_chirho(
            &source_chirho,
            file_chirho,
            &["E0209", "must not have a context", "MkShownChirho"],
        );
    }
}

#[test]
fn conflicting_instance_bindings_are_rejected_chirho() {
    rejected_like_ghc_chirho(
        CONFLICT_SOURCE_CHIRHO,
        "ConflictChirho.hs",
        &[
            "E0209",
            "Conflicting definitions for `==`",
            "Conflicting definitions for `/=`",
        ],
    );
}

#[test]
fn a_default_signature_without_a_binding_is_rejected_chirho() {
    rejected_like_ghc_chirho(
        MISSING_DEFAULT_SOURCE_CHIRHO,
        "DefaultChirho.hs",
        &[
            "E0209",
            "generic-default signature without a binding `sizeChirho`",
        ],
    );
}

#[test]
fn name_resolution_errors_are_still_reported_first_chirho() {
    // The rules run before deriving but report after naming, as GHC's renamer
    // errors come before its validity checks.
    let message_chirho = match haskelujah_driver::typecheck_source_chirho(
        ORDER_SOURCE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "OrderChirho.hs",
    ) {
        Ok(_) => String::new(),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("NoSuchTypeOrderChirho"),
        "naming must reject the unknown type: {message_chirho}"
    );
    assert!(
        !message_chirho.contains("E0209"),
        "naming errors come first: {message_chirho}"
    );
}

#[test]
fn should_compile_newtypes_keep_checking_chirho() {
    // Rank-N, record and GADT-syntax newtypes that check clean before row 683.
    for (file_chirho, source_chirho) in [
        (
            "tc233.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc233.hs"
            ),
        ),
        (
            "T12133.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T12133.hs"
            ),
        ),
        (
            "LocalGivenEqs.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/LocalGivenEqs.hs"
            ),
        ),
        (
            "Vta1.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/Vta1.hs"),
        ),
        (
            "tc225.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/tc225.hs"
            ),
        ),
        (
            "T14154.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T14154.hs"
            ),
        ),
        (
            "T16141.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T16141.hs"
            ),
        ),
        (
            "T18023.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T18023.hs"
            ),
        ),
        (
            "T18891.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T18891.hs"
            ),
        ),
    ] {
        checks_like_ghc_chirho(file_chirho, source_chirho);
    }
}

#[test]
fn should_compile_default_signatures_keep_checking_chirho() {
    // Every default signature here has its binding, some with no arguments.
    for (file_chirho, source_chirho) in [
        (
            "DeepSubsumption05.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/DeepSubsumption05.hs"
            ),
        ),
        (
            "ExplicitSpecificity4.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/ExplicitSpecificity4.hs"
            ),
        ),
        (
            "T11608.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T11608.hs"
            ),
        ),
        (
            "T13142.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T13142.hs"
            ),
        ),
        (
            "T16832.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T16832.hs"
            ),
        ),
        (
            "T17723a.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T17723a.hs"
            ),
        ),
        (
            "T26457.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T26457.hs"
            ),
        ),
    ] {
        checks_like_ghc_chirho(file_chirho, source_chirho);
    }
}

#[test]
fn derived_instances_are_not_judged_as_written_chirho() {
    // syn-perf derives Show and Typeable on two dozen newtypes. Deriving runs after
    // the rules, so no generated instance body is judged as if written.
    checks_like_ghc_chirho(
        "syn-perf.hs",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/syn-perf.hs"),
    );
}

#[test]
fn datatype_context_newtypes_lower_to_one_field_chirho() {
    // The one-field precondition is asserted on the lowered AST itself. The head of
    // a nonempty context is misnamed, a separate pre-existing lowering issue.
    let module_chirho = lowered_chirho(DATATYPE_CONTEXTS_SOURCE_CHIRHO);
    assert_eq!(
        newtype_constructors_chirho(&module_chirho),
        [
            ("SameChirho".to_string(), 1),
            ("ListedChirho".to_string(), 1),
            ("BarChirho".to_string(), 1),
        ]
    );
    validator_is_silent_chirho("datatype contexts", &module_chirho);
}

#[test]
fn an_unreadable_constructor_name_is_not_judged_chirho() {
    // `(:|)` is a valid prefix constructor that lowering cannot name yet.
    let module_chirho =
        lowered_chirho("module OperatorChirho where\nnewtype ConsChirho = (:|) Int\n");
    for (name_chirho, field_count_chirho) in newtype_constructors_chirho(&module_chirho) {
        assert!(
            name_chirho.is_empty() || field_count_chirho == 1,
            "a named constructor carries its one field: {name_chirho} {field_count_chirho}"
        );
    }
    validator_is_silent_chirho("prefix operator constructor", &module_chirho);
}

#[test]
fn gadt_newtypes_without_a_constructor_context_are_not_judged_chirho() {
    validator_is_silent_chirho(
        "GADT newtypes",
        &lowered_chirho(GADT_NEWTYPES_SOURCE_CHIRHO),
    );
}

#[test]
fn signatures_and_adjacent_equations_are_not_conflicts_chirho() {
    validator_is_silent_chirho(
        "instance equations",
        &lowered_chirho(EQUATIONS_SOURCE_CHIRHO),
    );
}

#[test]
fn default_bindings_attach_in_any_order_chirho() {
    validator_is_silent_chirho(
        "default signatures",
        &lowered_chirho(DEFAULT_ORDER_SOURCE_CHIRHO),
    );
}

#[test]
fn a_default_binding_after_an_associated_declaration_stays_attached_chirho() {
    let module_chirho = lowered_chirho(ASSOCIATED_BETWEEN_SOURCE_CHIRHO);
    let defaults_chirho = class_method_defaults_chirho(&module_chirho);
    for wanted_chirho in ["sizeChirho", "countChirho"] {
        assert!(
            defaults_chirho.contains(&(wanted_chirho.to_string(), true, true)),
            "{wanted_chirho} must keep its default binding and signature: {defaults_chirho:?}"
        );
    }
    validator_is_silent_chirho("associated declaration between", &module_chirho);
    checks_like_ghc_chirho("AssociatedChirho.hs", ASSOCIATED_BETWEEN_SOURCE_CHIRHO);
}

#[test]
fn a_default_binding_after_a_class_fixity_stays_attached_chirho() {
    let module_chirho = lowered_chirho(FIXITY_BETWEEN_SOURCE_CHIRHO);
    let defaults_chirho = class_method_defaults_chirho(&module_chirho);
    assert!(
        defaults_chirho.contains(&("sizeChirho".to_string(), true, true)),
        "sizeChirho must keep its default binding and signature: {defaults_chirho:?}"
    );
    validator_is_silent_chirho("fixity between", &module_chirho);
    checks_like_ghc_chirho("FixityBetweenChirho.hs", FIXITY_BETWEEN_SOURCE_CHIRHO);
}

#[test]
fn class_fixities_and_their_top_level_siblings_keep_checking_chirho() {
    checks_like_ghc_chirho("SiblingsChirho.hs", SIBLINGS_SOURCE_CHIRHO);
    checks_like_ghc_chirho(
        "T10816.hs",
        include_str!("../../../../../ghc-tests-chirho/rename-chirho/should_compile/T10816.hs"),
    );
}

#[test]
fn a_default_body_after_a_class_fixity_runs_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "DefaultBodiesChirho.hs",
        DEFAULT_BODIES_SOURCE_CHIRHO,
        "4\n42\n5\n6\n",
    );
}
