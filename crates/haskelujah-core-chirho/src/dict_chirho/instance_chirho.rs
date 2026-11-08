// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Instance dictionary generation: builtin prim bindings, ground instance dicts,
//! GND dicts, conditional ground dicts, and list dictionary helpers.

#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    pub fn generate_builtin_prim_bindings_chirho(&mut self) {
        let builtins_chirho = Self::builtin_rows_chirho();

        let primop_for_chirho =
            |class_chirho: &str, method_chirho: &str, type_key_chirho: &str| -> &str {
                match (class_chirho, method_chirho, type_key_chirho) {
                    ("Eq", "==", "[Char]") => "eqStr#",
                    ("Eq", "==", "Double") => "eqFloat#",
                    ("Eq", "==", _) => "==#",
                    ("Ord", "compare", "Char") => "compareChar#",
                    ("Ord", "compare", "Double") => "compareFloat#",
                    ("Ord", "compare", "[Char]") => "compareStr#",
                    ("Ord", "compare", _) => "compare#",
                    ("Num", "+", "Double") => "+.#",
                    ("Num", "+", _) => "+#",
                    ("Num", "*", "Double") => "*.#",
                    ("Num", "*", _) => "*#",
                    ("Num", "-", "Double") => "-.#",
                    ("Num", "-", _) => "-#",
                    ("Num", "negate", "Double") => "negateFloat#",
                    ("Num", "negate", _) => "negate#",
                    ("Fractional", "/", _) => "/.#",
                    ("Fractional", "recip", _) => "recip#",
                    ("Show", "show", "[Char]") => "showStr#",
                    ("Show", "show", "[Int]") => "showList#",
                    ("Show", "show", "Char") => "showChar#",
                    ("Show", "show", "Double") => "showFloat#",
                    ("Show", "show", "Maybe Int") => "showMaybe#",
                    ("Show", "show", "Maybe String") => "showMaybe#",
                    ("Show", "show", "Maybe Double") => "showMaybe#",
                    ("Show", "show", "(Int,Int)") => "showTuple2#",
                    ("Show", "show", "(Int,String)") => "showTuple2#",
                    ("Show", "show", "(String,Int)") => "showTuple2#",
                    ("Show", "show", "(String,String)") => "showTuple2#",
                    ("Show", "show", "(Int,Bool)") => "showTuple2#",
                    ("Show", "show", "(Bool,Int)") => "showTuple2#",
                    ("Show", "show", "(Int,Double)") => "showTuple2#",
                    ("Show", "show", "(Double,Int)") => "showTuple2#",
                    ("Show", "show", "(Bool,Bool)") => "showTuple2#",
                    ("Show", "show", "(Double,Double)") => "showTuple2#",
                    ("Show", "show", "(Bool,String)") => "showTuple2#",
                    ("Show", "show", "(String,Bool)") => "showTuple2#",
                    ("Show", "show", "Either Int Int") => "showEither#",
                    ("Show", "show", "Either String Int") => "showEither#",
                    ("Show", "show", "Either Int String") => "showEither#",
                    ("Show", "show", "Either String String") => "showEither#",
                    ("Show", "show", "Ordering") => "showOrdering#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with('(') => "showTuple2#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with("Maybe") => "showMaybe#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with("Either") => "showEither#",
                    ("Show", "show", tk_chirho) if tk_chirho.starts_with('[') => "showList#",
                    ("Show", "show", "Bool") => "showBool#",
                    ("Show", "show", _) => "showInt#",
                    ("Read", "read", "Int") => "readInt#",
                    ("Read", "read", "Double") => "readFloat#",
                    ("Read", "read", "Bool") => "readBool#",
                    // IsList — identity for [a]
                    ("IsList", "fromList", _) => "id#",
                    ("IsList", "toList", _) => "id#",
                    _ => "+#", // fallback
                }
            };

        for (identity_chirho, method_chirho, type_key_chirho, kind_chirho) in builtins_chirho {
            let class_chirho = identity_chirho.legacy_name_chirho();
            // Named in its class's namespace: the legacy spelling, or the reserved one when a
            // source class has taken that spelling. A displaced class this pass could not lay
            // out has none, and the spelling is the source class's (row 692, C10).
            let Some(namespace_chirho) = self.compiler_rows_class_name_chirho(identity_chirho)
            else {
                continue;
            };
            let prim_name_chirho =
                format!("$prim_{namespace_chirho}_{method_chirho}_{type_key_chirho}");

            // Skip if this binding already exists (user may have provided one)
            let already_exists_chirho = self
                .names_chirho
                .values()
                .any(|n_chirho| n_chirho == &prim_name_chirho);
            if already_exists_chirho {
                continue;
            }

            // Special case: Num fromInteger at Double → the Int-to-Double conversion in this
            // row's namespace, never an identity over an Int's payload (row 692, C10, #33179).
            if identity_chirho == CompilerClassChirho::NumChirho
                && method_chirho == "fromInteger"
                && type_key_chirho == "Double"
            {
                self.generate_from_integer_double_row_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Show [Int] → generate a recursive Core function
            // that pattern-matches the list and uses showInt#/++# to build the
            // string. This avoids the showList# primop which can't force thunks.
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "[Int]" {
                self.generate_show_key_binding_chirho(&prim_name_chirho, "[Int]");
                continue;
            }

            // Special case: Show Bool → case on True/False returning string
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "Bool" {
                self.generate_show_bool_binding_chirho(&prim_name_chirho);
                continue;
            }

            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "[Char]" {
                self.generate_show_string_binding_chirho(&prim_name_chirho);
                continue;
            }

            if class_chirho == "Show"
                && method_chirho == "show"
                && type_key_chirho.starts_with("Maybe ")
            {
                self.generate_show_maybe_binding_chirho(&prim_name_chirho, type_key_chirho);
                continue;
            }

            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho.starts_with('(')
            {
                self.generate_show_tuple_binding_chirho(&prim_name_chirho, type_key_chirho);
                continue;
            }

            if class_chirho == "Show"
                && method_chirho == "show"
                && type_key_chirho.starts_with("Either ")
            {
                self.generate_show_either_binding_chirho(&prim_name_chirho, type_key_chirho);
                continue;
            }

            // Special case: Show Ordering → case on LT/EQ/GT returning string
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "Ordering" {
                self.generate_show_ordering_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Eq Ordering → case dispatch comparing constructor tags
            if class_chirho == "Eq" && method_chirho == "==" && type_key_chirho == "Ordering" {
                self.generate_eq_ordering_binding_chirho(&prim_name_chirho);
                continue;
            }

            // Special case: Ord at a builtin enumeration (Bool, Ordering) → case
            // dispatch by constructor order. Their values are constructors,
            // which `compare#` cannot order: `compare False True` crashed and
            // `sortBy compare [True, False]` kept [True,False].
            if class_chirho == "Ord"
                && method_chirho == "compare"
                && let Some(constructors_chirho) =
                    constructor_orders_chirho::constructor_order_chirho(type_key_chirho)
            {
                self.generate_constructor_order_compare_chirho(
                    &prim_name_chirho,
                    constructors_chirho,
                );
                continue;
            }

            let int_ty_chirho = TyChirho::int_chirho();

            let rhs_chirho = match kind_chirho {
                1 => {
                    // Binary primop: \x y -> x op# y
                    let op_chirho = primop_for_chirho(class_chirho, method_chirho, type_key_chirho);
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    let y_chirho = self.fresh_binder_chirho("y", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: op_chirho.to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                                ],
                            }),
                        }),
                    }
                }
                2 => {
                    // Unary primop: \x -> op# x
                    let op_chirho = primop_for_chirho(class_chirho, method_chirho, type_key_chirho);
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: op_chirho.to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                        }),
                    }
                }
                _ => {
                    // Identity: \x -> x
                    let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }
                }
            };

            let binder_chirho = self.fresh_binder_chirho(&prim_name_chirho, int_ty_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate built-in Prelude function bindings that are not class
    /// methods but are essential for basic Haskell programs.
    ///
    /// Generate a recursive Core function for `show :: [Int] -> String`.
    ///
    /// Produces two top-level bindings:
    /// ```text
    /// $showListTail_Int = \xs -> case xs of
    ///     [] -> "]"
    ///     (:) x rest -> ++# "," (++# (showInt# x) ($showListTail_Int rest))
    ///
    /// $prim_Show_show_[Int] = \xs -> case xs of
    ///     [] -> "[]"
    ///     (:) x rest -> ++# "[" (++# (showInt# x) ($showListTail_Int rest))
    /// ```

    /// Generate `$prim_Show_show_Bool = \x -> showBool# x`
    fn generate_show_bool_binding_chirho(&mut self, prim_name_chirho: &str) {
        let bool_ty_chirho = TyChirho::bool_chirho();
        let str_ty_chirho = TyChirho::string_chirho();

        let x_chirho = self.fresh_binder_chirho("x", bool_ty_chirho);

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho.clone(),
            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                name_chirho: "showBool#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            }),
        };

        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, str_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// Generate `$prim_Show_show_Ordering = \x -> case x of LT -> "LT"; EQ -> "EQ"; GT -> "GT"`
    fn generate_show_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let str_ty_chirho = TyChirho::string_chirho();
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let scr_chirho = self.fresh_binder_chirho("_sord", ord_ty_chirho);

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "LT".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "EQ".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "GT".to_string(),
                    )),
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(body_chirho),
        };

        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, str_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// Generate `$prim_Eq_==_Ordering = \x y -> case x of { LT -> case y of { LT -> True; _ -> False }; EQ -> case y of { EQ -> True; _ -> False }; GT -> case y of { GT -> True; _ -> False } }`
    fn generate_eq_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let bool_ty_chirho = TyChirho::bool_chirho();
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_y1_chirho = self.fresh_binder_chirho("_sy1", ord_ty_chirho.clone());
        let scr_y2_chirho = self.fresh_binder_chirho("_sy2", ord_ty_chirho.clone());
        let scr_y3_chirho = self.fresh_binder_chirho("_sy3", ord_ty_chirho.clone());

        let true_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "True".to_string(),
            args_chirho: vec![],
        };
        let false_expr_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "False".to_string(),
            args_chirho: vec![],
        };

        // case y of { LT -> True; _ -> False }
        let inner_lt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y1_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        // case y of { EQ -> True; _ -> False }
        let inner_eq_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y2_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        // case y of { GT -> True; _ -> False }
        let inner_gt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y3_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: true_expr_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: false_expr_chirho.clone(),
                },
            ],
        };

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: bool_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_lt_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_eq_chirho,
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: inner_gt_chirho,
                },
            ],
        };

        let rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_chirho,
            body_chirho: Box::new(CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(body_chirho),
            }),
        };

        let fn_ty_chirho = TyChirho::fun_chirho(
            ord_ty_chirho.clone(),
            TyChirho::fun_chirho(ord_ty_chirho, bool_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    /// - `not = \x -> not# x`  (boolean negation)
    /// - `id = \x -> x`        (identity)
    /// - `const = \x y -> x`   (constant function)

    /// Generate instance dictionary bindings for ground instances.
    ///
    /// For each instance with no context (like `Eq Int`, `Num Int`),
    /// generates a top-level binding:
    /// ```text
    /// $fEqInt = $DictEq $prim_Eq_==_Int
    /// $fNumInt = $DictNum $prim_Num_+_Int ... (Num has no superclass slots)
    /// ```
    pub fn generate_instance_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        // Two-pass approach to avoid dict id mismatch from HashMap
        // non-deterministic iteration order.  When a class with superclass
        // deps (e.g. Num → Eq) is processed before its superclass's
        // instances, `resolve_or_fresh_id_chirho` would create a fresh id
        // for the super dict reference that doesn't match the actual
        // binding created later.
        //
        // Pass 1: pre-register binder ids for ALL instance dicts so that
        //         `resolve_or_fresh_id_chirho` will find them in pass 2.
        // Pass 2: build dict bodies that reference the already-registered ids.

        // Collect eligible (class, instance, type_key, layout) tuples.
        let mut eligible_chirho: Vec<(String, String, DictLayoutChirho)> = Vec::new();
        // A generic head (`generic_heads_chirho.rs`) keeps its typed key and
        // name, fills its slots from the rows as written, and is found under
        // its constructor through an alias private to this pass:
        // (class, typed key) → generic head.
        let mut generic_head_by_key_chirho: HashMap<
            (String, String),
            generic_heads_chirho::GenericHeadChirho,
        > = HashMap::new();
        // A multi-parameter instance by (class, key): its superclass fields and its registry
        // entry read its whole head (row 694, #31909).
        let mut multi_heads_chirho: HashMap<(String, String), std::sync::Arc<InstDeclChirho>> =
            HashMap::new();
        // Row 692, C10: (namespace, key) of each dictionary a displaced compiler class's own
        // instance fills, with that class's identity.
        let mut compiler_entries_chirho: HashMap<(String, String), CompilerClassChirho> =
            HashMap::new();

        for (class_name_chirho, instances_chirho) in
            class_env_chirho.instances_chirho().iter_chirho()
        {
            let spelled_layout_chirho = self.layouts_chirho.get(class_name_chirho).cloned();

            for inst_chirho in instances_chirho {
                // Only handle ground instances (no context) for now
                if !inst_chirho.context_chirho.is_empty() {
                    continue;
                }

                let type_key_chirho = generic_heads_chirho::head_key_chirho(
                    &inst_chirho.head_ty_chirho,
                    &inst_chirho.extra_head_tys_chirho,
                );
                // A displaced compiler class's own instances fill its namespace, and nothing
                // else does; its spelling keeps the source class (row 692, C10).
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if let Some(identity_chirho) =
                    self.displaced_identity_chirho(inst_chirho.class_identity_chirho())
                {
                    // Laid out or not, a displaced class's instance never enters the spelling.
                    if identity_chirho.legacy_name_chirho() == class_name_chirho
                        && let Some(compiler_layout_chirho) =
                            self.compiler_layouts_chirho.get(&identity_chirho)
                    {
                        let namespace_chirho = compiler_layout_chirho.class_name_chirho.clone();
                        let compiler_layout_chirho = compiler_layout_chirho.clone();
                        compiler_entries_chirho.insert(
                            (namespace_chirho.clone(), type_key_chirho.clone()),
                            identity_chirho,
                        );
                        eligible_chirho.push((
                            namespace_chirho,
                            type_key_chirho,
                            compiler_layout_chirho,
                        ));
                    }
                    continue;
                }
                let Some(layout_chirho) = spelled_layout_chirho.clone() else {
                    continue;
                };
                // A newtype-strategy instance takes its representation's dictionary,
                // built by `generate_gnd_dicts_chirho` (row 694).
                if self.takes_representation_dictionary_chirho(class_name_chirho, &type_key_chirho)
                {
                    continue;
                }
                if let Some(generic_head_chirho) = self.generic_head_chirho(
                    class_env_chirho,
                    class_name_chirho,
                    inst_chirho,
                    &layout_chirho,
                    &type_key_chirho,
                ) {
                    generic_head_by_key_chirho.insert(
                        (class_name_chirho.clone(), type_key_chirho.clone()),
                        generic_head_chirho,
                    );
                }
                if !inst_chirho.extra_head_tys_chirho.is_empty() {
                    multi_heads_chirho.insert(
                        (class_name_chirho.clone(), type_key_chirho.clone()),
                        inst_chirho.clone(),
                    );
                }
                eligible_chirho.push((
                    class_name_chirho.clone(),
                    type_key_chirho,
                    layout_chirho.clone(),
                ));
            }
        }

        // Inject synthetic ground instances for compound Show types.
        // These are monomorphised instances for Show (Maybe Int), etc.
        // They are compiler Show instances: a source class displacing Show leaves them to the
        // compiler class's namespace (row 692, C10).
        let show_namespace_chirho = match self
            .compiler_layouts_chirho
            .get(&CompilerClassChirho::ShowChirho)
        {
            Some(layout_chirho) => Some((layout_chirho.clone(), true)),
            None if self
                .displaced_compiler_classes_chirho
                .contains_key(&CompilerClassChirho::ShowChirho) =>
            {
                None
            }
            None => self
                .layouts_chirho
                .get("Show")
                .cloned()
                .map(|layout_chirho| (layout_chirho, false)),
        };
        if let Some((show_layout_chirho, displaced_chirho)) = show_namespace_chirho {
            let show_name_chirho = show_layout_chirho.class_name_chirho.clone();
            for type_key_chirho in &[
                "Maybe Int",
                "Maybe String",
                "Maybe Double",
                "(Int,Int)",
                "(Int,String)",
                "(String,Int)",
                "(String,String)",
                "(Int,Bool)",
                "(Bool,Int)",
                "(Int,Double)",
                "(Double,Int)",
                "(Bool,Bool)",
                "(Double,Double)",
                "(Bool,String)",
                "(String,Bool)",
            ] {
                if !eligible_chirho.iter().any(|(c_chirho, t_chirho, _)| {
                    *c_chirho == show_name_chirho && t_chirho == *type_key_chirho
                }) {
                    if displaced_chirho {
                        compiler_entries_chirho.insert(
                            (show_name_chirho.clone(), type_key_chirho.to_string()),
                            CompilerClassChirho::ShowChirho,
                        );
                    }
                    eligible_chirho.push((
                        show_name_chirho.clone(),
                        type_key_chirho.to_string(),
                        show_layout_chirho.clone(),
                    ));
                }
            }
        }

        // Pass 1: pre-register all dict binder ids.
        let mut pre_registered_chirho: Vec<(String, String, DictLayoutChirho, BinderChirho)> =
            Vec::new();

        for (class_name_chirho, type_key_chirho, layout_chirho) in eligible_chirho {
            let dict_name_chirho = format!("$f{}{}", class_name_chirho, type_key_chirho);
            let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
            let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

            pre_registered_chirho.push((
                class_name_chirho,
                type_key_chirho,
                layout_chirho,
                dict_binder_chirho,
            ));
        }

        // The spelled rows each displacing source class may own, which no compiler dictionary
        // takes (row 692, C10).
        let namesake_rows_chirho: HashMap<CompilerClassChirho, HashSet<(String, String)>> =
            compiler_entries_chirho
                .values()
                .copied()
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|identity_chirho| {
                    (
                        identity_chirho,
                        self.namesake_rows_chirho(class_env_chirho, identity_chirho),
                    )
                })
                .collect();

        // Pass 2: build dict bodies — super dict ids are now findable via
        // `resolve_or_fresh_id_chirho` because pass 1 registered them.
        for (class_name_chirho, type_key_chirho, layout_chirho, dict_binder_chirho) in
            pre_registered_chirho
        {
            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments: a multi-parameter instance's are its class's
            // templates at its whole head, and it is registered for its uses only when every one
            // was built (row 694, #31909, #31948); a one-parameter instance's are named by its
            // own key, as before.
            let registered_head_chirho = match multi_heads_chirho
                .get(&(class_name_chirho.clone(), type_key_chirho.clone()))
            {
                Some(inst_chirho) => {
                    let (superclasses_chirho, built_chirho) = self
                        .multi_head_superclass_fields_chirho(
                            class_env_chirho,
                            inst_chirho,
                            &layout_chirho,
                        );
                    field_args_chirho.extend(superclasses_chirho);
                    built_chirho.then_some(inst_chirho)
                }
                None => {
                    for slot_chirho in &layout_chirho.super_slots_chirho {
                        field_args_chirho
                            .push(self.superclass_field_chirho(slot_chirho, &type_key_chirho));
                    }
                    None
                }
            };

            // Method implementation arguments (primitives)
            let generic_head_chirho = generic_head_by_key_chirho
                .get(&(class_name_chirho.clone(), type_key_chirho.clone()));
            let written_key_chirho =
                generic_head_chirho.map(|head_chirho| head_chirho.written_key_chirho.as_str());
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let compiler_entry_chirho = compiler_entries_chirho
                    .get(&(class_name_chirho.clone(), type_key_chirho.clone()))
                    .copied();
                let prim_id_chirho = match compiler_entry_chirho {
                    Some(identity_chirho) => self.compiler_method_row_chirho(
                        identity_chirho,
                        &prim_name_chirho,
                        method_name_chirho,
                        &type_key_chirho,
                        &namesake_rows_chirho[&identity_chirho],
                    ),
                    None => self.resolve_or_missing_method_id_chirho(
                        class_env_chirho,
                        &class_name_chirho,
                        method_name_chirho,
                        &type_key_chirho,
                        &prim_name_chirho,
                        written_key_chirho,
                    ),
                };
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;
            if let Some(inst_chirho) = registered_head_chirho {
                self.register_multi_head_chirho(inst_chirho, dict_id_chirho);
            }

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // The constructor a use site carries finds the same dictionary.
            // The alias is never a binder name and nothing exports this
            // registry, so a consumer module decides over its own instances;
            // it never displaces a key registered before or after it.
            if let Some(alias_head_chirho) = generic_head_chirho {
                self.instance_dicts_chirho
                    .entry((
                        class_name_chirho.clone(),
                        alias_head_chirho.constructor_chirho.clone(),
                    ))
                    .or_insert(dict_id_chirho);
            }
            self.instance_dicts_chirho
                .insert((class_name_chirho, type_key_chirho), dict_id_chirho);
        }
    }

    fn resolve_or_missing_method_id_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
        prim_name_chirho: &str,
        written_key_chirho: Option<&str>,
    ) -> CoreIdChirho {
        if let Some(prim_id_chirho) = self.lookup_body_backed_name_id_chirho(prim_name_chirho) {
            return prim_id_chirho;
        }

        // A generic head's row under the spelling it was written in.
        if let Some(written_key_chirho) = written_key_chirho {
            let written_row_chirho =
                format!("$prim_{class_name_chirho}_{method_name_chirho}_{written_key_chirho}");
            if let Some(prim_id_chirho) =
                self.lookup_body_backed_name_id_chirho(&written_row_chirho)
            {
                return prim_id_chirho;
            }
        }

        if let Some(prim_id_chirho) =
            self.written_method_row_chirho(class_name_chirho, method_name_chirho, type_key_chirho)
        {
            return prim_id_chirho;
        }

        if let Some(default_id_chirho) = self.method_default_row_chirho(
            class_name_chirho,
            method_name_chirho,
            written_key_chirho.unwrap_or(type_key_chirho),
            prim_name_chirho,
        ) {
            return default_id_chirho;
        }

        if class_name_chirho == "Monoid" && method_name_chirho == "mappend" {
            let semigroup_prim_name_chirho = format!("$prim_Semigroup_<>_{}", type_key_chirho);
            if let Some(prim_id_chirho) =
                self.lookup_body_backed_name_id_chirho(&semigroup_prim_name_chirho)
            {
                return prim_id_chirho;
            }
        }

        self.generate_missing_method_binding_chirho(
            class_env_chirho,
            class_name_chirho,
            method_name_chirho,
            type_key_chirho,
            prim_name_chirho,
        )
    }

    /// GHC's default row for a method the instance does not write, read from the real rows
    /// under `donor_key_chirho`: Eq's two methods from each other (row 606), Ord's from its
    /// `compare` (row 612) or its own `<=` (row 626), `compare` itself from `==` and `<=`, and
    /// Enum's `succ` and `pred` from `toEnum` and `fromEnum` (row 669). None when no real row
    /// donates, so the caller's named failure stays.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn method_default_row_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        donor_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.equality_default_row_chirho(
            class_name_chirho,
            method_name_chirho,
            donor_key_chirho,
            prim_name_chirho,
        )
        .or_else(|| {
            self.ordering_default_row_chirho(
                class_name_chirho,
                method_name_chirho,
                donor_key_chirho,
                prim_name_chirho,
            )
        })
        .or_else(|| {
            self.enum_default_row_chirho(
                class_name_chirho,
                method_name_chirho,
                donor_key_chirho,
                prim_name_chirho,
            )
        })
    }

    /// A row that fails by name, with its method's own calling convention, as
    /// every row named for a key has: whoever finds `$prim_<C>_<m>_<key>` by
    /// name passes the method's arguments and nothing else.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    fn generate_missing_method_binding_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> CoreIdChirho {
        let method_scheme_chirho = class_env_chirho
            .classes_chirho()
            .get_chirho(class_name_chirho)
            .and_then(|decl_chirho| decl_chirho.methods_chirho.get(method_name_chirho));

        let arity_chirho = method_scheme_chirho
            .map(Self::scheme_arity_chirho)
            .unwrap_or(0);

        self.missing_method_row_chirho(
            None,
            prim_name_chirho,
            format!(
                "missing method {}.{} for {}",
                class_name_chirho, method_name_chirho, type_key_chirho
            ),
            arity_chirho,
            method_scheme_chirho.map(|scheme_chirho| scheme_chirho.ty_chirho.clone()),
        )
    }

    /// The stub of `generate_missing_method_binding_chirho`: `arity_chirho` arguments, then
    /// `error message_chirho`. It is bound under `row_id_chirho` when a reference already names
    /// that id, so the reference is no longer dangling (row 671).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn missing_method_row_chirho(
        &mut self,
        row_id_chirho: Option<CoreIdChirho>,
        prim_name_chirho: &str,
        message_chirho: String,
        arity_chirho: usize,
        binder_ty_chirho: Option<TyChirho>,
    ) -> CoreIdChirho {
        let mut rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "error".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                message_chirho,
            ))],
        };

        const TYPE_VARIABLE_SEED_EXHAUSTED_CHIRHO: &str =
            "Dictionary type-variable seed exhausted Chirho";
        let fresh_ty_var_seed_chirho = self
            .next_id_chirho
            .checked_add(10_000)
            .expect(TYPE_VARIABLE_SEED_EXHAUSTED_CHIRHO);
        for arg_idx_chirho in (0..arity_chirho).rev() {
            let arg_ty_chirho = TyChirho::VarChirho(TyVarChirho(
                fresh_ty_var_seed_chirho
                    .checked_add(
                        u32::try_from(arg_idx_chirho).expect(TYPE_VARIABLE_SEED_EXHAUSTED_CHIRHO),
                    )
                    .expect(TYPE_VARIABLE_SEED_EXHAUSTED_CHIRHO),
            ));
            let arg_binder_chirho =
                self.fresh_binder_chirho(&format!("missing_arg_{arg_idx_chirho}"), arg_ty_chirho);
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_binder_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }

        let binder_ty_chirho =
            binder_ty_chirho.unwrap_or(TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)));
        let binder_chirho = match row_id_chirho {
            Some(id_chirho) => BinderChirho {
                id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: binder_ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            None => self.fresh_binder_chirho(prim_name_chirho, binder_ty_chirho),
        };
        let binder_id_chirho = binder_chirho.id_chirho;
        // A stub is never a representation's method (row 694).
        self.missing_method_ids_chirho.insert(binder_id_chirho);

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        binder_id_chirho
    }

    pub(in crate::dict_chirho) fn scheme_arity_chirho(scheme_chirho: &SchemeChirho) -> usize {
        Self::ty_arity_chirho(&scheme_chirho.ty_chirho)
    }

    fn ty_arity_chirho(ty_chirho: &TyChirho) -> usize {
        match ty_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => 1 + Self::ty_arity_chirho(result_chirho),
            TyChirho::ForallChirho { body_chirho, .. } => Self::ty_arity_chirho(body_chirho),
            _ => 0,
        }
    }
}

