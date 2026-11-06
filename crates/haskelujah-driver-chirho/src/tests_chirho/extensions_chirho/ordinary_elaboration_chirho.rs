// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real frontend controls for the ordinary kind/type elaboration handoff.

use crate::{FrontendResultChirho, run_frontend_chirho};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_typing_chirho::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
use std::collections::HashMap;

pub(super) fn frontend_chirho(source_chirho: &str) -> Result<FrontendResultChirho, String> {
    run_frontend_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
    )
    .map_err(|diagnostics_chirho| {
        diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    })
}

/// `frontend_chirho` for a GHC program: its `main` is the program's IO entry, as GHC checks
/// a Main module's (row 692). Library and value sources keep `frontend_chirho`.
pub(super) fn program_frontend_chirho(source_chirho: &str) -> Result<FrontendResultChirho, String> {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let types_chirho = HashMap::new();
    let type_synonyms_chirho = crate::ImportedTypeSynonymsChirho::new();
    let type_families_chirho = crate::seed_builtin_type_families_chirho();
    let class_env_chirho = haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    crate::run_frontend_with_options_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        crate::FrontendImportsChirho {
            ifaces_chirho: &ifaces_chirho,
            types_chirho: &types_chirho,
            type_synonyms_chirho: &type_synonyms_chirho,
            type_families_chirho: &type_families_chirho,
            class_env_chirho: &class_env_chirho,
        },
        &crate::FrontendOptionsChirho::for_program_chirho("main"),
    )
    .map_err(|diagnostics_chirho| {
        diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    })
}

fn declaration_chirho(
    result_chirho: &FrontendResultChirho,
    name_chirho: &str,
) -> TypeConstructorDeclarationChirho {
    result_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho: declared_chirho,
                ..
            } if declared_chirho.text_chirho() == name_chirho => {
                declared_chirho.type_constructor_chirho().cloned()
            }
            _ => None,
        })
        .expect("positive source declaration")
}

#[test]
fn ordinary_headers_bodies_and_the_inference_handoff_run_chirho() {
    let result_chirho = program_frontend_chirho("{-# LANGUAGE KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata WrapChirho (f :: Type -> Type) a = WrapChirho (f a)\nidentityChirho :: WrapChirho Maybe Int -> WrapChirho Maybe Int\nidentityChirho x = x\nmain = pure ()\n").expect("kinded ordinary declaration checks");
    let declaration_chirho = declaration_chirho(&result_chirho, "WrapChirho");
    let key_chirho = declaration_chirho.identity_chirho.key_chirho();
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .type_families_chirho
        .declaration_schemes_chirho()
        .get_chirho(&key_chirho)
        .expect("frontend publishes checked ordinary telescope");
    assert_eq!(scheme_chirho.visible_arity_chirho, 2);
    assert_eq!(
        scheme_chirho
            .binders_chirho
            .iter()
            .filter(|binder_chirho| binder_chirho.visibility_chirho
                == TelescopeVisibilityChirho::RequiredChirho)
            .count(),
        2
    );
    assert!(scheme_chirho.owner_chirho.is_none());
}

