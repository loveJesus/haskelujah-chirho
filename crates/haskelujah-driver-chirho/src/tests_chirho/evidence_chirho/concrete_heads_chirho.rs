// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A concrete head the compiling module declares proves `>>=`, `>>` and `fmap` (row 699, B1
//! and B2). The checker's proof, stamped with its use's origin, names the ground application
//! its predicate resolved to. That application's head is one of this module's data or newtype
//! declarations, and exactly one context-free instance of the class matches it. Every other
//! record for these names proves nothing. An identity-less record never reaches them by span or
//! by position, and a module whose own class declares `fmap` keeps that method's join.
//! workflow: monadic-dispatch-chirho
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho,
};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::infer_chirho::{InferResultChirho, MethodOccurrenceRecordChirho};
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

use super::occurrence_join_chirho::{infer_result_chirho, record_chirho};
use super::occurrence_producers_chirho::lower_chirho;
use crate::evidence_join_chirho::ModuleDeclarationsChirho;
use crate::join_occurrence_evidence_chirho;

/// A module declaring both concrete heads below: one by a newtype, one by data.
const DECLARING_MODULE_CHIRHO: &str = "module Main where\nnewtype StateChirho s a = StateChirho (s -> (a, s))\ndata ReaderChirho r a = ReaderChirho (r -> a)\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// The same program shape, declaring neither head.
const BARE_MODULE_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// A module whose own class declares `fmap`, beside a head it declares.
const OWN_FMAP_MODULE_CHIRHO: &str = "module Main where\nimport Prelude hiding (fmap)\nclass MapsChirho f where\n  fmap :: (a -> b) -> f a -> f b\nnewtype BoxChirho a = BoxChirho a\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// One record for one occurrence, and everything the join reads about it.
struct ProofCaseChirho {
    source_chirho: &'static str,
    name_chirho: &'static str,
    class_chirho: &'static str,
    key_chirho: &'static str,
    applied_chirho: Option<TyChirho>,
    instances_chirho: Vec<InstDeclChirho>,
    /// Whether the record carries the use's origin, and the occurrence that same provenance.
    stamped_chirho: bool,
    /// A scheme the environment binds the name to, as a constrained function's would be.
    entry_chirho: Option<SchemeChirho>,
}

impl ProofCaseChirho {
    /// `>>` used at `StateChirho [Int]` in the declaring module, with its one context-free
    /// instance, stamped with its origin.
    fn then_chirho() -> Self {
        Self {
            source_chirho: DECLARING_MODULE_CHIRHO,
            name_chirho: ">>",
            class_chirho: "Monad",
            key_chirho: "StateChirho",
            applied_chirho: Some(applied_chirho("StateChirho")),
            instances_chirho: vec![instance_chirho("Monad", "StateChirho", Vec::new())],
            stamped_chirho: true,
            entry_chirho: None,
        }
    }

    /// The proof the case's one occurrence takes. A stamped record joins its occurrence by
    /// provenance. An unstamped one is offered to an occurrence with neither provenance nor a
    /// span, the population the positional path serves.
    fn joined_chirho(self) -> Option<(String, String)> {
        let mut supply_chirho = OriginSupplyChirho::new_chirho();
        let origin_chirho = supply_chirho.fresh_chirho();
        let mut infer_chirho = infer_result_chirho(vec![MethodOccurrenceRecordChirho {
            applied_ty_chirho: self.applied_chirho,
            class_name_chirho: self.class_chirho.to_string(),
            origin_chirho: self.stamped_chirho.then_some(origin_chirho),
            ..record_chirho(
                self.name_chirho,
                0,
                self.key_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
        }]);
        declare_seeded_classes_chirho(&mut infer_chirho);
        for instance_chirho in self.instances_chirho {
            infer_chirho
                .class_env_chirho
                .instances_chirho
                .entry(instance_chirho.class_name_chirho.clone())
                .or_default()
                .push(instance_chirho);
        }
        if let Some(entry_chirho) = self.entry_chirho {
            infer_chirho
                .env_chirho
                .bind_chirho(self.name_chirho.to_string(), entry_chirho);
        }
        let occurrence_chirho = CoreIdChirho(30);
        let provenance_chirho = if self.stamped_chirho {
            HashMap::from([(
                occurrence_chirho,
                ProvenanceChirho::new_chirho(origin_chirho, OccurrenceRoleChirho::Reference),
            )])
        } else {
            HashMap::new()
        };
        join_occurrence_evidence_chirho(
            &infer_chirho,
            &HashMap::from([(
                occurrence_chirho,
                (self.name_chirho.to_string(), CoreIdChirho(1)),
            )]),
            &HashMap::new(),
            &HashMap::new(),
            &provenance_chirho,
            &ModuleDeclarationsChirho::of_module_chirho(&lower_chirho(self.source_chirho)),
        )
        .evidence_chirho
        .remove(&occurrence_chirho)
    }
}

/// The seeded classes' shapes the join reads: Functor declares `fmap`, Monad `>>=` and `>>`.
fn declare_seeded_classes_chirho(infer_chirho: &mut InferResultChirho) {
    for (class_chirho, methods_chirho) in
        [("Functor", &["fmap"][..]), ("Monad", &[">>=", ">>"][..])]
    {
        infer_chirho.class_env_chirho.classes_chirho.insert(
            class_chirho.to_string(),
            ClassDeclChirho {
                name_chirho: class_chirho.to_string(),
                supers_chirho: vec![],
                super_templates_chirho: Vec::new(),
                var_chirho: TyVarChirho(0),
                methods_chirho: methods_chirho
                    .iter()
                    .map(|method_chirho| {
                        (
                            method_chirho.to_string(),
                            SchemeChirho::mono_chirho(TyChirho::unit_chirho()),
                        )
                    })
                    .collect(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
    }
}

/// `head [Int]`, a ground application.
fn applied_chirho(head_chirho: &str) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(head_chirho.to_string())),
        Box::new(TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))),
    )
}

