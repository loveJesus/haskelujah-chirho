// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Core expression language
//!
//! A small, explicitly typed intermediate representation analogous to
//! GHC's System FC Core. Every binder carries its type, pattern matching
//! is restricted to flat case expressions, and all syntactic sugar has
//! been removed by the desugaring pass.

use std::collections::{BTreeSet, HashMap};
use std::fmt;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

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

/// Inline annotation for a Core binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineAnnotationChirho {
    /// `{-# INLINE f #-}` — always inline this binding at call sites.
    AlwaysChirho,
    /// `{-# NOINLINE f #-}` — never inline this binding.
    NeverChirho,
    /// `{-# INLINABLE f #-}` — available for inlining but not mandatory.
    InlinableChirho,
    /// No pragma — eligible for auto-inlining based on size heuristics.
    NoneChirho,
}

/// A top-level Core binding (a named function or value).
#[derive(Debug, Clone, PartialEq)]
pub struct CoreBindingChirho {
    pub binder_chirho: BinderChirho,
    pub rhs_chirho: CoreExprChirho,
    pub is_rec_chirho: bool,
    /// Inline annotation from `{-# INLINE/NOINLINE/INLINABLE #-}` pragmas.
    pub inline_chirho: InlineAnnotationChirho,
}

/// A Core module — the result of desugaring a source module.
/// A foreign export declaration in Core IR.
///
/// Records a Haskell binding that should be made callable from C/JS
/// via a wrapper with a specific calling convention and external name.
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignExportChirho {
    /// The Haskell name of the binding being exported.
    pub haskell_name_chirho: String,
    /// The C/external name for the export stub.
    pub foreign_name_chirho: String,
    /// Calling convention (e.g. "ccall", "javascript").
    pub calling_conv_chirho: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoreModuleChirho {
    pub name_chirho: String,
    pub bindings_chirho: Vec<CoreBindingChirho>,
    /// CoreId → name mapping for all identifiers created during desugaring.
    pub names_chirho: HashMap<CoreIdChirho, String>,
    /// SPECIALIZE pragmas: binding name → list of specialization type strings.
    pub specialize_pragmas_chirho: HashMap<String, Vec<String>>,
    /// Foreign exports: Haskell functions to expose via C/JS calling convention.
    pub foreign_exports_chirho: Vec<ForeignExportChirho>,
    /// Where the top-level identities come from: the entry, the compiler's own bindings, and
    /// builtin references (row 628).
    pub provenance_chirho: CoreProvenanceChirho,
}

// ---------------------------------------------------------------------------
// Native identity (row 628)
// ---------------------------------------------------------------------------

/// Where a module's top-level identities come from. The native backends decide what is the
/// program's entry, what is the compiler's own binding and what is a builtin reference from
/// this alone, never from a spelling or from an id that nothing binds. Producers fill it: the
/// driver records the entry and the builtin references, the dictionary pass its generated
/// bindings, and the multi-module merge offsets, joins and remaps it.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoreProvenanceChirho {
    /// The program's entry binding, `Main.main`: the only binding that keeps
    /// [`NATIVE_ENTRY_SYMBOL_CHIRHO`].
    pub entry_chirho: Option<CoreIdChirho>,
    /// Bindings the compiler generated: the dictionary pass's bodies, dictionaries, selectors
    /// and instance methods. Only these may be recognized by a backend through a spelling.
    pub compiler_bindings_chirho: BTreeSet<CoreIdChirho>,
    /// References no program binding owns that name a builtin or runtime function. A free
    /// reference outside this set is a missing binding, and fails closed.
    pub builtin_references_chirho: BTreeSet<CoreIdChirho>,
}

impl CoreProvenanceChirho {
    /// This provenance with every id moved by `offset_chirho`, as the merge moves a module's ids.
    pub fn offset_chirho(&self, offset_chirho: u32) -> Self {
        let shift_chirho = |id_chirho: &CoreIdChirho| CoreIdChirho(id_chirho.0 + offset_chirho);
        Self {
            entry_chirho: self.entry_chirho.as_ref().map(shift_chirho),
            compiler_bindings_chirho: self
                .compiler_bindings_chirho
                .iter()
                .map(shift_chirho)
                .collect(),
            builtin_references_chirho: self
                .builtin_references_chirho
                .iter()
                .map(shift_chirho)
                .collect(),
        }
    }

