// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Instance dictionary generation: builtin prim bindings, ground instance dicts,
//! GND dicts, conditional ground dicts, and list dictionary helpers.

#![allow(unused_imports)]
use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    pub fn generate_builtin_prim_bindings_chirho(&mut self) {
        let builtins_chirho: Vec<(&str, &str, &str, u8)> = vec![
            // (class, method, type_key, kind)
            // kind: 0 = identity, 1 = binary primop, 2 = unary primop
            // Eq
            ("Eq", "==", "Int", 1),
            ("Eq", "==", "Char", 1),
            ("Eq", "==", "Bool", 1),
            // Ord
            ("Ord", "compare", "Int", 1),
            ("Ord", "compare", "Char", 1),
            ("Ord", "compare", "Double", 1),
            ("Ord", "compare", "Bool", 1),
            ("Ord", "compare", "[Char]", 1),
            // Show
            ("Show", "show", "Int", 2),
            ("Show", "show", "Char", 2),
            // Show Bool is handled specially below (case True/False -> string)
            ("Show", "show", "Bool", 0),
            // Num
            ("Num", "+", "Int", 1),
            ("Num", "*", "Int", 1),
            ("Num", "-", "Int", 1),
            ("Num", "negate", "Int", 2),
            ("Num", "fromInteger", "Int", 0),
            // Double
            ("Eq", "==", "Double", 1),
            ("Show", "show", "Double", 2),
            ("Num", "+", "Double", 1),
            ("Num", "*", "Double", 1),
            ("Num", "-", "Double", 1),
            ("Num", "negate", "Double", 2),
            ("Num", "fromInteger", "Double", 0),
            // Fractional Double
            ("Fractional", "/", "Double", 1),
            ("Fractional", "recip", "Double", 2),
            ("Fractional", "fromRational", "Double", 0),
            // Read instances
            ("Read", "read", "Int", 2),
            ("Read", "read", "Double", 2),
            ("Read", "read", "Bool", 2),
            // String (i.e. [Char])
            ("Eq", "==", "[Char]", 1),
            ("Show", "show", "[Char]", 2),
            // Lists
            ("Show", "show", "[Int]", 2),
            // Maybe
            ("Show", "show", "Maybe Int", 2),
            ("Show", "show", "Maybe String", 2),
            ("Show", "show", "Maybe Double", 2),
            // Tuples
            ("Show", "show", "(Int,Int)", 2),
            ("Show", "show", "(Int,String)", 2),
            ("Show", "show", "(String,Int)", 2),
            ("Show", "show", "(String,String)", 2),
            ("Show", "show", "(Int,Bool)", 2),
            ("Show", "show", "(Bool,Int)", 2),
            ("Show", "show", "(Int,Double)", 2),
            ("Show", "show", "(Double,Int)", 2),
            ("Show", "show", "(Bool,Bool)", 2),
            ("Show", "show", "(Double,Double)", 2),
            ("Show", "show", "(Bool,String)", 2),
            ("Show", "show", "(String,Bool)", 2),
            // 3-tuples
            ("Show", "show", "(Int,Int,Int)", 2),
            ("Show", "show", "(Int,Int,String)", 2),
            ("Show", "show", "(String,Int,Int)", 2),
            ("Show", "show", "(Int,String,Int)", 2),
            // Either
            ("Show", "show", "Either Int Int", 2),
            ("Show", "show", "Either String Int", 2),
            ("Show", "show", "Either Int String", 2),
            ("Show", "show", "Either String String", 2),
            // Ordering
            ("Show", "show", "Ordering", 2),
            ("Eq", "==", "Ordering", 1),
            ("Ord", "compare", "Ordering", 1),
            // IsString (OverloadedStrings)
            ("IsString", "fromString", "[Char]", 0),
            // IsList (OverloadedLists) — identity for [a] (type key matches TyVarChirho(9037))
            ("IsList", "fromList", "[t9037]", 0),
            ("IsList", "toList", "[t9037]", 0),
        ];

        let primop_for_chirho =
            |class_chirho: &str, method_chirho: &str, type_key_chirho: &str| -> &str {
                match (class_chirho, method_chirho, type_key_chirho) {
                    ("Eq", "==", "[Char]") => "eqStr#",
                    ("Eq", "==", "Double") => "eqFloat#",
                    ("Eq", "==", _) => "==#",
                    ("Ord", "compare", "Char") => "compareChar#",
                    ("Ord", "compare", "Double") => "compareFloat#",
                    ("Ord", "compare", "[Char]") => "compareStr#",
                    ("Ord", "compare", "Bool") => "compare#",
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

        for (class_chirho, method_chirho, type_key_chirho, kind_chirho) in builtins_chirho {
            let prim_name_chirho = format!(
                "$prim_{}_{}_{}",
                class_chirho, method_chirho, type_key_chirho
            );

            // Skip if this binding already exists (user may have provided one)
            let already_exists_chirho = self
                .names_chirho
                .values()
                .any(|n_chirho| n_chirho == &prim_name_chirho);
            if already_exists_chirho {
                continue;
            }

            // Special case: Show [Int] → generate a recursive Core function
            // that pattern-matches the list and uses showInt#/++# to build the
            // string. This avoids the showList# primop which can't force thunks.
            if class_chirho == "Show" && method_chirho == "show" && type_key_chirho == "[Int]" {
                self.generate_show_list_int_binding_chirho(&prim_name_chirho);
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

            // Special case: Ord Ordering → case dispatch on constructor tag ordering
            if class_chirho == "Ord" && method_chirho == "compare" && type_key_chirho == "Ordering"
            {
                self.generate_compare_ordering_binding_chirho(&prim_name_chirho);
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

    /// Generate `$prim_Ord_compare_Ordering = \x y -> ...` using nested case dispatch.
    /// LT < EQ < GT, so: LT vs LT -> EQ, LT vs _ -> LT, EQ vs LT -> GT, EQ vs EQ -> EQ, EQ vs GT -> LT, GT vs GT -> EQ, GT vs _ -> GT
    fn generate_compare_ordering_binding_chirho(&mut self, prim_name_chirho: &str) {
        let ord_ty_chirho = TyChirho::int_chirho(); // placeholder

        let x_chirho = self.fresh_binder_chirho("x", ord_ty_chirho.clone());
        let y_chirho = self.fresh_binder_chirho("y", ord_ty_chirho.clone());
        let scr_x_chirho = self.fresh_binder_chirho("_sx", ord_ty_chirho.clone());
        let scr_y1_chirho = self.fresh_binder_chirho("_sy1", ord_ty_chirho.clone());
        let scr_y2_chirho = self.fresh_binder_chirho("_sy2", ord_ty_chirho.clone());
        let scr_y3_chirho = self.fresh_binder_chirho("_sy3", ord_ty_chirho.clone());

        let lt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "LT".to_string(),
            args_chirho: vec![],
        };
        let eq_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "EQ".to_string(),
            args_chirho: vec![],
        };
        let gt_chirho = CoreExprChirho::ConAppChirho {
            con_name_chirho: "GT".to_string(),
            args_chirho: vec![],
        };

        // case x of LT -> case y of { LT -> EQ; _ -> LT }
        let inner_lt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y1_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of EQ -> case y of { LT -> GT; EQ -> EQ; GT -> LT }
        let inner_eq_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y2_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: lt_chirho.clone(),
                },
            ],
        };

        // case x of GT -> case y of { GT -> EQ; _ -> GT }
        let inner_gt_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            bind_chirho: scr_y3_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: eq_chirho.clone(),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: gt_chirho.clone(),
                },
            ],
        };

        let body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            bind_chirho: scr_x_chirho,
            result_ty_chirho: ord_ty_chirho.clone(),
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
            TyChirho::fun_chirho(ord_ty_chirho.clone(), ord_ty_chirho),
        );
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, fn_ty_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }

    fn generate_show_list_int_binding_chirho(&mut self, prim_name_chirho: &str) {
        let str_ty_chirho = TyChirho::string_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        // Generate $showListTail_Int first (referenced by $prim_Show_show_[Int])
        let tail_fn_name_chirho = "$showListTail_Int";
        let tail_fn_id_chirho = self.resolve_or_fresh_id_chirho(tail_fn_name_chirho);

        // Build the tail function body
        let tail_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let tail_wild_chirho = self.fresh_binder_chirho("_w", str_ty_chirho.clone());
        let tail_x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
        let tail_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        // (:) x rest -> ++# "," (++# (showInt# x) ($showListTail_Int rest))
        let tail_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(",".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "showInt#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(tail_x_chirho.id_chirho)],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                tail_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let tail_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(tail_xs_chirho.id_chirho)),
            bind_chirho: tail_wild_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![tail_x_chirho, tail_rest_chirho],
                    rhs_chirho: tail_cons_rhs_chirho,
                },
            ],
        };

        let tail_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: tail_xs_chirho,
            body_chirho: Box::new(tail_body_chirho),
        };

        let tail_binder_chirho = BinderChirho {
            id_chirho: tail_fn_id_chirho,
            name_chirho: tail_fn_name_chirho.to_string(),
            ty_chirho: TyChirho::fun_chirho(str_ty_chirho.clone(), str_ty_chirho.clone()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: tail_binder_chirho,
            rhs_chirho: tail_fn_rhs_chirho,
            is_rec_chirho: true, // recursive
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        // Now generate $prim_Show_show_[Int]
        let main_xs_chirho = self.fresh_binder_chirho("xs", str_ty_chirho.clone());
        let main_wild_chirho = self.fresh_binder_chirho("_w", str_ty_chirho.clone());
        let main_x_chirho = self.fresh_binder_chirho("x", int_ty_chirho);
        let main_rest_chirho = self.fresh_binder_chirho("rest", str_ty_chirho.clone());

        // (:) x rest -> ++# "[" (++# (showInt# x) ($showListTail_Int rest))
        let main_cons_rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "++#".to_string(),
            args_chirho: vec![
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho("[".to_string())),
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "showInt#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(main_x_chirho.id_chirho)],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_fn_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                main_rest_chirho.id_chirho,
                            )),
                        },
                    ],
                },
            ],
        };

        let main_body_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(main_xs_chirho.id_chirho)),
            bind_chirho: main_wild_chirho,
            result_ty_chirho: str_ty_chirho.clone(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                        "[]".to_string(),
                    )),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![main_x_chirho, main_rest_chirho],
                    rhs_chirho: main_cons_rhs_chirho,
                },
            ],
        };

        let main_fn_rhs_chirho = CoreExprChirho::LamChirho {
            binder_chirho: main_xs_chirho,
            body_chirho: Box::new(main_body_chirho),
        };

        let binder_chirho = self.fresh_binder_chirho(
            prim_name_chirho,
            TyChirho::fun_chirho(str_ty_chirho.clone(), str_ty_chirho),
        );
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho: main_fn_rhs_chirho,
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
    /// $fNumInt = $DictNum $fEqInt $fShowInt $prim_Num_+_Int ...
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

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for inst_chirho in instances_chirho {
                // Only handle ground instances (no context) for now
                if !inst_chirho.context_chirho.is_empty() {
                    continue;
                }

                let type_key_chirho = if inst_chirho.extra_head_tys_chirho.is_empty() {
                    format!("{}", inst_chirho.head_ty_chirho)
                } else {
                    let extra_chirho: Vec<String> = inst_chirho
                        .extra_head_tys_chirho
                        .iter()
                        .map(|t_chirho| format!("{}", t_chirho))
                        .collect();
                    format!("{}_{}", inst_chirho.head_ty_chirho, extra_chirho.join("_"))
                };
                eligible_chirho.push((
                    class_name_chirho.clone(),
                    type_key_chirho,
                    layout_chirho.clone(),
                ));
            }
        }

        // Inject synthetic ground instances for compound Show types.
        // These are monomorphised instances for Show (Maybe Int), etc.
        if let Some(show_layout_chirho) = self.layouts_chirho.get("Show").cloned() {
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
                    c_chirho == "Show" && t_chirho == *type_key_chirho
                }) {
                    eligible_chirho.push((
                        "Show".to_string(),
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

        // Pass 2: build dict bodies — super dict ids are now findable via
        // `resolve_or_fresh_id_chirho` because pass 1 registered them.
        for (class_name_chirho, type_key_chirho, layout_chirho, dict_binder_chirho) in
            pre_registered_chirho
        {
            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments
            for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, type_key_chirho);
                let super_dict_id_chirho = self.resolve_or_fresh_id_chirho(&super_dict_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(super_dict_id_chirho));
            }

            // Method implementation arguments (primitives)
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let prim_id_chirho = self.resolve_or_missing_method_id_chirho(
                    class_env_chirho,
                    &class_name_chirho,
                    method_name_chirho,
                    &type_key_chirho,
                    &prim_name_chirho,
                );
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

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
    ) -> CoreIdChirho {
        if let Some(prim_id_chirho) = self.lookup_body_backed_name_id_chirho(prim_name_chirho) {
            return prim_id_chirho;
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
            0,
        )
    }

    fn generate_missing_method_binding_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        class_name_chirho: &str,
        method_name_chirho: &str,
        type_key_chirho: &str,
        prim_name_chirho: &str,
        leading_dict_arity_chirho: usize,
    ) -> CoreIdChirho {
        let method_scheme_chirho = class_env_chirho
            .classes_chirho
            .get(class_name_chirho)
            .and_then(|decl_chirho| decl_chirho.methods_chirho.get(method_name_chirho));

        let arity_chirho = method_scheme_chirho
            .map(Self::scheme_arity_chirho)
            .unwrap_or(0);

        let mut rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: "error".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                format!(
                    "missing method {}.{} for {}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                ),
            ))],
        };

        let fresh_ty_var_seed_chirho = self.next_id_chirho + 10_000;
        for arg_idx_chirho in (0..arity_chirho).rev() {
            let arg_ty_chirho = TyChirho::VarChirho(TyVarChirho(
                fresh_ty_var_seed_chirho + arg_idx_chirho as u32,
            ));
            let arg_binder_chirho =
                self.fresh_binder_chirho(&format!("missing_arg_{arg_idx_chirho}"), arg_ty_chirho);
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_binder_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }

        for dict_idx_chirho in (0..leading_dict_arity_chirho).rev() {
            let dict_binder_chirho = self.fresh_binder_chirho(
                &format!("missing_dict_{dict_idx_chirho}"),
                TyChirho::ConChirho("$Dict_Missing".to_string()),
            );
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dict_binder_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }

        let mut binder_ty_chirho = method_scheme_chirho
            .map(|scheme_chirho| scheme_chirho.ty_chirho.clone())
            .unwrap_or_else(|| TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)));
        for _ in 0..leading_dict_arity_chirho {
            binder_ty_chirho = TyChirho::fun_chirho(
                TyChirho::ConChirho("$Dict_Missing".to_string()),
                binder_ty_chirho,
            );
        }
        let binder_chirho = self.fresh_binder_chirho(prim_name_chirho, binder_ty_chirho);
        let binder_id_chirho = binder_chirho.id_chirho;

        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });

        binder_id_chirho
    }

    fn scheme_arity_chirho(scheme_chirho: &SchemeChirho) -> usize {
        Self::ty_arity_chirho(&scheme_chirho.ty_chirho)
    }

    fn ty_arity_chirho(ty_chirho: &TyChirho) -> usize {
        match ty_chirho {
            TyChirho::FunChirho(_, result_chirho, _) => 1 + Self::ty_arity_chirho(result_chirho),
            TyChirho::ForallChirho { body_chirho, .. } => Self::ty_arity_chirho(body_chirho),
            _ => 0,
        }
    }

    /// Generate dictionary bindings for GeneralizedNewtypeDeriving instances.
    ///
    /// GND instances have a context like `Num Int => Num Age` where `Age` is
    /// a newtype wrapping `Int`. For each method in the class, we generate a
    /// `$prim_Class_method_NewtypeKey` alias that points to the underlying
    /// type's prim binding, then construct the dictionary.
    pub fn generate_gnd_dicts_chirho(&mut self, class_env_chirho: &ClassEnvChirho) {
        // Collect GND-eligible instances: context is non-empty AND the
        // head type is a known newtype.
        let mut gnd_instances_chirho: Vec<(String, String, String, DictLayoutChirho)> = Vec::new();

        for (class_name_chirho, instances_chirho) in &class_env_chirho.instances_chirho {
            let layout_chirho = match self.layouts_chirho.get(class_name_chirho) {
                Some(l_chirho) => l_chirho.clone(),
                None => continue,
            };

            for inst_chirho in instances_chirho {
                if inst_chirho.context_chirho.is_empty() {
                    continue; // Ground instance — handled by generate_instance_dicts_chirho
                }

                let type_key_chirho = format!("{}", inst_chirho.head_ty_chirho);

                // Check if the head type is a known newtype
                if let Some((_, underlying_key_chirho)) =
                    self.newtype_info_chirho.get(&type_key_chirho)
                {
                    gnd_instances_chirho.push((
                        class_name_chirho.clone(),
                        type_key_chirho,
                        underlying_key_chirho.clone(),
                        layout_chirho.clone(),
                    ));
                }
            }
        }

        for (class_name_chirho, type_key_chirho, underlying_key_chirho, layout_chirho) in
            gnd_instances_chirho
        {
            // Generate $prim aliases: $prim_Class_method_Newtype = $prim_Class_method_Underlying
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let newtype_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let underlying_prim_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, underlying_key_chirho
                );
                let underlying_id_chirho = self.resolve_or_missing_method_id_chirho(
                    class_env_chirho,
                    &class_name_chirho,
                    method_name_chirho,
                    &underlying_key_chirho,
                    &underlying_prim_chirho,
                );
                let alias_binder_chirho = self.fresh_binder_chirho(
                    &newtype_prim_chirho,
                    TyChirho::VarChirho(TyVarChirho(self.next_id_chirho)),
                );
                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho: alias_binder_chirho,
                    rhs_chirho: CoreExprChirho::VarChirho(underlying_id_chirho),
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }

            // Now build the dictionary (same as ground instances)
            let dict_name_chirho = format!("$f{}{}", class_name_chirho, type_key_chirho);
            let dict_ty_chirho = TyChirho::ConChirho(format!("$Dict_{}", class_name_chirho));
            let dict_binder_chirho = self.fresh_binder_chirho(&dict_name_chirho, dict_ty_chirho);

            let con_name_chirho = format!("$Dict_{}", class_name_chirho);
            let mut field_args_chirho: Vec<CoreExprChirho> = Vec::new();

            // Superclass dictionary arguments — reference the newtype's superclass dicts
            for (super_name_chirho, _) in &layout_chirho.super_slots_chirho {
                let super_dict_name_chirho = format!("$f{}{}", super_name_chirho, type_key_chirho);
                // Check if we already have this dict; if not, try underlying type
                let super_id_chirho = if self
                    .instance_dicts_chirho
                    .contains_key(&(super_name_chirho.clone(), type_key_chirho.clone()))
                {
                    self.resolve_or_fresh_id_chirho(&super_dict_name_chirho)
                } else {
                    let underlying_super_dict_chirho =
                        format!("$f{}{}", super_name_chirho, underlying_key_chirho);
                    self.resolve_or_fresh_id_chirho(&underlying_super_dict_chirho)
                };
                field_args_chirho.push(CoreExprChirho::VarChirho(super_id_chirho));
            }

            // Method arguments — reference the newtype's prim bindings
            for (method_name_chirho, _) in &layout_chirho.method_slots_chirho {
                let prim_name_chirho = format!(
                    "$prim_{}_{}_{}",
                    class_name_chirho, method_name_chirho, type_key_chirho
                );
                let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
                field_args_chirho.push(CoreExprChirho::VarChirho(prim_id_chirho));
            }

            let dict_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho: field_args_chirho,
            };

            let dict_id_chirho = dict_binder_chirho.id_chirho;
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: dict_binder_chirho,
                rhs_chirho: dict_expr_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            self.instance_dicts_chirho
                .insert((class_name_chirho, type_key_chirho), dict_id_chirho);
        }
    }
}

mod conditional_chirho;
