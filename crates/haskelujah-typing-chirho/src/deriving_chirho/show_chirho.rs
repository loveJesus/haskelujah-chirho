// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Derived `Show`: the instance type checking sees, and the constructor metadata the
//! dictionary pass renders it from. The instance supplies `show`, the class's only method;
//! GHC's derived precedence rules are generated in Core from [`DerivedShowChirho`], so a
//! nested constructor, a negative field, a record or an infix constructor prints as GHC
//! prints it (row 533).
//! Workflow: spec-chirho/workflows-chirho/print-show-evidence-chirho.md.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho, TyVarChirho};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use super::{
    app_chirho, con_field_count_chirho, con_name_chirho, con_pat_chirho, declaration_heads_chirho,
    field_vars_chirho, gen_span_chirho, infix_chirho, instance_type_chirho, var_expr_chirho,
    var_name_chirho,
};

/// Haskell's fixity for an operator with no fixity declaration: `infixl 9`.
const DEFAULT_PRECEDENCE_CHIRHO: u8 = 9;

/// The module that declares `NonEmpty`, and the type's name.
const NON_EMPTY_MODULE_CHIRHO: &str = "Data.List.NonEmpty";
const NON_EMPTY_TYPE_CHIRHO: &str = "NonEmpty";

/// How derived Show renders one constructor, following GHC's derived `showsPrec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowConFormChirho {
    /// `C a1 .. an`, parenthesized above precedence 10, fields at 11; a bare `C` with no fields.
    PrefixChirho,
    /// `C {l1 = a1, ..}`, parenthesized above precedence 10, fields at 0; one label per
    /// field, in declaration order.
    RecordChirho(Vec<String>),
    /// `a1 op a2` for a binary symbolic constructor: parenthesized above its precedence p,
    /// both operands at p + 1 whatever the associativity.
    InfixChirho(u8),
}

/// A field's type, spelled in the Show key grammar. `VarChirho` is one of the data type's
/// own parameters, replaced by the concrete key's argument when the dictionary pass renders it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowFieldTypeChirho {
    VarChirho(String),
    ConChirho(String),
    AppChirho(Box<ShowFieldTypeChirho>, Box<ShowFieldTypeChirho>),
    ListChirho(Box<ShowFieldTypeChirho>),
    TupleChirho(Vec<ShowFieldTypeChirho>),
}

/// One constructor of a type that derives Show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedShowConChirho {
    pub name_chirho: String,
    pub form_chirho: ShowConFormChirho,
    pub fields_chirho: Vec<ShowFieldTypeChirho>,
}

/// A type whose deriving clause names Show: its visible parameters and its constructors,
/// both in declaration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedShowChirho {
    pub type_vars_chirho: Vec<String>,
    pub constructors_chirho: Vec<DerivedShowConChirho>,
}

/// Every `data` type in the module whose deriving clause names Show, by type name.
///
/// A type with a GADT constructor, or a field whose type the Show key grammar cannot
/// spell, is left out, so its Show keeps its existing route. Newtypes and standalone
/// deriving are not described here.
/// workflow: print-show-evidence-chirho
pub fn derived_show_metadata_chirho(
    module_chirho: &ModuleChirho,
) -> HashMap<String, DerivedShowChirho> {
    let precedences_chirho = declared_precedences_chirho(module_chirho);
    let mut metadata_chirho = HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::DataDeclChirho {
            name_chirho,
            type_vars_chirho,
            constructors_chirho,
            deriving_chirho,
            kind_sig_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        if !deriving_chirho
            .iter()
            .any(|class_chirho| class_chirho.text_chirho() == "Show")
        {
            continue;
        }
        let Some(constructors_chirho) = constructors_chirho
            .iter()
            .map(|constructor_chirho| {
                derived_show_con_chirho(constructor_chirho, &precedences_chirho)
            })
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        // The derived instance's own binders: a kind-signature tail is a parameter here
        // exactly as in the instance head (`data E :: Type -> Type` has one).
        // workflow: print-show-evidence-chirho
        let type_vars_chirho = declaration_heads_chirho::visible_instance_binders_chirho(
            type_vars_chirho,
            kind_sig_chirho.as_ref(),
            *span_chirho,
        )
        .iter()
        .map(|type_var_chirho| type_var_chirho.name_chirho.text_chirho().to_string())
        .collect();
        metadata_chirho.insert(
            name_chirho.text_chirho().to_string(),
            DerivedShowChirho {
                type_vars_chirho,
                constructors_chirho,
            },
        );
    }
    // `NonEmpty` has no declaration in the module, but base derives its Show. A module
    // that imports Data.List.NonEmpty and names no type `NonEmpty` itself can only mean
    // that one, so it gets base's metadata: the seam row 533 left for unit 517.
    // workflow: print-show-evidence-chirho
    if imports_builtin_non_empty_chirho(module_chirho) {
        metadata_chirho.insert(
            NON_EMPTY_TYPE_CHIRHO.to_string(),
            non_empty_show_metadata_chirho(),
        );
    }
    metadata_chirho
}

