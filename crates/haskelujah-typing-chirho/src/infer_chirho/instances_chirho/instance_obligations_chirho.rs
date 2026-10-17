// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-time obligations of instance declarations: an instance is not
//! merely registered, it has to be allowed to exist, and its methods have to
//! be what the class says. This module holds the duplicate-instance rule
//! (GHC-59692) and the check of every method against its type at the
//! instance. Superclass obligations (GHC-39999 "arising from the superclasses
//! of an instance declaration") are not checked yet; they are their own
//! measured unit.
//! workflow: language-features-chirho/instance-obligations-chirho

use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

use super::*;

/// An instance declared by the module under inference.
#[derive(Debug, Clone)]
pub(super) struct LocalInstanceChirho {
    pub(super) instance_chirho: InstDeclChirho,
    /// `instance A` of a nullary class: the head type is a placeholder.
    pub(super) nullary_chirho: bool,
    /// The class and every type constructor of the head AS WRITTEN, qualifier
    /// included, in source order. The checker identifies types by bare name,
    /// which is not identity: `Strict.StateT` and `Lazy.StateT` are different
    /// types with one bare name (the constraints package declares
    /// `Lifting Functor` for both). Within one module the same SPELLING does
    /// denote the same entity, so heads are compared only under equal spellings.
    pub(super) written_names_chirho: Vec<String>,
    pub(super) span_chirho: SpanChirho,
}

/// `M.T` and `M.:+:` are qualified; `T`, `:+:` and the operator `.` are not.
/// A module prefix starts with an uppercase letter and ends at a dot that is
/// followed by more of the name.
fn is_qualified_spelling_chirho(written_chirho: &str) -> bool {
    written_chirho
        .chars()
        .next()
        .is_some_and(|first_chirho| first_chirho.is_uppercase())
        && written_chirho
            .find('.')
            .is_some_and(|dot_chirho| dot_chirho + 1 < written_chirho.len())
}

