// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{NextErrorKindChirho, check_source_chirho, compile_source_chirho, eval_binding_chirho};
use haskelujah_core_chirho::expr_chirho::{AltConChirho, CoreExprChirho, CoreIdChirho};
use haskelujah_runtime_chirho::{ValueChirho, show_number_chirho::ShowNumberOperationChirho};

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/show_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}

/// Match the whole spine in source, then observe characters. No host string renderer
/// or stateless formatter can satisfy the source dictionary/default/append path.
fn text_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: &str) {
    let names_chirho: Vec<_> = (0..expected_chirho.chars().count())
        .map(|index_chirho| format!("character{index_chirho}Chirho"))
        .collect();
    assert!(!names_chirho.is_empty() && names_chirho.len() <= 64);
    let body_chirho = if names_chirho.len() == 1 {
        names_chirho[0].clone()
    } else {
        format!("({})", names_chirho.join(","))
    };
    let source_chirho = format!(
        "{source_chirho}\nprobeTextChirho ({}:[]) = {body_chirho}\nprobeResultChirho = probeTextChirho {entry_chirho}\n",
        names_chirho.join(":")
    );
    let (value_chirho, mut machine_chirho) =
        eval_binding_chirho(&source_chirho, "probeResultChirho", 500_000)
            .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
    let values_chirho = if names_chirho.len() == 1 {
        vec![value_chirho]
    } else {
        let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho else {
            panic!("source text probe must return its tuple");
        };
        machine_chirho
            .heap_chirho
            .read_chirho(machine_chirho.heap_chirho.follow_ind_chirho(address_chirho))
            .payload_chirho
            .clone()
    };
    assert_eq!(values_chirho.len(), names_chirho.len());
    for (value_chirho, expected_chirho) in values_chirho.into_iter().zip(expected_chirho.chars()) {
        assert_eq!(
            machine_chirho.force_to_prim_chirho(value_chirho),
            ValueChirho::CharChirho(expected_chirho),
            "{entry_chirho}"
        );
    }
    assert!(!machine_chirho.integer_budget_chirho.exhausted_chirho());
}

#[test]
fn source_show_defaults_integer_and_preserves_signed_precedence_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "ShowNumbersChirho");
    for (entry_chirho, expected_chirho) in [
        ("hugeChirho", "340282366920938463463374607431768211457"),
        ("resultChirho", "5"),
        ("sumChirho", "3"),
        ("negativeChirho", "-5"),
        ("intPrecChirho", "(-5)"),
        ("integerSixChirho", "-5"),
        ("integerSevenChirho", "(-5)"),
        ("bigChirho", "(-12345678901234567890123)"),
        ("positiveChirho", "18446744073709551616"),
        ("minimumChirho", "-9223372036854775808"),
        ("minimumPrecChirho", "(-9223372036854775808)"),
        ("signedTailChirho", "12!"),
        ("integerTailChirho", "12!"),
    ] {
        text_chirho(&source_chirho, entry_chirho, expected_chirho);
    }
}

#[test]
fn formatting_tails_and_partial_applications_stay_lazy_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "LazyShowChirho");
    for (entry_chirho, expected_chirho) in [
        ("intChirho", "12"),
        ("intParenChirho", "(-5)"),
        ("integerChirho", "11"),
        ("integerParenChirho", "(-11"),
        ("stringChirho", "ab"),
    ] {
        text_chirho(&source_chirho, entry_chirho, expected_chirho);
    }
    for entry_chirho in ["partialIntChirho", "partialIntegerChirho"] {
        eval_binding_chirho(&source_chirho, entry_chirho, 100_000).unwrap();
    }
}

#[test]
fn source_list_default_local_where_and_show_support_execute_chirho() {
    for (file_chirho, entries_chirho) in [
        (
            "ShowListMethodChirho",
            vec![
                ("resultChirho", "[1,2,3]"),
                ("integerChirho", "[1,2,3]!"),
                ("emptyChirho", "[]!"),
            ],
        ),
        (
            "SourceShowDefaultsChirho",
            vec![("resultChirho", "box!"), ("listChirho", "[box,box]!")],
        ),
        (
            "ShowSupportChirho",
            vec![
                ("resultChirho", "(x)"),
                ("unwrappedChirho", "x!"),
                ("characterChirho", "q!"),
                ("spaceChirho", " !"),
            ],
        ),
        ("ExplicitShowHelperChirho", vec![("resultChirho", "(-5)!")]),
        ("HomeShowSignedIntChirho", vec![("resultChirho", "home")]),
    ] {
        let source_chirho = fixture_chirho("positive-chirho", file_chirho);
        for (entry_chirho, expected_chirho) in entries_chirho {
            text_chirho(&source_chirho, entry_chirho, expected_chirho);
        }
    }
}

fn demanded_failure_chirho(source_chirho: &str, entry_chirho: &str) {
    let error_chirho = match eval_binding_chirho(source_chirho, entry_chirho, 200_000) {
        Err(error_chirho) => error_chirho,
        Ok(_) => panic!("demanded source case must fail"),
    };
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::ExecutionChirho
    );
    assert!(
        error_chirho
            .message_chirho
            .contains("no matching alternative"),
        "{error_chirho}"
    );
}

struct DemandChirho {
    parameter_chirho: CoreIdChirho,
    evaluated_chirho: CoreIdChirho,
    whnf_only_chirho: bool,
}

/// Error tags identify the common empty-case fallback, not the failed argument.
/// Core proves source demand order and operand identity; execution below proves
/// the independent failures. No allocated Core ID or constructor tag is pinned.
#[derive(Default)]
struct DemandContractChirho {
    parameters_chirho: Vec<CoreIdChirho>,
    demands_chirho: Vec<DemandChirho>,
    seen_chirho: std::collections::HashSet<ShowNumberOperationChirho>,
}

