// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A concrete head the compiling module declares proves `>>=`, `>>`, `fmap` and `pure` (row 699,
//! B1, B2 and #32342). The checker's proof, stamped with its use's origin, names the ground
//! application its predicate resolved to. That application's head is one of this module's data or
//! newtype declarations, and exactly one context-free instance of the class matches it. Every
//! other record for these names proves nothing. An identity-less record never reaches them by span
//! or by position. A module whose own class declares `fmap` keeps that method's join, and no own
//! class gives `pure` one. W1's own `pure` and `>>` are checked through the real front end.
//! workflow: monadic-dispatch-chirho
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho,
};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::{
    ClassDeclChirho, ClassDeclPartsChirho, InstDeclChirho, InstDeclPartsChirho, PredChirho,
    PredPartsChirho,
};
use haskelujah_typing_chirho::infer_chirho::{InferResultChirho, MethodOccurrenceRecordChirho};
use haskelujah_typing_chirho::ty_chirho::{
    SchemeChirho, SchemePredChirho, SchemePredPartsChirho, TyChirho, TyVarChirho,
};

use super::occurrence_join_chirho::{infer_result_chirho, record_chirho, span_chirho};
use super::occurrence_producers_chirho::lower_chirho;
use super::occurrence_records_chirho::{frontend_chirho, occurrence_texts_chirho};
use crate::evidence_join_chirho::{ModuleDeclarationsChirho, admits_concrete_head_proof_chirho};
use crate::join_occurrence_evidence_chirho;

/// A module declaring both concrete heads below: one by a newtype, one by data.
const DECLARING_MODULE_CHIRHO: &str = "module Main where\nnewtype StateChirho s a = StateChirho (s -> (a, s))\ndata ReaderChirho r a = ReaderChirho (r -> a)\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// The same program shape, declaring neither head.
const BARE_MODULE_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// A module whose own class declares `fmap`, beside a head it declares.
const OWN_FMAP_MODULE_CHIRHO: &str = "module Main where\nimport Prelude hiding (fmap)\nclass MapsChirho f where\n  fmap :: (a -> b) -> f a -> f b\nnewtype BoxChirho a = BoxChirho a\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// A module whose own class declares `pure`, beside a head it declares.
const OWN_PURE_MODULE_CHIRHO: &str = "module Main where\nimport Prelude hiding (pure)\nclass PointedChirho f where\n  pure :: a -> f a\nnewtype BoxChirho a = BoxChirho a\nmain :: IO ()\nmain = print (1 :: Int)\n";

/// W1's exact bytes, the program its every-engine test runs.
const W1_SOURCE_CHIRHO: &str = include_str!(
    "../../../tests/typing_integration_chirho/declarations_chirho/monad_owned_dictionary_chirho/written_monads_chirho/state_omitted_then_chirho.hs"
);

/// One record for one occurrence, and everything the join reads about it.
#[derive(Clone)]
struct ProofCaseChirho {
    source_chirho: &'static str,
    name_chirho: &'static str,
    class_chirho: &'static str,
    key_chirho: &'static str,
    applied_chirho: Option<TyChirho>,
    capture_chirho: Option<PredChirho>,
    instances_chirho: Vec<InstDeclChirho>,
    /// Whether the record carries the use's origin, and the occurrence that same provenance.
    stamped_chirho: bool,
    /// Whether an unstamped record and its occurrence share one genuine span, the population the
    /// span join serves; otherwise both have none, the population the positional path serves.
    genuine_span_chirho: bool,
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
            capture_chirho: None,
            instances_chirho: vec![instance_chirho("Monad", "StateChirho", Vec::new())],
            stamped_chirho: true,
            genuine_span_chirho: false,
            entry_chirho: None,
        }
    }

    /// `pure` used at `StateChirho [Int]` in the declaring module, with Applicative's one
    /// context-free instance there, stamped with its origin.
    fn pure_chirho() -> Self {
        Self {
            name_chirho: "pure",
            class_chirho: "Applicative",
            instances_chirho: vec![instance_chirho("Applicative", "StateChirho", Vec::new())],
            ..Self::then_chirho()
        }
    }

    /// The proof the case's one occurrence takes. A stamped record joins its occurrence by
    /// provenance. An unstamped one is offered to an occurrence without provenance, through the
    /// span they share or, with none, by position.
    fn joined_chirho(self) -> Option<(String, String)> {
        let mut supply_chirho = OriginSupplyChirho::new_chirho();
        let origin_chirho = supply_chirho.fresh_chirho();
        let record_span_chirho = if self.genuine_span_chirho {
            span_chirho(100, 104)
        } else {
            SpanChirho::DUMMY_CHIRHO
        };
        let mut infer_chirho = infer_result_chirho(vec![MethodOccurrenceRecordChirho {
            applied_ty_chirho: self.applied_chirho,
            ground_pred_chirho: self.capture_chirho,
            class_name_chirho: self.class_chirho.to_string(),
            origin_chirho: self.stamped_chirho.then_some(origin_chirho),
            ..record_chirho(self.name_chirho, 0, self.key_chirho, record_span_chirho)
        }]);
        declare_seeded_classes_chirho(&mut infer_chirho);
        for instance_chirho in self.instances_chirho {
            infer_chirho
                .class_env_chirho
                .add_instance_chirho(instance_chirho);
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
        let occurrence_spans_chirho = if self.genuine_span_chirho {
            HashMap::from([(occurrence_chirho, record_span_chirho)])
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
            &occurrence_spans_chirho,
            &provenance_chirho,
            &ModuleDeclarationsChirho::of_module_chirho(&lower_chirho(self.source_chirho)),
        )
        .evidence_chirho
        .remove(&occurrence_chirho)
    }
}

