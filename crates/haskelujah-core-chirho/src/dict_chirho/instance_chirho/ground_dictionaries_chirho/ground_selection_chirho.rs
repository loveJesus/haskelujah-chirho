// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The instance a ground goal selects, by the checker's own structural one-way head match
//! (`ClassEnvChirho::instance_sub_goals_chirho`), and the reasons a ground goal is refused,
//! each by name. No key string is parsed or matched here. A registry key is spelled from a
//! structured type only to look up the dictionary registry under its own spelling.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::BTreeSet;

use super::ground_budget_chirho::GroundBudgetChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

/// Why a ground goal has no constructed dictionary. Each names what could not be decided, in
/// words a person reads: instances with lettered variables, never a row key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::dict_chirho) enum GroundRefusalChirho {
    /// No instance head matches the goal.
    UnresolvedChirho(PredChirho),
    /// Several instance heads match. No overlap or origin authority is modeled, so none is
    /// chosen.
    OverlapChirho(PredChirho, Vec<String>),
    /// The matched instance's rows are not declared in this module.
    RowsElsewhereChirho(String),
    /// The matched instance shares its declaration's place with another, so its rows are
    /// not proven its own.
    RowsAmbiguousChirho(String),
    /// A context-free instance has no dictionary in this module.
    NoGroundDictionaryChirho(String),
    /// The matched instance has an equality context, which no route models.
    EqualityContextChirho(String),
    /// The matched instance's context has a predicate with an extra class argument, which
    /// the evidence does not capture, so its rows could not tell its dictionaries apart.
    UnindexableContextChirho(String),
    /// A goal with extra class arguments: the superclasses' arguments are not recorded.
    MultiParameterChirho(PredChirho),
    /// The goal's class lays out no dictionary in this pass.
    NoLayoutChirho(String),
    /// A context obligation names a type constructor its goal does not, whose identity no
    /// producer proved.
    UnprovenConstructorChirho(String, String),
    /// A context obligation still holds a variable after the head match.
    OpenObligationChirho(PredChirho),
    /// A bound was reached: depth, dictionaries, visits, type work, goal weight or one
    /// candidate's copy weight.
    BudgetChirho(&'static str, usize),
}

