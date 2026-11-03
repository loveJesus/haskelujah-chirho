// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! # Promoted data constructors' identity (row 686)
//!
//! A promoted constructor, `'True` or an unquoted `True` where no type `True` is in
//! scope, is identified by an opaque key naming its defining module, never by its
//! spelling. `'A.T` and `'B.T` are different constructors; an alias or a re-export of one
//! constructor is the same one; a source `data Bool = True` is not the standard `True`.
//!
//! Naming resolves each written spelling once per module into a
//! [`PromotionScopeChirho`]. The typing converters and Core's type keys read it, so every
//! conversion agrees. An AST that leaves its module (an exported synonym) carries the
//! resolution on its names instead (`RawNameChirho::promotion_chirho`). Ordinary datatype
//! names are never rewritten.
//! workflow: compiler-pipeline-chirho/promoted-constructor-identity-chirho

use std::collections::{HashMap, HashSet};

use crate::name_chirho::NameChirho;
use crate::ty_chirho::{ConstraintChirho, TypeChirho};

/// The prefix of every promoted key. `%` never starts a Haskell name, so a key never
/// collides with a written type or constructor name, and helpers that split names on
/// `.` must check it first.
pub const PROMOTED_KEY_PREFIX_CHIRHO: &str = "%promoted:";

const UNKNOWN_TAG_CHIRHO: &str = "unknown";

/// The module the compiler's own constructors are defined in: `Bool`, `Ordering` and the
/// list constructors.
pub const BUILTIN_CONSTRUCTOR_MODULE_CHIRHO: &str = "GHC.Types";

/// The module the compiler's unit constructor is defined in.
pub const BUILTIN_TUPLE_MODULE_CHIRHO: &str = "GHC.Tuple";

/// Whether `text` is a promoted-constructor key rather than a written name.
pub fn is_promoted_key_chirho(text_chirho: &str) -> bool {
    text_chirho.starts_with(PROMOTED_KEY_PREFIX_CHIRHO)
}

/// Whether `text` is a promoted key whose constructor naming could not prove.
pub fn is_unknown_promoted_key_chirho(text_chirho: &str) -> bool {
    text_chirho
        .strip_prefix(PROMOTED_KEY_PREFIX_CHIRHO)
        .is_some_and(|rest_chirho| rest_chirho.starts_with(UNKNOWN_TAG_CHIRHO))
}

/// One promoted constructor's identity: `%promoted:<tag>:<len>:<module><name>`, where the
/// byte length of the module part delimits it from the constructor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromotedKeyChirho(String);

impl PromotedKeyChirho {
    /// A constructor that source module `module_chirho` defines.
    pub fn source_chirho(module_chirho: &str, constructor_chirho: &str) -> Self {
        Self::tagged_chirho("source", module_chirho, constructor_chirho)
    }

    /// A constructor the compiler itself defines, such as `GHC.Types`' `True` or list
    /// cons. Only a trusted builtin interface or the compiler's own producers grant one.
    pub fn builtin_chirho(module_chirho: &str, constructor_chirho: &str) -> Self {
        Self::tagged_chirho("builtin", module_chirho, constructor_chirho)
    }

    /// A promoted spelling whose constructor could not be proven. It is reflexive (the
    /// same spelling meets itself) and classified Unknown: never apart from another
    /// constructor and never trusted as rigid.
    pub fn unknown_chirho(qualifier_chirho: &str, text_chirho: &str) -> Self {
        Self::tagged_chirho(UNKNOWN_TAG_CHIRHO, qualifier_chirho, text_chirho)
    }

    /// The compiler's promoted `True` or `False`, as its own type-level producers write it.
    pub fn builtin_bool_chirho(value_chirho: bool) -> Self {
        Self::builtin_chirho(
            BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
            if value_chirho { "True" } else { "False" },
        )
    }