/// `instance context => class (head v)`.
fn instance_chirho(
    class_chirho: &str,
    head_chirho: &str,
    context_chirho: Vec<PredChirho>,
) -> InstDeclChirho {
    InstDeclChirho {
        class_name_chirho: class_chirho.to_string(),
        head_ty_chirho: TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(head_chirho.to_string())),
            Box::new(TyChirho::VarChirho(TyVarChirho(1))),
        ),
        extra_head_tys_chirho: vec![],
        context_chirho,
    }
}

#[test]
fn bind_then_and_fmap_take_their_proof_at_a_declared_concrete_head_chirho() {
    for name_chirho in [">>=", ">>"] {
        assert_eq!(
            ProofCaseChirho {
                name_chirho,
                ..ProofCaseChirho::then_chirho()
            }
            .joined_chirho(),
            Some(("Monad".to_string(), "StateChirho".to_string())),
            "{name_chirho}"
        );
    }
    // A data declaration ties a head as a newtype does.
    assert_eq!(
        ProofCaseChirho {
            name_chirho: "fmap",
            class_chirho: "Functor",
            key_chirho: "ReaderChirho",
            applied_chirho: Some(applied_chirho("ReaderChirho")),
            instances_chirho: vec![instance_chirho("Functor", "ReaderChirho", Vec::new())],
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        Some(("Functor".to_string(), "ReaderChirho".to_string()))
    );
    // One instance registered twice is one instance.
    let registered_chirho = instance_chirho("Monad", "StateChirho", Vec::new());
    assert_eq!(
        ProofCaseChirho {
            instances_chirho: vec![registered_chirho.clone(), registered_chirho],
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        Some(("Monad".to_string(), "StateChirho".to_string()))
    );
}

#[test]
fn no_other_record_proves_a_concrete_head_chirho() {
    let more_specific_chirho = InstDeclChirho {
        head_ty_chirho: TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("StateChirho".to_string())),
            Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                TyVarChirho(2),
            )))),
        ),
        ..instance_chirho("Monad", "StateChirho", Vec::new())
    };
    let refused_chirho = [
        // The checker recorded no ground application.
        (
            "no applied type",
            ProofCaseChirho {
                applied_chirho: None,
                ..ProofCaseChirho::then_chirho()
            },
        ),
        // A head this module does not declare: a builtin one, or the right spelling elsewhere.
        (
            "builtin head",
            ProofCaseChirho {
                key_chirho: "Maybe",
                applied_chirho: Some(applied_chirho("Maybe")),
                instances_chirho: vec![instance_chirho("Monad", "Maybe", Vec::new())],
                ..ProofCaseChirho::then_chirho()
            },
        ),
        (
            "undeclared head",
            ProofCaseChirho {
                source_chirho: BARE_MODULE_CHIRHO,
                ..ProofCaseChirho::then_chirho()
            },
        ),
        // A qualified homonym of a declared head; no qualifier is removed.
        (
            "qualified homonym",
            ProofCaseChirho {
                key_chirho: "M.StateChirho",
                applied_chirho: Some(applied_chirho("M.StateChirho")),
                instances_chirho: vec![instance_chirho("Monad", "M.StateChirho", Vec::new())],
                ..ProofCaseChirho::then_chirho()
            },
        ),
        // A key that is not the applied head.
        (
            "key beside the head",
            ProofCaseChirho {
                key_chirho: "IO",
                ..ProofCaseChirho::then_chirho()
            },
        ),
        // No instance, a contextual one, or two distinct ones.
        (
            "unmatched",
            ProofCaseChirho {
                instances_chirho: Vec::new(),
                ..ProofCaseChirho::then_chirho()
            },
        ),
        (
            "contextual",
            ProofCaseChirho {
                instances_chirho: vec![instance_chirho(
                    "Monad",
                    "StateChirho",
                    vec![PredChirho::new_chirho(
                        "Show",
                        TyChirho::VarChirho(TyVarChirho(1)),
                    )],
                )],
                ..ProofCaseChirho::then_chirho()
            },
        ),
        (
            "ambiguous",
            ProofCaseChirho {
                instances_chirho: vec![
                    instance_chirho("Monad", "StateChirho", Vec::new()),
                    more_specific_chirho,
                ],
                ..ProofCaseChirho::then_chirho()
            },
        ),
        // A record with no origin is no use's own proof, and no span or position joins it.
        (
            "identity-less",
            ProofCaseChirho {
                stamped_chirho: false,
                ..ProofCaseChirho::then_chirho()
            },
        ),
    ];
    for (reason_chirho, case_chirho) in refused_chirho {
        assert_eq!(case_chirho.joined_chirho(), None, "{reason_chirho}");
    }
}

