// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Lower through the existing lexical-name producer, then use IDs exclusively.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::{
    decl_chirho::DeclChirho,
    expr_chirho::{ExprChirho as AstExprChirho, LocalBindChirho, RhsChirho},
    lit_chirho::LitChirho,
    module_chirho::ModuleChirho as AstModuleChirho,
    name_chirho::NameChirho,
    pat_chirho::PatChirho,
    ty_chirho::TypeChirho,
};
use haskelujah_naming_chirho::env_chirho::{NameEnvChirho, NamespaceChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::*;

#[path = "classes_lower_chirho.rs"]
mod classes_lower_chirho;
#[path = "data_lower_chirho.rs"]
mod data_lower_chirho;
#[path = "defaults_lower_chirho.rs"]
mod defaults_lower_chirho;
#[path = "instances_lower_chirho.rs"]
mod instances_lower_chirho;
#[path = "literals_lower_chirho.rs"]
mod literals_lower_chirho;
#[path = "patterns_lower_chirho.rs"]
mod patterns_lower_chirho;
#[path = "signatures_chirho.rs"]
mod signatures_chirho;
#[path = "syntax_chirho/mod.rs"]
mod syntax_chirho;

#[path = "interface_chirho/lower_chirho.rs"]
mod imports_lower_chirho;

type ResultChirho<TChirho> = Result<TChirho, HirErrorChirho>;

struct LowerChirho<'literals_chirho> {
    env_chirho: NameEnvChirho,
    names_chirho: Vec<NameInfoChirho>,
    module_chirho: String,
    unit_chirho: UnitIdentityChirho,
    next_expr_chirho: u32,
    limits_chirho: LimitsChirho,
    explicit_forall_chirho: bool,
    rebindable_chirho: bool,
    implicit_prelude_chirho: bool,
    prelude_exports_chirho: Option<std::sync::Arc<PreludeExportsChirho>>,
    imported_names_chirho: std::collections::BTreeMap<OriginalNameChirho, NameIdChirho>,
    import_scope_chirho: HashMap<(String, NamespaceChirho), Vec<NameIdChirho>>,
    imported_classes_chirho: Vec<ClassDeclChirho>,
    sliced_units_chirho: std::collections::BTreeSet<UnitIdentityChirho>,
    integer_literals_chirho:
        &'literals_chirho HashMap<haskelujah_ast_chirho::provenance_chirho::OriginIdChirho, String>,
}