#[test]
fn ordinary_kind_mismatch_is_fatal_under_deferral_chirho() {
    let error_chirho = program_frontend_chirho("{-# LANGUAGE KindSignatures #-}\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule Main where\nimport Data.Kind (Type)\ndata WrapChirho (f :: Type -> Type) = WrapChirho (f Int)\nbadChirho :: WrapChirho Int\nbadChirho = undefined\nmain = pure ()\n").err().expect("a wrong constructor kind cannot defer");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn typed_builtin_class_telescopes_accept_nat_and_polykinded_typeable_chirho() {
    let result_chirho = program_frontend_chirho("{-# LANGUAGE DataKinds, FlexibleContexts, KindSignatures, PolyKinds #-}\nmodule Main where\nimport GHC.TypeLits (KnownNat)\nimport Data.Typeable (Typeable)\nimport Data.Proxy (Proxy(..))\nknownChirho :: KnownNat 5 => ()\nknownChirho = ()\npolyChirho :: Typeable Maybe => ()\npolyChirho = ()\nmain = pure ()\n").expect("typed builtin class kinds check");
    let schemes_chirho = result_chirho
        .infer_result_chirho
        .type_families_chirho
        .declaration_schemes_chirho();
    let known_chirho = schemes_chirho
        .get_chirho("%class:compiler:KnownNat")
        .expect("positive KnownNat producer");
    assert_eq!(
        known_chirho.binders_chirho[0].kind_chirho.to_string(),
        "Nat"
    );
    let typeable_chirho = schemes_chirho
        .get_chirho("%class:compiler:Typeable")
        .expect("positive Typeable producer");
    assert_eq!(typeable_chirho.hidden_arity_chirho(), 1);
    assert_eq!(typeable_chirho.visible_arity_chirho, 1);
}

#[test]
fn typed_builtin_class_telescopes_reject_wrong_nat_kind_chirho() {
    let error_chirho = program_frontend_chirho("{-# LANGUAGE DataKinds, FlexibleContexts #-}\nmodule Main where\nimport GHC.TypeLits (KnownNat)\nbadChirho :: KnownNat Bool => ()\nbadChirho = ()\nmain = pure ()\n").err().expect("KnownNat cannot accept an ordinary type");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn declaration_sccs_share_headers_and_generalize_before_dependants_chirho() {
    program_frontend_chirho("{-# LANGUAGE PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata FirstChirho a = FirstChirho (SecondChirho a)\ndata SecondChirho a = SecondChirho (FirstChirho a)\ndata PhantomChirho (a :: k) = PhantomChirho\naChirho :: PhantomChirho Int\naChirho = PhantomChirho\nbChirho :: PhantomChirho Maybe\nbChirho = PhantomChirho\nmain = pure ()\n").expect("SCC and per-use generalized telescope");
}

#[test]
fn independent_signature_and_pattern_scopes_restore_chirho() {
    program_frontend_chirho("{-# LANGUAGE ScopedTypeVariables #-}\nmodule Main where\nfChirho :: forall a. a -> a\nfChirho (x :: a) = x\ngChirho :: forall a. a -> a\ngChirho (x :: a) = x\nmain = pure ()\n").expect("independent lexical type-variable scopes");
}

#[test]
fn scoped_signature_binder_is_the_patterns_canonical_variable_chirho() {
    use haskelujah_ast_chirho::pat_chirho::PatChirho;
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    let result_chirho = program_frontend_chirho("{-# LANGUAGE ScopedTypeVariables #-}\nmodule Main where\nfChirho :: forall a. a -> a\nfChirho (x :: a) = x\nmain = pure ()\n").expect("scoped signature handoff");
    let binder_chirho = result_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::TypeSigChirho {
                ty_chirho: TypeChirho::ForallChirho { vars_chirho, .. },
                ..
            } => vars_chirho
                .first()
                .and_then(|variable_chirho| variable_chirho.type_variable_chirho())
                .cloned(),
            _ => None,
        })
        .expect("outer binder identity");
    let pattern_chirho = result_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                ..
            } if name_chirho.text_chirho() == "fChirho" => {
                let mut pattern_chirho = &matches_chirho[0].pats_chirho[0];
                while let PatChirho::ParenChirho { inner_chirho, .. } = pattern_chirho {
                    pattern_chirho = inner_chirho;
                }
                if let PatChirho::TypeAnnotChirho {
                    ty_chirho: TypeChirho::VarChirho(name_chirho),
                    ..
                } = pattern_chirho
                {
                    name_chirho.type_variable_chirho().cloned()
                } else {
                    None
                }
            }
            _ => None,
        })
        .expect("pattern annotation identity");
    assert_eq!(binder_chirho, pattern_chirho);
}