impl DemandContractChirho {
    fn visit_chirho(&mut self, expression_chirho: &CoreExprChirho) {
        match expression_chirho {
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                if let Some(operation_chirho) =
                    ShowNumberOperationChirho::from_primitive_chirho(name_chirho)
                {
                    assert_eq!(self.parameters_chirho.len(), 3);
                    let order_chirho = match operation_chirho {
                        ShowNumberOperationChirho::IntChirho => [0, 1],
                        ShowNumberOperationChirho::IntegerChirho => [1, 0],
                    };
                    assert_eq!(
                        self.demands_chirho
                            .iter()
                            .map(|demand_chirho| demand_chirho.parameter_chirho)
                            .collect::<Vec<_>>(),
                        order_chirho.map(|index_chirho| self.parameters_chirho[index_chirho])
                    );
                    assert!(
                        self.demands_chirho
                            .iter()
                            .all(|demand_chirho| demand_chirho.whnf_only_chirho)
                    );
                    let expected_chirho: Vec<_> = self.parameters_chirho[..2]
                        .iter()
                        .map(|parameter_chirho| {
                            let demand_chirho = self
                                .demands_chirho
                                .iter()
                                .find(|demand_chirho| {
                                    demand_chirho.parameter_chirho == *parameter_chirho
                                })
                                .unwrap();
                            CoreExprChirho::VarChirho(demand_chirho.evaluated_chirho)
                        })
                        .collect();
                    assert_eq!(
                        args_chirho, &expected_chirho,
                        "only evaluated p,n, positionally; never the tail"
                    );
                    assert!(
                        self.seen_chirho.insert(operation_chirho),
                        "one body per exact provider"
                    );
                }
                for argument_chirho in args_chirho {
                    self.visit_chirho(argument_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for argument_chirho in args_chirho {
                    self.visit_chirho(argument_chirho);
                }
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                self.visit_chirho(fun_chirho);
                self.visit_chirho(arg_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                self.parameters_chirho.push(binder_chirho.id_chirho);
                self.visit_chirho(body_chirho);
                self.parameters_chirho.pop();
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => self.visit_chirho(body_chirho),
            CoreExprChirho::TyAppChirho { expr_chirho, .. } => self.visit_chirho(expr_chirho),
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_, rhs_chirho) in binds_chirho {
                    self.visit_chirho(rhs_chirho);
                }
                self.visit_chirho(body_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                self.visit_chirho(scrutinee_chirho);
                let demanded_chirho = match scrutinee_chirho.as_ref() {
                    CoreExprChirho::VarChirho(id_chirho)
                        if self.parameters_chirho.contains(id_chirho) =>
                    {
                        Some(*id_chirho)
                    }
                    _ => None,
                };
                let whnf_only_chirho = alts_chirho.len() == 1
                    && matches!(alts_chirho[0].con_chirho, AltConChirho::DefaultChirho);
                for alternative_chirho in alts_chirho {
                    if let Some(parameter_chirho) = demanded_chirho {
                        self.demands_chirho.push(DemandChirho {
                            parameter_chirho,
                            evaluated_chirho: bind_chirho.id_chirho,
                            whnf_only_chirho,
                        });
                    }
                    self.visit_chirho(&alternative_chirho.rhs_chirho);
                    if demanded_chirho.is_some() {
                        self.demands_chirho.pop();
                    }
                }
            }
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
        }
    }
}

#[test]
fn int_and_integer_keep_their_distinct_source_forcing_orders_chirho() {
    let source_chirho = fixture_chirho("failure-chirho", "ShowDemandChirho");
    let core_chirho = compile_source_chirho(&source_chirho).unwrap();
    let mut contract_chirho = DemandContractChirho::default();
    for binding_chirho in &core_chirho.bindings_chirho {
        contract_chirho.visit_chirho(&binding_chirho.rhs_chirho);
    }
    assert_eq!(contract_chirho.seen_chirho.len(), 2);
    for entry_chirho in ["intOrderChirho", "integerOrderChirho", "positivePChirho"] {
        demanded_failure_chirho(&source_chirho, entry_chirho);
    }
    let demanded_chirho = format!(
        "{source_chirho}\nforceTailChirho [] = True\nforceTailChirho (_:restChirho) = forceTailChirho restChirho\nforceResultChirho = forceTailChirho tailChirho\n"
    );
    demanded_failure_chirho(&demanded_chirho, "forceResultChirho");
}

#[test]
fn source_append_roots_a_formatting_tail_across_collections_chirho() {
    let (value_chirho, machine_chirho) = eval_binding_chirho(
        &fixture_chirho("positive-chirho", "ShowGcChirho"),
        "resultChirho",
        3_000_000,
    )
    .unwrap();
    assert_eq!(value_chirho.to_string(), "8193");
    assert!(machine_chirho.last_gc_stats_chirho.is_some());
    assert!(!machine_chirho.integer_budget_chirho.exhausted_chirho());
}

#[test]
fn class_identity_and_present_duplicates_are_not_partial_absence_chirho() {
    for file_chirho in ["HomeShowDefaultChirho", "DuplicateShowChirho"] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("negative-chirho", file_chirho)).unwrap_err();
        assert!(
            matches!(
                error_chirho.kind_chirho,
                NextErrorKindChirho::TypeChirho | NextErrorKindChirho::InvalidChirho
            ),
            "{error_chirho}"
        );
    }
    for file_chirho in ["AbsentShowChirho", "AbsentListShowChirho"] {
        assert_eq!(
            compile_source_chirho(&fixture_chirho("boundary-chirho", file_chirho))
                .unwrap_err()
                .kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
}
