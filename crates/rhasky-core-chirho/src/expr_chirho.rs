// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Core expression language
//!
//! A small, explicitly typed intermediate representation analogous to
//! GHC's System FC Core. Every binder carries its type, pattern matching
//! is restricted to flat case expressions, and all syntactic sugar has
//! been removed by the desugaring pass.

use std::collections::HashMap;
use std::fmt;

use rhasky_span_chirho::SpanChirho;
use rhasky_typing_chirho::ty_chirho::TyChirho;

/// A unique identifier for a Core variable (distinct from AST names).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoreIdChirho(pub u32);

impl fmt::Display for CoreIdChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "v{}", self.0)
    }
}

/// A Core variable binding: a name + its type.
#[derive(Debug, Clone, PartialEq)]
pub struct BinderChirho {
    pub id_chirho: CoreIdChirho,
    pub name_chirho: String,
    pub ty_chirho: TyChirho,
    pub span_chirho: SpanChirho,
}

impl fmt::Display for BinderChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "({} :: {})", self.name_chirho, self.ty_chirho)
    }
}

/// A Core expression — the heart of the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum CoreExprChirho {
    /// Variable reference.
    VarChirho(CoreIdChirho),

    /// Literal value.
    LitChirho(CoreLitChirho),

    /// Function application (f x). Always saturated or partial.
    AppChirho {
        fun_chirho: Box<CoreExprChirho>,
        arg_chirho: Box<CoreExprChirho>,
    },

    /// Lambda abstraction (\x -> body).
    LamChirho {
        binder_chirho: BinderChirho,
        body_chirho: Box<CoreExprChirho>,
    },

    /// Let binding (recursive or non-recursive).
    LetChirho {
        rec_chirho: bool,
        binds_chirho: Vec<(BinderChirho, CoreExprChirho)>,
        body_chirho: Box<CoreExprChirho>,
    },

    /// Case expression (the only form of pattern matching in Core).
    /// `case scrutinee of binder { alt1; alt2; ... }`
    CaseChirho {
        scrutinee_chirho: Box<CoreExprChirho>,
        /// The binder bound to the scrutinee value.
        bind_chirho: BinderChirho,
        /// The type of the result (all alts must agree).
        result_ty_chirho: TyChirho,
        alts_chirho: Vec<CoreAltChirho>,
    },

    /// Type abstraction (/\a -> body). For polymorphism.
    TyLamChirho {
        ty_var_chirho: String,
        body_chirho: Box<CoreExprChirho>,
    },

    /// Type application (expr @Type). For polymorphism.
    TyAppChirho {
        expr_chirho: Box<CoreExprChirho>,
        ty_chirho: TyChirho,
    },

    /// Primitive operation application. The `name_chirho` is the primop name
    /// (e.g. "+#", "-#", "*#", "div#", "mod#", "==#", "/=#", "<#", "<=#",
    /// ">#", ">=#", "negate#") and `args_chirho` are the operands.
    PrimOpChirho {
        name_chirho: String,
        args_chirho: Vec<CoreExprChirho>,
    },

    /// Data constructor application. Unlike `AppChirho`, this explicitly
    /// marks the head as a data constructor so the STG lowerer can emit
    /// `ConApp` instructions directly.
    ConAppChirho {
        con_name_chirho: String,
        args_chirho: Vec<CoreExprChirho>,
    },
}

/// A literal in Core (simpler than AST literals — all desugared).
#[derive(Debug, Clone, PartialEq)]
pub enum CoreLitChirho {
    IntChirho(i64),
    FloatChirho(f64),
    CharChirho(char),
    StringChirho(String),
}

impl fmt::Display for CoreLitChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntChirho(v_chirho) => write!(f_chirho, "{v_chirho}"),
            Self::FloatChirho(v_chirho) => write!(f_chirho, "{v_chirho}"),
            Self::CharChirho(v_chirho) => write!(f_chirho, "'{v_chirho}'"),
            Self::StringChirho(v_chirho) => write!(f_chirho, "\"{v_chirho}\""),
        }
    }
}

