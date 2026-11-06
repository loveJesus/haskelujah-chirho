// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generic instance heads: three spellings of one context-free instance,
//! brought back together inside one dictionary pass. The checker types the
//! head `SBoolChirho b` as `(SBoolChirho t0)`, over a variable it freshened.
//! The desugarer names the method rows by the head as written
//! (`$prim_Eq_==_SBoolChirho b`). A use site keeps only the outer head
//! (`SBoolChirho`). So the dictionary, `$fEq(SBoolChirho t0)`, was registered
//! under a key no use site carries and held a missing-method stub: the
//! instance never ran (claude2_chirho #27335, E1-E5). In a one-parameter class
//! without superclasses, a head that is a constructor applied to distinct
//! variables, with no competing head for that constructor among the instances
//! this module sees, is the only instance a `T x1 .. xn` can mean here. Its
//! dictionary keeps its typed name and key and fills its slots from the rows
//! as written; its constructor becomes an alias in this pass's private
//! registry. Nothing exports the alias, so a consumer module decides over its
//! own instances, a downstream specialized one included (gpt_chirho #27342,
//! #27349, #27365, #27368).
//!
//! A multi-parameter instance's head is read whole (row 694, #31909). Its key joins every head
//! type (`head_key_chirho`). Each superclass field is its class's template at that head, served
//! by the one context-free instance that matches it; a field this pass cannot build refuses the
//! module by name. A complete instance is registered in the conditional registry under that key
//! with its record, where a use at a complete ground head finds it by matching.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::super::ConditionalDictChirho;
use super::equality_defaults_chirho::equality_counterpart_chirho;
use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

/// A generic head: the alias a use site finds it by, and the spelling of its
/// rows.
pub(super) struct GenericHeadChirho {
    /// The head constructor, the key a use site carries (`SBoolChirho`): an
    /// alias in this pass's registry, never a binder name.
    pub(super) constructor_chirho: String,
    /// The head as the desugarer wrote it into the rows (`SBoolChirho b`).
    pub(super) written_key_chirho: String,
}

impl DictPassCtxChirho {
    /// The instance's generic head: `generic_spine_chirho` (G1-G4, G6), then
    /// G5, the slots confirmed. Every method of the class has a source row
    /// that `resolve_or_missing_method_id_chirho` will take: the exact row for
    /// the typed key `type_key_chirho`, or the row under ONE written spelling
    /// `T x1 .. xn` of distinct variable tokens, which correspond to the typed
    /// variables by position, a bijection. Coverage and lookup read the same
    /// spellings in the same order (agy_chirho #27369, gpt_chirho #27370), so a
    /// bare `T` row covers nothing. A missing row would make a stub reachable
    /// and a second spelling is ambiguous; either refuses. An Eq method without
    /// a source row is covered when its counterpart has one: the slot takes the
    /// counterpart's default under the written spelling (row 606). The list head `[v]`
    /// is the one exception to the missing row: its rows have one written
    /// spelling, so a slot no row covers is a method the instance genuinely
    /// omits, and its generated stub fails by name when forced, as GHC's
    /// NoMethodError does (gpt_chirho #28111).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    /// workflow: language-features-chirho/own-class-method-occurrences-chirho
    pub(super) fn generic_head_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        inst_chirho: &InstDeclChirho,
        layout_chirho: &DictLayoutChirho,
        type_key_chirho: &str,
    ) -> Option<GenericHeadChirho> {
        let (constructor_chirho, arity_chirho) =
            generic_spine_chirho(class_env_chirho, class_name_chirho, inst_chirho)?;
        let mut written_key_chirho: Option<&str> = None;
        let mut uncovered_chirho = Vec::new();
        for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
            let row_prefix_chirho = format!("$prim_{class_name_chirho}_{method_name_chirho}_");
            let mut covered_chirho = self
                .body_backed_names_chirho
                .contains_key(&format!("{row_prefix_chirho}{type_key_chirho}"));
            for row_name_chirho in self.body_backed_names_chirho.keys() {
                let Some(spelling_chirho) = row_name_chirho.strip_prefix(&row_prefix_chirho) else {
                    continue;
                };
                if !written_spine_chirho(spelling_chirho, constructor_chirho, arity_chirho) {
                    continue;
                }
                if written_key_chirho.is_some_and(|known_chirho| known_chirho != spelling_chirho) {
                    return None;
                }
                written_key_chirho = Some(spelling_chirho);
                covered_chirho = true;
            }
            if !covered_chirho && constructor_chirho != "[]" {
                uncovered_chirho.push(method_name_chirho.as_str());
            }
        }
        // An Eq method without a source row is covered by its counterpart's default (row 606).
        let covered_by_default_chirho = |method_name_chirho: &str| {
            equality_counterpart_chirho(class_name_chirho, method_name_chirho)
                .is_some_and(|counterpart_chirho| !uncovered_chirho.contains(&counterpart_chirho))
        };
        if !uncovered_chirho
            .iter()
            .all(|method_name_chirho| covered_by_default_chirho(method_name_chirho))
        {
            return None;
        }
        Some(GenericHeadChirho {
            constructor_chirho: constructor_chirho.to_string(),
            written_key_chirho: written_key_chirho.unwrap_or(type_key_chirho).to_string(),
        })
    }
}

