// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real frontend controls for the ordinary kind/type elaboration handoff.

use crate::{FrontendResultChirho, run_frontend_chirho};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_typing_chirho::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
use std::collections::HashMap;

fn frontend_chirho(source_chirho: &str) -> Result<FrontendResultChirho, String> {
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
    let result_chirho = frontend_chirho("{-# LANGUAGE KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata WrapChirho (f :: Type -> Type) a = WrapChirho (f a)\nidentityChirho :: WrapChirho Maybe Int -> WrapChirho Maybe Int\nidentityChirho x = x\nmain = pure ()\n").expect("kinded ordinary declaration checks");
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
    let error_chirho = frontend_chirho("{-# LANGUAGE KindSignatures #-}\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule Main where\nimport Data.Kind (Type)\ndata WrapChirho (f :: Type -> Type) = WrapChirho (f Int)\nbadChirho :: WrapChirho Int\nbadChirho = undefined\nmain = pure ()\n").err().expect("a wrong constructor kind cannot defer");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn typed_builtin_class_telescopes_accept_nat_and_polykinded_typeable_chirho() {
    let result_chirho = frontend_chirho("{-# LANGUAGE DataKinds, FlexibleContexts, KindSignatures, PolyKinds #-}\nmodule Main where\nimport GHC.TypeLits (KnownNat)\nimport Data.Typeable (Typeable)\nimport Data.Proxy (Proxy(..))\nknownChirho :: KnownNat 5 => ()\nknownChirho = ()\npolyChirho :: Typeable Maybe => ()\npolyChirho = ()\nmain = pure ()\n").expect("typed builtin class kinds check");
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
    let error_chirho = frontend_chirho("{-# LANGUAGE DataKinds, FlexibleContexts #-}\nmodule Main where\nimport GHC.TypeLits (KnownNat)\nbadChirho :: KnownNat Bool => ()\nbadChirho = ()\nmain = pure ()\n").err().expect("KnownNat cannot accept an ordinary type");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn declaration_sccs_share_headers_and_generalize_before_dependants_chirho() {
    frontend_chirho("{-# LANGUAGE PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ndata FirstChirho a = FirstChirho (SecondChirho a)\ndata SecondChirho a = SecondChirho (FirstChirho a)\ndata PhantomChirho (a :: k) = PhantomChirho\naChirho :: PhantomChirho Int\naChirho = PhantomChirho\nbChirho :: PhantomChirho Maybe\nbChirho = PhantomChirho\nmain = pure ()\n").expect("SCC and per-use generalized telescope");
}

#[test]
fn independent_signature_and_pattern_scopes_restore_chirho() {
    frontend_chirho("{-# LANGUAGE ScopedTypeVariables #-}\nmodule Main where\nfChirho :: forall a. a -> a\nfChirho (x :: a) = x\ngChirho :: forall a. a -> a\ngChirho (x :: a) = x\nmain = pure ()\n").expect("independent lexical type-variable scopes");
}

#[test]
fn scoped_signature_binder_is_the_patterns_canonical_variable_chirho() {
    use haskelujah_ast_chirho::pat_chirho::PatChirho;
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    let result_chirho = frontend_chirho("{-# LANGUAGE ScopedTypeVariables #-}\nmodule Main where\nfChirho :: forall a. a -> a\nfChirho (x :: a) = x\nmain = pure ()\n").expect("scoped signature handoff");
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
    frontend_chirho("{-# LANGUAGE GADTs, TypeOperators #-}\nmodule Main where\nfChirho :: (a ~ Int) => a -> Int\nfChirho x = x\nmain = pure ()\n").expect("homogeneous equality kind checks");
    let error_chirho = frontend_chirho("{-# LANGUAGE GADTs, TypeOperators, PolyKinds #-}\nmodule Main where\nbadChirho :: (Int ~ Maybe) => ()\nbadChirho = ()\nmain = pure ()\n").err().expect("equality operands must have the same kind");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn a_standalone_polymorphic_kind_cannot_be_specialized_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE StandaloneKindSignatures, ExplicitForAll, PolyKinds, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype TChirho :: forall k. k -> Type\ndata TChirho (a :: Type) = MkTChirho a\nmain = pure ()\n").err().expect("a universal kind cannot specialize to Type");
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn promoted_result_retains_the_parents_hidden_kind_prefix_chirho() {
    frontend_chirho("{-# LANGUAGE DataKinds, KindSignatures, PolyKinds #-}\nmodule Main where\nimport Data.Proxy (Proxy)\ndata PhantomChirho (a :: k) = MkChirho\npChirho :: Proxy ('MkChirho :: PhantomChirho Maybe)\npChirho = undefined\nmain = pure ()\n").expect("promoted constructor and parent use the same complete telescope");
}

#[test]
fn transparent_kind_synonyms_use_the_checked_common_catalogue_chirho() {
    frontend_chirho("{-# LANGUAGE KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\ntype KindChirho = Type\ndata BoxChirho (a :: KindChirho) = BoxChirho a\nmain = pure ()\n").expect("kind synonym is transparent in the same solver");
}

#[test]
fn associated_families_and_class_method_kinds_are_orchestrated_chirho() {
    frontend_chirho("{-# LANGUAGE TypeFamilies, KindSignatures #-}\nmodule Main where\nimport Data.Kind (Type)\nclass CChirho (f :: Type -> Type) where\n  type FChirho f a :: Type\n  methodChirho :: f Int -> f Int\nmain = pure ()\n").expect("associated family and class method headers check");
}

#[test]
fn transparent_synonym_cycles_are_refused_without_rejecting_nominal_recursion_chirho() {
    let error_chirho = frontend_chirho("module Main where\ntype FirstChirho = SecondChirho\ntype SecondChirho = FirstChirho\nmain = pure ()\n").err().expect("a transparent synonym cycle is not finite");
    assert!(
        error_chirho.contains("recursive type synonym"),
        "{error_chirho}"
    );
    frontend_chirho("module Main where\ndata FirstChirho = FirstChirho SecondChirho\ntype SecondChirho = FirstChirho\nmain = pure ()\n").expect("a nominal data boundary breaks synonym expansion");
}

#[test]
fn constraint_aliases_keep_the_checked_hidden_prefix_and_producer_chirho() {
    use haskelujah_ast_chirho::name_chirho::{CompilerClassChirho, PredicateClassIdentityChirho};
    let result_chirho = frontend_chirho("{-# LANGUAGE ConstraintKinds, PolyKinds, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\ntype ReflectionChirho a = Typeable a\nprobeChirho :: ReflectionChirho Maybe => ()\nprobeChirho = ()\nmain = pure ()\n").expect("checked constraint synonym converts");
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
    let result_chirho = frontend_chirho("{-# LANGUAGE PolyKinds, ExplicitForAll, KindSignatures, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\nprobeChirho :: forall k (a :: k). Typeable a => ()\nprobeChirho = ()\nmain = pure ()\n").expect("context-only signature checks");
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
    frontend_chirho("{-# LANGUAGE ConstraintKinds, GADTs, TypeOperators #-}\nmodule Main where\ntype SameChirho a b = (a ~ b)\nidentityChirho :: SameChirho a Int => a -> Int\nidentityChirho x = x\nmain = pure ()\n").expect("equality alias supplies the same signature equality");
}

#[test]
fn a_source_namesake_never_acquires_a_compiler_class_identity_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        ClassIdentityUnknownChirho, PredicateClassProofChirho,
    };
    let result_chirho = frontend_chirho("{-# LANGUAGE FlexibleContexts #-}\nmodule Main where\nclass Typeable a\nprobeChirho :: Typeable Int => ()\nprobeChirho = ()\nmain = pure ()\n").expect("source class context remains representable");
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
fn a_constraint_tuple_alias_preserves_every_predicate_telescope_chirho() {
    let result_chirho = frontend_chirho("{-# LANGUAGE ConstraintKinds, PolyKinds, FlexibleContexts #-}\nmodule Main where\nimport Data.Typeable (Typeable)\ntype ContextChirho a = (Typeable a, Eq Int)\nprobeChirho :: ContextChirho Maybe => ()\nprobeChirho = ()\nmain = pure ()\n").expect("constraint tuple alias checks");
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