    /// The compiler's promoted list nil `'[]`.
    pub fn builtin_nil_chirho() -> Self {
        Self::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "[]")
    }

    /// The compiler's promoted list cons `':`.
    pub fn builtin_cons_chirho() -> Self {
        Self::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, ":")
    }

    /// The compiler's promoted unit `'()`.
    pub fn builtin_unit_chirho() -> Self {
        Self::builtin_chirho(BUILTIN_TUPLE_MODULE_CHIRHO, "()")
    }

    fn tagged_chirho(tag_chirho: &str, module_chirho: &str, name_chirho: &str) -> Self {
        Self(format!(
            "{PROMOTED_KEY_PREFIX_CHIRHO}{tag_chirho}:{}:{module_chirho}{name_chirho}",
            module_chirho.len()
        ))
    }

    /// The key text, as a type constructor's name carries it.
    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

/// What a promoted spelling resolves to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionResolutionChirho {
    /// The constructor with this identity.
    ResolvedChirho(PromotedKeyChirho),
    /// Naming saw a promoted spelling but could not prove its constructor.
    UnresolvedChirho(PromotedKeyChirho),
}

impl PromotionResolutionChirho {
    /// The key a conversion writes.
    pub fn key_chirho(&self) -> &PromotedKeyChirho {
        match self {
            Self::ResolvedChirho(key_chirho) | Self::UnresolvedChirho(key_chirho) => key_chirho,
        }
    }

    /// The Unknown resolution of a spelling with no proof.
    pub fn unresolved_chirho(name_chirho: &NameChirho) -> Self {
        Self::UnresolvedChirho(PromotedKeyChirho::unknown_chirho(
            name_chirho.qualifier_chirho().unwrap_or_default(),
            name_chirho.text_chirho(),
        ))
    }
}

/// One module's promoted-constructor scope, built by naming after resolution.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PromotionScopeChirho {
    /// Constructor spellings as written (`T`, `M.T`) and what each resolves to.
    constructors_chirho: HashMap<String, PromotionResolutionChirho>,
    /// Type-namespace spellings in scope: an unquoted name among them is the type.
    types_chirho: HashSet<String>,
}

impl PromotionScopeChirho {
    /// A scope from its two tables.
    pub fn new_chirho(
        constructors_chirho: HashMap<String, PromotionResolutionChirho>,
        types_chirho: HashSet<String>,
    ) -> Self {
        Self {
            constructors_chirho,
            types_chirho,
        }
    }

    /// The identity of an explicitly quoted constructor (`'T`, `'M.T`).
    pub fn quoted_chirho(&self, name_chirho: &NameChirho) -> PromotionResolutionChirho {
        self.constructors_chirho
            .get(&name_chirho.full_name_chirho())
            .cloned()
            .unwrap_or_else(|| PromotionResolutionChirho::unresolved_chirho(name_chirho))
    }

    /// The identity of an unquoted spelling in a type: `None` when the type namespace
    /// has it (GHC looks there first) or when no constructor has it.
    pub fn unquoted_chirho(&self, name_chirho: &NameChirho) -> Option<PromotionResolutionChirho> {
        let spelling_chirho = name_chirho.full_name_chirho();
        if self.types_chirho.contains(&spelling_chirho) {
            return None;
        }
        self.constructors_chirho.get(&spelling_chirho).cloned()
    }
}

/// The key a promoted spelling converts to, or `None` when nothing classified it: the
/// name's own resolution first (stamped where an AST left its module), then the module's
/// scope. `quoted_chirho` says whether the source wrote a tick.
pub fn promoted_key_chirho(
    name_chirho: &NameChirho,
    quoted_chirho: bool,
    scope_chirho: Option<&PromotionScopeChirho>,
) -> Option<PromotedKeyChirho> {
    if let Some(resolution_chirho) = name_chirho.promotion_chirho() {
        return Some(resolution_chirho.key_chirho().clone());
    }
    let scope_chirho = scope_chirho?;
    if quoted_chirho {
        Some(scope_chirho.quoted_chirho(name_chirho).key_chirho().clone())
    } else {
        scope_chirho
            .unquoted_chirho(name_chirho)
            .map(|resolution_chirho| resolution_chirho.key_chirho().clone())
    }
}