#[test]
fn a_constrained_function_s_predicate_proves_no_method_at_a_concrete_head_chirho() {
    // `>>` bound as a function constrained by Show: the record is authoritative through that
    // scheme, yet Show declares no `>>`, so its predicate is no method's proof. The same record
    // under Monad, which declares `>>`, is admitted.
    let show_constrained_chirho = SchemeChirho {
        vars_chirho: vec![TyVarChirho(0)],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Show".to_string(),
            ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
    };
    assert_eq!(
        ProofCaseChirho {
            class_chirho: "Show",
            instances_chirho: vec![instance_chirho("Show", "StateChirho", Vec::new())],
            entry_chirho: Some(show_constrained_chirho.clone()),
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        None
    );
    assert_eq!(
        ProofCaseChirho {
            entry_chirho: Some(show_constrained_chirho),
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        Some(("Monad".to_string(), "StateChirho".to_string()))
    );
}

#[test]
fn the_indexed_own_path_is_unchanged_and_never_serves_fmap_chirho() {
    let own_key_chirho = "$own:0";
    assert_eq!(
        ProofCaseChirho {
            name_chirho: ">>=",
            key_chirho: own_key_chirho,
            applied_chirho: None,
            instances_chirho: Vec::new(),
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        Some(("Monad".to_string(), own_key_chirho.to_string()))
    );
    assert_eq!(
        ProofCaseChirho {
            name_chirho: "fmap",
            class_chirho: "Functor",
            key_chirho: own_key_chirho,
            applied_chirho: None,
            instances_chirho: Vec::new(),
            ..ProofCaseChirho::then_chirho()
        }
        .joined_chirho(),
        None
    );
}

#[test]
fn identity_less_records_reach_only_the_names_they_reached_before_chirho() {
    // The positional path still pairs an identity-less `show` with its identity-less record, so
    // the control reaches that path. A seeded `fmap` is refused there, while a module's own
    // class's `fmap` keeps the join it had.
    let identity_less_chirho = |source_chirho, name_chirho, class_chirho| ProofCaseChirho {
        source_chirho,
        name_chirho,
        class_chirho,
        key_chirho: "BoxChirho",
        applied_chirho: None,
        instances_chirho: Vec::new(),
        stamped_chirho: false,
        entry_chirho: None,
    };
    assert_eq!(
        identity_less_chirho(BARE_MODULE_CHIRHO, "show", "Show").joined_chirho(),
        Some(("Show".to_string(), "BoxChirho".to_string()))
    );
    assert_eq!(
        identity_less_chirho(BARE_MODULE_CHIRHO, "fmap", "Functor").joined_chirho(),
        None
    );
    assert_eq!(
        identity_less_chirho(OWN_FMAP_MODULE_CHIRHO, "fmap", "Functor").joined_chirho(),
        Some(("Functor".to_string(), "BoxChirho".to_string()))
    );
}