#[test]
fn equality_syntax_uses_the_same_kind_solver_chirho() {
    program_frontend_chirho("{-# LANGUAGE GADTs, TypeOperators #-}\nmodule Main where\nfChirho :: (a ~ Int) => a -> Int\nfChirho x = x\nmain = pure ()\n").expect("homogeneous equality kind checks");
    let error_chirho = program_frontend_chirho("{-# LANGUAGE GADTs, TypeOperators, PolyKinds #-}\nmodule Main where\nbadChirho :: (Int ~ Maybe) => ()\nbadChirho = ()\nmain = pure ()\n").err().expect("equality operands must have the same kind");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn a_standalone_polymorphic_kind_cannot_be_specialized_chirho() {
    let error_chirho = program_frontend_chirho("{-# LANGUAGE StandaloneKindSignatures, ExplicitForAll, PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype TChirho :: forall k. k -> Type\ndata TChirho (a :: Type) = MkTChirho a\nmain = pure ()\n").err().expect("a universal kind cannot specialize to Type");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn promoted_result_retains_the_parents_hidden_kind_prefix_chirho() {
    program_frontend_chirho("{-# LANGUAGE DataKinds, KindSignatures, PolyKinds #-}\nmodule Main where\nimport Data.Proxy (Proxy)\ndata PhantomChirho (a :: k) = MkChirho\npChirho :: Proxy ('MkChirho :: PhantomChirho Maybe)\npChirho = undefined\nmain = pure ()\n").expect("promoted constructor and parent use the same complete telescope");
}

#[test]
fn transparent_kind_synonyms_use_the_checked_common_catalogue_chirho() {
    program_frontend_chirho("{-# LANGUAGE KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype KindChirho = Type\ndata BoxChirho (a :: KindChirho) = BoxChirho a\nmain = pure ()\n").expect("kind synonym is transparent in the same solver");
}

#[test]
fn associated_families_and_class_method_kinds_are_orchestrated_chirho() {
    program_frontend_chirho("{-# LANGUAGE TypeFamilies, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\nclass CChirho (f :: Type -> Type) where\n  type FChirho f a :: Type\n  methodChirho :: f Int -> f Int\nmain = pure ()\n").expect("associated family and class method headers check");
}

#[test]
fn transparent_synonym_cycles_are_refused_without_rejecting_nominal_recursion_chirho() {
    let error_chirho = program_frontend_chirho("module Main where\ntype FirstChirho = SecondChirho\ntype SecondChirho = FirstChirho\nmain = pure ()\n").err().expect("a transparent synonym cycle is not finite");
    assert!(
        error_chirho.contains("recursive type synonym"),
        "{error_chirho}"
    );
    program_frontend_chirho("module Main where\ndata FirstChirho = FirstChirho SecondChirho\ntype SecondChirho = FirstChirho\nmain = pure ()\n").expect("a nominal data boundary breaks synonym expansion");
}

#[test]
fn constraint_aliases_keep_the_checked_hidden_prefix_and_producer_chirho() {
    use haskelujah_ast_chirho::name_chirho::{CompilerClassChirho, PredicateClassIdentityChirho};
    let result_chirho = program_frontend_chirho("{-# LANGUAGE ConstraintKinds, PolyKinds, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\ntype ReflectionChirho a = Typeable a\nprobeChirho :: ReflectionChirho Maybe => ()\nprobeChirho = ()\nmain = pure ()\n").expect("checked constraint synonym converts");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .expect("signature retained");
    let predicate_chirho = scheme_chirho
        .preds_chirho
        .iter()
        .find(|predicate_chirho| predicate_chirho.class_name_chirho == "Typeable")
        .expect("the alias expands its class predicate");
    assert_eq!(predicate_chirho.kind_tys_chirho.len(), 1);
    assert!(matches!(
        predicate_chirho.class_identity_chirho().identity_chirho(),
        Some(PredicateClassIdentityChirho::CompilerChirho(
            CompilerClassChirho::TypeableChirho
        ))
    ));
    assert_eq!(predicate_chirho.ty_chirho.to_string(), "Maybe");
}

#[test]
fn context_only_variables_and_hidden_kinds_are_quantified_chirho() {
    let result_chirho = program_frontend_chirho("{-# LANGUAGE PolyKinds, ExplicitForAll, KindSignatures, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\nprobeChirho :: forall k (a :: k). Typeable a => ()\nprobeChirho = ()\nmain = pure ()\n").expect("context-only signature checks");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .expect("signature retained");
    for predicate_chirho in &scheme_chirho.preds_chirho {
        for term_chirho in predicate_chirho
            .kind_tys_chirho
            .iter()
            .chain(std::iter::once(&predicate_chirho.ty_chirho))
            .chain(&predicate_chirho.extra_tys_chirho)
        {
            for variable_chirho in term_chirho.free_vars_chirho() {
                assert!(
                    scheme_chirho.vars_chirho.contains(&variable_chirho),
                    "context variable escaped its signature"
                );
            }
        }
    }
    assert_eq!(scheme_chirho.vars_chirho.len(), 2, "{scheme_chirho:#?}");
}

#[test]
fn equality_inside_a_constraint_alias_retains_its_obligation_chirho() {
    program_frontend_chirho("{-# LANGUAGE ConstraintKinds, GADTs, TypeOperators #-}\nmodule Main where\ntype SameChirho a b = (a ~ b)\nidentityChirho :: SameChirho a Int => a -> Int\nidentityChirho x = x\nmain = pure ()\n").expect("equality alias supplies the same signature equality");
}

#[test]
fn a_source_namesake_never_acquires_a_compiler_class_identity_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        ClassIdentityUnknownChirho, PredicateClassProofChirho,
    };
    let result_chirho = program_frontend_chirho("{-# LANGUAGE FlexibleContexts #-}\nmodule Main where\nclass Typeable a\nprobeChirho :: Typeable Int => ()\nprobeChirho = ()\nmain = pure ()\n").expect("source class context remains representable");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .expect("source context retained");
    let predicate_chirho = scheme_chirho
        .preds_chirho
        .iter()
        .find(|predicate_chirho| predicate_chirho.class_name_chirho == "Typeable")
        .expect("source predicate retained");
    assert_eq!(
        predicate_chirho.class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
    assert!(predicate_chirho.kind_tys_chirho.is_empty());
}

/// Diagnostic for the program-purpose regression (row 692, gpt_chirho #32611): the namesake
/// module with a signed `main`, as a value module and as a program, judged by the test
/// above's two assertions. With the unsigned program above, the first of the three to fail
/// names the handoff.
const SIGNED_NAMESAKE_SOURCE_CHIRHO: &str = "{-# LANGUAGE FlexibleContexts #-}\nmodule Main where\nclass Typeable a\nprobeChirho :: Typeable Int => ()\nprobeChirho = ()\nmain :: IO ()\nmain = pure ()\n";

fn assert_source_namesake_stays_unproven_chirho(result_chirho: FrontendResultChirho) {
    use haskelujah_ast_chirho::name_chirho::{
        ClassIdentityUnknownChirho, PredicateClassProofChirho,
    };
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .expect("source context retained");
    let predicate_chirho = scheme_chirho
        .preds_chirho
        .iter()
        .find(|predicate_chirho| predicate_chirho.class_name_chirho == "Typeable")
        .expect("source predicate retained");
    assert_eq!(
        predicate_chirho.class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho)
    );
    assert!(predicate_chirho.kind_tys_chirho.is_empty());
}