/// The seeded classes' shapes the join reads: Functor declares `fmap`, Applicative `pure`, and
/// Monad `>>=` and `>>`.
fn declare_seeded_classes_chirho(infer_chirho: &mut InferResultChirho) {
    for (class_chirho, methods_chirho) in [
        ("Functor", &["fmap"][..]),
        ("Applicative", &["pure"][..]),
        ("Monad", &[">>=", ">>"][..]),
    ] {
        infer_chirho
            .class_env_chirho
            .add_class_chirho(ClassDeclChirho::legacy_chirho(ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
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
            }));
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
    InstDeclChirho::legacy_chirho(InstDeclPartsChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: class_chirho.to_string(),
        head_ty_chirho: TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(head_chirho.to_string())),
            Box::new(TyChirho::VarChirho(TyVarChirho(1))),
        ),
        extra_head_tys_chirho: vec![],
        context_chirho,
    })
}

#[test]
fn a_complete_capture_matches_all_arguments_and_a_conflicting_capture_never_falls_back_chirho() {
    let mut matching_instance_chirho = instance_chirho("Monad", "StateChirho", Vec::new());
    matching_instance_chirho.kind_tys_chirho = vec![TyChirho::ConChirho("Type".to_string())];
    matching_instance_chirho.extra_head_tys_chirho = vec![TyChirho::bool_chirho()];
    let capture_chirho = PredChirho::legacy_chirho(PredPartsChirho {
        kind_tys_chirho: matching_instance_chirho.kind_tys_chirho.clone(),
        class_name_chirho: "Monad".to_string(),
        ty_chirho: applied_chirho("StateChirho"),
        extra_tys_chirho: matching_instance_chirho.extra_head_tys_chirho.clone(),
    });
    let complete_chirho = ProofCaseChirho {
        instances_chirho: vec![matching_instance_chirho],
        capture_chirho: Some(capture_chirho.clone()),
        ..ProofCaseChirho::then_chirho()
    };
    assert_eq!(
        complete_chirho.joined_chirho(),
        Some(("Monad".to_string(), "StateChirho".to_string()))
    );
    let mut wrong_class_chirho = capture_chirho.clone();
    wrong_class_chirho.class_name_chirho = "Functor".to_string();
    let mut wrong_application_chirho = capture_chirho.clone();
    wrong_application_chirho.ty_chirho = applied_chirho("ReaderChirho");
    for capture_chirho in [wrong_class_chirho, wrong_application_chirho] {
        assert_eq!(
            ProofCaseChirho {
                capture_chirho: Some(capture_chirho),
                ..ProofCaseChirho::then_chirho()
            }
            .joined_chirho(),
            None,
            "a conflicting capture must not fall through to the legacy predicate"
        );
    }
    let mut wrong_prefix_chirho = capture_chirho.clone();
    wrong_prefix_chirho.kind_tys_chirho.clear();
    let mut wrong_extra_chirho = capture_chirho;
    wrong_extra_chirho.extra_tys_chirho[0] = TyChirho::int_chirho();
    for capture_chirho in [wrong_prefix_chirho, wrong_extra_chirho] {
        let mut matching_instance_chirho = instance_chirho("Monad", "StateChirho", Vec::new());
        matching_instance_chirho.kind_tys_chirho = vec![TyChirho::ConChirho("Type".to_string())];
        matching_instance_chirho.extra_head_tys_chirho = vec![TyChirho::bool_chirho()];
        assert_eq!(
            ProofCaseChirho {
                instances_chirho: vec![matching_instance_chirho],
                capture_chirho: Some(capture_chirho),
                ..ProofCaseChirho::then_chirho()
            }
            .joined_chirho(),
            None,
            "the complete predicate's hidden and extra arguments must reach matching"
        );
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
    let mut more_specific_chirho = instance_chirho("Monad", "StateChirho", Vec::new());
    more_specific_chirho.head_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("StateChirho".to_string())),
        Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
            TyVarChirho(2),
        )))),
    );
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

