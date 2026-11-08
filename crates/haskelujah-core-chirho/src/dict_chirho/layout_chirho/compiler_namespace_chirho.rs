// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The reserved namespace of a displaced compiler class (row 692, C10).
//!
//! A source class that takes a compiler class's spelling owns everything spelled under it: the
//! layout, the selectors, the dictionaries and the rows. The compiler class keeps its own, laid
//! out from its validated compiler record and named under one namespace derived from its
//! identity. Only an occurrence whose proof is that compiler class dispatches through it; every
//! other proof, an undisplaced compiler class, a source class or an unknown one, keeps the
//! spelled route it has always taken. A displaced compiler request this pass cannot serve, a
//! class it could not lay out included, is refused by name and never served from the spelling
//! (gpt #32666). Where no source class displaces a compiler class nothing here exists, and every
//! name stays as it was.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use haskelujah_ast_chirho::name_chirho::{
    CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
};
use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho};
use haskelujah_typing_chirho::ty_chirho::SchemePredChirho;

use crate::dict_chirho::{DictLayoutChirho, DictPassCtxChirho, SuperclassSlotChirho};
use crate::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreLitChirho};

/// The class name a displaced compiler class's own layout, selectors, dictionaries and rows are
/// spelled under. No source class can spell it, since a class name never begins with `$`.
pub(in crate::dict_chirho) fn compiler_namespace_name_chirho(
    identity_chirho: CompilerClassChirho,
) -> String {
    format!("$compiler_{}", identity_chirho.legacy_name_chirho())
}

/// The compiler class a proof proves, if it proves one. A source or unknown proof proves none.
pub(in crate::dict_chirho) fn compiler_identity_chirho(
    proof_chirho: &PredicateClassProofChirho,
) -> Option<CompilerClassChirho> {
    match proof_chirho {
        PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::CompilerChirho(
            identity_chirho,
        )) => Some(*identity_chirho),
        _ => None,
    }
}

/// The validated compiler record of `identity_chirho` when a source class has taken its
/// spelling: the record active under that spelling is not this record. No compiler record, or
/// an ambiguous one, displaces nothing.
fn displaced_record_chirho(
    class_env_chirho: &ClassEnvChirho,
    identity_chirho: CompilerClassChirho,
) -> Option<Arc<ClassDeclChirho>> {
    let record_chirho = class_env_chirho.compiler_class_record_chirho(identity_chirho)?;
    let active_is_compiler_chirho = class_env_chirho
        .classes_chirho()
        .get_chirho(identity_chirho.legacy_name_chirho())
        .is_some_and(|active_chirho| Arc::ptr_eq(active_chirho, record_chirho));
    (!active_is_compiler_chirho).then(|| record_chirho.clone())
}

/// A compiler record's superclasses as compiler identities, in declared order, when every
/// declared superclass has its proven template; otherwise none can be named by proof.
fn proven_superclasses_chirho(record_chirho: &ClassDeclChirho) -> Option<Vec<CompilerClassChirho>> {
    if record_chirho.super_templates_chirho.len() != record_chirho.supers_chirho.len() {
        return None;
    }
    record_chirho
        .super_templates_chirho
        .iter()
        .map(|template_chirho| compiler_identity_chirho(template_chirho.class_identity_chirho()))
        .collect()
}

/// A method name as the method-name selector map is searched: as written, then a qualified
/// name's last part without its parentheses, then that last part (`Prelude.(+)`, `(+)`, `+`).
fn method_spellings_chirho(name_chirho: &str) -> Vec<&str> {
    let mut spellings_chirho = vec![name_chirho];
    if let Some(short_chirho) = name_chirho.rsplit('.').next() {
        if let Some(stripped_chirho) = short_chirho
            .strip_prefix('(')
            .and_then(|inner_chirho| inner_chirho.strip_suffix(')'))
        {
            spellings_chirho.push(stripped_chirho);
        }
        spellings_chirho.push(short_chirho);
    }
    spellings_chirho
}