#[test]
fn a_source_namesake_stays_unproven_in_a_signed_value_module_chirho() {
    assert_source_namesake_stays_unproven_chirho(
        frontend_chirho(SIGNED_NAMESAKE_SOURCE_CHIRHO)
            .expect("source class context remains representable"),
    );
}

#[test]
fn a_source_namesake_stays_unproven_in_a_signed_program_chirho() {
    assert_source_namesake_stays_unproven_chirho(
        program_frontend_chirho(SIGNED_NAMESAKE_SOURCE_CHIRHO)
            .expect("source class context remains representable"),
    );
}

#[test]
fn a_constraint_tuple_alias_preserves_every_predicate_telescope_chirho() {
    let result_chirho = program_frontend_chirho("{-# LANGUAGE ConstraintKinds, PolyKinds, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\ntype ContextChirho a = (Typeable a, Eq Int)\nprobeChirho :: ContextChirho Maybe => ()\nprobeChirho = ()\nmain = pure ()\n").expect("constraint tuple alias checks");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("probeChirho")
        .expect("signature retained");
    assert_eq!(scheme_chirho.preds_chirho.len(), 2);
    assert_eq!(
        scheme_chirho
            .preds_chirho
            .iter()
            .find(|predicate_chirho| predicate_chirho.class_name_chirho == "Typeable")
            .expect("Typeable retained")
            .kind_tys_chirho
            .len(),
        1
    );
    assert!(scheme_chirho.preds_chirho.iter().any(|predicate_chirho| {
        predicate_chirho.class_name_chirho == "Eq" && predicate_chirho.kind_tys_chirho.is_empty()
    }));
}