impl GroundRefusalChirho {
    /// The refusal in words, for the driver's diagnostic. The driver writes promoted and
    /// family keys back as the constructors and families they name.
    pub(in crate::dict_chirho) fn describe_chirho(&self) -> String {
        match self {
            Self::UnresolvedChirho(goal_chirho) => format!("no instance for `{goal_chirho}`"),
            Self::OverlapChirho(goal_chirho, instances_chirho) => format!(
                "overlapping instances for `{goal_chirho}`: {}",
                instances_chirho
                    .iter()
                    .map(|instance_chirho| format!("`{instance_chirho}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::RowsElsewhereChirho(instance_chirho) => {
                format!("the methods of instance `{instance_chirho}` are not in this module")
            }
            Self::RowsAmbiguousChirho(instance_chirho) => format!(
                "the methods of instance `{instance_chirho}` cannot be told apart from another \
                 declaration at the same place"
            ),
            Self::NoGroundDictionaryChirho(instance_chirho) => {
                format!("instance `{instance_chirho}` has no dictionary in this module")
            }
            Self::EqualityContextChirho(instance_chirho) => {
                format!("the equality context of instance `{instance_chirho}` is not supported")
            }
            Self::UnindexableContextChirho(instance_chirho) => format!(
                "the context of instance `{instance_chirho}` has a multi-parameter predicate, \
                 which is not supported"
            ),
            Self::MultiParameterChirho(goal_chirho) => {
                format!("the multi-parameter goal `{goal_chirho}` is not supported")
            }
            Self::NoLayoutChirho(class_chirho) => {
                format!("class `{class_chirho}` has no dictionary layout here")
            }
            Self::UnprovenConstructorChirho(obligation_chirho, constructor_chirho) => format!(
                "the context obligation `{obligation_chirho}` names `{constructor_chirho}`, \
                 which its goal does not"
            ),
            Self::OpenObligationChirho(obligation_chirho) => format!(
                "the context obligation `{}` is not ground",
                readable_pred_chirho(obligation_chirho)
            ),
            Self::BudgetChirho(bound_chirho, limit_chirho) => {
                format!("the {bound_chirho} bound ({limit_chirho}) was reached")
            }
        }
    }
}

/// The unique instance whose head matches `goal_chirho`, with its context under the matching
/// substitution, in the instance's own context order. Every candidate is preflighted and
/// charged before the checker's matcher copies anything (`ground_budget_chirho.rs`); every
/// candidate is tried, so an overlap is never missed.
pub(in crate::dict_chirho) fn select_ground_instance_chirho<'env_chirho>(
    class_env_chirho: &'env_chirho ClassEnvChirho,
    goal_chirho: &PredChirho,
    goal_weight_chirho: usize,
    budget_chirho: &mut GroundBudgetChirho,
) -> Result<(&'env_chirho InstDeclChirho, Vec<PredChirho>), GroundRefusalChirho> {
    let instances_chirho = class_env_chirho
        .instances_chirho
        .get(&goal_chirho.class_name_chirho)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut selected_chirho: Option<(&InstDeclChirho, Vec<PredChirho>)> = None;
    let mut others_chirho: Vec<String> = Vec::new();
    for instance_chirho in instances_chirho {
        budget_chirho.candidate_chirho(instance_chirho, goal_weight_chirho)?;
        let Some(sub_goals_chirho) =
            ClassEnvChirho::instance_sub_goals_chirho(instance_chirho, goal_chirho)
        else {
            continue;
        };
        if selected_chirho.is_none() {
            selected_chirho = Some((instance_chirho, sub_goals_chirho));
        } else {
            others_chirho.push(readable_instance_chirho(instance_chirho));
        }
    }
    let Some(selected_chirho) = selected_chirho else {
        return Err(GroundRefusalChirho::UnresolvedChirho(goal_chirho.clone()));
    };
    if !others_chirho.is_empty() {
        let mut instances_chirho = vec![readable_instance_chirho(selected_chirho.0)];
        instances_chirho.extend(others_chirho);
        return Err(GroundRefusalChirho::OverlapChirho(
            goal_chirho.clone(),
            instances_chirho,
        ));
    }
    Ok(selected_chirho)
}

/// The dictionary registry's key for a head and its extra heads, spelled exactly as the
/// instance-dictionary generator spells it (`instance_chirho.rs`).
fn registry_key_chirho(head_chirho: &TyChirho, extras_chirho: &[TyChirho]) -> String {
    if extras_chirho.is_empty() {
        format!("{head_chirho}")
    } else {
        let extras_chirho: Vec<String> = extras_chirho
            .iter()
            .map(|ty_chirho| format!("{ty_chirho}"))
            .collect();
        format!("{head_chirho}_{}", extras_chirho.join("_"))
    }
}

/// The registry key of an instance's own dictionary.
pub(in crate::dict_chirho) fn instance_registry_key_chirho(
    instance_chirho: &InstDeclChirho,
) -> String {
    registry_key_chirho(
        &instance_chirho.head_ty_chirho,
        &instance_chirho.extra_head_tys_chirho,
    )
}

/// The registry key a dictionary for exactly this ground goal is filed under, such as a
/// ground list specialization (`$fShow[Int]` under `[Int]`).
pub(in crate::dict_chirho) fn goal_registry_key_chirho(goal_chirho: &PredChirho) -> String {
    registry_key_chirho(&goal_chirho.ty_chirho, &goal_chirho.extra_tys_chirho)
}

/// Whether a type holds no inference meta, no rigid or quantified variable and no binder.
pub(in crate::dict_chirho) fn ty_is_ground_chirho(ty_chirho: &TyChirho) -> bool {
    match ty_chirho {
        TyChirho::VarChirho(_)
        | TyChirho::ForallVarChirho(_)
        | TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. } => false,
        TyChirho::ConChirho(_) => true,
        TyChirho::AppChirho(function_chirho, argument_chirho) => {
            ty_is_ground_chirho(function_chirho) && ty_is_ground_chirho(argument_chirho)
        }
        TyChirho::FunChirho(parameter_chirho, result_chirho, _) => {
            ty_is_ground_chirho(parameter_chirho) && ty_is_ground_chirho(result_chirho)
        }
        TyChirho::TupleChirho(elements_chirho) => elements_chirho.iter().all(ty_is_ground_chirho),
        TyChirho::ListChirho(element_chirho) => ty_is_ground_chirho(element_chirho),
    }
}

/// Whether every argument of a predicate, extra class arguments included, is ground.
pub(in crate::dict_chirho) fn pred_is_ground_chirho(pred_chirho: &PredChirho) -> bool {
    ty_is_ground_chirho(&pred_chirho.ty_chirho)
        && pred_chirho.extra_tys_chirho.iter().all(ty_is_ground_chirho)
}

/// Every named type constructor a predicate's arguments mention. List, tuple and arrow are
/// the compiler's own structure and carry no name.
pub(in crate::dict_chirho) fn pred_constructors_chirho(pred_chirho: &PredChirho) -> BTreeSet<&str> {
    let mut constructors_chirho = BTreeSet::new();
    for ty_chirho in std::iter::once(&pred_chirho.ty_chirho).chain(&pred_chirho.extra_tys_chirho) {
        collect_constructors_chirho(ty_chirho, &mut constructors_chirho);
    }
    constructors_chirho
}