/// The head constructor and arity of an instance the class environment alone
/// shows to be generic.
/// - G1: the class is declared with one parameter, and the instance supplies
///   one head (gpt_chirho #27365).
/// - G2: the instance has no context, so a row that needs a context
///   dictionary never becomes an unconditional one.
/// - G3: the typed head is a constructor applied to one or more pairwise
///   distinct type variables.
/// - G4: no other instance of the class has that head constructor or a
///   variable head. A use site that kept only the constructor then has exactly
///   one candidate, and no specialized row is passed over.
/// - G6: the class has no superclass. A superclass slot is named by the
///   instance's own key and the class records superclass names without their
///   arguments, so `class S [a] => C a` would fill the slot with `S`'s
///   instance for `T` where GHC uses `S [T ..]` (gpt_chirho #27365, #27366).
fn generic_spine_chirho<'a>(
    class_env_chirho: &ClassEnvChirho,
    class_name_chirho: &str,
    inst_chirho: &'a InstDeclChirho,
) -> Option<(&'a str, usize)> {
    let class_chirho = class_env_chirho.classes_chirho.get(class_name_chirho)?;
    if class_chirho.is_mptc_chirho()
        || !class_chirho.supers_chirho.is_empty()
        || !inst_chirho.extra_head_tys_chirho.is_empty()
        || !inst_chirho.context_chirho.is_empty()
    {
        return None;
    }
    let (constructor_chirho, arity_chirho) =
        distinct_variable_spine_chirho(&inst_chirho.head_ty_chirho)?;
    let competing_heads_chirho = class_env_chirho
        .instances_chirho
        .get(class_name_chirho)?
        .iter()
        .filter(|other_chirho| {
            head_constructor_chirho(&other_chirho.head_ty_chirho)
                .is_none_or(|other_head_chirho| other_head_chirho == constructor_chirho)
        })
        .count();
    (competing_heads_chirho == 1).then_some((constructor_chirho, arity_chirho))
}

