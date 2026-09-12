// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type-namespace use resolution and lexical type-variable scope.
//!
//! [`NameEnvChirho`] already contains imported and module-local type names by
//! the end of name-resolution collection. This pass walks source types against
//! that environment and adds the missing use-side half of name resolution.
//! Lexical type variables are tracked separately because a type constructor and
//! a type variable share Haskell's type namespace but obey different binding
//! rules.

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::{AstKindChirho, ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, MultiplicityChirho, TypeChirho};
use haskelujah_diagnostics_chirho::{DiagnosticBundleChirho, DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::env_chirho::{NameEnvChirho, NamespaceChirho};
use crate::iface_chirho::ModuleIfaceChirho;
use crate::resolve_chirho::{
    UNDEFINED_TYPE_CODE_CHIRHO, compute_imported_names_chirho,
    report_undefined_with_suggestions_chirho,
};
use crate::type_exports_chirho::{canonical_type_name_chirho, canonical_value_name_chirho};

/// Equality is built-in syntax rather than a normal imported class binding.
const BUILTIN_CONSTRAINT_NAMES_CHIRHO: &[&str] = &["~", "~~", "∼"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FreeTyVarPolicyChirho {
    /// The surrounding declaration implicitly quantifies otherwise-free vars.
    ImplicitChirho,
    /// Every variable use must have a lexical binder in scope.
    RequireBoundChirho,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TypeScopeIssueChirho(u8);

impl TypeScopeIssueChirho {
    const UNKNOWN_TYPE_NAME_CHIRHO: Self = Self(0);
    const UNKNOWN_PROMOTED_CONSTRUCTOR_CHIRHO: Self = Self(1);
    const FREE_TYPE_VARIABLE_CHIRHO: Self = Self(2);
}

struct TypeScopeWalkerChirho<'scope_chirho> {
    env_chirho: &'scope_chirho NameEnvChirho,
    diagnostics_chirho: &'scope_chirho mut DiagnosticBundleChirho,
    module_name_chirho: String,
    bound_tyvars_chirho: HashMap<String, usize>,
    reported_chirho: HashSet<(TypeScopeIssueChirho, String, SpanChirho)>,
    instance_associated_type_scope_chirho: HashMap<String, HashSet<String>>,
    existential_quantification_chirho: bool,
    data_kinds_chirho: bool,
    star_is_type_chirho: bool,
    imported_constructor_inventory_may_be_incomplete_chirho: bool,
    type_data_lowering_may_be_incomplete_chirho: bool,
}

impl<'scope_chirho> TypeScopeWalkerChirho<'scope_chirho> {
    fn new_chirho(
        module_chirho: &ModuleChirho,
        env_chirho: &'scope_chirho NameEnvChirho,
        available_modules_chirho: &[ModuleIfaceChirho],
        diagnostics_chirho: &'scope_chirho mut DiagnosticBundleChirho,
    ) -> Self {
        Self {
            env_chirho,
            diagnostics_chirho,
            module_name_chirho: module_chirho.name_chirho.full_name_chirho(),
            bound_tyvars_chirho: HashMap::new(),
            reported_chirho: HashSet::new(),
            instance_associated_type_scope_chirho: collect_instance_associated_type_scope_chirho(
                module_chirho,
                available_modules_chirho,
            ),
            existential_quantification_chirho: extension_enabled_chirho(
                module_chirho,
                "ExistentialQuantification",
            ) || extension_enabled_chirho(
                module_chirho,
                "GADTs",
            ),
            data_kinds_chirho: extension_enabled_chirho(module_chirho, "DataKinds")
                || extension_enabled_chirho(module_chirho, "TypeInType"),
            star_is_type_chirho: extension_enabled_with_default_chirho(
                module_chirho,
                "StarIsType",
                true,
            ),
            // Source interfaces cannot yet represent constructors introduced
            // by data-family instances. Until they carry an explicit
            // completeness bit, absence from an imported constructor
            // inventory is not proof that a promoted constructor is missing.
            imported_constructor_inventory_may_be_incomplete_chirho: !module_chirho
                .imports_chirho
                .is_empty(),
            // `type data` is currently lowered through the type-alias recovery
            // path. Keep that lossy shape from turning its first constructor
            // into an invented out-of-scope type use.
            type_data_lowering_may_be_incomplete_chirho: extension_enabled_chirho(
                module_chirho,
                "TypeData",
            ),
        }
    }

    fn walk_module_chirho(&mut self, module_chirho: &ModuleChirho) {
        for decl_chirho in &module_chirho.decls_chirho {
            self.walk_decl_chirho(decl_chirho);
        }
        for (_, class_chirho, via_type_chirho) in &module_chirho.deriving_via_chirho {
            self.check_type_name_chirho(class_chirho, false);
            self.walk_type_chirho(via_type_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
        }
    }

    fn walk_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        match decl_chirho {
            DeclChirho::TypeSigChirho { ty_chirho, .. }
            | DeclChirho::ForeignDeclChirho { ty_chirho, .. } => {
                self.walk_signature_type_chirho(ty_chirho);
            }
            DeclChirho::DataDeclChirho {
                type_vars_chirho,
                constructors_chirho,
                deriving_chirho,
                kind_sig_chirho,
                ..
            } => {
                for class_chirho in deriving_chirho {
                    self.check_type_name_chirho(class_chirho, false);
                }
                if let Some(signature_chirho) = kind_sig_chirho
                    .as_ref()
                    .and_then(|sig_chirho| sig_chirho.standalone_chirho())
                {
                    self.walk_signature_type_chirho(signature_chirho);
                }
                let pushed_chirho = self.push_decl_binders_chirho(type_vars_chirho);
                if let Some(result_chirho) = kind_sig_chirho
                    .as_ref()
                    .and_then(|sig_chirho| sig_chirho.result_chirho())
                {
                    // Inline result kinds share the declaration's implicit
                    // kind binders; only a standalone signature applies the
                    // outer-forall-or-nothing rule.
                    self.walk_type_chirho(result_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
                for constructor_chirho in constructors_chirho {
                    if !matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. }) {
                        self.walk_constructor_chirho(constructor_chirho);
                    }
                }
                self.pop_binders_chirho(&pushed_chirho);
                // GADT-style declaration-head variables have no scope in an
                // explicit constructor signature; each signature quantifies
                // independently.
                for constructor_chirho in constructors_chirho {
                    if matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. }) {
                        self.walk_constructor_chirho(constructor_chirho);
                    }
                }
            }
            DeclChirho::NewtypeDeclChirho {
                type_vars_chirho,
                constructor_chirho,
                deriving_chirho,
                kind_sig_chirho,
                ..
            } => {
                for class_chirho in deriving_chirho {
                    self.check_type_name_chirho(class_chirho, false);
                }
                if let Some(signature_chirho) = kind_sig_chirho
                    .as_ref()
                    .and_then(|sig_chirho| sig_chirho.standalone_chirho())
                {
                    self.walk_signature_type_chirho(signature_chirho);
                }
                let pushed_chirho = self.push_decl_binders_chirho(type_vars_chirho);
                if let Some(result_chirho) = kind_sig_chirho
                    .as_ref()
                    .and_then(|sig_chirho| sig_chirho.result_chirho())
                {
                    self.walk_type_chirho(result_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
                if matches!(constructor_chirho, ConDeclChirho::GadtChirho { .. }) {
                    self.pop_binders_chirho(&pushed_chirho);
                    self.walk_constructor_chirho(constructor_chirho);
                } else {
                    self.walk_constructor_chirho(constructor_chirho);
                    self.pop_binders_chirho(&pushed_chirho);
                }
            }
            DeclChirho::TypeAliasDeclChirho {
                type_vars_chirho,
                rhs_chirho,
                ..
            } => {
                let mut pushed_chirho = self.push_decl_binders_chirho(type_vars_chirho);
                // GHC9.14 still implicitly quantifies free kind names in an
                // OUTERMOST synonym RHS ascription (with GHC-16382). Nested
                // annotations do not gain this scope, nor do arbitrary RHS
                // type variables. Workflow: type-scope-resolution-chirho.
                let mut outer_rhs_chirho = rhs_chirho;
                while let TypeChirho::ParenChirho { inner_chirho, .. } = outer_rhs_chirho {
                    outer_rhs_chirho = inner_chirho;
                }
                if let TypeChirho::KindAnnotChirho { kind_chirho, .. } = outer_rhs_chirho {
                    let mut implicit_chirho = Vec::new();
                    collect_type_kind_variable_names_chirho(kind_chirho, &mut implicit_chirho);
                    let mut seen_chirho = HashSet::new();
                    implicit_chirho.retain(|name_chirho| {
                        !self.bound_tyvars_chirho.contains_key(name_chirho)
                            && seen_chirho.insert(name_chirho.clone())
                    });
                    pushed_chirho.extend(self.push_name_binders_chirho(implicit_chirho));
                }
                if !(self.type_data_lowering_may_be_incomplete_chirho
                    && type_alias_may_be_type_data_recovery_chirho(type_vars_chirho, rhs_chirho))
                {
                    self.walk_type_chirho(rhs_chirho, FreeTyVarPolicyChirho::RequireBoundChirho);
                }
                self.pop_binders_chirho(&pushed_chirho);
            }
            DeclChirho::TypeFamilyDeclChirho {
                type_vars_chirho,
                result_chirho,
                equations_chirho,
                ..
            } => {
                if let Some(signature_chirho) = result_chirho
                    .kind_sig_chirho
                    .as_ref()
                    .and_then(|signature_chirho| signature_chirho.standalone_chirho())
                {
                    self.walk_signature_type_chirho(signature_chirho);
                }
                let pushed_chirho = self.push_decl_binders_chirho(type_vars_chirho);
                if let Some(result_kind_chirho) = result_chirho
                    .kind_sig_chirho
                    .as_ref()
                    .and_then(|signature_chirho| signature_chirho.result_chirho())
                {
                    self.walk_type_chirho(
                        result_kind_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
                self.pop_binders_chirho(&pushed_chirho);
                for equation_chirho in equations_chirho {
                    for lhs_ty_chirho in &equation_chirho.lhs_types_chirho {
                        self.walk_type_chirho(lhs_ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                    }
                    self.walk_type_chirho(
                        &equation_chirho.rhs_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
            }
            DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                lhs_types_chirho,
                rhs_chirho,
                ..
            } => {
                self.check_type_name_chirho(family_name_chirho, false);
                for lhs_ty_chirho in lhs_types_chirho {
                    self.walk_type_chirho(lhs_ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
                self.walk_type_chirho(rhs_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
            }
            DeclChirho::ClassDeclChirho {
                context_chirho,
                type_vars_chirho,
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => {
                let pushed_chirho = self.push_decl_binders_chirho(type_vars_chirho);
                for constraint_chirho in context_chirho {
                    self.walk_constraint_chirho(
                        constraint_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
                for method_chirho in methods_chirho {
                    self.walk_signature_type_chirho(&method_chirho.ty_chirho);
                }
                for associated_tf_chirho in associated_tfs_chirho {
                    let default_binders_chirho = if associated_tf_chirho.default_params_chirho.len()
                        == associated_tf_chirho.type_vars_chirho.len()
                    {
                        &associated_tf_chirho.default_params_chirho
                    } else {
                        &associated_tf_chirho.type_vars_chirho
                    };
                    let associated_pushed_chirho = self.push_name_binders_chirho(
                        default_binders_chirho
                            .iter()
                            .map(|name_chirho| name_chirho.text_chirho().to_string()),
                    );
                    if let Some(default_rhs_chirho) = &associated_tf_chirho.default_rhs_chirho {
                        self.walk_type_chirho(
                            default_rhs_chirho,
                            FreeTyVarPolicyChirho::RequireBoundChirho,
                        );
                    }
                    self.pop_binders_chirho(&associated_pushed_chirho);
                }
                self.pop_binders_chirho(&pushed_chirho);
            }
            DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                assoc_tf_instances_chirho,
                ..
            } => {
                self.check_type_name_chirho(class_chirho, false);
                for constraint_chirho in context_chirho {
                    self.walk_constraint_chirho(
                        constraint_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
                for ty_chirho in types_chirho {
                    self.walk_type_chirho(ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
                for assoc_tf_chirho in assoc_tf_instances_chirho {
                    self.check_instance_associated_type_name_chirho(
                        class_chirho,
                        &assoc_tf_chirho.family_name_chirho,
                    );
                    for lhs_ty_chirho in &assoc_tf_chirho.lhs_types_chirho {
                        self.walk_type_chirho(lhs_ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                    }
                    self.walk_type_chirho(
                        &assoc_tf_chirho.rhs_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
            }
            DeclChirho::DefaultDeclChirho { types_chirho, .. } => {
                for ty_chirho in types_chirho {
                    self.walk_type_chirho(ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
            }
            DeclChirho::StandaloneDerivingDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                ..
            } => {
                self.check_type_name_chirho(class_chirho, false);
                for constraint_chirho in context_chirho {
                    self.walk_constraint_chirho(
                        constraint_chirho,
                        FreeTyVarPolicyChirho::ImplicitChirho,
                    );
                }
                for ty_chirho in types_chirho {
                    self.walk_type_chirho(ty_chirho, FreeTyVarPolicyChirho::ImplicitChirho);
                }
            }
            DeclChirho::FunBindChirho { .. }
            | DeclChirho::PatBindChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::PatSynDeclChirho { .. }
            | DeclChirho::SpliceDeclChirho { .. } => {}
        }
    }

    fn walk_signature_type_chirho(&mut self, ty_chirho: &TypeChirho) {
        // GHC's forall-or-nothing rule applies only when an invisible
        // `forall ... .` is the outermost type form. Without one, free
        // variables are implicitly quantified at the signature boundary.
        // A required `forall ... ->` deliberately does not trigger the rule.
        let free_var_policy_chirho = if matches!(ty_chirho, TypeChirho::ForallChirho { .. }) {
            FreeTyVarPolicyChirho::RequireBoundChirho
        } else {
            FreeTyVarPolicyChirho::ImplicitChirho
        };
        self.walk_type_chirho(ty_chirho, free_var_policy_chirho);
    }

    fn walk_constructor_chirho(&mut self, constructor_chirho: &ConDeclChirho) {
        let field_policy_chirho = if self.existential_quantification_chirho
            || constructor_has_unrepresented_scope_prefix_chirho(constructor_chirho)
        {
            FreeTyVarPolicyChirho::ImplicitChirho
        } else {
            FreeTyVarPolicyChirho::RequireBoundChirho
        };
        match constructor_chirho {
            ConDeclChirho::OrdinaryChirho { fields_chirho, .. } => {
                for (_, field_ty_chirho) in fields_chirho {
                    self.walk_type_chirho(field_ty_chirho, field_policy_chirho);
                }
            }
            ConDeclChirho::RecordChirho { fields_chirho, .. } => {
                for field_chirho in fields_chirho {
                    // The flat record-field lowering path mis-scopes a leading
                    // invisible `forall`; see bug-record-field-forall-lowering-chirho.md.
                    // Required `forall a ->` is already preserved by that path.
                    // Continue resolving reliable fields, but do not manufacture
                    // scope errors from a corrupt rank-N AST.
                    if record_field_type_scope_reliable_chirho(&field_chirho.ty_chirho) {
                        self.walk_type_chirho(&field_chirho.ty_chirho, field_policy_chirho);
                    }
                }
            }
            ConDeclChirho::GadtChirho { ty_chirho, .. } => {
                self.walk_signature_type_chirho(ty_chirho);
            }
        }
    }

    fn walk_type_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        free_var_policy_chirho: FreeTyVarPolicyChirho,
    ) {
        match ty_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                self.check_type_variable_use_chirho(name_chirho, free_var_policy_chirho);
            }
            TypeChirho::ConChirho(name_chirho) => {
                self.check_type_name_chirho(name_chirho, true);
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.walk_type_chirho(fun_chirho, free_var_policy_chirho);
                self.walk_type_chirho(arg_chirho, free_var_policy_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                self.walk_type_chirho(arg_chirho, free_var_policy_chirho);
                if let Some(MultiplicityChirho::ExpressionChirho(expression_chirho)) = mult_chirho {
                    self.walk_type_chirho(expression_chirho, free_var_policy_chirho);
                }
                self.walk_type_chirho(result_chirho, free_var_policy_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.walk_type_chirho(element_chirho, free_var_policy_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                self.walk_type_chirho(element_chirho, free_var_policy_chirho);
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.walk_type_chirho(inner_chirho, free_var_policy_chirho);
            }
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                ..
            } => {
                self.walk_type_chirho(type_chirho, free_var_policy_chirho);
                self.walk_type_chirho(kind_chirho, free_var_policy_chirho);
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.walk_constraint_chirho(constraint_chirho, free_var_policy_chirho);
                }
                self.walk_type_chirho(body_chirho, free_var_policy_chirho);
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let pushed_chirho =
                    self.push_explicit_binders_chirho(vars_chirho, free_var_policy_chirho);
                self.walk_type_chirho(body_chirho, free_var_policy_chirho);
                self.pop_binders_chirho(&pushed_chirho);
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                self.check_promoted_constructor_chirho(name_chirho);
            }
            TypeChirho::WildcardChirho { .. } | TypeChirho::LitChirho { .. } => {}
        }
    }

    fn walk_constraint_chirho(
        &mut self,
        constraint_chirho: &ConstraintChirho,
        free_var_policy_chirho: FreeTyVarPolicyChirho,
    ) {
        match constraint_chirho {
            ConstraintChirho::ClassChirho {
                class_chirho,
                args_chirho,
                ..
            } => {
                let class_text_chirho = class_chirho.text_chirho();
                // Implicit-parameter labels name evidence, not type constructors.
                // The legacy `?` recovery marker is likewise not a source type;
                // retained argument types are still walked below in either case.
                if !class_text_chirho.starts_with('?')
                    && !is_builtin_constraint_name_chirho(class_chirho)
                    && name_parts_chirho(class_chirho).0.is_none()
                    && is_lexical_type_variable_chirho(class_text_chirho)
                {
                    self.check_type_variable_use_chirho(class_chirho, free_var_policy_chirho);
                } else if !class_text_chirho.starts_with('?')
                    && !is_builtin_constraint_name_chirho(class_chirho)
                {
                    self.check_type_name_chirho(class_chirho, false);
                }
                for arg_chirho in args_chirho {
                    self.walk_type_chirho(arg_chirho, free_var_policy_chirho);
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let pushed_chirho =
                    self.push_explicit_binders_chirho(vars_chirho, free_var_policy_chirho);
                for nested_chirho in context_chirho {
                    self.walk_constraint_chirho(nested_chirho, free_var_policy_chirho);
                }
                self.walk_constraint_chirho(body_chirho, free_var_policy_chirho);
                self.pop_binders_chirho(&pushed_chirho);
            }
        }
    }

    fn push_decl_binders_chirho(
        &mut self,
        vars_chirho: &[haskelujah_ast_chirho::decl_chirho::TyVarChirho],
    ) -> Vec<String> {
        let declared_names_chirho: Vec<String> = vars_chirho
            .iter()
            .map(|var_chirho| var_chirho.name_chirho.text_chirho().to_string())
            .collect();
        let declared_set_chirho: HashSet<String> = declared_names_chirho.iter().cloned().collect();
        let mut implicit_kind_names_chirho = Vec::new();
        let mut seen_kind_names_chirho = HashSet::new();
        for var_chirho in vars_chirho {
            if let Some(kind_chirho) = &var_chirho.kind_annotation_chirho {
                collect_kind_variable_names_chirho(kind_chirho, &mut implicit_kind_names_chirho);
            }
        }
        implicit_kind_names_chirho.retain(|name_chirho| {
            !declared_set_chirho.contains(name_chirho)
                && seen_kind_names_chirho.insert(name_chirho.clone())
        });

        let mut pushed_chirho = self.push_name_binders_chirho(implicit_kind_names_chirho);
        pushed_chirho.extend(self.push_name_binders_chirho(declared_names_chirho));
        for variable_chirho in vars_chirho {
            if let Some(kind_chirho) = &variable_chirho.kind_annotation_chirho {
                self.walk_binder_kind_chirho(
                    kind_chirho,
                    variable_chirho.name_chirho.span_chirho(),
                    FreeTyVarPolicyChirho::ImplicitChirho,
                );
            }
        }
        pushed_chirho
    }

    fn push_explicit_binders_chirho(
        &mut self,
        vars_chirho: &[haskelujah_ast_chirho::decl_chirho::TyVarChirho],
        free_var_policy_chirho: FreeTyVarPolicyChirho,
    ) -> Vec<String> {
        let mut pushed_chirho = Vec::with_capacity(vars_chirho.len());
        for var_chirho in vars_chirho {
            if let Some(kind_chirho) = &var_chirho.kind_annotation_chirho {
                self.walk_binder_kind_chirho(
                    kind_chirho,
                    var_chirho.name_chirho.span_chirho(),
                    free_var_policy_chirho,
                );
            }
            let name_chirho = var_chirho.name_chirho.text_chirho().to_string();
            self.push_one_binder_chirho(name_chirho.clone());
            pushed_chirho.push(name_chirho);
        }
        pushed_chirho
    }

    fn walk_binder_kind_chirho(
        &mut self,
        kind_chirho: &AstKindChirho,
        span_chirho: SpanChirho,
        free_var_policy_chirho: FreeTyVarPolicyChirho,
    ) {
        match kind_chirho {
            AstKindChirho::TypeSyntaxChirho(type_chirho) => {
                self.walk_type_chirho(type_chirho, free_var_policy_chirho);
            }
            AstKindChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | AstKindChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let pushed_chirho =
                    self.push_explicit_binders_chirho(vars_chirho, free_var_policy_chirho);
                self.walk_binder_kind_chirho(body_chirho, span_chirho, free_var_policy_chirho);
                self.pop_binders_chirho(&pushed_chirho);
            }
            AstKindChirho::ConChirho(name_chirho) => self.check_type_name_chirho(name_chirho, true),
            AstKindChirho::VarChirho(name_chirho)
                if is_lexical_type_variable_chirho(name_chirho)
                    && free_var_policy_chirho == FreeTyVarPolicyChirho::RequireBoundChirho
                    && !self.bound_tyvars_chirho.contains_key(name_chirho) =>
            {
                self.report_free_type_variable_text_chirho(name_chirho, span_chirho);
            }
            AstKindChirho::ArrowChirho(arg_chirho, result_chirho) => {
                self.walk_binder_kind_chirho(arg_chirho, span_chirho, free_var_policy_chirho);
                self.walk_binder_kind_chirho(result_chirho, span_chirho, free_var_policy_chirho);
            }
            AstKindChirho::AppChirho(fun_chirho, arg_chirho)
            | AstKindChirho::KindAppChirho(fun_chirho, arg_chirho)
            | AstKindChirho::KindAnnotChirho {
                type_chirho: fun_chirho,
                kind_chirho: arg_chirho,
                ..
            } => {
                self.walk_binder_kind_chirho(fun_chirho, span_chirho, free_var_policy_chirho);
                self.walk_binder_kind_chirho(arg_chirho, span_chirho, free_var_policy_chirho);
            }
            AstKindChirho::StarChirho
            | AstKindChirho::ConstraintChirho
            | AstKindChirho::VarChirho(_) => {}
        }
    }

    fn push_name_binders_chirho(
        &mut self,
        names_chirho: impl IntoIterator<Item = String>,
    ) -> Vec<String> {
        let pushed_chirho: Vec<String> = names_chirho.into_iter().collect();
        for name_chirho in &pushed_chirho {
            self.push_one_binder_chirho(name_chirho.clone());
        }
        pushed_chirho
    }

    fn push_one_binder_chirho(&mut self, name_chirho: String) {
        *self.bound_tyvars_chirho.entry(name_chirho).or_insert(0) += 1;
    }

    fn pop_binders_chirho(&mut self, names_chirho: &[String]) {
        for name_chirho in names_chirho.iter().rev() {
            let remove_chirho = {
                let count_chirho = self
                    .bound_tyvars_chirho
                    .get_mut(name_chirho)
                    .expect("a popped type binder must have been pushed");
                *count_chirho -= 1;
                *count_chirho == 0
            };
            if remove_chirho {
                self.bound_tyvars_chirho.remove(name_chirho);
            }
        }
    }

    fn check_type_variable_use_chirho(
        &mut self,
        name_chirho: &NameChirho,
        free_var_policy_chirho: FreeTyVarPolicyChirho,
    ) {
        let text_chirho = name_chirho.text_chirho();
        // Empty names are parser recovery sentinels, not source identifiers.
        // Implicit-parameter names bind evidence, not lexical type variables.
        if text_chirho.is_empty() || text_chirho.starts_with('?') {
            return;
        }
        if free_var_policy_chirho == FreeTyVarPolicyChirho::RequireBoundChirho
            && text_chirho != "_"
            && !self.bound_tyvars_chirho.contains_key(text_chirho)
        {
            self.report_free_type_variable_chirho(name_chirho);
        }
    }

    fn check_promoted_constructor_chirho(&mut self, name_chirho: &NameChirho) {
        if name_chirho.text_chirho().is_empty() {
            return;
        }
        if is_builtin_promoted_constructor_name_chirho(name_chirho) {
            return;
        }
        let (qualifier_chirho, raw_text_chirho) = name_parts_chirho(name_chirho);
        let text_chirho = canonical_value_name_chirho(raw_text_chirho);
        let found_chirho = if let Some(qualifier_chirho) = qualifier_chirho {
            self.current_module_local_name_in_scope_chirho(
                qualifier_chirho,
                &text_chirho,
                NamespaceChirho::ValueChirho,
            ) || self
                .env_chirho
                .lookup_qualified_chirho(
                    qualifier_chirho,
                    &text_chirho,
                    NamespaceChirho::ValueChirho,
                )
                .is_some()
        } else {
            self.env_chirho.lookup_value_chirho(&text_chirho).is_some()
        };
        if found_chirho {
            return;
        }
        if self.imported_constructor_inventory_may_be_incomplete_chirho {
            return;
        }

        let full_name_chirho = name_chirho.full_name_chirho();
        let key_chirho = (
            TypeScopeIssueChirho::UNKNOWN_PROMOTED_CONSTRUCTOR_CHIRHO,
            full_name_chirho.clone(),
            name_chirho.span_chirho(),
        );
        if self.reported_chirho.insert(key_chirho) {
            report_undefined_with_suggestions_chirho(
                self.diagnostics_chirho,
                &full_name_chirho,
                NamespaceChirho::ValueChirho,
                name_chirho.span_chirho(),
                Some(self.env_chirho),
            );
        }
    }

    fn check_type_name_chirho(&mut self, name_chirho: &NameChirho, allow_promotion_chirho: bool) {
        if name_chirho.text_chirho().is_empty() {
            return;
        }
        if is_builtin_type_name_chirho(name_chirho)
            || (self.star_is_type_chirho && is_unqualified_star_chirho(name_chirho))
        {
            return;
        }
        if self.type_name_in_scope_chirho(name_chirho, allow_promotion_chirho) {
            return;
        }
        self.report_unknown_type_name_chirho(name_chirho);
    }

    fn check_instance_associated_type_name_chirho(
        &mut self,
        class_chirho: &NameChirho,
        associated_type_chirho: &NameChirho,
    ) {
        let class_name_chirho = canonical_type_name_chirho(&class_chirho.full_name_chirho());
        let associated_name_chirho =
            canonical_type_name_chirho(associated_type_chirho.text_chirho());
        let known_parent_chirho = self
            .instance_associated_type_scope_chirho
            .get(&class_name_chirho);
        match known_parent_chirho {
            Some(visible_members_chirho)
                if visible_members_chirho.contains(&associated_name_chirho) => {}
            Some(_) => self.report_unknown_type_name_chirho(associated_type_chirho),
            None => self.check_type_name_chirho(associated_type_chirho, false),
        }
    }

    fn report_unknown_type_name_chirho(&mut self, name_chirho: &NameChirho) {
        let full_name_chirho = name_chirho.full_name_chirho();
        let key_chirho = (
            TypeScopeIssueChirho::UNKNOWN_TYPE_NAME_CHIRHO,
            full_name_chirho.clone(),
            name_chirho.span_chirho(),
        );
        if self.reported_chirho.insert(key_chirho) {
            report_undefined_with_suggestions_chirho(
                self.diagnostics_chirho,
                &full_name_chirho,
                NamespaceChirho::TypeChirho,
                name_chirho.span_chirho(),
                Some(self.env_chirho),
            );
        }
    }

    fn type_name_in_scope_chirho(
        &self,
        name_chirho: &NameChirho,
        allow_promotion_chirho: bool,
    ) -> bool {
        let (qualifier_chirho, raw_text_chirho) = name_parts_chirho(name_chirho);
        let text_chirho = canonical_type_name_chirho(raw_text_chirho);
        if let Some(qualifier_chirho) = qualifier_chirho {
            let type_found_chirho = self.current_module_local_name_in_scope_chirho(
                qualifier_chirho,
                &text_chirho,
                NamespaceChirho::TypeChirho,
            ) || self
                .env_chirho
                .lookup_qualified_chirho(
                    qualifier_chirho,
                    &text_chirho,
                    NamespaceChirho::TypeChirho,
                )
                .is_some();
            type_found_chirho
                || (allow_promotion_chirho
                    && self.data_kinds_chirho
                    && is_promotable_constructor_spelling_chirho(&text_chirho)
                    && (self.current_module_local_name_in_scope_chirho(
                        qualifier_chirho,
                        &text_chirho,
                        NamespaceChirho::ValueChirho,
                    ) || self
                        .env_chirho
                        .lookup_qualified_chirho(
                            qualifier_chirho,
                            &text_chirho,
                            NamespaceChirho::ValueChirho,
                        )
                        .is_some()))
        } else {
            self.env_chirho.lookup_type_chirho(&text_chirho).is_some()
                || (allow_promotion_chirho
                    && self.data_kinds_chirho
                    && is_promotable_constructor_spelling_chirho(&text_chirho)
                    && self.env_chirho.lookup_value_chirho(&text_chirho).is_some())
        }
    }

    fn current_module_local_name_in_scope_chirho(
        &self,
        qualifier_chirho: &str,
        text_chirho: &str,
        namespace_chirho: NamespaceChirho,
    ) -> bool {
        qualifier_chirho == self.module_name_chirho
            && self
                .env_chirho
                .lookup_chirho(text_chirho, namespace_chirho)
                .is_some_and(|info_chirho| !info_chirho.imported_chirho)
    }

    fn report_free_type_variable_chirho(&mut self, name_chirho: &NameChirho) {
        self.report_free_type_variable_text_chirho(
            name_chirho.text_chirho(),
            name_chirho.span_chirho(),
        );
    }

    fn report_free_type_variable_text_chirho(
        &mut self,
        name_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        let key_chirho = (
            TypeScopeIssueChirho::FREE_TYPE_VARIABLE_CHIRHO,
            name_chirho.to_string(),
            span_chirho,
        );
        if self.reported_chirho.insert(key_chirho) {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(UNDEFINED_TYPE_CODE_CHIRHO),
                    format!("type variable not in scope: `{name_chirho}`"),
                    span_chirho,
                ));
        }
    }
}

fn type_alias_may_be_type_data_recovery_chirho(
    type_vars_chirho: &[haskelujah_ast_chirho::decl_chirho::TyVarChirho],
    rhs_chirho: &TypeChirho,
) -> bool {
    type_vars_chirho.is_empty()
        && matches!(
            rhs_chirho,
            TypeChirho::ConChirho(name_chirho)
                if is_promotable_constructor_spelling_chirho(name_chirho.text_chirho())
        )
}

fn collect_instance_associated_type_scope_chirho(
    module_chirho: &ModuleChirho,
    available_modules_chirho: &[ModuleIfaceChirho],
) -> HashMap<String, HashSet<String>> {
    let mut scope_chirho = HashMap::new();

    for decl_chirho in &module_chirho.decls_chirho {
        let DeclChirho::ClassDeclChirho {
            name_chirho,
            associated_tfs_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        scope_chirho.insert(
            canonical_type_name_chirho(name_chirho.text_chirho()),
            associated_tfs_chirho
                .iter()
                .map(|family_chirho| {
                    canonical_type_name_chirho(family_chirho.name_chirho.text_chirho())
                })
                .collect(),
        );
    }

    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        let Some(iface_chirho) = available_modules_chirho
            .iter()
            .rev()
            .find(|iface_chirho| iface_chirho.name_chirho == module_name_chirho)
        else {
            continue;
        };
        let visible_types_chirho: HashSet<String> =
            compute_imported_names_chirho(&iface_chirho.exports_chirho, &import_chirho.spec_chirho)
                .into_iter()
                .filter_map(|(name_chirho, namespace_chirho, _span_chirho)| {
                    (namespace_chirho == NamespaceChirho::TypeChirho)
                        .then(|| canonical_type_name_chirho(&name_chirho))
                })
                .collect();
        let qualifier_chirho = import_chirho
            .alias_chirho
            .as_ref()
            .map(|alias_chirho| alias_chirho.text_chirho().to_string())
            .unwrap_or(module_name_chirho);

        for (parent_name_chirho, associated_names_chirho) in
            &iface_chirho.exports_chirho.associated_types_chirho
        {
            let parent_name_chirho = canonical_type_name_chirho(parent_name_chirho);
            if !visible_types_chirho.contains(&parent_name_chirho) {
                continue;
            }
            let visible_members_chirho: HashSet<String> = associated_names_chirho
                .iter()
                .map(|name_chirho| canonical_type_name_chirho(name_chirho))
                .filter(|name_chirho| visible_types_chirho.contains(name_chirho))
                .collect();
            scope_chirho.insert(
                format!("{qualifier_chirho}.{parent_name_chirho}"),
                visible_members_chirho.clone(),
            );
            if !import_chirho.qualified_chirho {
                scope_chirho.insert(parent_name_chirho, visible_members_chirho);
            }
        }
    }

    scope_chirho
}

fn record_field_type_scope_reliable_chirho(ty_chirho: &TypeChirho) -> bool {
    match ty_chirho {
        TypeChirho::ForallChirho { .. } => false,
        TypeChirho::KindAnnotChirho {
            type_chirho,
            kind_chirho,
            ..
        } => {
            record_field_type_scope_reliable_chirho(type_chirho)
                && record_field_type_scope_reliable_chirho(kind_chirho)
        }
        TypeChirho::RequiredForallChirho { body_chirho, .. } => {
            record_field_type_scope_reliable_chirho(body_chirho)
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            record_field_type_scope_reliable_chirho(fun_chirho)
                && record_field_type_scope_reliable_chirho(arg_chirho)
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            ..
        } => {
            record_field_type_scope_reliable_chirho(arg_chirho)
                && record_field_type_scope_reliable_chirho(result_chirho)
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => elements_chirho
            .iter()
            .all(record_field_type_scope_reliable_chirho),
        TypeChirho::ListChirho { element_chirho, .. }
        | TypeChirho::ParenChirho {
            inner_chirho: element_chirho,
            ..
        } => record_field_type_scope_reliable_chirho(element_chirho),
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            context_chirho
                .iter()
                .all(record_field_constraint_scope_reliable_chirho)
                && record_field_type_scope_reliable_chirho(body_chirho)
        }
        TypeChirho::VarChirho(_)
        | TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::WildcardChirho { .. }
        | TypeChirho::LitChirho { .. } => true,
    }
}

/// Whether lowering discarded an existential constructor prefix.
///
/// For prefix constructors the CST node starts at `forall` or its context,
/// while the retained constructor name starts later. Infix constructors also
/// have an earlier node start (their left operand), so they are deliberately
/// excluded. Until `ConDeclChirho` can carry existential binders and context,
/// fields behind this exact AST trust boundary must retain implicit scope.
fn constructor_has_unrepresented_scope_prefix_chirho(constructor_chirho: &ConDeclChirho) -> bool {
    let (name_chirho, span_chirho) = match constructor_chirho {
        ConDeclChirho::OrdinaryChirho {
            name_chirho,
            span_chirho,
            ..
        }
        | ConDeclChirho::RecordChirho {
            name_chirho,
            span_chirho,
            ..
        } => (name_chirho, *span_chirho),
        ConDeclChirho::GadtChirho { .. } => return false,
    };
    let name_text_chirho = name_chirho.text_chirho();
    name_text_chirho
        .chars()
        .next()
        .is_some_and(char::is_uppercase)
        && name_chirho.span_chirho().file_id_chirho() == span_chirho.file_id_chirho()
        && name_chirho.span_chirho().start_chirho() > span_chirho.start_chirho()
}

fn record_field_constraint_scope_reliable_chirho(constraint_chirho: &ConstraintChirho) -> bool {
    match constraint_chirho {
        ConstraintChirho::QuantifiedChirho { .. } => false,
        ConstraintChirho::ClassChirho { args_chirho, .. } => args_chirho
            .iter()
            .all(record_field_type_scope_reliable_chirho),
    }
}

/// Check source-level type uses after the module namespace has been collected.
///
/// Workflow: see the type-scope resolution DAG under
/// `spec-chirho/workflows-chirho/compiler-pipeline-chirho`.
pub(crate) fn check_module_type_scope_chirho(
    module_chirho: &ModuleChirho,
    env_chirho: &NameEnvChirho,
    available_modules_chirho: &[ModuleIfaceChirho],
    diagnostics_chirho: &mut DiagnosticBundleChirho,
) {
    TypeScopeWalkerChirho::new_chirho(
        module_chirho,
        env_chirho,
        available_modules_chirho,
        diagnostics_chirho,
    )
    .walk_module_chirho(module_chirho);
}

fn extension_enabled_chirho(module_chirho: &ModuleChirho, name_chirho: &str) -> bool {
    extension_enabled_with_default_chirho(module_chirho, name_chirho, false)
}

fn extension_enabled_with_default_chirho(
    module_chirho: &ModuleChirho,
    name_chirho: &str,
    default_chirho: bool,
) -> bool {
    let disabled_name_chirho = format!("No{name_chirho}");
    module_chirho.extensions_chirho.iter().fold(
        default_chirho,
        |enabled_chirho, extension_chirho| {
            if extension_chirho == name_chirho {
                true
            } else if extension_chirho == &disabled_name_chirho {
                false
            } else {
                enabled_chirho
            }
        },
    )
}

fn is_unqualified_star_chirho(name_chirho: &NameChirho) -> bool {
    let (qualifier_chirho, text_chirho) = name_parts_chirho(name_chirho);
    qualifier_chirho.is_none() && text_chirho == "*"
}

fn is_lexical_type_variable_chirho(name_chirho: &str) -> bool {
    name_chirho != "_"
        && name_chirho
            .chars()
            .next()
            .is_some_and(|first_chirho| first_chirho.is_lowercase() || first_chirho == '_')
}

fn is_promotable_constructor_spelling_chirho(name_chirho: &str) -> bool {
    name_chirho
        .chars()
        .next()
        .is_some_and(|first_chirho| first_chirho.is_uppercase() || first_chirho == ':')
}

fn is_builtin_constraint_name_chirho(name_chirho: &NameChirho) -> bool {
    let (qualifier_chirho, text_chirho) = name_parts_chirho(name_chirho);
    qualifier_chirho.is_none() && BUILTIN_CONSTRAINT_NAMES_CHIRHO.contains(&text_chirho)
}

fn is_builtin_type_name_chirho(name_chirho: &NameChirho) -> bool {
    let (qualifier_chirho, raw_text_chirho) = name_parts_chirho(name_chirho);
    if qualifier_chirho.is_some() {
        return false;
    }
    if matches!(raw_text_chirho, "(##)" | "(# #)")
        || is_boxed_tuple_constructor_text_chirho(raw_text_chirho)
        || is_unboxed_tuple_or_sum_constructor_text_chirho(raw_text_chirho)
    {
        return true;
    }
    let text_chirho = canonical_type_name_chirho(raw_text_chirho);
    if BUILTIN_CONSTRAINT_NAMES_CHIRHO.contains(&text_chirho.as_str())
        || matches!(
            text_chirho.as_str(),
            ":" | "':" | "[]" | "()" | "->" | "(->)" | "(##)" | "(# #)"
        )
    {
        return true;
    }
    false
}

fn is_builtin_promoted_constructor_name_chirho(name_chirho: &NameChirho) -> bool {
    let (qualifier_chirho, text_chirho) = name_parts_chirho(name_chirho);
    qualifier_chirho.is_none()
        && (matches!(text_chirho, ":" | "':" | "[]" | "()")
            || is_boxed_tuple_constructor_text_chirho(text_chirho))
}

fn is_boxed_tuple_constructor_text_chirho(text_chirho: &str) -> bool {
    text_chirho
        .strip_prefix('(')
        .and_then(|inner_chirho| inner_chirho.strip_suffix(')'))
        .is_some_and(|inner_chirho| {
            !inner_chirho.is_empty() && inner_chirho.chars().all(|char_chirho| char_chirho == ',')
        })
}

fn is_unboxed_tuple_or_sum_constructor_text_chirho(text_chirho: &str) -> bool {
    text_chirho
        .strip_prefix("(#")
        .and_then(|inner_chirho| inner_chirho.strip_suffix("#)"))
        .is_some_and(|inner_chirho| {
            !inner_chirho.is_empty()
                && inner_chirho
                    .chars()
                    .all(|char_chirho| matches!(char_chirho, ',' | '|'))
        })
}

fn collect_kind_variable_names_chirho(kind_chirho: &AstKindChirho, names_chirho: &mut Vec<String>) {
    match kind_chirho {
        AstKindChirho::TypeSyntaxChirho(type_chirho) => {
            collect_type_kind_variable_names_chirho(type_chirho, names_chirho);
        }
        AstKindChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        }
        | AstKindChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let mut local_chirho = Vec::new();
            collect_kind_variable_names_chirho(body_chirho, &mut local_chirho);
            for binder_chirho in vars_chirho.iter().rev() {
                local_chirho.retain(|name_chirho| name_chirho != binder_chirho.text_chirho());
                if let Some(annotation_chirho) = &binder_chirho.kind_annotation_chirho {
                    collect_kind_variable_names_chirho(annotation_chirho, &mut local_chirho);
                }
            }
            names_chirho.extend(local_chirho);
        }
        AstKindChirho::VarChirho(name_chirho) if is_lexical_type_variable_chirho(name_chirho) => {
            names_chirho.push(name_chirho.clone());
        }
        AstKindChirho::ArrowChirho(arg_chirho, result_chirho) => {
            collect_kind_variable_names_chirho(arg_chirho, names_chirho);
            collect_kind_variable_names_chirho(result_chirho, names_chirho);
        }
        AstKindChirho::AppChirho(fun_chirho, arg_chirho)
        | AstKindChirho::KindAppChirho(fun_chirho, arg_chirho)
        | AstKindChirho::KindAnnotChirho {
            type_chirho: fun_chirho,
            kind_chirho: arg_chirho,
            ..
        } => {
            collect_kind_variable_names_chirho(fun_chirho, names_chirho);
            collect_kind_variable_names_chirho(arg_chirho, names_chirho);
        }
        AstKindChirho::StarChirho
        | AstKindChirho::ConstraintChirho
        | AstKindChirho::ConChirho(_)
        | AstKindChirho::VarChirho(_) => {}
    }
}

fn collect_type_kind_variable_names_chirho(
    type_chirho: &TypeChirho,
    names_chirho: &mut Vec<String>,
) {
    match type_chirho {
        TypeChirho::VarChirho(name_chirho) => {
            if is_lexical_type_variable_chirho(name_chirho.text_chirho()) {
                names_chirho.push(name_chirho.text_chirho().to_owned());
            }
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAppChirho {
            fun_chirho,
            arg_chirho,
            ..
        }
        | TypeChirho::KindAnnotChirho {
            type_chirho: fun_chirho,
            kind_chirho: arg_chirho,
            ..
        } => {
            collect_type_kind_variable_names_chirho(fun_chirho, names_chirho);
            collect_type_kind_variable_names_chirho(arg_chirho, names_chirho);
        }
        TypeChirho::FunChirho {
            arg_chirho,
            result_chirho,
            mult_chirho,
            ..
        } => {
            collect_type_kind_variable_names_chirho(arg_chirho, names_chirho);
            if let Some(MultiplicityChirho::ExpressionChirho(mult_chirho)) = mult_chirho {
                collect_type_kind_variable_names_chirho(mult_chirho, names_chirho);
            }
            collect_type_kind_variable_names_chirho(result_chirho, names_chirho);
        }
        TypeChirho::TupleChirho {
            elements_chirho, ..
        }
        | TypeChirho::PromotedListChirho {
            elements_chirho, ..
        } => {
            for element_chirho in elements_chirho {
                collect_type_kind_variable_names_chirho(element_chirho, names_chirho);
            }
        }
        TypeChirho::ListChirho {
            element_chirho: inner_chirho,
            ..
        }
        | TypeChirho::ParenChirho { inner_chirho, .. } => {
            collect_type_kind_variable_names_chirho(inner_chirho, names_chirho);
        }
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            ..
        }
        | TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            ..
        } => {
            let mut local_chirho = Vec::new();
            collect_type_kind_variable_names_chirho(body_chirho, &mut local_chirho);
            for binder_chirho in vars_chirho.iter().rev() {
                local_chirho.retain(|name_chirho| name_chirho != binder_chirho.text_chirho());
                if let Some(annotation_chirho) = &binder_chirho.kind_annotation_chirho {
                    collect_kind_variable_names_chirho(annotation_chirho, &mut local_chirho);
                }
            }
            names_chirho.extend(local_chirho);
        }
        TypeChirho::QualChirho {
            context_chirho,
            body_chirho,
            ..
        } => {
            for constraint_chirho in context_chirho {
                collect_constraint_kind_names_chirho(constraint_chirho, names_chirho);
            }
            collect_type_kind_variable_names_chirho(body_chirho, names_chirho);
        }
        TypeChirho::ConChirho(_)
        | TypeChirho::PromotedConChirho { .. }
        | TypeChirho::LitChirho { .. }
        | TypeChirho::WildcardChirho { .. } => {}
    }
}

fn collect_constraint_kind_names_chirho(
    constraint_chirho: &ConstraintChirho,
    names_chirho: &mut Vec<String>,
) {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => {
            if is_lexical_type_variable_chirho(class_chirho.text_chirho()) {
                names_chirho.push(class_chirho.text_chirho().to_owned());
            }
            for argument_chirho in args_chirho {
                collect_type_kind_variable_names_chirho(argument_chirho, names_chirho);
            }
        }
        ConstraintChirho::QuantifiedChirho {
            vars_chirho,
            context_chirho,
            body_chirho,
            ..
        } => {
            let mut local_chirho = Vec::new();
            for constraint_chirho in context_chirho {
                collect_constraint_kind_names_chirho(constraint_chirho, &mut local_chirho);
            }
            collect_constraint_kind_names_chirho(body_chirho, &mut local_chirho);
            for binder_chirho in vars_chirho.iter().rev() {
                local_chirho.retain(|name_chirho| name_chirho != binder_chirho.text_chirho());
                if let Some(annotation_chirho) = &binder_chirho.kind_annotation_chirho {
                    collect_kind_variable_names_chirho(annotation_chirho, &mut local_chirho);
                }
            }
            names_chirho.extend(local_chirho);
        }
    }
}

fn name_parts_chirho(name_chirho: &NameChirho) -> (Option<&str>, &str) {
    match name_chirho {
        NameChirho::RawChirho(raw_chirho) => (
            raw_chirho.qualifier_chirho.as_deref(),
            &raw_chirho.text_chirho,
        ),
        NameChirho::ResolvedChirho(resolved_chirho) => (
            resolved_chirho.raw_chirho.qualifier_chirho.as_deref(),
            &resolved_chirho.raw_chirho.text_chirho,
        ),
    }
}

#[cfg(test)]
#[path = "type_scope_tests_chirho.rs"]
mod tests_chirho;