/// `:`, which only the compiler defines.
fn is_cons_spelling_chirho(name_chirho: &NameChirho) -> bool {
    name_chirho.text_chirho() == ":" && name_chirho.qualifier_chirho().is_none()
}

/// The identity of a constructor only the compiler defines, written with its own syntax:
/// list cons `:` and nil `[]`, unit `()` and the tuple constructors `(,)`, `(,,)`, ….
/// No module can declare these spellings, so a tick on one needs no scope to resolve.
fn compiler_constructor_key_chirho(name_chirho: &NameChirho) -> Option<PromotedKeyChirho> {
    if name_chirho.qualifier_chirho().is_some() {
        return None;
    }
    match name_chirho.text_chirho() {
        ":" => Some(PromotedKeyChirho::builtin_cons_chirho()),
        "[]" => Some(PromotedKeyChirho::builtin_nil_chirho()),
        "()" => Some(PromotedKeyChirho::builtin_unit_chirho()),
        text_chirho
            if text_chirho.len() > 2
                && text_chirho.starts_with('(')
                && text_chirho.ends_with(')')
                && text_chirho[1..text_chirho.len() - 1]
                    .bytes()
                    .all(|byte_chirho| byte_chirho == b',') =>
        {
            Some(PromotedKeyChirho::builtin_chirho(
                BUILTIN_TUPLE_MODULE_CHIRHO,
                text_chirho,
            ))
        }
        _ => None,
    }
}

/// The spelling a quoted promoted constructor converts to under a scope or a stamp: a
/// compiler constructor (`':`, `'[]`, `'()`, `'(,)`) its compiler identity, anything else
/// its identity key. `None` when nothing classified it, for a caller that never ran
/// naming, which keeps its legacy spelling. Both typing converters and Core's type keys
/// use this one rule.
pub fn quoted_promoted_spelling_chirho(
    name_chirho: &NameChirho,
    scope_chirho: Option<&PromotionScopeChirho>,
) -> Option<String> {
    if name_chirho.promotion_chirho().is_none() && scope_chirho.is_none() {
        return None;
    }
    if let Some(key_chirho) = compiler_constructor_key_chirho(name_chirho) {
        return Some(key_chirho.0);
    }
    promoted_key_chirho(name_chirho, true, scope_chirho).map(|key_chirho| key_chirho.0)
}

/// The spelling an unquoted name converts to when it is a promoted constructor, the type
/// namespace lacking it; `None` for an ordinary type name.
pub fn unquoted_promoted_spelling_chirho(
    name_chirho: &NameChirho,
    scope_chirho: Option<&PromotionScopeChirho>,
) -> Option<String> {
    if is_cons_spelling_chirho(name_chirho)
        && (scope_chirho.is_some() || name_chirho.promotion_chirho().is_some())
    {
        return Some(PromotedKeyChirho::builtin_cons_chirho().0);
    }
    promoted_key_chirho(name_chirho, false, scope_chirho).map(|key_chirho| key_chirho.0)
}

