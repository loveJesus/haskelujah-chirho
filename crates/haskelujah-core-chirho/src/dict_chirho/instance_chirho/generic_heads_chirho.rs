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
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::{DictLayoutChirho, DictPassCtxChirho};

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
    /// and a second spelling is ambiguous; either refuses.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
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
            if !covered_chirho {
                return None;
            }
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
/// type variables: the constructor and n. Anything else is not generic.
fn distinct_variable_spine_chirho(head_chirho: &TyChirho) -> Option<(&str, usize)> {
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
/// is written `T U a`, and its uppercase token refuses it.
fn written_spine_chirho(
    spelling_chirho: &str,
    constructor_chirho: &str,
    arity_chirho: usize,
) -> bool {
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

#[cfg(test)]
#[path = "generic_heads_tests_chirho.rs"]
mod tests_chirho;