mod conditional_chirho;
mod constructor_orders_chirho;
// The default rows for methods an instance does not write live together in
// `instance_chirho/defaults_chirho/` (rows 606, 612, 626, 669 and 671).
#[path = "instance_chirho/defaults_chirho/context_slots_chirho.rs"]
mod context_slots_chirho;
#[path = "instance_chirho/defaults_chirho/enum_defaults_chirho.rs"]
mod enum_defaults_chirho;
#[path = "instance_chirho/defaults_chirho/equality_defaults_chirho.rs"]
mod equality_defaults_chirho;
pub(in crate::dict_chirho) mod generic_heads_chirho;
// Ground dictionaries for conditional instances live together in
// `instance_chirho/ground_dictionaries_chirho/`.
#[path = "instance_chirho/ground_dictionaries_chirho/ground_admission_chirho.rs"]
mod ground_admission_chirho;
#[path = "instance_chirho/ground_dictionaries_chirho/ground_budget_chirho.rs"]
mod ground_budget_chirho;
#[path = "instance_chirho/ground_dictionaries_chirho/ground_builder_chirho.rs"]
mod ground_builder_chirho;
#[path = "instance_chirho/ground_dictionaries_chirho/ground_route_chirho.rs"]
mod ground_route_chirho;
#[path = "instance_chirho/ground_dictionaries_chirho/ground_selection_chirho.rs"]
mod ground_selection_chirho;
#[cfg(test)]
#[path = "instance_chirho/ground_dictionaries_chirho/ground_selection_tests_chirho.rs"]
mod ground_selection_tests_chirho;
#[path = "instance_chirho/ground_dictionaries_chirho/instance_rows_chirho.rs"]
mod instance_rows_chirho;
#[cfg(test)]
#[path = "instance_chirho/ground_dictionaries_chirho/instance_rows_tests_chirho.rs"]
mod instance_rows_tests_chirho;
pub(in crate::dict_chirho) use instance_rows_chirho::InstanceRowsChirho;
pub use instance_rows_chirho::{InstanceDeclarationChirho, InstanceRowKeyChirho};
mod list_orders_chirho;
mod newtype_dicts_chirho;
pub use newtype_dicts_chirho::NewtypeInfoChirho;
#[path = "instance_chirho/defaults_chirho/ordering_defaults_chirho.rs"]
mod ordering_defaults_chirho;
mod ordering_operators_chirho;
mod string_spellings_chirho;
pub(in crate::dict_chirho) mod structured_equalities_chirho;
mod structured_orders_chirho;