impl LowerChirho<'_> {
    fn error_chirho(
        &self,
        message_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> HirErrorChirho {
        HirErrorChirho {
            kind_chirho: HirErrorKindChirho::UnsupportedChirho,
            message_chirho: message_chirho.into(),
            span_chirho,
        }
    }

    fn room_chirho(&self, depth_chirho: usize, span_chirho: SpanChirho) -> ResultChirho<()> {
        if self.names_chirho.len() + self.next_expr_chirho as usize
            >= self.limits_chirho.nodes_chirho
            || depth_chirho > self.limits_chirho.depth_chirho
        {
            return Err(HirErrorChirho {
                kind_chirho: HirErrorKindChirho::ResourceChirho,
                message_chirho: "next HIR module allocation/depth ceiling exhausted".into(),
                span_chirho,
            });
        }
        Ok(())
    }

    fn bind_chirho(
        &mut self,
        text_chirho: &str,
        namespace_chirho: NamespaceChirho,
        span_chirho: SpanChirho,
        local_chirho: bool,
        wired_chirho: Option<WiredNameChirho>,
    ) -> ResultChirho<NameIdChirho> {
        self.room_chirho(0, span_chirho)?;
        // Bound the actual name strings before the environment/catalogue copies them.
        self.charge_names_chirho(
            if wired_chirho.is_some() {
                "GHC.Internal.Types".len()
            } else {
                self.module_chirho.len()
            } + text_chirho.len().saturating_mul(2),
            span_chirho,
        )?;
        let id_chirho =
            self.env_chirho
                .bind_chirho(text_chirho.into(), namespace_chirho, span_chirho);
        // This environment has no hidden seeds; each positive producer appends once.
        if id_chirho.0 as usize != self.names_chirho.len() {
            return Err(self.error_chirho("name producer and HIR catalogue disagree", span_chirho));
        }
        self.names_chirho.push(NameInfoChirho {
            namespace_chirho,
            unit_chirho: if wired_chirho.is_some() {
                UnitIdentityChirho::CompilerChirho
            } else {
                self.unit_chirho.clone()
            },
            module_chirho: if wired_chirho.is_some() {
                "GHC.Internal.Types".into()
            } else {
                self.module_chirho.clone()
            },
            occurrence_chirho: text_chirho.into(),
            local_chirho,
            wired_chirho,
            span_chirho,
        });
        Ok(NameIdChirho(id_chirho.0))
    }

    fn bound_name_chirho(
        &self,
        name_chirho: &NameChirho,
        namespace_chirho: NamespaceChirho,
    ) -> ResultChirho<NameIdChirho> {
        self.env_chirho
            .lookup_chirho(name_chirho.text_chirho(), namespace_chirho)
            .map(|info_chirho| NameIdChirho(info_chirho.def_id_chirho.0))
            .ok_or_else(|| {
                self.invalid_class_chirho(
                    "binding has no declaration-owned name",
                    name_chirho.span_chirho(),
                )
            })
    }

    fn lookup_chirho(
        &self,
        name_chirho: &NameChirho,
        namespace_chirho: NamespaceChirho,
    ) -> ResultChirho<NameIdChirho> {
        let text_chirho = name_chirho.text_chirho();
        let tuple_chirho = text_chirho.starts_with('(')
            && text_chirho.ends_with(')')
            && text_chirho[1..text_chirho.len() - 1]
                .chars()
                .all(|character_chirho| character_chirho == ',');
        if matches!(text_chirho, ":" | "[]") || tuple_chirho {
            return Err(self.error_chirho(
                "wired syntax constructor expression is not yet modeled",
                name_chirho.span_chirho(),
            ));
        }
        if name_chirho
            .full_name_chirho()
            .starts_with(&format!("{}.", self.module_chirho))
        {
            return Err(self.error_chirho(
                "self-qualified home references need a top-level-only scope lookup",
                name_chirho.span_chirho(),
            ));
        }
        self.lookup_text_chirho(
            &name_chirho.full_name_chirho(),
            namespace_chirho,
            name_chirho.span_chirho(),
        )
    }

    fn type_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        depth_chirho: usize,
    ) -> ResultChirho<SurfaceTypeChirho> {
        self.room_chirho(depth_chirho, ty_chirho.span_chirho())?;
        self.next_expr_chirho += 1;
        Ok(match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let id_chirho = self.lookup_chirho(name_chirho, NamespaceChirho::TypeChirho)?;
                match self.names_chirho[id_chirho.0 as usize].wired_chirho {
                    Some(WiredNameChirho::BoolChirho) => SurfaceTypeChirho::BoolChirho,
                    Some(WiredNameChirho::CharChirho) => SurfaceTypeChirho::CharChirho,
                    Some(WiredNameChirho::IntegerChirho) => SurfaceTypeChirho::IntegerChirho,
                    _ => SurfaceTypeChirho::DeclaredChirho(id_chirho),
                }
            }
            TypeChirho::VarChirho(name_chirho) => SurfaceTypeChirho::VariableChirho(
                self.lookup_chirho(name_chirho, NamespaceChirho::TypeChirho)?,
            ),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => SurfaceTypeChirho::AppChirho(
                Box::new(self.type_chirho(fun_chirho, depth_chirho + 1)?),
                Box::new(self.type_chirho(arg_chirho, depth_chirho + 1)?),
            ),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                mult_chirho: None,
                ..
            } => SurfaceTypeChirho::FunChirho(
                Box::new(self.type_chirho(arg_chirho, depth_chirho + 1)?),
                Box::new(self.type_chirho(result_chirho, depth_chirho + 1)?),
            ),
            TypeChirho::ListChirho { element_chirho, .. } => SurfaceTypeChirho::ListChirho(
                Box::new(self.type_chirho(element_chirho, depth_chirho + 1)?),
            ),
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } if elements_chirho.len() != 1 => SurfaceTypeChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|item_chirho| self.type_chirho(item_chirho, depth_chirho + 1))
                    .collect::<ResultChirho<_>>()?,
            ),
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.type_chirho(inner_chirho, depth_chirho + 1)?
            }
            _ => return Err(self.error_chirho(
                "next S0 requires a checked monomorphic type; this type form is not implemented",
                ty_chirho.span_chirho(),
            )),
        })
    }

    fn node_chirho(
        &mut self,
        kind_chirho: ExprKindChirho,
        span_chirho: SpanChirho,
    ) -> ResultChirho<ExprChirho> {
        self.room_chirho(0, span_chirho)?;
        let id_chirho = ExprIdChirho(self.next_expr_chirho);
        self.next_expr_chirho += 1;
        Ok(ExprChirho {
            id_chirho,
            kind_chirho,
            span_chirho,
        })
    }

    fn rhs_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => {
                self.expr_chirho(expr_chirho, depth_chirho + 1)
            }
            _ => Err(self.error_chirho(
                "next does not yet elaborate guarded right-hand sides",
                SpanChirho::DUMMY_CHIRHO,
            )),
        }
    }

    fn local_bindings_chirho(
        &mut self,
        bindings_chirho: &[LocalBindChirho],
        depth_chirho: usize,
    ) -> ResultChirho<Vec<BindingChirho>> {
        let mut signatures_chirho = HashMap::new();
        let mut declared_chirho = HashSet::new();
        for binding_chirho in bindings_chirho {
            match binding_chirho {
                LocalBindChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    if !declared_chirho.insert(name_chirho.full_name_chirho()) {
                        return Err(self.error_chirho("duplicate local binding", *span_chirho));
                    }
                    self.bind_chirho(
                        name_chirho.text_chirho(),
                        NamespaceChirho::ValueChirho,
                        *span_chirho,
                        true,
                        None,
                    )?;
                }
                LocalBindChirho::TypeSigChirho {
                    name_chirho,
                    ty_chirho,
                    ..
                } => {
                    if signatures_chirho
                        .insert(name_chirho.full_name_chirho(), ty_chirho)
                        .is_some()
                    {
                        return Err(self
                            .error_chirho("duplicate local signature", name_chirho.span_chirho()));
                    }
                }
                _ => {
                    return Err(self.error_chirho(
                        "next does not yet elaborate local pattern bindings",
                        SpanChirho::DUMMY_CHIRHO,
                    ));
                }
            }
        }
        let mut result_chirho = Vec::new();
        for binding_chirho in bindings_chirho {
            if let LocalBindChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
            } = binding_chirho
            {
                let (signature_chirho, body_chirho) = self.binding_parts_chirho(
                    signatures_chirho.remove(&name_chirho.full_name_chirho()),
                    matches_chirho,
                    *span_chirho,
                    depth_chirho + 1,
                )?;
                result_chirho.push(BindingChirho {
                    name_chirho: self
                        .bound_name_chirho(name_chirho, NamespaceChirho::ValueChirho)?,
                    restricted_chirho: signature_chirho.is_none()
                        && matches_chirho
                            .first()
                            .is_some_and(|arm_chirho| arm_chirho.pats_chirho.is_empty()),
                    body_chirho,
                    signature_chirho,
                    span_chirho: *span_chirho,
                });
            }
        }
        if !signatures_chirho.is_empty() {
            return Err(
                self.error_chirho("local signature has no binding", SpanChirho::DUMMY_CHIRHO)
            );
        }
        Ok(result_chirho)
    }

    fn expr_chirho(
        &mut self,
        expression_chirho: &AstExprChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        let span_chirho = expression_chirho.span_chirho();
        self.room_chirho(depth_chirho, span_chirho)?;
        let kind_chirho = match expression_chirho {
            AstExprChirho::VarChirho(name_chirho) | AstExprChirho::ConChirho(name_chirho) => ExprKindChirho::VarChirho(self.lookup_chirho(name_chirho, NamespaceChirho::ValueChirho)?),
            AstExprChirho::LitChirho(literal_chirho @ LitChirho::IntChirho(..)) => return self.integer_expression_chirho(literal_chirho),
            AstExprChirho::NegChirho { expr_chirho, .. } => {
                if matches!(expr_chirho.as_ref(), AstExprChirho::InfixChirho {..}) {
                    return Err(self.error_chirho("prefix negation needs resolved-name fixity regrouping", span_chirho));
                }
                let argument_chirho = self.expr_chirho(expr_chirho, depth_chirho + 1)?;
                return self.syntax_application_chirho("negate", vec![argument_chirho], span_chirho);
            }
            AstExprChirho::LitChirho(LitChirho::CharChirho(value_chirho, ..)) => ExprKindChirho::CharChirho(*value_chirho),
            AstExprChirho::LitChirho(LitChirho::StringChirho(value_chirho, ..)) => ExprKindChirho::StringChirho(value_chirho.clone()),
            AstExprChirho::InfixChirho {left_chirho, op_chirho, right_chirho, ..} => return self.infix_chirho(left_chirho, op_chirho, right_chirho, span_chirho, depth_chirho),
            AstExprChirho::AppChirho { fun_chirho, arg_chirho, .. } => ExprKindChirho::AppChirho(Box::new(self.expr_chirho(fun_chirho, depth_chirho + 1)?), Box::new(self.expr_chirho(arg_chirho, depth_chirho + 1)?)),
            AstExprChirho::LamChirho { pats_chirho, body_chirho, .. } => return self.equation_chirho(&[haskelujah_ast_chirho::expr_chirho::MatchArmChirho {
                pats_chirho: pats_chirho.clone(), rhs_chirho: RhsChirho::UnguardedChirho((**body_chirho).clone()), where_binds_chirho: Vec::new(), span_chirho,
            }], span_chirho, depth_chirho + 1),
            AstExprChirho::CaseChirho { scrutinee_chirho, alts_chirho, .. } => self.case_chirho(scrutinee_chirho, alts_chirho, depth_chirho + 1)?,
            AstExprChirho::LetChirho { binds_chirho, body_chirho, .. } => {
                self.env_chirho.push_scope_chirho();
                let result_chirho = (|| {
                    let bindings_chirho = self.local_bindings_chirho(binds_chirho, depth_chirho + 1)?;
                    Ok(ExprKindChirho::LetChirho(bindings_chirho, Box::new(self.expr_chirho(body_chirho, depth_chirho + 1)?)))
                })();
                self.env_chirho.pop_scope_chirho();
                result_chirho?
            }
            AstExprChirho::IfChirho { cond_chirho, then_chirho, else_chirho, .. } if self.rebindable_chirho => {
                let condition_chirho = self.expr_chirho(cond_chirho, depth_chirho + 1)?;
                let yes_chirho = self.expr_chirho(then_chirho, depth_chirho + 1)?;
                let no_chirho = self.expr_chirho(else_chirho, depth_chirho + 1)?;
                return self.syntax_application_chirho("ifThenElse", vec![condition_chirho, yes_chirho, no_chirho], span_chirho);
            }
            AstExprChirho::IfChirho { cond_chirho, then_chirho, else_chirho, .. } => ExprKindChirho::IfChirho(Box::new(self.expr_chirho(cond_chirho, depth_chirho + 1)?), Box::new(self.expr_chirho(then_chirho, depth_chirho + 1)?), Box::new(self.expr_chirho(else_chirho, depth_chirho + 1)?)),
            AstExprChirho::AnnChirho { expr_chirho, ty_chirho, .. } => ExprKindChirho::AnnChirho(Box::new(self.expr_chirho(expr_chirho, depth_chirho + 1)?), self.type_chirho(ty_chirho, depth_chirho + 1)?),
            AstExprChirho::ListChirho { elements_chirho, .. } => ExprKindChirho::ListChirho(elements_chirho.iter().map(|item_chirho| self.expr_chirho(item_chirho, depth_chirho + 1)).collect::<ResultChirho<_>>()?),
            AstExprChirho::TupleChirho { elements_chirho, .. } if elements_chirho.len() != 1 => ExprKindChirho::TupleChirho(elements_chirho.iter().map(|item_chirho| self.expr_chirho(item_chirho, depth_chirho + 1)).collect::<ResultChirho<_>>()?),
            AstExprChirho::ParenChirho { inner_chirho, .. } => return self.expr_chirho(inner_chirho, depth_chirho + 1),
            _ => return Err(self.error_chirho("next does not yet elaborate this expression (numeric literals require the real Num declaration)", span_chirho)),
        };
        self.node_chirho(kind_chirho, span_chirho)
    }
}