/// The written spellings of an instance head; see `written_names_chirho`.
pub(super) fn written_head_names_chirho(
    class_chirho: &NameChirho,
    types_chirho: &[TypeChirho],
) -> Vec<String> {
    fn collect_chirho(ty_chirho: &TypeChirho, names_chirho: &mut Vec<String>) {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => names_chirho.push(name_chirho.full_name_chirho()),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                collect_chirho(fun_chirho, names_chirho);
                collect_chirho(arg_chirho, names_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                collect_chirho(arg_chirho, names_chirho);
                collect_chirho(result_chirho, names_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    collect_chirho(element_chirho, names_chirho);
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                collect_chirho(element_chirho, names_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                collect_chirho(inner_chirho, names_chirho)
            }
            _ => {}
        }
    }
    let mut names_chirho = vec![class_chirho.full_name_chirho()];
    for ty_chirho in types_chirho {
        collect_chirho(ty_chirho, &mut names_chirho);
    }
    names_chirho
}

fn instance_head_text_chirho(local_chirho: &LocalInstanceChirho) -> String {
    let instance_chirho = &local_chirho.instance_chirho;
    if local_chirho.nullary_chirho {
        return instance_chirho.class_name_chirho.clone();
    }
    let mut text_chirho = format!(
        "{} {}",
        instance_chirho.class_name_chirho, instance_chirho.head_ty_chirho
    );
    for extra_chirho in &instance_chirho.extra_head_tys_chirho {
        text_chirho.push_str(&format!(" {extra_chirho}"));
    }
    text_chirho
}

fn head_tys_chirho(instance_chirho: &InstDeclChirho) -> impl Iterator<Item = &TyChirho> {
    std::iter::once(&instance_chirho.head_ty_chirho).chain(&instance_chirho.extra_head_tys_chirho)
}

/// A head is comparable only when its spine head is a real constructor. Main's
/// lowering turns what it cannot represent in an instance head (type
/// operators, promoted constructors, type-level literals, kind-indexed
/// variables) into fresh type variables, which would make unrelated heads
/// alpha-equal (T11754, T13943, T22647).
fn spine_head_is_constructor_chirho(ty_chirho: &TyChirho) -> bool {
    let mut head_chirho = ty_chirho;
    while let TyChirho::AppChirho(fun_chirho, _arg_chirho) = head_chirho {
        head_chirho = fun_chirho;
    }
    matches!(
        head_chirho,
        TyChirho::ConChirho(_)
            | TyChirho::ListChirho(_)
            | TyChirho::TupleChirho(_)
            | TyChirho::FunChirho(..)
    )
}

fn mentions_constructor_chirho(ty_chirho: &TyChirho, wanted_chirho: &dyn Fn(&str) -> bool) -> bool {
    match ty_chirho {
        TyChirho::ConChirho(name_chirho) => wanted_chirho(name_chirho),
        TyChirho::AppChirho(fun_chirho, arg_chirho) => {
            mentions_constructor_chirho(fun_chirho, wanted_chirho)
                || mentions_constructor_chirho(arg_chirho, wanted_chirho)
        }
        TyChirho::FunChirho(arg_chirho, res_chirho, _) => {
            mentions_constructor_chirho(arg_chirho, wanted_chirho)
                || mentions_constructor_chirho(res_chirho, wanted_chirho)
        }
        TyChirho::TupleChirho(elems_chirho) => elems_chirho
            .iter()
            .any(|elem_chirho| mentions_constructor_chirho(elem_chirho, wanted_chirho)),
        TyChirho::ListChirho(elem_chirho) => {
            mentions_constructor_chirho(elem_chirho, wanted_chirho)
        }
        TyChirho::ForallChirho { body_chirho, .. } => {
            mentions_constructor_chirho(body_chirho, wanted_chirho)
        }
        _ => false,
    }
}

impl InferCtxChirho {
    /// Run once every local instance (deriving included, which the driver
    /// expands to instance declarations) has been registered.
    pub(super) fn check_local_instance_obligations_chirho(&mut self, module_chirho: &ModuleChirho) {
        self.report_duplicate_instances_chirho(module_chirho);
    }

    /// GHC-59692: an instance whose head is alpha-equal to an earlier local
    /// instance's, or to an instance from outside this module (Prelude's
    /// `Eq (a, b)`, tcfail073); an import list cannot hide instances.
    ///
    /// Types and classes are identified by NAME here, so two local instances
    /// are compared only under identical written spellings, and the non-local
    /// half fires only when a written name can denote nothing but the seeded
    /// entity: every name is unqualified, the class is not declared in this
    /// module, no head mentions a type declared here (T11552 defines its own
    /// `MaybeT`; T10592 its own `Eq`), and the whole Prelude is in scope
    /// unqualified (a `hiding` or import list may drop a name that another
    /// import then supplies).
    fn report_duplicate_instances_chirho(&mut self, module_chirho: &ModuleChirho) {
        let mut local_names_chirho: HashSet<String> = HashSet::new();
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
                | DeclChirho::ClassDeclChirho { name_chirho, .. }
                | DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => {
                    local_names_chirho.insert(name_chirho.text_chirho().to_string());
                }
                _ => {}
            }
        }
        let implicit_prelude_chirho = !module_chirho
            .extensions_chirho
            .iter()
            .any(|extension_chirho| extension_chirho == "NoImplicitPrelude")
            // The driver injects `import Prelude` itself; a written one is the
            // same thing unless it is qualified or carries an import/hiding list.
            && module_chirho
                .imports_chirho
                .iter()
                .filter(|import_chirho| import_chirho.module_chirho.full_name_chirho() == "Prelude")
                .all(|import_chirho| {
                    !import_chirho.qualified_chirho && import_chirho.spec_chirho.is_none()
                });
        let instances_chirho = self.local_instances_chirho.clone();
        for (later_index_chirho, later_chirho) in instances_chirho.iter().enumerate() {
            let comparable_chirho = later_chirho.nullary_chirho
                || head_tys_chirho(&later_chirho.instance_chirho)
                    .all(spine_head_is_constructor_chirho);
            if !comparable_chirho {
                continue;
            }
            let equal_to_later_chirho = |other_chirho: &InstDeclChirho| {
                ClassEnvChirho::instance_heads_alpha_equal_chirho(
                    other_chirho,
                    &later_chirho.instance_chirho,
                )
            };
            let earlier_local_chirho =
                instances_chirho[..later_index_chirho]
                    .iter()
                    .any(|earlier_chirho| {
                        earlier_chirho.nullary_chirho == later_chirho.nullary_chirho
                            && earlier_chirho.written_names_chirho
                                == later_chirho.written_names_chirho
                            && equal_to_later_chirho(&earlier_chirho.instance_chirho)
                    });
            let names_unshadowed_chirho = implicit_prelude_chirho
                && !later_chirho.nullary_chirho
                && later_chirho
                    .written_names_chirho
                    .iter()
                    .all(|written_chirho| !is_qualified_spelling_chirho(written_chirho))
                && !local_names_chirho.contains(&later_chirho.instance_chirho.class_name_chirho)
                && !head_tys_chirho(&later_chirho.instance_chirho).any(|ty_chirho| {
                    mentions_constructor_chirho(ty_chirho, &|name_chirho| {
                        local_names_chirho.contains(name_chirho)
                    })
                });
            let non_local_chirho = names_unshadowed_chirho && {
                // Every local instance is registered too, under any spelling.
                let local_copies_chirho = instances_chirho
                    .iter()
                    .filter(|local_chirho| equal_to_later_chirho(&local_chirho.instance_chirho))
                    .count();
                let registered_copies_chirho = self
                    .class_env_chirho
                    .instances_chirho
                    .get(&later_chirho.instance_chirho.class_name_chirho)
                    .map_or(0, |registered_chirho| {
                        registered_chirho
                            .iter()
                            .filter(|registered_chirho| equal_to_later_chirho(registered_chirho))
                            .count()
                    });
                registered_copies_chirho > local_copies_chirho
            };
            if earlier_local_chirho || non_local_chirho {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(UNSATISFIED_CONSTRAINT_CODE_CHIRHO),
                        format!(
                            "duplicate instance declarations: instance {}",
                            instance_head_text_chirho(later_chirho)
                        ),
                        later_chirho.span_chirho,
                    ));
            }
        }
    }
}