impl DictPassCtxChirho {
    /// Lay out each displaced compiler class the pass meets: the instances' class proofs, the
    /// occurrence, reference and ground-goal proofs, and `table_identities_chirho`, closing over
    /// their superclasses. A displaced class keeps a namespace only when each of its
    /// superclasses is proven and either undisplaced or itself kept. Otherwise it is still
    /// recorded as displaced, its compiler proofs are refused by name, and none is ever served
    /// from the spelling (gpt #32666, #32683).
    pub(in crate::dict_chirho) fn build_compiler_layouts_chirho(
        &mut self,
        class_env_chirho: &ClassEnvChirho,
        table_identities_chirho: impl IntoIterator<Item = CompilerClassChirho>,
    ) {
        let mut pending_chirho = self.compiler_classes_met_chirho(class_env_chirho);
        pending_chirho.extend(table_identities_chirho);
        let mut seen_chirho = HashSet::new();
        let mut displaced_chirho = Vec::new();
        while let Some(identity_chirho) = pending_chirho.pop() {
            if !seen_chirho.insert(identity_chirho) {
                continue;
            }
            let Some(record_chirho) = displaced_record_chirho(class_env_chirho, identity_chirho)
            else {
                continue;
            };
            let superclasses_chirho = proven_superclasses_chirho(&record_chirho);
            if let Some(superclasses_chirho) = &superclasses_chirho {
                pending_chirho.extend(superclasses_chirho.iter().copied());
            }
            self.displaced_compiler_classes_chirho
                .insert(identity_chirho, record_chirho.clone());
            displaced_chirho.push((identity_chirho, record_chirho, superclasses_chirho));
        }
        let displaced_identities_chirho: HashSet<CompilerClassChirho> = displaced_chirho
            .iter()
            .map(|(identity_chirho, _, _)| *identity_chirho)
            .collect();
        // Keep a class only when its superclasses are proven and each displaced one is kept.
        let mut kept_chirho: HashSet<CompilerClassChirho> = displaced_chirho
            .iter()
            .filter(|(_, _, superclasses_chirho)| superclasses_chirho.is_some())
            .map(|(identity_chirho, _, _)| *identity_chirho)
            .collect();
        loop {
            let refused_chirho: Vec<CompilerClassChirho> = displaced_chirho
                .iter()
                .filter(|(identity_chirho, _, superclasses_chirho)| {
                    kept_chirho.contains(identity_chirho)
                        && superclasses_chirho.iter().flatten().any(|super_chirho| {
                            displaced_identities_chirho.contains(super_chirho)
                                && !kept_chirho.contains(super_chirho)
                        })
                })
                .map(|(identity_chirho, _, _)| *identity_chirho)
                .collect();
            if refused_chirho.is_empty() {
                break;
            }
            for identity_chirho in refused_chirho {
                kept_chirho.remove(&identity_chirho);
            }
        }
        for (identity_chirho, record_chirho, superclasses_chirho) in displaced_chirho {
            let (true, Some(superclasses_chirho)) =
                (kept_chirho.contains(&identity_chirho), superclasses_chirho)
            else {
                continue;
            };
            let super_namespaces_chirho = superclasses_chirho
                .into_iter()
                .map(|super_chirho| {
                    if kept_chirho.contains(&super_chirho) {
                        compiler_namespace_name_chirho(super_chirho)
                    } else {
                        super_chirho.legacy_name_chirho().to_string()
                    }
                })
                .collect();
            let layout_chirho = self.class_layout_chirho(
                &compiler_namespace_name_chirho(identity_chirho),
                super_namespaces_chirho,
                &record_chirho,
            );
            self.compiler_layouts_chirho
                .insert(identity_chirho, layout_chirho);
        }
        // A spelled layout's superclass slot whose validated template proves a kept displaced
        // compiler class takes that class's namespace, even when the class itself is undisplaced:
        // the compiler's `Real` keeps the compiler's `Num`. A refused one keeps its spelling, and
        // its proof makes every filler refuse it by name. This runs before any selector or
        // dictionary is generated (row 692, C10, gpt #32919).
        for layout_chirho in self.layouts_chirho.values_mut() {
            for slot_chirho in &mut layout_chirho.super_slots_chirho {
                if let Some(identity_chirho) =
                    compiler_identity_chirho(&slot_chirho.class_proof_chirho)
                    && kept_chirho.contains(&identity_chirho)
                {
                    slot_chirho.namespace_chirho = compiler_namespace_name_chirho(identity_chirho);
                }
            }
        }
    }

