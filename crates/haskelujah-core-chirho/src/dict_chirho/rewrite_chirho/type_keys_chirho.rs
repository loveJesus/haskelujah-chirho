// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The type key an expression or binder is known at (the RC5 seam). `infer_type_key_chirho`
//! moved here from `dict_chirho/mod.rs`, beside its three callers.

use std::collections::HashMap;

use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
    /// Attempt to infer the type key (e.g. "Int", "Color") from a Core
    /// expression.  This is used to select the correct instance dictionary
    /// when a class method is applied to concrete arguments.
    pub(super) fn infer_type_key_chirho(&self, expr_chirho: &CoreExprChirho) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => match lit_chirho {
                crate::expr_chirho::CoreLitChirho::IntChirho(_) => Some("Int".to_string()),
                crate::expr_chirho::CoreLitChirho::CharChirho(_) => Some("Char".to_string()),
                crate::expr_chirho::CoreLitChirho::StringChirho(_) => Some("[Char]".to_string()),
                crate::expr_chirho::CoreLitChirho::FloatChirho(_) => Some("Double".to_string()),
            },
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => {
                // Look up constructor → type mapping
                if let Some(type_name_chirho) = self.con_types_chirho.get(con_name_chirho) {
                    Some(type_name_chirho.clone())
                } else {
                    // Built-in constructors
                    match con_name_chirho.as_str() {
                        "True" | "False" => Some("Bool".to_string()),
                        "LT" | "EQ" | "GT" => Some("Ordering".to_string()),
                        "()" | "$tuple0" => Some("()".to_string()),
                        "Identity" => Some("Identity".to_string()),
                        "Proxy" => Some("Proxy".to_string()),
                        "Const" => Some("Const".to_string()),
                        "Sum" => Some("Sum".to_string()),
                        "Product" => Some("Product".to_string()),
                        "All" => Some("All".to_string()),
                        "Any" => Some("Any".to_string()),
                        "Min" => Some("Min".to_string()),
                        "Max" => Some("Max".to_string()),
                        "First" => Some("First".to_string()),
                        "Last" => Some("Last".to_string()),
                        "Down" => Some("Down".to_string()),
                        "Endo" => Some("Endo".to_string()),
                        "Nothing" => Some("Maybe Int".to_string()),
                        "Left" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                let inner_str_chirho = inner_key_chirho.as_deref().unwrap_or("Int");
                                return Some(format!("Either {} Int", inner_str_chirho));
                            }
                            Some("Either Int Int".to_string())
                        }
                        "Right" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                let inner_str_chirho = inner_key_chirho.as_deref().unwrap_or("Int");
                                return Some(format!("Either Int {}", inner_str_chirho));
                            }
                            Some("Either Int Int".to_string())
                        }
                        "Just" => {
                            if let Some(inner_chirho) = args_chirho.first() {
                                let inner_key_chirho = self.infer_type_key_chirho(inner_chirho);
                                match inner_key_chirho.as_deref() {
                                    Some("Int") => return Some("Maybe Int".to_string()),
                                    Some("[Char]") => return Some("Maybe String".to_string()),
                                    Some("Double") => return Some("Maybe Double".to_string()),
                                    _ => return Some("Maybe Int".to_string()),
                                }
                            }
                            Some("Maybe Int".to_string())
                        }
                        name_chirho
                            if name_chirho == "(,)"
                                || name_chirho.starts_with("$tuple")
                                || name_chirho == "(,,)"
                                || name_chirho == "(,,,)" =>
                        {
                            if !args_chirho.is_empty() {
                                let keys_chirho: Vec<String> = args_chirho
                                    .iter()
                                    .map(|a_chirho| {
                                        let k_chirho = self
                                            .infer_type_key_chirho(a_chirho)
                                            .unwrap_or("Int".to_string());
                                        if k_chirho == "[Char]" {
                                            "String".to_string()
                                        } else {
                                            k_chirho
                                        }
                                    })
                                    .collect();
                                return Some(format!("({})", keys_chirho.join(",")));
                            }
                            Some("(Int,Int)".to_string())
                        }
                        "[]" => {
                            // Empty list — default to [Int]
                            Some("[Int]".to_string())
                        }
                        ":" => {
                            // Cons cell — infer element type from head
                            if let Some(head_chirho) = args_chirho.first() {
                                let elem_ty_chirho = self.infer_type_key_chirho(head_chirho);
                                match elem_ty_chirho.as_deref() {
                                    Some("Int") => Some("[Int]".to_string()),
                                    Some("[Char]") => Some("[[Char]]".to_string()),
                                    Some("Char") => Some("[Char]".to_string()),
                                    Some("()") => Some("[()]".to_string()),
                                    Some("Double") => Some("[Double]".to_string()),
                                    Some("Bool") => Some("[Bool]".to_string()),
                                    Some("Ordering") => Some("[Ordering]".to_string()),
                                    Some("Sum") => Some("[Sum]".to_string()),
                                    Some("Product") => Some("[Product]".to_string()),
                                    Some("All") => Some("[All]".to_string()),
                                    Some("Any") => Some("[Any]".to_string()),
                                    Some("Min") => Some("[Min]".to_string()),
                                    Some("Max") => Some("[Max]".to_string()),
                                    Some("First") => Some("[First]".to_string()),
                                    Some("Last") => Some("[Last]".to_string()),
                                    Some("Down") => Some("[Down]".to_string()),
                                    Some("Endo") => Some("[Endo]".to_string()),
                                    _ => Some("[Int]".to_string()), // default
                                }
                            } else {
                                Some("[Int]".to_string())
                            }
                        }
                        _ => None,
                    }
                }
            }
            CoreExprChirho::VarChirho(id_chirho) => {
                // Check if the var name is a known constructor
                if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                    if let Some(type_name_chirho) = self.con_types_chirho.get(name_chirho) {
                        return Some(type_name_chirho.clone());
                    }
                    // Built-in constructors
                    match name_chirho.as_str() {
                        "True" | "False" => return Some("Bool".to_string()),
                        "Nothing" => return Some("Maybe Int".to_string()),
                        "LT" | "EQ" | "GT" => return Some("Ordering".to_string()),
                        "()" | "$tuple0" => return Some("()".to_string()),
                        "Identity" => return Some("Identity".to_string()),
                        "Proxy" => return Some("Proxy".to_string()),
                        "Const" => return Some("Const".to_string()),
                        "Sum" => return Some("Sum".to_string()),
                        "Product" => return Some("Product".to_string()),
                        "All" => return Some("All".to_string()),
                        "Any" => return Some("Any".to_string()),
                        "Min" => return Some("Min".to_string()),
                        "Max" => return Some("Max".to_string()),
                        "First" => return Some("First".to_string()),
                        "Last" => return Some("Last".to_string()),
                        "Down" => return Some("Down".to_string()),
                        "Endo" => return Some("Endo".to_string()),
                        _ => {}
                    }
                }
                None
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                // Infer result type of known primops
                match name_chirho.as_str() {
                    "enumFromTo#" => Some("[Int]".to_string()),
                    // The desugarer's blind minus keeps its operand's type, so `(-1) / 2 ::
                    // Double` keys `/` at Double through the literal's record (row 547).
                    "negate#" => Some(
                        args_chirho
                            .first()
                            .and_then(|operand_chirho| self.infer_type_key_chirho(operand_chirho))
                            .unwrap_or_else(|| "Int".to_string()),
                    ),
                    "+#" | "-#" | "*#" | "div#" | "mod#" | "readInt#" => Some("Int".to_string()),
                    // `x ^ n` has its base's type; an unknown base stays unknown (row 570).
                    "^#" => args_chirho
                        .first()
                        .and_then(|base_chirho| self.infer_type_key_chirho(base_chirho)),
                    "chr#" => Some("Char".to_string()),
                    "ord#" => Some("Int".to_string()),
                    "+.#" | "-.#" | "*.#" | "/.#" | "^.#" | "negateFloat#" | "recip#"
                    | "readFloat#" => Some("Double".to_string()),
                    "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" | "not#" | "eqFloat#" | "<.#"
                    | "<=.#" | ">.#" | ">=.#" | "readBool#" => Some("Bool".to_string()),
                    "compare#" | "compareChar#" | "compareFloat#" | "compareStr#" => {
                        Some("Ordering".to_string())
                    }
                    "showInt#" | "showFloat#" | "showStr#" | "showList#" | "showMaybe#"
                    | "showTuple2#" | "showEither#" | "showOrdering#" | "showBool#" | "++#" => {
                        Some("[Char]".to_string())
                    }
                    _ => None,
                }
            }
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                // For App(Var(fromInteger), Lit(n)), the result type is the one
                // the checker solved for this literal occurrence (`1 / 2 :: Double`,
                // a signature, or defaulting give Double); with no evidence, Int.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if let CoreExprChirho::VarChirho(id_chirho) = fun_chirho.as_ref() {
                    if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                        if name_chirho == "fromInteger" {
                            return Some(
                                self.occurrence_evidence_chirho
                                    .get(id_chirho)
                                    .filter(|(class_chirho, _)| class_chirho == "Num")
                                    .map_or_else(
                                        || "Int".to_string(),
                                        |(_, key_chirho)| key_chirho.clone(),
                                    ),
                            );
                        }
                        // Well-known Prelude function return types:
                        // These functions have fixed return types regardless
                        // of their argument types, so we can infer the result
                        // type for dict selection without full type inference.
                        match name_chirho.as_str() {
                            // :: [a] -> Int
                            "length" | "sum" | "product" => {
                                return Some("Int".to_string());
                            }
                            // :: [a] -> [a]  (element type from arg)
                            "take" | "drop" | "reverse" | "sort" | "init" | "tail" | "nub"
                            | "cycle" => {
                                if let Some(arg_ty_chirho) = self.infer_type_key_chirho(arg_chirho)
                                {
                                    return Some(arg_ty_chirho);
                                }
                                return None;
                            }
                            // :: [a] -> a  (element type from list arg)
                            "head" | "last" | "minimum" | "maximum" => {
                                if let Some(arg_ty_chirho) = self.infer_type_key_chirho(arg_chirho)
                                {
                                    // Strip outer list: [Int] → Int
                                    if arg_ty_chirho.starts_with('[')
                                        && arg_ty_chirho.ends_with(']')
                                    {
                                        return Some(
                                            arg_ty_chirho[1..arg_ty_chirho.len() - 1].to_string(),
                                        );
                                    }
                                    return Some(arg_ty_chirho);
                                }
                                return None;
                            }
                            // :: a -> Bool
                            "null" | "even" | "odd" | "elem" | "notElem" => {
                                return Some("Bool".to_string());
                            }
                            // :: a -> Int
                            "fromEnum" | "ord" => {
                                return Some("Int".to_string());
                            }
                            // :: Int -> Char
                            "chr" => {
                                return Some("Char".to_string());
                            }
                            // :: Int -> a (for toEnum, we don't know the
                            // result type without context, so skip)
                            _ => {}
                        }
                        // Constructor applications: App(Just, x) → Maybe <x-type>
                        match name_chirho.as_str() {
                            "Identity" | "Const" | "Sum" | "Product" | "All" | "Any" | "Min"
                            | "Max" | "First" | "Last" | "Down" | "Endo" => {
                                return Some(name_chirho.clone());
                            }
                            "Just" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let inner_key_chirho = if inner_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &inner_chirho
                                };
                                return Some(format!("Maybe {}", inner_key_chirho));
                            }
                            "Left" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                return Some(format!("Either {} Int", inner_chirho));
                            }
                            "Right" => {
                                let inner_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                return Some(format!("Either Int {}", inner_chirho));
                            }
                            _ => {}
                        }
                        // Check if the name is a known constructor and return its type
                        if let Some(type_name_chirho) = self.con_types_chirho.get(name_chirho) {
                            return Some(type_name_chirho.clone());
                        }
                    }
                }
                // Detect App(App(Var((,)), a), b) → tuple type
                // Detect App(App(Var(compare), a), b) → Ordering
                // Detect App(App(Var(Left/Right), a), b) → Either type
                if let CoreExprChirho::AppChirho {
                    fun_chirho: inner_fun_chirho,
                    arg_chirho: first_arg_chirho,
                } = fun_chirho.as_ref()
                {
                    if let CoreExprChirho::VarChirho(id_chirho) = inner_fun_chirho.as_ref() {
                        if let Some(name_chirho) = self.names_chirho.get(id_chirho) {
                            if name_chirho == "(,)" || name_chirho == "$tuple2" {
                                let a_chirho = self
                                    .infer_type_key_chirho(first_arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let b_chirho = self
                                    .infer_type_key_chirho(arg_chirho)
                                    .unwrap_or_else(|| "Int".to_string());
                                let a_key_chirho = if a_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &a_chirho
                                };
                                let b_key_chirho = if b_chirho == "[Char]" {
                                    "String"
                                } else {
                                    &b_chirho
                                };
                                return Some(format!("({},{})", a_key_chirho, b_key_chirho));
                            }
                            // compare :: a -> a -> Ordering
                            if name_chirho == "compare"
                                || name_chirho.starts_with("$prim_Ord_compare")
                            {
                                return Some("Ordering".to_string());
                            }
                            // Two-arg Prelude functions:
                            // take/drop :: Int -> [a] -> [a]
                            // zip :: [a] -> [b] -> [(a,b)]
                            // map :: (a -> b) -> [a] -> [b]
                            // filter :: (a -> Bool) -> [a] -> [a]
                            match name_chirho.as_str() {
                                "take" | "drop" | "filter" | "takeWhile" | "dropWhile" => {
                                    // Result type = list arg type
                                    if let Some(ty_chirho) = self.infer_type_key_chirho(arg_chirho)
                                    {
                                        return Some(ty_chirho);
                                    }
                                }
                                "zip" | "zipWith" | "map" | "concatMap" => {
                                    // Complex return types; skip for now
                                }
                                "elem" | "notElem" | "any" | "all" => {
                                    return Some("Bool".to_string());
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // If the function is a PrimOp with a known return type (e.g. readInt#
                // returns Int, not [Char]), prefer the function's type over the argument's.
                // This prevents `show (readInt# "10")` from dispatching as ShowStr.
                let fun_ty_chirho = self.infer_type_key_chirho(fun_chirho);
                if let Some(ref fty_chirho) = fun_ty_chirho {
                    // readInt# / readFloat# / readBool# have unambiguous return types —
                    // always use them.  Other primops (like ++#) return [Char] which
                    // already matched in the PrimOpChirho arm above, so we won't reach
                    // here for those.  General function types are inferred below.
                    let is_typed_primop_chirho =
                        matches!(fun_chirho.as_ref(), CoreExprChirho::PrimOpChirho { .. })
                            && !fty_chirho.is_empty();
                    if is_typed_primop_chirho {
                        return Some(fty_chirho.clone());
                    }
                }
                // For general App chains like `f 1.5 2.5`, try to infer
                // from the argument first, but ONLY when the outermost
                // function is a literal, primop, or non-list-typed expression.
                // Propagating the arg's list type (e.g. [Int]) for a user
                // function like `myLen []` would be wrong: myLen returns Int
                // but its argument is [Int].  Guard against this: only
                // propagate a list-type inference if the function itself
                // also returns a list type.
                if let Some(tk_chirho) = self.infer_type_key_chirho(arg_chirho) {
                    let is_list_type_chirho = tk_chirho.starts_with('[');
                    // Only propagate list arg types if the function also
                    // returns a list (fun_ty is Some list) or is unknown.
                    // For user-defined functions (fun_ty = None), list types
                    // should NOT be propagated as the result type.
                    if is_list_type_chirho && fun_ty_chirho.is_none() {
                        // Don't propagate: user function likely returns a
                        // scalar, not a list.
                        return None;
                    }
                    return Some(tk_chirho);
                }
                fun_ty_chirho
            }
            _ => None,
        }
    }

    pub(in crate::dict_chirho) fn binder_type_key_chirho(
        &self,
        binder_chirho: &BinderChirho,
    ) -> Option<String> {
        match &binder_chirho.ty_chirho {
            TyChirho::FunChirho(_, _, _) => None,
            TyChirho::ForallChirho { body_chirho, .. } => Some(format!("{}", body_chirho)),
            _ => Some(format!("{}", binder_chirho.ty_chirho)),
        }
    }

    fn key_token_is_type_var_chirho(token_chirho: &str) -> bool {
        token_chirho.strip_prefix('t').is_some_and(|rest_chirho| {
            !rest_chirho.is_empty()
                && rest_chirho
                    .chars()
                    .all(|c_chirho| c_chirho.is_ascii_digit())
        })
    }

    fn type_key_has_type_var_chirho(type_key_chirho: &str) -> bool {
        type_key_chirho
            .split(|c_chirho: char| !(c_chirho.is_ascii_alphanumeric() || c_chirho == '_'))
            .any(Self::key_token_is_type_var_chirho)
    }

    pub(in crate::dict_chirho) fn concrete_local_type_key_chirho(
        type_key_chirho: String,
    ) -> Option<String> {
        if type_key_chirho.is_empty() || Self::type_key_has_type_var_chirho(&type_key_chirho) {
            None
        } else {
            Some(type_key_chirho)
        }
    }

    pub(super) fn concrete_binder_type_key_chirho(
        &self,
        binder_chirho: &BinderChirho,
    ) -> Option<String> {
        self.binder_type_key_chirho(binder_chirho)
            .and_then(Self::concrete_local_type_key_chirho)
    }

    pub(super) fn infer_local_value_binding_type_key_chirho(
        &self,
        rhs_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        self.infer_type_key_for_rewrite_chirho(rhs_chirho, local_type_keys_chirho)
            .or_else(|| {
                self.expr_contains_numeric_default_marker_chirho(rhs_chirho)
                    .then(|| "Int".to_string())
            })
            .and_then(Self::concrete_local_type_key_chirho)
    }

    pub(super) fn extend_alt_type_keys_chirho(
        &self,
        scrutinee_type_key_chirho: Option<String>,
        bind_chirho: &BinderChirho,
        alt_chirho: &CoreAltChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> HashMap<CoreIdChirho, String> {
        let mut alt_type_keys_chirho = local_type_keys_chirho.clone();
        if let Some(type_key_chirho) = scrutinee_type_key_chirho.clone() {
            alt_type_keys_chirho.insert(bind_chirho.id_chirho, type_key_chirho);
        }
        match (&alt_chirho.con_chirho, alt_chirho.binders_chirho.as_slice()) {
            (
                AltConChirho::DataConChirho(con_name_chirho),
                [head_binder_chirho, tail_binder_chirho],
            ) if con_name_chirho == ":" => {
                if let Some(scrutinee_list_key_chirho) = scrutinee_type_key_chirho {
                    if scrutinee_list_key_chirho.starts_with('[')
                        && scrutinee_list_key_chirho.ends_with(']')
                    {
                        let elem_type_key_chirho = scrutinee_list_key_chirho
                            [1..scrutinee_list_key_chirho.len() - 1]
                            .to_string();
                        alt_type_keys_chirho
                            .insert(head_binder_chirho.id_chirho, elem_type_key_chirho);
                        alt_type_keys_chirho
                            .insert(tail_binder_chirho.id_chirho, scrutinee_list_key_chirho);
                    }
                }
            }
            (AltConChirho::DataConChirho(con_name_chirho), binders_chirho)
                if Self::is_tuple_constructor_name_chirho(con_name_chirho) =>
            {
                if let Some(scrutinee_tuple_key_chirho) = scrutinee_type_key_chirho {
                    if let Some(elem_type_keys_chirho) =
                        Self::tuple_payload_type_keys_chirho(&scrutinee_tuple_key_chirho)
                    {
                        if elem_type_keys_chirho.len() == binders_chirho.len() {
                            for (binder_chirho, elem_type_key_chirho) in
                                binders_chirho.iter().zip(elem_type_keys_chirho.into_iter())
                            {
                                alt_type_keys_chirho
                                    .insert(binder_chirho.id_chirho, elem_type_key_chirho);
                            }
                        }
                    }
                }
            }
            (AltConChirho::DataConChirho(con_name_chirho), [inner_binder_chirho])
                if con_name_chirho == "Just" =>
            {
                if let Some(scrutinee_maybe_key_chirho) = scrutinee_type_key_chirho {
                    if let Some(inner_type_key_chirho) =
                        scrutinee_maybe_key_chirho.strip_prefix("Maybe ")
                    {
                        alt_type_keys_chirho.insert(
                            inner_binder_chirho.id_chirho,
                            inner_type_key_chirho.to_string(),
                        );
                    }
                }
            }
            _ => {
                for binder_chirho in &alt_chirho.binders_chirho {
                    if let Some(type_key_chirho) = self.binder_type_key_chirho(binder_chirho) {
                        alt_type_keys_chirho.insert(binder_chirho.id_chirho, type_key_chirho);
                    }
                }
            }
        }
        alt_type_keys_chirho
    }

    pub(in crate::dict_chirho) fn infer_type_key_for_rewrite_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            // A type application carries the type we are trying to recover, so USE it when
            // it is concrete. Previously this arm discarded `ty_chirho` and recursed into
            // the inner expression — which meant an explicit annotation was ignored:
            //   show (id [1,2,3] :: [Int])   rendered as a heap address
            // because the inner expression is an application, has no binder to read a type
            // off, and the `Show` fallback is `showInt#`. The sibling resolver
            // `infer_strict_dispatch_key_for_rewrite_chirho` already reads `ty_chirho`;
            // these two had simply diverged.
            // See spec-chirho/bug-native-print-list-pointer-chirho.md
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
                ..
            } => Self::concrete_local_type_key_chirho(format!("{ty_chirho}")).or_else(|| {
                self.infer_type_key_for_rewrite_chirho(inner_chirho, local_type_keys_chirho)
            }),
            // `x ^ n` has its base's type, read with this traversal's keys, so a negated
            // power of a top-level Double negates a Double (row 570).
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } if name_chirho == "^#" => args_chirho.first().and_then(|base_chirho| {
                self.infer_type_key_for_rewrite_chirho(base_chirho, local_type_keys_chirho)
            }),
            _ => self.infer_type_key_chirho(expr_chirho),
        }
    }

    pub(super) fn infer_strict_dispatch_key_for_rewrite_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        match expr_chirho {
            CoreExprChirho::TyAppChirho { ty_chirho, .. } => Some(format!("{}", ty_chirho)),
            CoreExprChirho::VarChirho(id_chirho) => local_type_keys_chirho
                .get(id_chirho)
                .cloned()
                .or_else(|| self.infer_type_key_chirho(expr_chirho)),
            CoreExprChirho::ConAppChirho { .. }
            | CoreExprChirho::LitChirho(_)
            | CoreExprChirho::PrimOpChirho { .. } => self.infer_type_key_chirho(expr_chirho),
            CoreExprChirho::AppChirho { .. } => {
                let mut cur_chirho = expr_chirho;
                while let CoreExprChirho::AppChirho { fun_chirho, .. } = cur_chirho {
                    cur_chirho = fun_chirho;
                }
                while let CoreExprChirho::TyAppChirho {
                    expr_chirho: inner_chirho,
                    ..
                } = cur_chirho
                {
                    cur_chirho = inner_chirho;
                }
                let CoreExprChirho::VarChirho(head_id_chirho) = cur_chirho else {
                    return None;
                };
                let name_chirho = self.names_chirho.get(head_id_chirho)?;
                let is_con_head_chirho = self.con_types_chirho.contains_key(name_chirho)
                    || matches!(
                        name_chirho.as_str(),
                        "Just"
                            | "Left"
                            | "Right"
                            | "Identity"
                            | "Proxy"
                            | "Const"
                            | "Sum"
                            | "Product"
                            | "All"
                            | "Any"
                            | "Min"
                            | "Max"
                            | "First"
                            | "Last"
                            | "Down"
                            | "Endo"
                            | ":"
                            | "(,)"
                            | "(,,)"
                            | "(,,,)"
                    )
                    || name_chirho.starts_with("$tuple");
                if is_con_head_chirho {
                    return self.infer_type_key_chirho(expr_chirho);
                }
                if self
                    .dict_param_bindings_chirho
                    .contains_key(&self.canonical_id_chirho(*head_id_chirho))
                {
                    let mut args_chirho = Vec::new();
                    let mut app_chirho = expr_chirho;
                    while let CoreExprChirho::AppChirho {
                        fun_chirho,
                        arg_chirho,
                    } = app_chirho
                    {
                        args_chirho.push(arg_chirho.as_ref());
                        app_chirho = fun_chirho.as_ref();
                    }
                    args_chirho.reverse();
                    return self.infer_dict_param_call_type_key_chirho(
                        &args_chirho,
                        local_type_keys_chirho,
                    );
                }
                None
            }
            _ => None,
        }
    }

    pub(super) fn infer_dict_param_call_type_key_chirho(
        &self,
        args_chirho: &[&CoreExprChirho],
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
    ) -> Option<String> {
        args_chirho.iter().find_map(|a_chirho| {
            self.infer_strict_dispatch_key_for_rewrite_chirho(a_chirho, local_type_keys_chirho)
                .or_else(|| {
                    self.expr_contains_numeric_default_marker_chirho(a_chirho)
                        .then(|| "Int".to_string())
                })
        })
    }

    fn is_tuple_constructor_name_chirho(name_chirho: &str) -> bool {
        name_chirho == "(,)"
            || name_chirho == "(,,)"
            || name_chirho == "(,,,)"
            || name_chirho.starts_with("$tuple")
    }

    fn tuple_payload_type_keys_chirho(value_key_chirho: &str) -> Option<Vec<String>> {
        let trimmed_chirho = value_key_chirho.trim();
        if !trimmed_chirho.starts_with('(')
            || !trimmed_chirho.ends_with(')')
            || trimmed_chirho.len() <= 2
        {
            return None;
        }

        let inner_chirho = &trimmed_chirho[1..trimmed_chirho.len() - 1];
        let mut parts_chirho = Vec::new();
        let mut depth_chirho = 0i32;
        let mut start_chirho = 0usize;
        let mut saw_comma_chirho = false;
        for (idx_chirho, ch_chirho) in inner_chirho.char_indices() {
            match ch_chirho {
                '(' | '[' => depth_chirho += 1,
                ')' | ']' => depth_chirho -= 1,
                ',' if depth_chirho == 0 => {
                    saw_comma_chirho = true;
                    parts_chirho.push(inner_chirho[start_chirho..idx_chirho].trim().to_string());
                    start_chirho = idx_chirho + ch_chirho.len_utf8();
                }
                _ => {}
            }
        }
        if !saw_comma_chirho {
            return None;
        }

        parts_chirho.push(inner_chirho[start_chirho..].trim().to_string());
        if parts_chirho
            .iter()
            .any(|part_chirho| part_chirho.is_empty())
        {
            return None;
        }
        Some(parts_chirho)
    }
}