#[test]
fn alias_datatype_scc_publication_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, LiberalTypeSynonyms #-}
module AliasDatatypeSccChirho where
data DChirho aChirho = MkDChirho (BChirho aChirho) (AChirho aChirho)
type BChirho aChirho = forall bChirho. bChirho -> DChirho aChirho
type AChirho aChirho = BChirho aChirho
"#,
    );
    result_chirho.expect("GHC-accepted publication oracle checks");
}

#[test]
fn header_nested_forall_publication_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}
module HeaderForallKindChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
data HChirho (fChirho :: forall kChirho. kChirho -> Type) = MkHChirho
type SChirho = HChirho Proxy
"#,
    );
    result_chirho.expect("GHC-accepted publication oracle checks");
}

#[test]
fn header_nested_forall_wrong_kind_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}
module HeaderForallWrongKindChirho where
import Data.Kind (Type)
data HChirho (fChirho :: forall kChirho. kChirho -> Type) = MkHChirho
type SChirho = HChirho Maybe
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("a monokind constructor cannot satisfy a universal kind");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn result_nested_forall_publication_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}
module ResultForallHeaderChirho where
import Data.Kind (Type)
type RChirho :: Type -> forall kChirho. kChirho -> Type
data RChirho aChirho bChirho = MkRChirho
"#,
    );
    result_chirho.expect("GHC-accepted publication oracle checks");
}

#[test]
fn gadt_alias_scc_replays_complete_telescope_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, LiberalTypeSynonyms #-}
module GadtAliasSccChirho where
data TChirho where
  MkChirho :: SChirho -> TChirho
type SChirho = forall aChirho. aChirho -> TChirho
"#,
    );
    result_chirho.expect("fresh GHC-accepted publication oracle checks");
}

#[test]
fn gadt_inline_named_kind_cannot_be_specialized_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, DataKinds, KindSignatures #-}
module GadtParentTwoKindsChirho where
data TChirho (aChirho :: kChirho) where
  MkBoolChirho :: TChirho 'True
  MkTypeChirho :: TChirho Int
booleanChirho :: TChirho 'True
booleanChirho = MkBoolChirho
ordinaryChirho :: TChirho Int
ordinaryChirho = MkTypeChirho
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC-rejected publication oracle refuses");
    assert!(
        error_chirho.contains("error[E0300]") || error_chirho.contains("error[E0206]"),
        "{error_chirho}"
    );
}

#[test]
fn gadt_standalone_universal_kind_instantiates_per_constructor_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, PolyKinds, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}
module GadtStandaloneTwoKindsChirho where
import Data.Kind (Type)
type TChirho :: forall kChirho. kChirho -> Type
data TChirho aChirho where
  MkBoolChirho :: TChirho 'True
  MkTypeChirho :: TChirho Int
booleanChirho :: TChirho 'True
booleanChirho = MkBoolChirho
ordinaryChirho :: TChirho Int
ordinaryChirho = MkTypeChirho
"#,
    );
    result_chirho.expect("fresh GHC-accepted publication oracle checks");
}