/// `T v1 .. vn` with n >= 1, `T` a constructor and the `vi` pairwise distinct
/// type variables: the constructor and n. A list over one variable, `[v]`, is
/// the list constructor `[]` applied to it (gpt_chirho #28111). Anything else
/// is not generic.
fn distinct_variable_spine_chirho(head_chirho: &TyChirho) -> Option<(&str, usize)> {
    if let TyChirho::ListChirho(element_chirho) = head_chirho {
        return matches!(element_chirho.as_ref(), TyChirho::VarChirho(_)).then_some(("[]", 1));
    }
    let mut variables_chirho = Vec::new();
    let mut spine_chirho = head_chirho;
    while let TyChirho::AppChirho(function_chirho, argument_chirho) = spine_chirho {
        let TyChirho::VarChirho(variable_chirho) = argument_chirho.as_ref() else {
            return None;
        };
        if variables_chirho.contains(variable_chirho) {
            return None;
        }
        variables_chirho.push(*variable_chirho);
        spine_chirho = function_chirho;
    }
    match spine_chirho {
        TyChirho::ConChirho(name_chirho)
            if !variables_chirho.is_empty() && name_chirho.starts_with(char::is_uppercase) =>
        {
            Some((name_chirho.as_str(), variables_chirho.len()))
        }
        _ => None,
    }
}

/// The constructor at the head of an instance head's spine, or `None` for a
/// variable head, which competes with every constructor.
fn head_constructor_chirho(head_chirho: &TyChirho) -> Option<&str> {
    match head_chirho {
        TyChirho::AppChirho(function_chirho, _) => head_constructor_chirho(function_chirho),
        TyChirho::ConChirho(name_chirho) => Some(name_chirho.as_str()),
        TyChirho::ListChirho(_) => Some("[]"),
        TyChirho::TupleChirho(_) => Some("(,)"),
        TyChirho::FunChirho(..) => Some("->"),
        _ => None,
    }
}

/// A written key that spells the generic head: `T x1 .. xn`, the constructor
/// then exactly n pairwise distinct variable tokens. The desugarer's key drops
/// parentheses, so only an all-variable argument list is unambiguous: `T (U a)`
/// is written `T U a`, and its uppercase token refuses it. The list head is
/// written `[v]`, one variable token in brackets, and nothing else spells it.
fn written_spine_chirho(
    spelling_chirho: &str,
    constructor_chirho: &str,
    arity_chirho: usize,
) -> bool {
    if constructor_chirho == "[]" {
        return arity_chirho == 1
            && spelling_chirho
                .strip_prefix('[')
                .and_then(|inner_chirho| inner_chirho.strip_suffix(']'))
                .is_some_and(is_variable_token_chirho);
    }
    let mut tokens_chirho = spelling_chirho.split(' ');
    if tokens_chirho.next() != Some(constructor_chirho) {
        return false;
    }
    let variables_chirho: Vec<&str> = tokens_chirho.collect();
    variables_chirho.len() == arity_chirho
        && variables_chirho
            .iter()
            .enumerate()
            .all(|(index_chirho, token_chirho)| {
                is_variable_token_chirho(token_chirho)
                    && !variables_chirho[..index_chirho].contains(token_chirho)
            })
}

/// A type variable as the desugarer writes one: a lowercase letter, then
/// identifier characters. `_`, a promoted `'Con`, a literal, an operator and
/// a constructor never are.
fn is_variable_token_chirho(token_chirho: &str) -> bool {
    let mut chars_chirho = token_chirho.chars();
    chars_chirho.next().is_some_and(char::is_lowercase)
        && chars_chirho
            .all(|ch_chirho| ch_chirho.is_alphanumeric() || matches!(ch_chirho, '_' | '\''))
}