fn collect_constructors_chirho<'ty_chirho>(
    ty_chirho: &'ty_chirho TyChirho,
    constructors_chirho: &mut BTreeSet<&'ty_chirho str>,
) {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) => {
            constructors_chirho.insert(name_chirho.as_str());
        }
        TyChirho::AppChirho(function_chirho, argument_chirho)
        | TyChirho::FunChirho(function_chirho, argument_chirho, _) => {
            collect_constructors_chirho(function_chirho, constructors_chirho);
            collect_constructors_chirho(argument_chirho, constructors_chirho);
        }
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                collect_constructors_chirho(element_chirho, constructors_chirho);
            }
        }
        TyChirho::ListChirho(element_chirho) => {
            collect_constructors_chirho(element_chirho, constructors_chirho);
        }
        TyChirho::ForallChirho { body_chirho, .. }
        | TyChirho::RequiredForallChirho { body_chirho, .. } => {
            collect_constructors_chirho(body_chirho, constructors_chirho);
        }
        TyChirho::VarChirho(_) | TyChirho::ForallVarChirho(_) => {}
    }
}

/// An instance as a person reads it: `SizeChirho (Maybe a)`, each freshened variable written
/// as a letter in order of first occurrence.
pub(in crate::dict_chirho) fn readable_instance_chirho(instance_chirho: &InstDeclChirho) -> String {
    let mut letters_chirho = Vec::new();
    format!(
        "{}",
        PredChirho {
            class_name_chirho: instance_chirho.class_name_chirho.clone(),
            ty_chirho: lettered_chirho(&instance_chirho.head_ty_chirho, &mut letters_chirho),
            extra_tys_chirho: instance_chirho
                .extra_head_tys_chirho
                .iter()
                .map(|ty_chirho| lettered_chirho(ty_chirho, &mut letters_chirho))
                .collect(),
        }
    )
}

/// A predicate as a person reads it, its variables lettered.
fn readable_pred_chirho(pred_chirho: &PredChirho) -> String {
    let mut letters_chirho = Vec::new();
    format!(
        "{}",
        PredChirho {
            class_name_chirho: pred_chirho.class_name_chirho.clone(),
            ty_chirho: lettered_chirho(&pred_chirho.ty_chirho, &mut letters_chirho),
            extra_tys_chirho: pred_chirho
                .extra_tys_chirho
                .iter()
                .map(|ty_chirho| lettered_chirho(ty_chirho, &mut letters_chirho))
                .collect(),
        }
    )
}

/// A copy of `ty_chirho` with each inference variable replaced by a letter, for display only.
fn lettered_chirho(ty_chirho: &TyChirho, letters_chirho: &mut Vec<TyVarChirho>) -> TyChirho {
    match ty_chirho {
        TyChirho::VarChirho(var_chirho) => {
            let known_index_chirho = letters_chirho
                .iter()
                .position(|known_chirho| known_chirho == var_chirho);
            let index_chirho = match known_index_chirho {
                Some(index_chirho) => index_chirho,
                None => {
                    letters_chirho.push(*var_chirho);
                    letters_chirho.len() - 1
                }
            };
            TyChirho::ConChirho(letter_chirho(index_chirho))
        }
        TyChirho::AppChirho(function_chirho, argument_chirho) => TyChirho::AppChirho(
            Box::new(lettered_chirho(function_chirho, letters_chirho)),
            Box::new(lettered_chirho(argument_chirho, letters_chirho)),
        ),
        TyChirho::FunChirho(parameter_chirho, result_chirho, multiplicity_chirho) => {
            TyChirho::FunChirho(
                Box::new(lettered_chirho(parameter_chirho, letters_chirho)),
                Box::new(lettered_chirho(result_chirho, letters_chirho)),
                *multiplicity_chirho,
            )
        }
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(|element_chirho| lettered_chirho(element_chirho, letters_chirho))
                .collect(),
        ),
        TyChirho::ListChirho(element_chirho) => {
            TyChirho::ListChirho(Box::new(lettered_chirho(element_chirho, letters_chirho)))
        }
        TyChirho::ConChirho(_)
        | TyChirho::ForallVarChirho(_)
        | TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. } => ty_chirho.clone(),
    }
}

/// `a` .. `z`, then `a1` .. `z1`, and so on.
fn letter_chirho(index_chirho: usize) -> String {
    let letter_chirho = char::from(b'a' + (index_chirho % 26) as u8);
    match index_chirho / 26 {
        0 => letter_chirho.to_string(),
        round_chirho => format!("{letter_chirho}{round_chirho}"),
    }
}