/// `Show a => a`, a constrained function's scheme, never a method's.
fn show_constrained_scheme_chirho() -> SchemeChirho {
    SchemeChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        preds_chirho: vec![SchemePredChirho::legacy_chirho(SchemePredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Show".to_string(),
            ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
            extra_tys_chirho: vec![],
        })],
        ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
    }
}

#[test]
fn a_constrained_function_s_predicate_proves_no_method_at_a_concrete_head_chirho() {
    // `>>` and `pure` bound as functions constrained by Show: each record is authoritative
    // through that scheme, yet Show declares neither, so its predicate is no method's proof. The
    // same record under the class that declares the method is admitted.
    for case_chirho in [
        ProofCaseChirho::then_chirho(),
        ProofCaseChirho::pure_chirho(),
    ] {
        let name_chirho = case_chirho.name_chirho;
        let declared_chirho = Some((
            case_chirho.class_chirho.to_string(),
            "StateChirho".to_string(),
        ));
        assert_eq!(
            ProofCaseChirho {
                class_chirho: "Show",
                instances_chirho: vec![instance_chirho("Show", "StateChirho", Vec::new())],
                entry_chirho: Some(show_constrained_scheme_chirho()),
                ..case_chirho.clone()
            }
            .joined_chirho(),
            None,
            "{name_chirho} under Show"
        );
        assert_eq!(
            ProofCaseChirho {
                entry_chirho: Some(show_constrained_scheme_chirho()),
                ..case_chirho
            }
            .joined_chirho(),
            declared_chirho,
            "{name_chirho} under its own class"
        );
    }
}

#[test]
fn pure_takes_its_proof_at_a_declared_concrete_head_only_chirho() {
    // #32342: Applicative's declared `pure` at `StateChirho [Int]`, by B1 and B2's own rule.
    assert_eq!(
        ProofCaseChirho::pure_chirho().joined_chirho(),
        Some(("Applicative".to_string(), "StateChirho".to_string()))
    );
    // An indexed own key, a builtin head, a head this module does not declare, a contextual
    // instance, and a record without origin prove nothing for it.
    let refused_chirho = [
        (
            "indexed own",
            ProofCaseChirho {
                key_chirho: "$own:0",
                applied_chirho: None,
                instances_chirho: Vec::new(),
                ..ProofCaseChirho::pure_chirho()
            },
        ),
        (
            "builtin head",
            ProofCaseChirho {
                key_chirho: "Maybe",
                applied_chirho: Some(applied_chirho("Maybe")),
                instances_chirho: vec![instance_chirho("Applicative", "Maybe", Vec::new())],
                ..ProofCaseChirho::pure_chirho()
            },
        ),
        (
            "undeclared head",
            ProofCaseChirho {
                source_chirho: BARE_MODULE_CHIRHO,
                ..ProofCaseChirho::pure_chirho()
            },
        ),
        (
            "contextual",
            ProofCaseChirho {
                instances_chirho: vec![instance_chirho(
                    "Applicative",
                    "StateChirho",
                    vec![PredChirho::new_chirho(
                        "Show",
                        TyChirho::VarChirho(TyVarChirho(1)),
                    )],
                )],
                ..ProofCaseChirho::pure_chirho()
            },
        ),
        (
            "identity-less",
            ProofCaseChirho {
                stamped_chirho: false,
                ..ProofCaseChirho::pure_chirho()
            },
        ),
    ];
    for (reason_chirho, case_chirho) in refused_chirho {
        assert_eq!(case_chirho.joined_chirho(), None, "pure: {reason_chirho}");
    }
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
    // The span join and the positional path still pair an identity-less `show` with its
    // identity-less record, so each control reaches its path. A seeded `fmap`, `>>`, and `pure`
    // (even beside a module's own class named `pure`) are refused on both. A module's own class's
    // `fmap` keeps the join it had.
    for genuine_span_chirho in [false, true] {
        let identity_less_chirho = |source_chirho, name_chirho, class_chirho| ProofCaseChirho {
            source_chirho,
            name_chirho,
            class_chirho,
            key_chirho: "BoxChirho",
            applied_chirho: None,
            instances_chirho: Vec::new(),
            stamped_chirho: false,
            genuine_span_chirho,
            entry_chirho: None,
        };
        let path_chirho = if genuine_span_chirho {
            "span"
        } else {
            "position"
        };
        for (source_chirho, name_chirho, class_chirho) in [
            (BARE_MODULE_CHIRHO, "show", "Show"),
            (OWN_FMAP_MODULE_CHIRHO, "fmap", "Functor"),
        ] {
            assert_eq!(
                identity_less_chirho(source_chirho, name_chirho, class_chirho).joined_chirho(),
                Some((class_chirho.to_string(), "BoxChirho".to_string())),
                "{name_chirho} by {path_chirho}"
            );
        }
        for (source_chirho, name_chirho, class_chirho) in [
            (BARE_MODULE_CHIRHO, "fmap", "Functor"),
            (BARE_MODULE_CHIRHO, ">>", "Monad"),
            (BARE_MODULE_CHIRHO, "pure", "Applicative"),
            (OWN_PURE_MODULE_CHIRHO, "pure", "Applicative"),
        ] {
            assert_eq!(
                identity_less_chirho(source_chirho, name_chirho, class_chirho).joined_chirho(),
                None,
                "{name_chirho} by {path_chirho}"
            );
        }
    }
}