/// base's `data NonEmpty a = a :| [a]` with `infixr 5 :|`: the head renders at 6, the tail
/// list at 6, and the whole is parenthesized above precedence 5.
fn non_empty_show_metadata_chirho() -> DerivedShowChirho {
    let element_chirho = ShowFieldTypeChirho::VarChirho("a".to_string());
    DerivedShowChirho {
        type_vars_chirho: vec!["a".to_string()],
        constructors_chirho: vec![DerivedShowConChirho {
            name_chirho: ":|".to_string(),
            form_chirho: ShowConFormChirho::InfixChirho(5),
            fields_chirho: vec![
                element_chirho.clone(),
                ShowFieldTypeChirho::ListChirho(Box::new(element_chirho)),
            ],
        }],
    }
}

/// Whether the module imports Data.List.NonEmpty and declares no type, class or family
/// named `NonEmpty` of its own.
fn imports_builtin_non_empty_chirho(module_chirho: &ModuleChirho) -> bool {
    let imports_chirho = module_chirho.imports_chirho.iter().any(|import_chirho| {
        import_chirho.module_chirho.full_name_chirho() == NON_EMPTY_MODULE_CHIRHO
    });
    let declares_chirho = module_chirho.decls_chirho.iter().any(|decl_chirho| {
        matches!(
            decl_chirho,
            DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
                | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. }
                if name_chirho.text_chirho() == NON_EMPTY_TYPE_CHIRHO
        )
    });
    imports_chirho && !declares_chirho
}

/// Operator → declared precedence, from the module's fixity declarations.
fn declared_precedences_chirho(module_chirho: &ModuleChirho) -> HashMap<String, u8> {
    let mut precedences_chirho = HashMap::new();
    for decl_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::FixityDeclChirho {
            precedence_chirho,
            ops_chirho,
            ..
        } = decl_chirho
        {
            for op_chirho in ops_chirho {
                precedences_chirho.insert(
                    op_chirho.text_chirho().to_string(),
                    precedence_chirho.unwrap_or(DEFAULT_PRECEDENCE_CHIRHO),
                );
            }
        }
    }
    precedences_chirho
}

fn derived_show_con_chirho(
    constructor_chirho: &ConDeclChirho,
    precedences_chirho: &HashMap<String, u8>,
) -> Option<DerivedShowConChirho> {
    match constructor_chirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            fields_chirho,
            ..
        } => {
            let name_chirho = name_chirho.text_chirho().to_string();
            let fields_chirho = fields_chirho
                .iter()
                .map(|(_strictness_chirho, ty_chirho)| show_field_type_chirho(ty_chirho))
                .collect::<Option<Vec<_>>>()?;
            // Only a binary symbolic constructor renders infix; the AST does not record
            // whether it was declared infix, and the prefix symbolic form is rejected at check.
            let form_chirho = if name_chirho.starts_with(':') && fields_chirho.len() == 2 {
                ShowConFormChirho::InfixChirho(
                    precedences_chirho
                        .get(&name_chirho)
                        .copied()
                        .unwrap_or(DEFAULT_PRECEDENCE_CHIRHO),
                )
            } else {
                ShowConFormChirho::PrefixChirho
            };
            Some(DerivedShowConChirho {
                name_chirho,
                form_chirho,
                fields_chirho,
            })
        }
        ConDeclChirho::RecordChirho {
            name_chirho,
            fields_chirho,
            ..
        } => {
            let mut labels_chirho = Vec::new();
            let mut types_chirho = Vec::new();
            for field_chirho in fields_chirho {
                let ty_chirho = show_field_type_chirho(&field_chirho.ty_chirho)?;
                for label_chirho in &field_chirho.names_chirho {
                    labels_chirho.push(label_chirho.text_chirho().to_string());
                    types_chirho.push(ty_chirho.clone());
                }
            }
            let form_chirho = if labels_chirho.is_empty() {
                ShowConFormChirho::PrefixChirho
            } else {
                ShowConFormChirho::RecordChirho(labels_chirho)
            };
            Some(DerivedShowConChirho {
                name_chirho: name_chirho.text_chirho().to_string(),
                form_chirho,
                fields_chirho: types_chirho,
            })
        }
        ConDeclChirho::GadtChirho { .. } => None,
    }
}