impl DictPassCtxChirho {
    /// The dictionary of the one context-free instance that matches `predicate_chirho`, as this
    /// pass registered it: `Show Bool` takes `$fShowBool`, and `Monad (ReaderChirho r)` the
    /// dictionary of `instance Monad (ReaderChirho r)`. The instance is found by matching its
    /// head, never by the predicate's spelling, and one instance seen twice is one. No match,
    /// several, one with a context, or one this pass built no dictionary for is the reason
    /// returned (row 694, #31909).
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn context_free_dictionary_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        predicate_chirho: &PredChirho,
    ) -> Result<CoreIdChirho, String> {
        let mut matching_chirho: Vec<(String, bool)> = class_env_chirho
            .instances_chirho
            .get(&predicate_chirho.class_name_chirho)
            .into_iter()
            .flatten()
            .filter_map(|inst_chirho| {
                ClassEnvChirho::instance_sub_goals_chirho(inst_chirho, predicate_chirho).map(
                    |goals_chirho| {
                        (
                            head_key_chirho(
                                &inst_chirho.head_ty_chirho,
                                &inst_chirho.extra_head_tys_chirho,
                            ),
                            goals_chirho.is_empty(),
                        )
                    },
                )
            })
            .collect();
        matching_chirho.sort();
        matching_chirho.dedup();
        let [(key_chirho, context_free_chirho)] = matching_chirho.as_slice() else {
            return Err(if matching_chirho.is_empty() {
                format!("no instance matches `{predicate_chirho}`")
            } else {
                format!("several instances match `{predicate_chirho}`")
            });
        };
        if !*context_free_chirho {
            return Err(format!(
                "the instance matching `{predicate_chirho}` has a context, whose dictionaries this pass does not build here"
            ));
        }
        self.existing_id_named_chirho(&format!(
            "$f{}{key_chirho}",
            predicate_chirho.class_name_chirho
        ))
        .ok_or_else(|| {
            format!("this pass built no dictionary for the instance matching `{predicate_chirho}`")
        })
    }

    /// The superclass fields of the multi-parameter instance `inst_chirho`, in layout order, and
    /// whether every one was built: one per template of its class, instantiated at the
    /// instance's whole head and served by the one context-free instance that matches it
    /// (`ShowSecondChirho Int Bool` holds `Show Bool`'s, row 694, #31909). A required field this
    /// pass cannot build refuses the module by name (`unsupported_dictionaries_chirho`, #31948)
    /// and holds an explicit error meanwhile, never an unbound id; that instance is never
    /// registered as built.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn multi_head_superclass_fields_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        inst_chirho: &InstDeclChirho,
        layout_chirho: &DictLayoutChirho,
    ) -> (Vec<CoreExprChirho>, bool) {
        let mut fields_chirho = Vec::new();
        let mut built_chirho = true;
        for (index_chirho, (super_name_chirho, _)) in
            layout_chirho.super_slots_chirho.iter().enumerate()
        {
            let field_chirho = superclass_at_head_chirho(
                class_env_chirho,
                inst_chirho,
                layout_chirho,
                index_chirho,
            )
            .and_then(|superclass_chirho| {
                self.context_free_dictionary_chirho(class_env_chirho, &superclass_chirho)
            });
            fields_chirho.push(match field_chirho {
                Ok(dictionary_chirho) => CoreExprChirho::VarChirho(dictionary_chirho),
                Err(reason_chirho) => {
                    built_chirho = false;
                    let refusal_chirho = format!(
                        "the `{super_name_chirho}` superclass dictionary of the instance `{}`: {reason_chirho}",
                        instance_label_chirho(inst_chirho)
                    );
                    self.unsupported_dictionaries_chirho
                        .push(refusal_chirho.clone());
                    explicit_error_chirho(refusal_chirho)
                }
            });
        }
        (fields_chirho, built_chirho)
    }

    /// Registers the multi-parameter instance `inst_chirho` in the conditional registry under
    /// its whole head key, with its record: a use at a complete ground head finds it by matching
    /// that head (`head_dictionary_chirho`, row 694, #31909). `builder_chirho` is its dictionary,
    /// or its factory over its context's dictionaries. An instance seen twice keeps its first
    /// entry.
    /// workflow: compiler-pipeline-chirho/deriving-strategies-chirho
    pub(super) fn register_multi_head_chirho(
        &mut self,
        inst_chirho: &InstDeclChirho,
        builder_chirho: CoreIdChirho,
    ) {
        self.conditional_dicts_chirho
            .entry((
                inst_chirho.class_name_chirho.clone(),
                head_key_chirho(
                    &inst_chirho.head_ty_chirho,
                    &inst_chirho.extra_head_tys_chirho,
                ),
            ))
            .or_insert_with(|| ConditionalDictChirho {
                builder_chirho,
                formals_chirho: Vec::new(),
                context_chirho: inst_chirho.context_chirho.clone(),
                instance_chirho: Some(inst_chirho.clone()),
            });
    }
}