#[test]
fn universal_kind_cannot_be_inferred_by_poly_meta_assignment_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}
module InferredUniversalKindChirho where
import Data.Kind (Type)
data HChirho (fChirho :: forall kChirho. kChirho -> Type)
type SChirho fChirho = HChirho fChirho
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC-rejected publication oracle refuses");
    assert!(
        error_chirho.contains("error[E0300]") || error_chirho.contains("error[E0206]"),
        "{error_chirho}"
    );
}

#[test]
fn universal_kind_cannot_capture_a_monomorphic_kind_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}
module MonomorphicCapturedKindChirho where
import Data.Kind (Type)
data HChirho (fChirho :: forall kChirho. kChirho -> Type)
type SChirho (fChirho :: jChirho -> Type) = HChirho fChirho
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC-rejected publication oracle refuses");
    assert!(
        error_chirho.contains("error[E0300]") || error_chirho.contains("error[E0206]"),
        "{error_chirho}"
    );
}

#[test]
fn universal_kind_argument_does_not_erase_nested_licence_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}
module NestedImpredicativeKindChirho where
import Data.Kind (Type)
data HChirho (fChirho :: forall kChirho. kChirho -> Type)
data CChirho (aChirho :: Type) (bChirho :: kChirho)
type BadChirho = HChirho (CChirho (forall aChirho. aChirho -> aChirho))
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC-rejected publication oracle refuses");
    assert!(
        error_chirho.contains("error[E0300]") || error_chirho.contains("error[E0206]"),
        "{error_chirho}"
    );
}

#[test]
fn partially_applied_type_cannot_regain_kind_universality_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures, ImpredicativeTypes #-}
module NestedImpredicativeLicensedChirho where
import Data.Kind (Type)
data HChirho (fChirho :: forall kChirho. kChirho -> Type)
data CChirho (aChirho :: Type) (bChirho :: kChirho)
type GoodChirho = HChirho (CChirho (forall aChirho. aChirho -> aChirho))
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC-rejected publication oracle refuses");
    assert!(
        error_chirho.contains("error[E0300]") || error_chirho.contains("error[E0206]"),
        "{error_chirho}"
    );
}

#[test]
fn universal_kind_argument_depth_is_bounded_before_deferral_chirho() {
    for depth_chirho in [8usize, 136] {
        let argument_chirho = format!(
            "{}Proxy{}",
            "(".repeat(depth_chirho),
            ")".repeat(depth_chirho)
        );
        let source_chirho = format!(
            "{{-# LANGUAGE RankNTypes, PolyKinds, KindSignatures #-}}\n{{-# OPTIONS_GHC -fdefer-type-errors #-}}\nmodule Main where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ndata HChirho (fChirho :: forall kChirho. kChirho -> Type)\ntype SChirho = HChirho {argument_chirho}\n"
        );
        let result_chirho = frontend_chirho(&source_chirho);
        if depth_chirho == 8 {
            result_chirho.expect("ordinary parentheses preserve a valid universal kind");
        } else {
            let error_chirho = result_chirho
                .err()
                .expect("the bounded universal traversal refuses excessive nesting");
            assert!(
                error_chirho.contains("error[E0300]")
                    && error_chirho.contains("depth bound exhausted"),
                "{error_chirho}"
            );
        }
    }
}

#[test]
fn inline_complete_header_generalizes_when_cusks_are_enabled_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, GADTs, PolyKinds, DataKinds, KindSignatures #-}
module InlineCusksEnabledChirho where
data TChirho (aChirho :: kChirho) where
  MkBoolChirho :: TChirho 'True
  MkTypeChirho :: TChirho Int
booleanChirho :: TChirho 'True
booleanChirho = MkBoolChirho
ordinaryChirho :: TChirho Int
ordinaryChirho = MkTypeChirho
"#,
    );
    result_chirho.expect("fresh GHC positive preserves the producer contract");
}

#[test]
fn inline_complete_header_stays_monomorphic_with_no_cusks_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, NoCUSKs, GADTs, PolyKinds, DataKinds, KindSignatures #-}
module InlineCusksDisabledChirho where
data TChirho (aChirho :: kChirho) where
  MkBoolChirho :: TChirho 'True
  MkTypeChirho :: TChirho Int