fn show_field_type_chirho(ty_chirho: &TypeChirho) -> Option<ShowFieldTypeChirho> {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) => Some(ShowFieldTypeChirho::ConChirho(
            name_chirho.text_chirho().to_string(),
        )),
        TypeChirho::VarChirho(name_chirho) => Some(ShowFieldTypeChirho::VarChirho(
            name_chirho.text_chirho().to_string(),
        )),
        TypeChirho::ListChirho { element_chirho, .. } => Some(ShowFieldTypeChirho::ListChirho(
            Box::new(show_field_type_chirho(element_chirho)?),
        )),
        TypeChirho::TupleChirho {
            elements_chirho, ..
        } => Some(ShowFieldTypeChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(show_field_type_chirho)
                .collect::<Option<Vec<_>>>()?,
        )),
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => Some(ShowFieldTypeChirho::AppChirho(
            Box::new(show_field_type_chirho(fun_chirho)?),
            Box::new(show_field_type_chirho(arg_chirho)?),
        )),
        TypeChirho::ParenChirho { inner_chirho, .. } => show_field_type_chirho(inner_chirho),
        _ => None,
    }
}

/// Generate `instance Show T where show = ...`, the instance type checking sees. The
/// dictionary pass replaces this body with the precedence-aware rendering whenever the
/// type's metadata covers every field.
pub(super) fn derive_show_chirho(
    type_name_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    constructors_chirho: &[ConDeclChirho],
    _span_chirho: SpanChirho,
) -> DeclChirho {
    let mut matches_chirho: Vec<MatchArmChirho> = Vec::new();

    for con_chirho in constructors_chirho {
        let name_chirho = con_name_chirho(con_chirho);
        let field_count_chirho = con_field_count_chirho(con_chirho);
        let a_vars_chirho = field_vars_chirho("a", field_count_chirho);
        let pat_chirho = con_pat_chirho(name_chirho, &a_vars_chirho);

        // Build: "ConName" ++ " " ++ show a1 ++ " " ++ show a2 ++ ...
        let body_chirho = if field_count_chirho == 0 {
            ExprChirho::LitChirho(LitChirho::StringChirho(
                name_chirho.to_string(),
                gen_span_chirho(),
                None,
            ))
        } else {
            // Start with "ConName"
            let mut result_chirho = ExprChirho::LitChirho(LitChirho::StringChirho(
                name_chirho.to_string(),
                gen_span_chirho(),
                None,
            ));

            for var_name_str_chirho in &a_vars_chirho {
                // Append " "
                result_chirho = infix_chirho(
                    result_chirho,
                    "++",
                    ExprChirho::LitChirho(LitChirho::StringChirho(
                        " ".to_string(),
                        gen_span_chirho(),
                        None,
                    )),
                );
                // Append show ai
                let show_field_chirho = app_chirho(
                    var_expr_chirho("show"),
                    var_expr_chirho(var_name_str_chirho),
                );
                result_chirho = infix_chirho(result_chirho, "++", show_field_chirho);
            }

            result_chirho
        };

        matches_chirho.push(MatchArmChirho {
            pats_chirho: vec![pat_chirho],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: gen_span_chirho(),
        });
    }

    let show_method_chirho = LocalBindChirho::FunBindChirho {
        name_chirho: var_name_chirho("show"),
        matches_chirho,
        span_chirho: gen_span_chirho(),
    };

    let context_chirho: Vec<_> = type_vars_chirho
        .iter()
        .map(
            |tv_chirho| haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                class_chirho: var_name_chirho("Show"),
                args_chirho: vec![TypeChirho::VarChirho(tv_chirho.name_chirho.clone())],
                span_chirho: gen_span_chirho(),
            },
        )
        .collect();

    DeclChirho::InstanceDeclChirho {
        context_chirho,
        class_chirho: var_name_chirho("Show"),
        types_chirho: vec![instance_type_chirho(type_name_chirho, type_vars_chirho)],
        methods_chirho: vec![show_method_chirho],
        assoc_tf_instances_chirho: vec![],
        span_chirho: gen_span_chirho(),
    }
}

#[cfg(test)]
#[path = "show_tests_chirho.rs"]
mod tests_chirho;