    /// The compiler classes this pass meets by proof, duplicates included.
    fn compiler_classes_met_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
    ) -> Vec<CompilerClassChirho> {
        let mut met_chirho = Vec::new();
        for (_class_name_chirho, instances_chirho) in
            class_env_chirho.instances_chirho().iter_chirho()
        {
            met_chirho.extend(instances_chirho.into_iter().filter_map(|instance_chirho| {
                compiler_identity_chirho(instance_chirho.class_identity_chirho())
            }));
        }
        met_chirho.extend(
            self.occurrence_evidence_chirho
                .values()
                .filter_map(|evidence_chirho| {
                    compiler_identity_chirho(&evidence_chirho.class_identity_chirho)
                }),
        );
        met_chirho.extend(
            self.reference_evidence_chirho
                .values()
                .flatten()
                .filter_map(|reference_chirho| {
                    compiler_identity_chirho(&reference_chirho.class_identity_chirho)
                }),
        );
        met_chirho.extend(self.ground_goals_chirho.values().flatten().filter_map(
            |predicate_chirho| compiler_identity_chirho(predicate_chirho.class_identity_chirho()),
        ));
        met_chirho
    }

    /// The spelled rows a source class displacing `identity_chirho` may own: each of its methods
    /// at each key one of its instances was checked or desugared at. A compiler dictionary never
    /// takes such a row, whoever wrote it (row 692, C10).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn namesake_rows_chirho(
        &self,
        class_env_chirho: &ClassEnvChirho,
        identity_chirho: CompilerClassChirho,
    ) -> HashSet<(String, String)> {
        let spelling_chirho = identity_chirho.legacy_name_chirho();
        let Some(active_chirho) = class_env_chirho
            .classes_chirho()
            .get_chirho(spelling_chirho)
        else {
            return HashSet::new();
        };
        let namesake_chirho = |instance_chirho: &InstDeclChirho| {
            compiler_identity_chirho(instance_chirho.class_identity_chirho())
                != Some(identity_chirho)
        };
        let mut keys_chirho: HashSet<String> = class_env_chirho
            .instances_chirho()
            .get_chirho(spelling_chirho)
            .into_iter()
            .flatten()
            .filter(|instance_chirho| namesake_chirho(instance_chirho))
            .map(|instance_chirho| {
                crate::dict_chirho::instance_chirho::generic_heads_chirho::head_key_chirho(
                    &instance_chirho.head_ty_chirho,
                    &instance_chirho.extra_head_tys_chirho,
                )
            })
            .collect();
        keys_chirho.extend(
            self.instance_rows_chirho
                .proven_rows_chirho()
                .filter(|(instance_chirho, _)| {
                    instance_chirho.class_name_chirho == spelling_chirho
                        && namesake_chirho(instance_chirho)
                })
                .map(|(_, row_key_chirho)| row_key_chirho.to_string()),
        );
        active_chirho
            .methods_chirho
            .keys()
            .flat_map(|method_chirho| {
                keys_chirho
                    .iter()
                    .map(move |key_chirho| (method_chirho.clone(), key_chirho.clone()))
            })
            .collect()
    }

    /// A displaced compiler dictionary's row for `method_name_chirho` at `ty_key_chirho`: the row
    /// its namespace names; else the legacy-spelled row a compiler producer wrote, such as the
    /// Prelude's, when no source instance of the displacing class can own that spelling; else
    /// a row that fails by name with the compiler method's arity. A source row never fills it.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn compiler_method_row_chirho(
        &mut self,
        identity_chirho: CompilerClassChirho,
        namespace_row_chirho: &str,
        method_name_chirho: &str,
        ty_key_chirho: &str,
        namesake_rows_chirho: &HashSet<(String, String)>,
    ) -> CoreIdChirho {
        if let Some(row_chirho) = self.lookup_body_backed_name_id_chirho(namespace_row_chirho) {
            return row_chirho;
        }
        let spelled_row_chirho = format!(
            "$prim_{}_{method_name_chirho}_{ty_key_chirho}",
            identity_chirho.legacy_name_chirho()
        );
        if !namesake_rows_chirho
            .contains(&(method_name_chirho.to_string(), ty_key_chirho.to_string()))
            && let Some(row_chirho) = self.lookup_body_backed_name_id_chirho(&spelled_row_chirho)
        {
            return row_chirho;
        }
        let arity_chirho = self
            .method_arities_chirho
            .get(&(
                compiler_namespace_name_chirho(identity_chirho),
                method_name_chirho.to_string(),
            ))
            .copied()
            .unwrap_or(0);
        self.missing_method_row_chirho(
            None,
            namespace_row_chirho,
            format!(
                "missing method {}.{method_name_chirho} for {ty_key_chirho}",
                identity_chirho.legacy_name_chirho()
            ),
            arity_chirho,
            None,
        )
    }

    /// The displaced compiler classes' selectors, in namespace order so ids are stable, kept
    /// by identity and never in the method-name map the source class owns.
    pub(in crate::dict_chirho) fn generate_compiler_selectors_chirho(&mut self) {
        let mut layouts_chirho: Vec<(CompilerClassChirho, DictLayoutChirho)> = self
            .compiler_layouts_chirho
            .iter()
            .map(|(identity_chirho, layout_chirho)| (*identity_chirho, layout_chirho.clone()))
            .collect();
        layouts_chirho.sort_by(|(_, left_chirho), (_, right_chirho)| {
            left_chirho
                .class_name_chirho
                .cmp(&right_chirho.class_name_chirho)
        });
        for (identity_chirho, layout_chirho) in layouts_chirho {
            for (method_name_chirho, selector_chirho) in
                self.generate_layout_selectors_chirho(&layout_chirho)
            {
                self.compiler_selectors_chirho
                    .insert((identity_chirho, method_name_chirho), selector_chirho);
            }
        }
    }

    /// The compiler class a proof proves when a source class displaces it, whether or not this
    /// pass could lay it out.
    pub(in crate::dict_chirho) fn displaced_identity_chirho(
        &self,
        proof_chirho: &PredicateClassProofChirho,
    ) -> Option<CompilerClassChirho> {
        compiler_identity_chirho(proof_chirho).filter(|identity_chirho| {
            self.displaced_compiler_classes_chirho
                .contains_key(identity_chirho)
        })
    }

    /// Whether an occurrence's proof is a displaced compiler class. Routes that read spelled rows
    /// from syntax rather than evidence leave such an occurrence to the evidence route, which
    /// serves it from the compiler namespace or refuses it by name (gpt #32666).
    pub(in crate::dict_chirho) fn displaced_occurrence_chirho(
        &self,
        id_chirho: CoreIdChirho,
    ) -> bool {
        self.occurrence_evidence_chirho
            .get(&id_chirho)
            .is_some_and(|evidence_chirho| {
                self.displaced_identity_chirho(&evidence_chirho.class_identity_chirho)
                    .is_some()
            })
    }

    /// Whether `method_name_chirho`, by any spelling, is one of the displaced compiler class's
    /// own methods, read from its compiler record.
    fn displaced_method_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        method_name_chirho: &str,
    ) -> bool {
        self.displaced_compiler_classes_chirho
            .get(&identity_chirho)
            .is_some_and(|record_chirho| {
                method_spellings_chirho(method_name_chirho)
                    .into_iter()
                    .any(|spelling_chirho| {
                        record_chirho.methods_chirho.contains_key(spelling_chirho)
                    })
            })
    }

    /// The selector of a displaced compiler class's method, by any spelling of its name. A
    /// method that class does not declare has none: the source class's selector of the same
    /// name is never a substitute.
    pub(in crate::dict_chirho) fn compiler_selector_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        method_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        method_spellings_chirho(method_name_chirho)
            .into_iter()
            .find_map(|spelling_chirho| {
                self.compiler_selectors_chirho
                    .get(&(identity_chirho, spelling_chirho.to_string()))
                    .copied()
            })
    }

    /// The dictionary a displaced compiler class's proof names at `ty_key_chirho`: its own
    /// namespace's, or the binding's own dictionary for an own key. None when this pass has none.
    pub(in crate::dict_chirho) fn compiler_dictionary_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        ty_key_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreIdChirho> {
        let layout_chirho = self.compiler_layouts_chirho.get(&identity_chirho)?;
        self.evidence_dict_for_class_chirho(
            &layout_chirho.class_name_chirho,
            Some(ty_key_chirho),
            dict_vars_chirho,
            evidence_classes_chirho,
            local_instance_dicts_chirho,
        )
    }

    /// A displaced compiler class's method at a proved key: its own selector applied to the
    /// dictionary the key names in its namespace, or to the binding's own dictionary for an own
    /// key. This is the selector form a literal's wrapper keeps, so every backend sees the shape
    /// it already handles. When this pass cannot serve it, its named refusal: never the source
    /// class's spelled route (gpt #32666). None when the proof is not a displaced compiler
    /// class, or the occurrence is not one of that class's methods; the caller's route stands.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn compiler_method_head_chirho(
        &self,
        proof_chirho: &PredicateClassProofChirho,
        method_name_chirho: &str,
        ty_key_chirho: &str,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let identity_chirho = self.displaced_identity_chirho(proof_chirho)?;
        if !self.displaced_method_chirho(identity_chirho, method_name_chirho) {
            return None;
        }
        let served_chirho = self
            .compiler_selector_chirho(identity_chirho, method_name_chirho)
            .zip(self.compiler_dictionary_chirho(
                identity_chirho,
                ty_key_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
            ))
            .map(
                |(selector_chirho, dictionary_chirho)| CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(dictionary_chirho)),
                },
            );
        Some(served_chirho.unwrap_or_else(|| {
            self.displaced_refusal_chirho(
                identity_chirho,
                &format!("method `{method_name_chirho}`"),
                ty_key_chirho,
            )
        }))
    }

    /// The dictionary a displaced compiler class's namespace holds at `ty_key_chirho`, found by
    /// its generated name: only when this pass kept that class and generated the dictionary, so a
    /// field never becomes a fresh unbound id (row 692, C10).
    pub(in crate::dict_chirho) fn compiler_superclass_id_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        ty_key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        if !self.compiler_layouts_chirho.contains_key(&identity_chirho) {
            return None;
        }
        self.existing_id_named_chirho(&format!(
            "$f{}{ty_key_chirho}",
            compiler_namespace_name_chirho(identity_chirho)
        ))
    }

    /// The field a superclass slot holds at `ty_key_chirho`, judged by the slot's class proof,
    /// never its spelling (row 692, C10, gpt #32919): a displaced compiler superclass takes its
    /// own namespace's dictionary, or its named refusal; any other slot keeps its spelled
    /// dictionary, as before.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn superclass_field_chirho(
        &mut self,
        slot_chirho: &SuperclassSlotChirho,
        ty_key_chirho: &str,
    ) -> CoreExprChirho {
        let Some(identity_chirho) = self.displaced_identity_chirho(&slot_chirho.class_proof_chirho)
        else {
            let spelled_chirho = format!("$f{}{ty_key_chirho}", slot_chirho.namespace_chirho);
            return CoreExprChirho::VarChirho(self.resolve_or_fresh_id_chirho(&spelled_chirho));
        };
        // The child's key names this superclass only with the identity-head witness; a
        // transformed head is refused by name, never served at the wrong key (gpt #32942).
        if !slot_chirho.identity_head_chirho {
            return self.displaced_refusal_chirho(
                identity_chirho,
                "superclass at a transformed head",
                ty_key_chirho,
            );
        }
        match self.compiler_superclass_id_chirho(identity_chirho, ty_key_chirho) {
            Some(id_chirho) => CoreExprChirho::VarChirho(id_chirho),
            None => {
                self.displaced_refusal_chirho(identity_chirho, "superclass slot", ty_key_chirho)
            }
        }
    }

    /// The named refusal of a displaced compiler class's request this pass cannot serve. The
    /// module is refused by name through the pass's unsupported list, and the request itself
    /// becomes an error naming it: never the source class's spelled route (gpt #32666).
    pub(in crate::dict_chirho) fn displaced_refusal_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        request_chirho: &str,
        ty_key_chirho: &str,
    ) -> CoreExprChirho {
        self.named_displaced_refusal_chirho(format!(
            "the compiler's `{}` {request_chirho} at `{ty_key_chirho}` has no dictionary this pass can build while the program's own class takes the spelling `{}`",
            identity_chirho.legacy_name_chirho(),
            identity_chirho.legacy_name_chirho()
        ))
    }

    /// One named refusal: recorded for the module's unsupported list, and the error the refused
    /// request becomes.
    fn named_displaced_refusal_chirho(&self, message_chirho: String) -> CoreExprChirho {
        self.displaced_refusals_chirho
            .borrow_mut()
            .insert(message_chirho.clone());
        CoreExprChirho::PrimOpChirho {
            name_chirho: "error".to_string(),
            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
                message_chirho,
            ))],
        }
    }

    /// The class name a reference's evidence for one predicate is paired with a callee's slot
    /// by: its namespace, read from its own class proof as `predicate_namespace_chirho` reads a
    /// scheme predicate's (row 692, gpt #32683).
    pub(in crate::dict_chirho) fn reference_namespace_chirho(
        &self,
        reference_chirho: &haskelujah_typing_chirho::infer_chirho::ReferenceEvidenceChirho,
    ) -> String {
        match self.displaced_identity_chirho(&reference_chirho.class_identity_chirho) {
            Some(identity_chirho) => compiler_namespace_name_chirho(identity_chirho),
            None => reference_chirho.class_name_chirho.clone(),
        }
    }

    /// The dictionary slots of a compiler-owned helper the pass writes (the Prelude's
    /// `fromIntegral`, MaybeT's return and bind), named by their compiler classes. Its body
    /// reaches those classes' selectors by spelling, so a displaced class among them makes the
    /// helper unservable here. It is recorded, and its references are refused by name (row 692,
    /// gpt #32683).
    pub(in crate::dict_chirho) fn register_compiler_helper_slots_chirho(
        &mut self,
        helper_chirho: CoreIdChirho,
        classes_chirho: &[CompilerClassChirho],
    ) {
        if let Some(identity_chirho) = classes_chirho.iter().copied().find(|identity_chirho| {
            self.displaced_compiler_classes_chirho
                .contains_key(identity_chirho)
        }) {
            self.displaced_compiler_helpers_chirho
                .insert(helper_chirho, identity_chirho);
        }
        self.dict_param_bindings_chirho.insert(
            helper_chirho,
            classes_chirho
                .iter()
                .map(|identity_chirho| identity_chirho.legacy_name_chirho().to_string())
                .collect(),
        );
    }

    /// The named refusal of a reference to a compiler-owned helper whose slots name a displaced
    /// class, or None.
    pub(in crate::dict_chirho) fn displaced_helper_refusal_chirho(
        &self,
        written_id_chirho: CoreIdChirho,
    ) -> Option<CoreExprChirho> {
        let canonical_chirho = self.canonical_id_chirho(written_id_chirho);
        let identity_chirho = *self
            .displaced_compiler_helpers_chirho
            .get(&canonical_chirho)?;
        let helper_chirho = self
            .names_chirho
            .get(&canonical_chirho)
            .map_or("a compiler helper", String::as_str);
        Some(self.named_displaced_refusal_chirho(format!(
            "the compiler-owned helper `{helper_chirho}` reaches the compiler's `{}` selectors by spelling, which the program's own class takes here",
            identity_chirho.legacy_name_chirho()
        )))
    }

    /// The class name a scheme predicate's dictionary is decided and keyed under: a displaced
    /// compiler class's reserved namespace, laid out or not, else the predicate's own spelling
    /// (row 692, gpt #32675). The definition and its callers read this one decision.
    pub(in crate::dict_chirho) fn predicate_namespace_chirho(
        &self,
        predicate_chirho: &SchemePredChirho,
    ) -> String {
        match self.displaced_identity_chirho(predicate_chirho.class_identity_chirho()) {
            Some(identity_chirho) => compiler_namespace_name_chirho(identity_chirho),
            None => predicate_chirho.class_name_chirho.clone(),
        }
    }

    /// The layout a class proof selects for the class `name_chirho` spells, by the decision
    /// `predicate_namespace_chirho` makes: a displaced compiler class's own layout, read by the
    /// identity its proof carries, or None when this pass could not lay it out; any other proof,
    /// the spelled layout. A displaced compiler class never takes the spelling a source class
    /// owns, and its identity is never read back from a namespace name (row 692, C10, #33179).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn proof_layout_chirho(
        &self,
        name_chirho: &str,
        proof_chirho: &PredicateClassProofChirho,
    ) -> Option<&DictLayoutChirho> {
        match self.displaced_identity_chirho(proof_chirho) {
            Some(identity_chirho) => self.compiler_layouts_chirho.get(&identity_chirho),
            None => self.layouts_chirho.get(name_chirho),
        }
    }

    /// The class name a compiler class's builtin rows are spelled under: its legacy spelling
    /// when no source class displaces it, its reserved namespace when one does. None for a
    /// displaced class this pass could not lay out: its rows have no namespace to live in, and
    /// the spelling is the source class's.
    pub(in crate::dict_chirho) fn compiler_rows_class_name_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
    ) -> Option<String> {
        match self.compiler_layouts_chirho.get(&identity_chirho) {
            Some(layout_chirho) => Some(layout_chirho.class_name_chirho.clone()),
            None if self
                .displaced_compiler_classes_chirho
                .contains_key(&identity_chirho) =>
            {
                None
            }
            None => Some(identity_chirho.legacy_name_chirho().to_string()),
        }
    }
}