/// Stamp every promoted-constructor spelling in `ty_chirho` with what `scope_chirho`
/// resolves it to, so the type can leave its module (an exported synonym's right side)
/// and keep its identity in an importer that never resolves it. The compiler's own
/// constructors (`:`, `[]`, `()`, tuples) are left alone: their identity needs no scope.
/// Every match is exhaustive, so a new type form fails to compile until it is classified.
pub fn stamp_promotions_chirho(ty_chirho: &mut TypeChirho, scope_chirho: &PromotionScopeChirho) {
    match ty_chirho {
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            if compiler_constructor_key_chirho(name_chirho).is_none() {
                let resolution_chirho = scope_chirho.quoted_chirho(name_chirho);
                name_chirho.set_promotion_chirho(resolution_chirho);
            }
        }
        TypeChirho::ConChirho(name_chirho) => {
            if !is_cons_spelling_chirho(name_chirho)
                && let Some(resolution_chirho) = scope_chirho.unquoted_chirho(name_chirho)
            {
                name_chirho.set_promotion_chirho(resolution_chirho);
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            stamp_promotions_chirho(fun_chirho, scope_chirho);
            stamp_promotions_chirho(arg_chirho, scope_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            stamp_promotions_chirho(arg_chirho, scope_chirho);
            stamp_promotions_chirho(result_chirho, scope_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                stamp_promotions_chirho(element_chirho, scope_chirho);
            }
        }
        TypeChirho::ListChirho { element_chirho, .. } => {
            stamp_promotions_chirho(element_chirho, scope_chirho);
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => {
            stamp_promotions_chirho(inner_chirho, scope_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                stamp_constraint_promotions_chirho(constraint_chirho, scope_chirho);
            }
            stamp_promotions_chirho(body_chirho, scope_chirho);
        }
        TypeChirho::ForallChirho { body_chirho, .. }
        | TypeChirho::RequiredForallChirho { body_chirho, .. } => {
            stamp_promotions_chirho(body_chirho, scope_chirho);
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => {}
    }
}

fn stamp_constraint_promotions_chirho(
    constraint_chirho: &mut ConstraintChirho,
    scope_chirho: &PromotionScopeChirho,
) {
    match constraint_chirho {
        ConstraintChirho::ClassChirho { args_chirho, .. } => {
            for arg_chirho in args_chirho {
                stamp_promotions_chirho(arg_chirho, scope_chirho);
            }
        }
        ConstraintChirho::QuantifiedChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for context_constraint_chirho in context_chirho {
                stamp_constraint_promotions_chirho(context_constraint_chirho, scope_chirho);
            }
            stamp_constraint_promotions_chirho(body_chirho, scope_chirho);
        }
    }
}

/// Text a person reads with every promoted key written back as the constructor it names:
/// `'True`, `'Z`, or `'M.T` for an unproven qualified spelling. When one text names two
/// different constructors with one spelling, both are qualified by their modules
/// (`'Main.T`, `'Def.T`), so a mismatch never reads as `'T` against `'T`. `Ty`'s Display
/// stays canonical, because Core's dictionary keys read it, so diagnostics are rewritten
/// here.
pub fn readable_promoted_keys_chirho(text_chirho: &str) -> String {
    let mut identities_chirho: HashMap<&str, HashSet<(&str, &str)>> = HashMap::new();
    for key_chirho in keys_in_text_chirho(text_chirho) {
        if let Some(parsed_chirho) = key_chirho.1 {
            identities_chirho
                .entry(parsed_chirho.name_chirho)
                .or_default()
                .insert((parsed_chirho.tag_chirho, parsed_chirho.module_chirho));
        }
    }
    let mut readable_chirho = String::with_capacity(text_chirho.len());
    let mut copied_to_chirho = 0;
    for (start_chirho, parsed_chirho) in keys_in_text_chirho(text_chirho) {
        readable_chirho.push_str(&text_chirho[copied_to_chirho..start_chirho]);
        let Some(parsed_chirho) = parsed_chirho else {
            readable_chirho.push_str(PROMOTED_KEY_PREFIX_CHIRHO);
            copied_to_chirho = start_chirho + PROMOTED_KEY_PREFIX_CHIRHO.len();
            continue;
        };
        let shared_chirho = identities_chirho
            .get(parsed_chirho.name_chirho)
            .is_some_and(|keys_chirho| keys_chirho.len() > 1);
        readable_chirho.push('\'');
        if !parsed_chirho.module_chirho.is_empty()
            && (shared_chirho || parsed_chirho.tag_chirho == UNKNOWN_TAG_CHIRHO)
        {
            readable_chirho.push_str(parsed_chirho.module_chirho);
            readable_chirho.push('.');
        }
        readable_chirho.push_str(parsed_chirho.name_chirho);
        copied_to_chirho = start_chirho + parsed_chirho.length_chirho;
    }
    readable_chirho.push_str(&text_chirho[copied_to_chirho..]);
    readable_chirho
}

/// One key found in a text: its tag, module and constructor, and its length in bytes.
struct ParsedKeyChirho<'text_chirho> {
    tag_chirho: &'text_chirho str,
    module_chirho: &'text_chirho str,
    name_chirho: &'text_chirho str,
    length_chirho: usize,
}

/// Every occurrence of the key prefix in `text_chirho`, in order, with its parse; `None`
/// where the prefix starts no whole key.
fn keys_in_text_chirho(text_chirho: &str) -> Vec<(usize, Option<ParsedKeyChirho<'_>>)> {
    let mut keys_chirho = Vec::new();
    let mut from_chirho = 0;
    while let Some(offset_chirho) = text_chirho[from_chirho..].find(PROMOTED_KEY_PREFIX_CHIRHO) {
        let start_chirho = from_chirho + offset_chirho;
        let parsed_chirho = parse_key_chirho(&text_chirho[start_chirho..]);
        from_chirho = start_chirho
            + parsed_chirho
                .as_ref()
                .map_or(PROMOTED_KEY_PREFIX_CHIRHO.len(), |key_chirho| {
                    key_chirho.length_chirho
                });
        keys_chirho.push((start_chirho, parsed_chirho));
    }
    keys_chirho
}

/// The key at the start of `key_chirho`. The module is length-delimited; the constructor
/// runs to the end of its identifier, operator or bracketed spelling.
fn parse_key_chirho(key_chirho: &str) -> Option<ParsedKeyChirho<'_>> {
    let body_chirho = key_chirho.strip_prefix(PROMOTED_KEY_PREFIX_CHIRHO)?;
    let (tag_chirho, after_tag_chirho) = body_chirho.split_once(':')?;
    let (length_chirho, after_length_chirho) = after_tag_chirho.split_once(':')?;
    let module_length_chirho: usize = length_chirho.parse().ok()?;
    let module_chirho = after_length_chirho.get(..module_length_chirho)?;
    let name_start_chirho = &after_length_chirho[module_length_chirho..];
    let name_length_chirho = constructor_spelling_length_chirho(name_start_chirho)?;
    Some(ParsedKeyChirho {
        tag_chirho,
        module_chirho,
        name_chirho: &name_start_chirho[..name_length_chirho],
        length_chirho: key_chirho.len() - name_start_chirho.len() + name_length_chirho,
    })
}

/// The byte length of the constructor spelling `text_chirho` starts with, if any.
fn constructor_spelling_length_chirho(text_chirho: &str) -> Option<usize> {
    let first_chirho = text_chirho.chars().next()?;
    let length_chirho = match first_chirho {
        '(' => text_chirho.find(')')? + 1,
        '[' => text_chirho.find(']')? + 1,
        letter_chirho if letter_chirho.is_alphanumeric() || letter_chirho == '_' => text_chirho
            .find(|char_chirho: char| {
                !(char_chirho.is_alphanumeric() || char_chirho == '_' || char_chirho == '\'')
            })
            .unwrap_or(text_chirho.len()),
        _ => text_chirho
            .find(|char_chirho: char| !"!#$%&*+./<=>?@\\^|-~:".contains(char_chirho))
            .unwrap_or(text_chirho.len()),
    };
    (length_chirho > 0).then_some(length_chirho)
}

#[cfg(test)]
#[path = "tests_chirho.rs"]
mod tests_chirho;