pub fn lower_module_chirho(
    module_chirho: &AstModuleChirho,
    unit_chirho: UnitIdentityChirho,
    limits_chirho: LimitsChirho,
) -> ResultChirho<ModuleChirho> {
    lower_module_with_integer_literals_chirho(
        module_chirho,
        unit_chirho,
        limits_chirho,
        &HashMap::new(),
    )
}

pub fn lower_module_with_integer_literals_chirho(
    module_chirho: &AstModuleChirho,
    unit_chirho: UnitIdentityChirho,
    limits_chirho: LimitsChirho,
    integer_literals_chirho: &HashMap<
        haskelujah_ast_chirho::provenance_chirho::OriginIdChirho,
        String,
    >,
) -> ResultChirho<ModuleChirho> {
    lower_module_with_interfaces_chirho(
        module_chirho,
        unit_chirho,
        limits_chirho,
        integer_literals_chirho,
        &std::collections::BTreeMap::new(),
    )
}

pub fn lower_module_with_interfaces_chirho(
    module_chirho: &AstModuleChirho,
    unit_chirho: UnitIdentityChirho,
    limits_chirho: LimitsChirho,
    integer_literals_chirho: &HashMap<
        haskelujah_ast_chirho::provenance_chirho::OriginIdChirho,
        String,
    >,
    interfaces_chirho: &std::collections::BTreeMap<String, std::sync::Arc<InterfaceChirho>>,
) -> ResultChirho<ModuleChirho> {
    lower_module_with_policy_chirho(
        module_chirho,
        unit_chirho,
        limits_chirho,
        integer_literals_chirho,
        interfaces_chirho,
        &LoweringPolicyChirho::default(),
    )
}