booleanChirho :: TChirho 'True
booleanChirho = MkBoolChirho
ordinaryChirho :: TChirho Int
ordinaryChirho = MkTypeChirho
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC negative preserves the producer boundary");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn ghc2021_does_not_enable_legacy_complete_headers_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021 #-}
{-# LANGUAGE PolyKinds, GADTs #-}

module ResultModeGhc2021Chirho where

import Data.Kind

data Test (a :: x) (b :: x) :: x -> Type
  where K :: Test Int Bool Double
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC negative preserves the producer boundary");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn explicit_cusks_override_the_language_mode_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021, CUSKs #-}
{-# LANGUAGE PolyKinds, GADTs #-}

module ResultCusksEnabledChirho where

import Data.Kind

data Test (a :: x) (b :: x) :: x -> Type
  where K :: Test Int Bool Double
"#,
    );
    result_chirho.expect("fresh GHC positive preserves the producer contract");
}

#[test]
fn explicit_no_cusks_override_haskell2010_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, NoCUSKs #-}
{-# LANGUAGE PolyKinds, GADTs #-}

module ResultCusksDisabledChirho where

import Data.Kind

data Test (a :: x) (b :: x) :: x -> Type
  where K :: Test Int Bool Double
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC negative preserves the producer boundary");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn ghc_types_universe_uses_its_typed_producer_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, ConstraintKinds, FlexibleContexts #-}
module GhcTypesUniverseChirho where
import qualified GHC.Types as KChirho
data Type = MkLocalChirho
data BoxChirho (aChirho :: KChirho.Type)
type AliasChirho = BoxChirho Int
"#,
    );
    result_chirho.expect("fresh GHC positive preserves the producer contract");
}

#[test]
fn source_type_homonym_never_becomes_a_universe_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, ConstraintKinds, FlexibleContexts #-}
module GhcTypesSourceHomonymChirho where
import qualified GHC.Types as KChirho
data Type = MkLocalChirho
data BoxChirho (aChirho :: Type)
type AliasChirho = BoxChirho Int
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC negative preserves the producer boundary");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn ghc_exts_constraint_uses_its_typed_producer_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, ConstraintKinds, FlexibleContexts #-}
module GhcExtsConstraintUniverseChirho where
import qualified GHC.Exts as KChirho
data Constraint = MkLocalChirho
data BoxChirho (aChirho :: KChirho.Constraint)
type AliasChirho = BoxChirho (Eq Int)
"#,
    );
    result_chirho.expect("fresh GHC positive preserves the producer contract");
}

#[test]
fn source_constraint_homonym_never_becomes_a_universe_chirho() {
    let result_chirho = frontend_chirho(
        r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, ConstraintKinds, FlexibleContexts #-}
module GhcExtsSourceConstraintChirho where
import qualified GHC.Exts as KChirho
data Constraint = MkLocalChirho
data BoxChirho (aChirho :: Constraint)
type AliasChirho = BoxChirho (Eq Int)
"#,
    );
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC negative preserves the producer boundary");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn lexical_quantifier_visibility_survives_nested_and_inferred_bindings_chirho() {
    for (label_chirho, source_chirho, accepted_chirho) in [
        (
            "nested specified positive",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/NestedInferredPositiveChirho.hs"
            ),
            true,
        ),
        (
            "nested inferred selection",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/NestedInferredWrongChirho.hs"
            ),
            false,
        ),
        (
            "inferred local binding",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/FreshInferredSelectionWrongChirho.hs"
            ),
            false,
        ),
        (
            "invisible flag compatibility",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/InvisibleCompatibilityPositiveChirho.hs"
            ),
            true,
        ),
    ] {
        let result_chirho = frontend_chirho(source_chirho);
        assert_eq!(
            result_chirho.is_ok(),
            accepted_chirho,
            "{label_chirho}: {}",
            result_chirho.err().unwrap_or_default()
        );
    }
}