/// The registry key of a head: its first type as it displays, then each further type of a
/// multi-parameter head, joined by `_` (`Int_Bool` for `ShowSecondChirho Int Bool`). Every
/// instance dictionary is named by it (`$fShowSecondChirhoInt_Bool`).
pub(super) fn head_key_chirho(first_chirho: &TyChirho, extras_chirho: &[TyChirho]) -> String {
    std::iter::once(first_chirho)
        .chain(extras_chirho)
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("_")
}

/// The `index_chirho`-th superclass of the multi-parameter instance `inst_chirho` at its own
/// head: its class's template instantiated there (`Show b` at `ShowSecondChirho Int Bool` is
/// `Show Bool`, row 694, #31909). The template must name the class of the layout's slot, and
/// that class no other slot, since a superclass selector is found by its class's name.
pub(super) fn superclass_at_head_chirho(
    class_env_chirho: &ClassEnvChirho,
    inst_chirho: &InstDeclChirho,
    layout_chirho: &DictLayoutChirho,
    index_chirho: usize,
) -> Result<PredChirho, String> {
    let class_chirho = class_env_chirho
        .classes_chirho
        .get(&inst_chirho.class_name_chirho)
        .ok_or_else(|| "its class is not in the class environment".to_string())?;
    let (slot_chirho, _) = layout_chirho
        .super_slots_chirho
        .get(index_chirho)
        .ok_or_else(|| "its class's layout has no such superclass slot".to_string())?;
    if layout_chirho
        .super_slots_chirho
        .iter()
        .filter(|(other_chirho, _)| other_chirho == slot_chirho)
        .count()
        != 1
    {
        return Err(format!(
            "its class names `{slot_chirho}` for more than one superclass"
        ));
    }
    let head_chirho: Vec<TyChirho> = std::iter::once(inst_chirho.head_ty_chirho.clone())
        .chain(inst_chirho.extra_head_tys_chirho.iter().cloned())
        .collect();
    let superclasses_chirho = class_chirho.superclass_preds_at_chirho(&head_chirho);
    if superclasses_chirho.len() != layout_chirho.super_slots_chirho.len() {
        return Err("its class's superclass templates do not match its layout".to_string());
    }
    superclasses_chirho
        .into_iter()
        .nth(index_chirho)
        .filter(|superclass_chirho| superclass_chirho.class_name_chirho == *slot_chirho)
        .ok_or_else(|| format!("its class's template for `{slot_chirho}` names another class"))
}

/// How a diagnostic names an instance: its class and whole head (`ShowSecondChirho Int Bool`).
pub(super) fn instance_label_chirho(inst_chirho: &InstDeclChirho) -> String {
    PredChirho {
        class_name_chirho: inst_chirho.class_name_chirho.clone(),
        ty_chirho: inst_chirho.head_ty_chirho.clone(),
        extra_tys_chirho: inst_chirho.extra_head_tys_chirho.clone(),
    }
    .to_string()
}

/// The placeholder of a superclass field whose refusal already stops the module: it fails by
/// name if ever forced, so the dictionary binds no unbound id meanwhile.
fn explicit_error_chirho(message_chirho: String) -> CoreExprChirho {
    CoreExprChirho::PrimOpChirho {
        name_chirho: "error".to_string(),
        args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
            message_chirho,
        ))],
    }
}

#[cfg(test)]
#[path = "generic_heads_tests_chirho.rs"]
mod tests_chirho;