/// A case alternative in Core.
#[derive(Debug, Clone, PartialEq)]
pub struct CoreAltChirho {
    pub con_chirho: AltConChirho,
    pub binders_chirho: Vec<BinderChirho>,
    pub rhs_chirho: CoreExprChirho,
}

/// The constructor in a case alternative.
#[derive(Debug, Clone, PartialEq)]
pub enum AltConChirho {
    /// Data constructor with its name.
    DataConChirho(String),
    /// Literal pattern.
    LitConChirho(CoreLitChirho),
    /// Default (wildcard) alternative.
    DefaultChirho,
}

impl fmt::Display for AltConChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataConChirho(name_chirho) => write!(f_chirho, "{name_chirho}"),
            Self::LitConChirho(lit_chirho) => write!(f_chirho, "{lit_chirho}"),
            Self::DefaultChirho => write!(f_chirho, "_"),
        }
    }
}

/// A top-level Core binding (a named function or value).
#[derive(Debug, Clone, PartialEq)]
pub struct CoreBindingChirho {
    pub binder_chirho: BinderChirho,
    pub rhs_chirho: CoreExprChirho,
    pub is_rec_chirho: bool,
}

/// A Core module — the result of desugaring a source module.
#[derive(Debug, Clone, PartialEq)]
pub struct CoreModuleChirho {
    pub name_chirho: String,
    pub bindings_chirho: Vec<CoreBindingChirho>,
    /// CoreId → name mapping for all identifiers created during desugaring.
    pub names_chirho: HashMap<CoreIdChirho, String>,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn core_lit_display_chirho() {
        assert_eq!(CoreLitChirho::IntChirho(42).to_string(), "42");
        assert_eq!(CoreLitChirho::CharChirho('x').to_string(), "'x'");
        assert_eq!(
            CoreLitChirho::StringChirho("hi".to_string()).to_string(),
            "\"hi\""
        );
    }

    #[test]
    fn binder_display_chirho() {
        let b_chirho = dummy_binder_chirho("x", 0);
        assert_eq!(b_chirho.to_string(), "(x :: Int)");
    }

    #[test]
    fn build_identity_core_chirho() {
        // \x -> x
        let x_binder_chirho = BinderChirho {
            id_chirho: CoreIdChirho(0),
            name_chirho: "x".to_string(),
            ty_chirho: TyChirho::VarChirho(rhasky_typing_chirho::ty_chirho::TyVarChirho(0)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let body_chirho = CoreExprChirho::VarChirho(CoreIdChirho(0));
        let lam_chirho = CoreExprChirho::LamChirho {
            binder_chirho: x_binder_chirho,
            body_chirho: Box::new(body_chirho),
        };
        // Should construct without panicking
        assert!(matches!(lam_chirho, CoreExprChirho::LamChirho { .. }));
    }

    #[test]
    fn build_let_binding_chirho() {
        let x_chirho = dummy_binder_chirho("x", 0);
        let let_expr_chirho = CoreExprChirho::LetChirho {
            rec_chirho: false,
            binds_chirho: vec![(
                x_chirho,
                CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
            )],
            body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
        };
        assert!(matches!(let_expr_chirho, CoreExprChirho::LetChirho { .. }));
    }

    #[test]
    fn build_case_expr_chirho() {
        let scrut_bind_chirho = dummy_binder_chirho("wild", 99);
        let case_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
            bind_chirho: scrut_bind_chirho,
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("False".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                },
            ],
        };
        if let CoreExprChirho::CaseChirho { alts_chirho, .. } = &case_chirho {
            assert_eq!(alts_chirho.len(), 2);
        }
    }

    #[test]
    fn build_core_module_chirho() {
        let main_binder_chirho = dummy_binder_chirho("main", 0);
        let module_chirho = CoreModuleChirho {
            name_chirho: "Main".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: main_binder_chirho,
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
            }],
            names_chirho: HashMap::new(),
        };
        assert_eq!(module_chirho.name_chirho, "Main");
        assert_eq!(module_chirho.bindings_chirho.len(), 1);
    }
}