#[test]
fn checked_type_application_kind_survives_an_unused_result_binder_chirho() {
    for (label_chirho, source_chirho, accepted_chirho) in [
        (
            "higher kind positive",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/UnusedHigherKindPositiveChirho.hs"
            ),
            true,
        ),
        (
            "higher kind wrong",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/UnusedHigherKindWrongChirho.hs"
            ),
            false,
        ),
        (
            "dependent binder positive",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/DependentBinderPositiveChirho.hs"
            ),
            true,
        ),
        (
            "dependent binder wrong",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/DependentBinderWrongChirho.hs"
            ),
            false,
        ),
    ] {
        let result_chirho = frontend_chirho(source_chirho);
        assert_eq!(
            result_chirho.is_ok(),
            accepted_chirho,
            "{label_chirho}: {}",
            result_chirho.err().unwrap_or_default()
        );
    }
}

#[test]
fn unused_quantifier_kinds_do_not_overconstrain_subsumption_chirho() {
    // The original negative prediction failed: fresh GHC accepts this vacuous
    // quantifier. Keep its unchanged source and actual positive contract.
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/UnusedBinderWrongChirho.hs"))
        .expect("GHC accepts unused quantified arguments with different kinds");
}

#[test]
fn returned_sigma_publishes_its_original_visibility_chirho() {
    let result_chirho = frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/quantifiers-chirho/cold-chirho/QuantifierOriginChirho.hs")).expect("actual GHC-valid supplier");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("exposeChirho")
        .expect("supplier scheme");
    let haskelujah_typing_chirho::TyChirho::FunChirho(_, body_chirho, _) = &scheme_chirho.ty_chirho
    else {
        panic!("supplier keeps arrow: {scheme_chirho:#?}")
    };
    let haskelujah_typing_chirho::TyChirho::ForallChirho { vars_chirho, .. } = body_chirho.as_ref()
    else {
        panic!("supplier keeps returned sigma: {scheme_chirho:#?}")
    };
    assert_eq!(
        vars_chirho
            .records_chirho()
            .iter()
            .map(|record_chirho| record_chirho.specificity_chirho)
            .collect::<Vec<_>>(),
        vec![
            Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho),
            Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho)
        ]
    );
}

#[test]
fn representation_normalization_liftedtypeequalitychirho_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/LiftedTypeEqualityChirho.hs"
    ))
    .expect("fresh GHC-positive representation equality checks");
}

#[test]
fn representation_normalization_liftedconstraintequalitychirho_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/LiftedConstraintEqualityChirho.hs"
    ))
    .expect("fresh GHC-positive representation equality checks");
}

#[test]
fn representation_normalization_unliftedtypedifferentchirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/UnliftedTypeDifferentChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative representation distinction rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn representation_normalization_sourcetypeopaquechirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/SourceTypeOpaqueChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative representation distinction rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn representation_normalization_liftedvtachirho_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/LiftedVtaChirho.hs"
    ))
    .expect("fresh GHC-positive representation equality checks");
}

#[test]
fn representation_normalization_unliftedvtawrongchirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/UnliftedVtaWrongChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative representation distinction rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn representation_normalization_rigidrepequalitywrongchirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/RigidRepEqualityWrongChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative representation distinction rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn representation_normalization_compatibleliftedinstanceschirho_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/CompatibleLiftedInstancesChirho.hs"
    ))
    .expect("fresh GHC-positive family normalization checks");
}

#[test]
fn representation_normalization_conflictingliftedinstanceschirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/ConflictingLiftedInstancesChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative family normalization rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}

#[test]
fn representation_normalization_closedliftedreductionchirho_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/ClosedLiftedReductionChirho.hs"
    ))
    .expect("fresh GHC-positive family normalization checks");
}

#[test]
fn representation_normalization_closedliftedwrongresultchirho_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/representation_chirho/ClosedLiftedWrongResultChirho.hs"
    ))
    .err()
    .expect("fresh GHC-negative family normalization rejects");
    assert!(error_chirho.contains("error[E"), "{error_chirho}");
}