pub fn lower_module_with_policy_chirho(
    module_chirho: &AstModuleChirho,
    unit_chirho: UnitIdentityChirho,
    limits_chirho: LimitsChirho,
    integer_literals_chirho: &HashMap<
        haskelujah_ast_chirho::provenance_chirho::OriginIdChirho,
        String,
    >,
    interfaces_chirho: &std::collections::BTreeMap<String, std::sync::Arc<InterfaceChirho>>,
    policy_chirho: &LoweringPolicyChirho,
) -> ResultChirho<ModuleChirho> {
    let rebindable_chirho = module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| extension_chirho == "RebindableSyntax");
    let mut lower_chirho = LowerChirho {
        rebindable_chirho,
        implicit_prelude_chirho: !rebindable_chirho
            && !module_chirho
                .extensions_chirho
                .iter()
                .any(|flag_chirho| flag_chirho == "NoImplicitPrelude")
            && !module_chirho
                .imports_chirho
                .iter()
                .any(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Prelude")
            && module_chirho.name_chirho.full_name_chirho() != "Prelude",
        prelude_exports_chirho: policy_chirho.prelude_exports_chirho.clone(),
        imported_names_chirho: Default::default(),
        import_scope_chirho: Default::default(),
        imported_classes_chirho: Vec::new(),
        sliced_units_chirho: if policy_chirho.completeness_chirho == CompletenessChirho::SliceChirho
        {
            std::collections::BTreeSet::from([unit_chirho.clone()])
        } else {
            Default::default()
        },
        integer_literals_chirho,
        env_chirho: NameEnvChirho::new_chirho(),
        names_chirho: Vec::new(),
        module_chirho: module_chirho.name_chirho.full_name_chirho(),
        unit_chirho,
        next_expr_chirho: 0,
        limits_chirho,
        explicit_forall_chirho: module_chirho
            .extensions_chirho
            .iter()
            .rev()
            .find(|extension_chirho| matches!(extension_chirho.as_str(), "GHC2021" | "Haskell2010"))
            .is_none_or(|edition_chirho| edition_chirho == "GHC2021"),
    };
    if !module_chirho.foreign_exports_chirho.is_empty() {
        return Err(lower_chirho.error_chirho(
            "next does not yet validate foreign exports",
            module_chirho.span_chirho,
        ));
    }
    if module_chirho
        .extensions_chirho
        .iter()
        .any(|extension_chirho| {
            !matches!(
                extension_chirho.as_str(),
                // Retain/validate the declaration; an actual default signature still declines.
                "Haskell2010"
                    | "GHC2021"
                    | "NoImplicitPrelude"
                    | "DefaultSignatures"
                    | "RebindableSyntax"
            )
        })
    {
        return Err(lower_chirho.error_chirho(
            "next S0 does not yet implement this extension profile",
            module_chirho.span_chirho,
        ));
    }
    let imports_chirho = lower_chirho.import_interfaces_chirho(
        module_chirho,
        interfaces_chirho,
        &policy_chirho.implicit_interfaces_chirho,
    )?;
    if lower_chirho.implicit_prelude_chirho {
        for (text_chirho, namespace_chirho, wired_chirho) in [
            (
                "Bool",
                NamespaceChirho::TypeChirho,
                WiredNameChirho::BoolChirho,
            ),
            (
                "Char",
                NamespaceChirho::TypeChirho,
                WiredNameChirho::CharChirho,
            ),
            (
                "True",
                NamespaceChirho::ValueChirho,
                WiredNameChirho::TrueChirho,
            ),
            (
                "False",
                NamespaceChirho::ValueChirho,
                WiredNameChirho::FalseChirho,
            ),
        ] {
            let id_chirho = lower_chirho.bind_chirho(
                text_chirho,
                namespace_chirho,
                SpanChirho::DUMMY_CHIRHO,
                false,
                Some(wired_chirho),
            )?;
            lower_chirho
                .import_scope_chirho
                .entry((text_chirho.into(), namespace_chirho))
                .or_default()
                .push(id_chirho);
        }
    }
    lower_chirho.predeclare_type_headers_chirho(&module_chirho.decls_chirho)?;
    let data_chirho = lower_chirho.data_declarations_chirho(&module_chirho.decls_chirho)?;
    let classes_chirho =
        lower_chirho.class_declarations_chirho(&module_chirho.decls_chirho, &data_chirho)?;
    let mut signatures_chirho = HashMap::new();
    let mut declared_chirho: HashSet<_> = data_chirho
        .iter()
        .flat_map(|data_chirho| data_chirho.constructors_chirho.iter())
        .map(|constructor_chirho| {
            lower_chirho.names_chirho[constructor_chirho.name_chirho.0 as usize]
                .occurrence_chirho
                .clone()
        })
        .collect();
    declared_chirho.extend(
        classes_chirho
            .iter()
            .flat_map(|class_chirho| &class_chirho.methods_chirho)
            .map(|method_chirho| {
                lower_chirho.names_chirho[method_chirho.name_chirho.0 as usize]
                    .occurrence_chirho
                    .clone()
            }),
    );
    for declaration_chirho in &module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::FunBindChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                if !declared_chirho.insert(name_chirho.full_name_chirho()) {
                    return Err(HirErrorChirho {
                        kind_chirho: HirErrorKindChirho::InvalidChirho,
                        message_chirho: "duplicate top-level value declaration".into(),
                        span_chirho: *span_chirho,
                    });
                }
                lower_chirho.bind_chirho(
                    name_chirho.text_chirho(),
                    NamespaceChirho::ValueChirho,
                    *span_chirho,
                    false,
                    None,
                )?;
            }
            DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } => {
                if signatures_chirho
                    .insert(name_chirho.full_name_chirho(), ty_chirho)
                    .is_some()
                {
                    return Err(lower_chirho
                        .error_chirho("duplicate type signature", name_chirho.span_chirho()));
                }
            }
            DeclChirho::DataDeclChirho { .. }
            | DeclChirho::ClassDeclChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. } => {}
            _ => {
                return Err(lower_chirho.error_chirho(
                    "next S0 does not yet elaborate this declaration",
                    module_chirho.span_chirho,
                ));
            }
        }
    }
    let mut bindings_chirho = Vec::new();
    for declaration_chirho in &module_chirho.decls_chirho {
        if let DeclChirho::FunBindChirho {
            name_chirho,
            matches_chirho,
            span_chirho,
        } = declaration_chirho
        {
            let (signature_chirho, body_chirho) = lower_chirho.binding_parts_chirho(
                signatures_chirho.remove(&name_chirho.full_name_chirho()),
                matches_chirho,
                *span_chirho,
                0,
            )?;
            bindings_chirho.push(BindingChirho {
                name_chirho: lower_chirho
                    .bound_name_chirho(name_chirho, NamespaceChirho::ValueChirho)?,
                restricted_chirho: signature_chirho.is_none()
                    && matches_chirho
                        .first()
                        .is_some_and(|arm_chirho| arm_chirho.pats_chirho.is_empty()),
                body_chirho,
                signature_chirho,
                span_chirho: *span_chirho,
            });
        }
    }
    if !signatures_chirho.is_empty() {
        return Err(lower_chirho.error_chirho(
            "type signature has no value binding",
            module_chirho.span_chirho,
        ));
    }
    // All selectors and top-level names exist before any default RHS is lowered.
    // Signed hidden defaults use the ordinary dependency planner/checker/Core path.
    bindings_chirho.extend(
        lower_chirho.default_bindings_chirho(&module_chirho.decls_chirho, &classes_chirho)?,
    );
    let mut available_classes_chirho = lower_chirho.imported_classes_chirho.clone();
    available_classes_chirho.extend(classes_chirho.iter().cloned());
    let instances_chirho = lower_chirho
        .instance_declarations_chirho(&module_chirho.decls_chirho, &available_classes_chirho)?;
    let exports_chirho = lower_chirho.exports_chirho(module_chirho)?;
    let fixities_chirho = lower_chirho.fixities_chirho(&module_chirho.decls_chirho)?;
    Ok(ModuleChirho {
        unit_chirho: lower_chirho.unit_chirho,
        module_chirho: lower_chirho.module_chirho,
        names_chirho: lower_chirho.names_chirho,
        imports_chirho,
        exports_chirho,
        fixities_chirho,
        completeness_chirho: policy_chirho.completeness_chirho,
        sliced_units_chirho: lower_chirho.sliced_units_chirho,
        data_chirho,
        classes_chirho,
        instances_chirho,
        bindings_chirho,
    })
}