#[test]
fn the_checker_proves_w1s_pure_and_then_at_their_declared_concrete_head_chirho() {
    // The real front end over W1's bytes, nothing built by hand. The `pure` of
    // `logChirho 5 >> pure 'z'` carries the checker's Applicative proof at `StateChirho [Int]`,
    // under the origin of an occurrence of `pure`. The concrete-head rule admits exactly that
    // record against W1's own class environment and declarations. So it does the three `>>` used
    // at that head, and none of main's IO statements.
    let mut result_chirho = frontend_chirho(W1_SOURCE_CHIRHO);
    let texts_chirho = occurrence_texts_chirho(&mut result_chirho);
    let declarations_chirho =
        ModuleDeclarationsChirho::of_module_chirho(&result_chirho.module_chirho);
    let class_env_chirho = &result_chirho.infer_result_chirho.class_env_chirho;
    let records_chirho = &result_chirho.infer_result_chirho.method_occurrences_chirho;
    let state_ints_chirho = applied_chirho("StateChirho");
    let admitted_chirho = |record_chirho: &MethodOccurrenceRecordChirho| {
        admits_concrete_head_proof_chirho(record_chirho, &declarations_chirho, class_env_chirho)
    };
    // W1 writes one `pure`: the one record whose origin is an occurrence of `pure` in W1 itself.
    let pure_records_chirho: Vec<_> = records_chirho
        .iter()
        .filter(|record_chirho| {
            record_chirho.name_chirho == "pure"
                && record_chirho.origin_chirho.is_some_and(|origin_chirho| {
                    texts_chirho.get(&origin_chirho).map(String::as_str) == Some("pure")
                })
        })
        .collect();
    let [pure_record_chirho] = pure_records_chirho.as_slice() else {
        panic!("one proof for W1's one `pure`: {records_chirho:?}");
    };
    let pure_record_chirho: &MethodOccurrenceRecordChirho = pure_record_chirho;
    assert_eq!(
        (
            pure_record_chirho.class_name_chirho.as_str(),
            pure_record_chirho.ty_key_chirho.as_str(),
            pure_record_chirho.applied_ty_chirho.as_ref(),
        ),
        ("Applicative", "StateChirho", Some(&state_ints_chirho)),
        "{pure_record_chirho:?}"
    );
    assert!(
        admitted_chirho(pure_record_chirho),
        "{pure_record_chirho:?}"
    );
    let then_records_chirho: Vec<_> = records_chirho
        .iter()
        .filter(|record_chirho| {
            record_chirho.name_chirho == ">>" && record_chirho.origin_chirho.is_some()
        })
        .collect();
    // Lines 44 and 46, and `sixSevenChirho`'s statement, at `StateChirho [Int]`; main's three IO
    // statements at `IO`.
    let at_state_chirho: Vec<_> = then_records_chirho
        .iter()
        .copied()
        .filter(|record_chirho| {
            record_chirho.applied_ty_chirho.as_ref() == Some(&state_ints_chirho)
        })
        .collect();
    assert_eq!(at_state_chirho.len(), 3, "{then_records_chirho:?}");
    assert!(
        at_state_chirho
            .iter()
            .all(|&record_chirho| admitted_chirho(record_chirho)),
        "{at_state_chirho:?}"
    );
    let at_io_chirho: Vec<_> = then_records_chirho
        .iter()
        .copied()
        .filter(|record_chirho| record_chirho.ty_key_chirho == "IO")
        .collect();
    assert_eq!(at_io_chirho.len(), 3, "{then_records_chirho:?}");
    assert!(
        at_io_chirho
            .iter()
            .all(|&record_chirho| !admitted_chirho(record_chirho)),
        "{at_io_chirho:?}"
    );
}