/// One instance declaration while its methods are checked: the class, the head
/// as written, its type variables, and its written context as givens over
/// those variables.
struct InstanceUnderCheckChirho {
    class_decl_chirho: ClassDeclChirho,
    head_tys_chirho: Vec<TyChirho>,
    var_map_chirho: HashMap<String, TyVarChirho>,
    context_chirho: Vec<SchemePredChirho>,
    /// False when the written context carries a premise this checker does not
    /// model: a quantified constraint, or a variable-headed one (`c a`, which
    /// lowering keeps as a `?` marker). The givens are then incomplete, so a
    /// method's leftover wanteds are not judged; every other check of the
    /// method still runs (gpt_chirho #27146).
    context_complete_chirho: bool,
    /// The class is declared in this module, so its methods' own quantifiers
    /// carry the names they were written with. A built-in or imported class's
    /// method schemes are seeded with fixed variable numbers and no names
    /// (agy_chirho A01-A04), so those quantifiers stay flexible, as before.
    local_class_chirho: bool,
}

impl InferCtxChirho {
    /// Check every method of every local instance as a binding with a
    /// signature: the method's type at the instance, with the instance's type
    /// variables (and, for a class declared here, the method's own quantifiers)
    /// rigid, and the instance context plus the method's context as givens.
    /// Before this, a method's unsolved wanteds were discarded, the instance
    /// context was never a given, and those variables were flexible, so
    /// `show x` with no `Show T`, or a method fixing the instance's `a` to
    /// `Int`, was accepted.
    ///
    /// The method's context keeps the class's own predicate at the instance
    /// (`Show (T a)` while checking `show` for `instance Show (T a)`), as the
    /// class declaration recorded it: recursion at the instance's own type
    /// and through its context resolves as it always did. A superclass reached
    /// through that given is the instance declaration's superclass obligation,
    /// which is its own unit (gpt_chirho #27158, agy_chirho A08).
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub(super) fn check_instance_methods_against_class_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) {
        let local_classes_chirho: HashSet<&str> = module_chirho
            .decls_chirho
            .iter()
            .filter_map(|decl_chirho| match decl_chirho {
                DeclChirho::ClassDeclChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
                _ => None,
            })
            .collect();
        for decl_chirho in &module_chirho.decls_chirho {
            let DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                methods_chirho,
                ..
            } = decl_chirho
            else {
                continue;
            };
            let Some(class_decl_chirho) = self
                .class_env_chirho
                .classes_chirho
                .get(class_chirho.text_chirho())
                .cloned()
            else {
                continue;
            };
            let mut var_map_chirho = HashMap::new();
            let head_tys_chirho: Vec<TyChirho> = types_chirho
                .iter()
                .map(|ty_chirho| self.ast_type_to_ty_chirho(ty_chirho, &mut var_map_chirho))
                .collect();
            let (context_chirho, context_complete_chirho) =
                self.instance_context_givens_chirho(context_chirho, &mut var_map_chirho);
            let instance_chirho = InstanceUnderCheckChirho {
                class_decl_chirho,
                head_tys_chirho,
                var_map_chirho,
                context_chirho,
                context_complete_chirho,
                local_class_chirho: local_classes_chirho.contains(class_chirho.text_chirho()),
            };
            for method_bind_chirho in methods_chirho {
                let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } = method_bind_chirho
                else {
                    continue;
                };
                let Some(method_scheme_chirho) = instance_chirho
                    .class_decl_chirho
                    .methods_chirho
                    .get(name_chirho.text_chirho())
                else {
                    continue;
                };
                self.check_instance_method_chirho(
                    &instance_chirho,
                    method_scheme_chirho,
                    matches_chirho,
                    *span_chirho,
                );
            }
        }
    }

    /// An instance's written context as givens over its head's variables,
    /// through the conversion a signature's context uses and the SAME variable
    /// map as the head (gpt_chirho #27143). An equality becomes a `~` given,
    /// as at registration. Also returns whether the context is complete.
    fn instance_context_givens_chirho(
        &mut self,
        context_chirho: &[AstConstraintChirho],
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> (Vec<SchemePredChirho>, bool) {
        let (mut preds_chirho, equalities_chirho, unmodelled_chirho) =
            self.convert_written_context_chirho(context_chirho, var_map_chirho);
        let variable_headed_chirho = preds_chirho
            .iter()
            .any(|pred_chirho| pred_chirho.class_name_chirho.starts_with('?'));
        preds_chirho.extend(
            equalities_chirho
                .into_iter()
                .map(|(lhs_chirho, rhs_chirho)| SchemePredChirho {
                    class_name_chirho: "~".to_string(),
                    ty_chirho: lhs_chirho,
                    extra_tys_chirho: vec![rhs_chirho],
                }),
        );
        (preds_chirho, !unmodelled_chirho && !variable_headed_chirho)
    }

    /// One method, checked against its type at this instance. The givens are
    /// live for the WHOLE check, inference, subsumption and the leftover
    /// wanteds alike, so an equality given rewrites a rigid variable wherever
    /// it is compared (agy_chirho A07). Everything the check touches is
    /// restored afterwards, so neither the module's other pending wanteds nor
    /// this method's rigid variables leak into the next.
    ///
    /// The module's pending wanteds and deferred equalities are set aside
    /// BEFORE the given scope opens and restored only after it has closed,
    /// its post-step included: that step discharges and retries whatever is
    /// pending under the givens still installed, and this method's private
    /// context (`instance Show T => C T`) must never solve an obligation of
    /// the rest of the module (gpt_chirho #27163).
    fn check_instance_method_chirho(
        &mut self,
        instance_chirho: &InstanceUnderCheckChirho,
        method_scheme_chirho: &SchemeChirho,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
    ) {
        let env_snapshot_chirho = self.env_chirho.clone();
        let class_env_snapshot_chirho = self.class_env_chirho.clone();
        let scoped_tyvars_snapshot_chirho = self.scoped_tyvars_chirho.clone();
        let skolem_bindings_snapshot_chirho = self.skolem_bindings_chirho.clone();
        let module_preds_chirho = std::mem::take(&mut self.deferred_preds_chirho);
        let module_equalities_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
        if self.scoped_type_variables_chirho {
            self.scoped_tyvars_chirho
                .extend(instance_chirho.var_map_chirho.clone());
        }

        // The method's type at this instance, made rigid as a signature is:
        // the body may use the instance's variables and the method's own
        // quantifiers, but can neither choose nor default them.
        let (method_preds_chirho, method_ty_chirho, method_vars_chirho) = self
            .instantiate_instance_method_expected_parts_chirho(
                &instance_chirho.class_decl_chirho,
                method_scheme_chirho,
                &instance_chirho.head_tys_chirho,
                instance_chirho.local_class_chirho,
            );
        // A fixed order, so the skolems are numbered deterministically.
        let mut vars_chirho: Vec<TyVarChirho> =
            instance_chirho.var_map_chirho.values().copied().collect();
        vars_chirho.sort();
        vars_chirho.extend(method_vars_chirho);
        let mut preds_chirho = instance_chirho.context_chirho.clone();
        preds_chirho.extend(method_preds_chirho);
        let rigid_vars_chirho = vars_chirho.clone();
        let (rigid_ty_chirho, givens_chirho) = self.skolemize_scheme_parts_chirho(&SchemeChirho {
            vars_chirho,
            preds_chirho,
            ty_chirho: method_ty_chirho,
        });
        // The skolems this check just made (not ones reused from an enclosing
        // signature) are open to evidence, as the flexible variables they
        // replace were (`own_parameter_key_chirho`): keyed as the binding's own
        // dictionary parameters they would send `==` under `Ord a` to a
        // dictionary the instance method does not have ("missing STG binding").
        for var_chirho in &rigid_vars_chirho {
            if skolem_bindings_snapshot_chirho
                .lookup_chirho(var_chirho)
                .is_some()
            {
                continue;
            }
            if let Some(TyChirho::ForallVarChirho(skolem_chirho)) =
                self.skolem_bindings_chirho.lookup_chirho(var_chirho)
            {
                self.instance_method_skolems_chirho
                    .insert(skolem_chirho.clone());
            }
        }
        let judged_chirho = instance_chirho.context_complete_chirho
            && self.givens_reach_their_superclasses_chirho(&givens_chirho);

        self.with_given_preds_chirho(givens_chirho, |self_chirho| {
            self_chirho.check_method_body_chirho(
                matches_chirho,
                span_chirho,
                &rigid_ty_chirho,
                judged_chirho,
            )
        });

        // What the method leaves behind. Its wanteds were judged inside the
        // scope, or withheld at a support boundary, and belong to no other
        // binding: they go, as the old truncate sent them. Its deferred
        // equalities that the givens could not settle return to the module's
        // list, where they always went, to be retried and reported at the
        // module's end as before, never dropped.
        let method_equalities_chirho = std::mem::replace(
            &mut self.deferred_equalities_chirho,
            module_equalities_chirho,
        );
        self.deferred_equalities_chirho
            .extend(method_equalities_chirho);
        self.deferred_preds_chirho = module_preds_chirho;

        self.env_chirho = env_snapshot_chirho;
        self.class_env_chirho = class_env_snapshot_chirho;
        self.scoped_tyvars_chirho = scoped_tyvars_snapshot_chirho;
        self.skolem_bindings_chirho = skolem_bindings_snapshot_chirho;
    }

    /// The method body against its rigid expected type, under the givens the
    /// caller has installed and with only this method's obligations pending:
    /// inference, then subsumption, whose substitution the leftover wanteds
    /// see (agy_chirho A05), then, when `judged_chirho`, the judgement of
    /// those leftovers by the module's final check, the rules a top-level
    /// binding gets. The givens first discharge and improve what they can,
    /// functional dependencies included, as a signature's givens do
    /// (agy_chirho A06). Local literal defaulting still decides whether a
    /// wanted holds, but the defaulting substitution the check returns is
    /// discarded, so nothing downstream changes by it: a bounded, check-only
    /// choice, not a claim about runtime evidence.
    fn check_method_body_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
        rigid_ty_chirho: &TyChirho,
        judged_chirho: bool,
    ) {
        let expected_ty_chirho = self.normalize_ty_chirho(rigid_ty_chirho);
        let (method_subst_chirho, inferred_ty_chirho) = self.infer_matches_against_expected_chirho(
            matches_chirho,
            span_chirho,
            &expected_ty_chirho,
        );
        let inferred_norm_chirho =
            self.normalize_ty_chirho(&method_subst_chirho.apply_ty_chirho(&inferred_ty_chirho));
        let expected_norm_chirho =
            self.normalize_ty_chirho(&method_subst_chirho.apply_ty_chirho(&expected_ty_chirho));
        match self.subsume_normalized_chirho(
            &inferred_norm_chirho,
            &expected_norm_chirho,
            span_chirho,
        ) {
            Ok(subsumed_chirho) => self.apply_subst_all_chirho(&subsumed_chirho),
            Err(err_chirho) => self.report_unify_error_chirho(&err_chirho),
        }
        if judged_chirho {
            // The module's own order: a deferred equality settled now
            // (`NoShow ~ b` from a `(a ~ b) =>` callee) decides the wanted it
            // feeds before that wanted is judged (agy_chirho A11).
            let _equalities_chirho = self.retry_deferred_equalities_chirho();
            let _improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
            let _defaults_chirho = self.check_deferred_preds_chirho(&method_subst_chirho);
        }
    }

    /// Whether every given lets the checker reach its superclasses. A class's
    /// superclasses are recorded by NAME, not with their arguments, so a
    /// multi-parameter given whose class has superclasses (`Convert a b` under
    /// `class Show a => Convert a b`) cannot be used for one: main rejects the
    /// top-level form of that program too, although GHC accepts it. With such
    /// a given the leftover-wanted verdict is withheld rather than judged
    /// wrongly (agy_chirho A09). The repair is superclass argument
    /// substitution in the class metadata, an isolated solver unit of its own
    /// (gpt_chirho #27158), not an exception that ignores the extra arguments.
    fn givens_reach_their_superclasses_chirho(&self, givens_chirho: &[PredChirho]) -> bool {
        givens_chirho.iter().all(|given_chirho| {
            given_chirho.extra_tys_chirho.is_empty()
                || self
                    .class_env_chirho
                    .superclasses_chirho(&given_chirho.class_name_chirho)
                    .is_empty()
        })
    }

    /// A class method's type at one instance: the class's variables become the
    /// instance head. With `keep_quantifiers_chirho` the method's own
    /// quantifiers stay themselves, keeping their written names so that the
    /// caller can make them rigid, and are returned; otherwise each becomes a
    /// fresh flexible variable, as before. Returns the method's context, its
    /// type, and the kept quantifiers.
    ///
    /// A class variable the head does not supply becomes a fresh FLEXIBLE
    /// variable, as before, and is never among the quantifiers returned: a
    /// head whose lowering came up short (a variable-headed application lowers
    /// to one argument, `instance MonadReader r (Reader r)`) must not turn the
    /// missing argument into a rigid one.
    pub(super) fn instantiate_instance_method_expected_parts_chirho(
        &mut self,
        class_decl_chirho: &ClassDeclChirho,
        method_scheme_chirho: &SchemeChirho,
        instance_head_tys_chirho: &[TyChirho],
        keep_quantifiers_chirho: bool,
    ) -> (Vec<SchemePredChirho>, TyChirho, Vec<TyVarChirho>) {
        let mut head_subst_chirho = SubstChirho::empty_chirho();
        let class_vars_chirho: Vec<TyVarChirho> = std::iter::once(class_decl_chirho.var_chirho)
            .chain(class_decl_chirho.extra_vars_chirho.iter().copied())
            .collect();
        for (index_chirho, class_var_chirho) in class_vars_chirho.iter().enumerate() {
            let head_ty_chirho = match instance_head_tys_chirho.get(index_chirho) {
                Some(head_ty_chirho) => head_ty_chirho.clone(),
                None => self.fresh_var_chirho(),
            };
            head_subst_chirho.insert_chirho(*class_var_chirho, head_ty_chirho);
        }
        let mut method_vars_chirho: Vec<TyVarChirho> = Vec::new();
        for method_var_chirho in &method_scheme_chirho.vars_chirho {
            if head_subst_chirho.lookup_chirho(method_var_chirho).is_some() {
                continue;
            }
            if keep_quantifiers_chirho {
                method_vars_chirho.push(*method_var_chirho);
            } else {
                let fresh_chirho = self.fresh_var_chirho();
                head_subst_chirho.insert_chirho(*method_var_chirho, fresh_chirho);
            }
        }
        let preds_chirho: Vec<SchemePredChirho> = method_scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| SchemePredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(
                    &head_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                ),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|ty_chirho| {
                        self.normalize_ty_chirho(&head_subst_chirho.apply_ty_chirho(ty_chirho))
                    })
                    .collect(),
            })
            .collect();
        let ty_chirho = self.normalize_ty_chirho(
            &head_subst_chirho.apply_ty_chirho(&method_scheme_chirho.ty_chirho),
        );
        (preds_chirho, ty_chirho, method_vars_chirho)
    }

    #[cfg(test)]
    pub(super) fn instantiate_instance_method_expected_ty_chirho(
        &mut self,
        class_decl_chirho: &ClassDeclChirho,
        method_scheme_chirho: &SchemeChirho,
        instance_head_tys_chirho: &[TyChirho],
    ) -> TyChirho {
        let (_preds_chirho, ty_chirho, _vars_chirho) = self
            .instantiate_instance_method_expected_parts_chirho(
                class_decl_chirho,
                method_scheme_chirho,
                instance_head_tys_chirho,
                false,
            );
        ty_chirho
    }
}