    /// Join another module's compiler bindings and builtin references. The entry is never
    /// joined: only the program's root module supplies it.
    pub fn absorb_chirho(&mut self, other_chirho: &Self) {
        self.compiler_bindings_chirho
            .extend(other_chirho.compiler_bindings_chirho.iter().copied());
        self.builtin_references_chirho
            .extend(other_chirho.builtin_references_chirho.iter().copied());
    }

    /// Follow a renaming of ids, as linking and deduplication rename references.
    pub fn remap_chirho(&mut self, remap_chirho: &HashMap<CoreIdChirho, CoreIdChirho>) {
        let follow_chirho =
            |id_chirho: CoreIdChirho| remap_chirho.get(&id_chirho).copied().unwrap_or(id_chirho);
        self.entry_chirho = self.entry_chirho.map(follow_chirho);
        self.compiler_bindings_chirho = self
            .compiler_bindings_chirho
            .iter()
            .map(|id_chirho| follow_chirho(*id_chirho))
            .collect();
        self.builtin_references_chirho = self
            .builtin_references_chirho
            .iter()
            .map(|id_chirho| follow_chirho(*id_chirho))
            .collect();
    }
}

/// The internal symbol of the program's entry, which the C `main` reaches.
pub const NATIVE_ENTRY_SYMBOL_CHIRHO: &str = "haskelujah_main";

/// `haskelujah_` and a spelling that native assemblers accept: letters, digits and `_` stay,
/// `'` is `_prime`, `.` is `_`, and any other character is `_uXXXX`. Several spellings share
/// a result (`f'` and `f_prime`), so a binding's symbol also carries its CoreId.
pub fn native_mangle_chirho(name_chirho: &str) -> String {
    let mut mangled_chirho = String::with_capacity(name_chirho.len() + 16);
    mangled_chirho.push_str("haskelujah_");
    for ch_chirho in name_chirho.chars() {
        match ch_chirho {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => mangled_chirho.push(ch_chirho),
            '\'' => mangled_chirho.push_str("_prime"),
            '.' => mangled_chirho.push('_'),
            _ => mangled_chirho.push_str(&format!("_u{:04x}", ch_chirho as u32)),
        }
    }
    mangled_chirho
}

/// The symbol of a top-level binding that is not the entry: its mangled spelling, a `.`, and
/// its CoreId. The CoreId makes it injective, and the `.` keeps it apart from every symbol a
/// backend writes by name (builtin runtime functions, RTS imports, lifted lambdas and
/// trampolines), none of which has one. The CoreId is an identity within one compiled Core
/// program, not a stable ABI across builds.
pub fn native_binding_symbol_chirho(name_chirho: &str, id_chirho: CoreIdChirho) -> String {
    format!("{}.{}", native_mangle_chirho(name_chirho), id_chirho.0)
}

/// The native symbol of every top-level binding of one module. Both native backends define
/// and reference top-level bindings through this table alone.
/// workflow: compiler-pipeline-chirho/native-symbol-identity-chirho
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NativeSymbolsChirho {
    symbols_chirho: HashMap<CoreIdChirho, String>,
}

impl NativeSymbolsChirho {
    /// The table for `module_chirho`: the recorded entry takes [`NATIVE_ENTRY_SYMBOL_CHIRHO`],
    /// and every other binding [`native_binding_symbol_chirho`].
    pub fn of_module_chirho(module_chirho: &CoreModuleChirho) -> Self {
        let entry_chirho = module_chirho.provenance_chirho.entry_chirho;
        let symbols_chirho = module_chirho
            .bindings_chirho
            .iter()
            .map(|binding_chirho| {
                let binder_chirho = &binding_chirho.binder_chirho;
                let symbol_chirho = if entry_chirho == Some(binder_chirho.id_chirho) {
                    NATIVE_ENTRY_SYMBOL_CHIRHO.to_string()
                } else {
                    native_binding_symbol_chirho(
                        &binder_chirho.name_chirho,
                        binder_chirho.id_chirho,
                    )
                };
                (binder_chirho.id_chirho, symbol_chirho)
            })
            .collect();
        Self { symbols_chirho }
    }

    /// The symbol of the top-level binding `id_chirho`, if one binds it.
    pub fn symbol_chirho(&self, id_chirho: CoreIdChirho) -> Option<&str> {
        self.symbols_chirho.get(&id_chirho).map(String::as_str)
    }
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
            ty_chirho: TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(0)),
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
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
            provenance_chirho: Default::default(),
        };
        assert_eq!(module_chirho.name_chirho, "Main");
        assert_eq!(module_chirho.bindings_chirho.len(), 1);
    }
}
