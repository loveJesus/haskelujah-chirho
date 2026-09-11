// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin kind terms and runtime-type consumers; workflow: declaration-kinds-chirho.
use super::{
    KindChirho, KindEnvChirho, KindInferCtxChirho, KindVarChirho, ModuleChirho, SpanChirho,
};
use haskelujah_ast_chirho::name_chirho::NameChirho;

pub(super) const TYPE_CHIRHO: &str = "GHC.Prim.TYPE";
const BOXED_REP_CHIRHO: &str = "GHC.Types.BoxedRep";

pub(super) fn boxed_rep_chirho(levity_chirho: &str) -> KindChirho {
    KindChirho::app_chirho(
        KindChirho::ConChirho(BOXED_REP_CHIRHO.into()),
        KindChirho::ConChirho(format!("GHC.Types.{levity_chirho}")),
    )
}

pub(super) fn runtime_type_chirho(representation_chirho: KindChirho) -> KindChirho {
    KindChirho::app_chirho(
        KindChirho::ConChirho(TYPE_CHIRHO.into()),
        representation_chirho,
    )
}

pub(super) fn builtin_term_chirho(name_chirho: &str) -> Option<KindChirho> {
    Some(match name_chirho {
        "Type" => KindChirho::StarChirho,
        "Constraint" => KindChirho::ConstraintChirho,
        "TYPE" => KindChirho::ConChirho(TYPE_CHIRHO.into()),
        "LiftedRep" => boxed_rep_chirho("Lifted"),
        "UnliftedRep" => boxed_rep_chirho("Unlifted"),
        "UnliftedType" => runtime_type_chirho(boxed_rep_chirho("Unlifted")),
        "Nat" => KindChirho::ConChirho("GHC.TypeNats.Nat".into()),
        "Symbol" => KindChirho::ConChirho("GHC.TypeLits.Symbol".into()),
        "Bool" | "Char" | "Ordering" | "RuntimeRep" | "Levity" | "Multiplicity" | "BoxedRep"
        | "Lifted" | "Unlifted" | "IntRep" | "WordRep" | "Int8Rep" | "Word8Rep" | "Int16Rep"
        | "Word16Rep" | "Int32Rep" | "Word32Rep" | "Int64Rep" | "Word64Rep" | "AddrRep"
        | "FloatRep" | "DoubleRep" | "TupleRep" | "SumRep" | "VecRep" | "Many" | "One" | "True"
        | "False" | "VecCount" | "VecElem" | "Vec2" | "Vec4" | "Vec8" | "Vec16" | "Vec32"
        | "Vec64" | "Int8ElemRep" | "Int16ElemRep" | "Int32ElemRep" | "Int64ElemRep"
        | "Word8ElemRep" | "Word16ElemRep" | "Word32ElemRep" | "Word64ElemRep" | "FloatElemRep"
        | "DoubleElemRep" => KindChirho::ConChirho(format!("GHC.Types.{name_chirho}")),
        _ => return None,
    })
}

impl KindInferCtxChirho {
    /// GHC does not infer representation polymorphism. Default only unsolved
    /// representation/levity holes, not names retained from source annotations.
    /// Visit this inference group's kind terms, not the module-wide environment.
    pub(super) fn default_inferred_runtime_variables_chirho(
        &mut self,
        kind_chirho: &KindChirho,
        named_variables_chirho: &std::collections::HashSet<KindVarChirho>,
    ) {
        let mut pending_chirho = vec![kind_chirho];
        while let Some(term_chirho) = pending_chirho.pop() {
            match term_chirho {
                KindChirho::AppChirho(fun_chirho, argument_chirho) => {
                    if let (
                        KindChirho::ConChirho(name_chirho),
                        KindChirho::VarChirho(variable_chirho),
                    ) = (fun_chirho.as_ref(), argument_chirho.as_ref())
                        && !named_variables_chirho.contains(variable_chirho)
                    {
                        let default_chirho = match name_chirho.as_str() {
                            TYPE_CHIRHO => Some(boxed_rep_chirho("Lifted")),
                            BOXED_REP_CHIRHO => builtin_term_chirho("Lifted"),
                            _ => None,
                        };
                        if let Some(default_chirho) = default_chirho {
                            self.subst_chirho
                                .map_chirho
                                .insert(*variable_chirho, default_chirho);
                        }
                    }
                    pending_chirho.extend([fun_chirho.as_ref(), argument_chirho.as_ref()]);
                }
                KindChirho::ArrowChirho(argument_chirho, result_chirho)
                | KindChirho::DependentChirho {
                    argument_chirho,
                    result_chirho,
                } => {
                    pending_chirho.extend([argument_chirho.as_ref(), result_chirho.as_ref()]);
                }
                _ => {}
            }
        }
    }

    /// Qualified spelling is interpreted through declared imports, never by
    /// stripping an arbitrary prefix. Conflicting aliases remain unresolved.
    pub(super) fn record_source_kind_qualifiers_chirho(&mut self, module_chirho: &ModuleChirho) {
        self.star_is_type_chirho = module_chirho
            .extensions_chirho
            .iter()
            .rev()
            .find_map(|extension_chirho| match extension_chirho.as_str() {
                "NoStarIsType" => Some(false),
                "StarIsType" | "Haskell98" | "Haskell2010" | "GHC2021" | "GHC2024" => Some(true),
                _ => None,
            })
            .unwrap_or(true);
        self.local_kind_module_chirho = Some(module_chirho.name_chirho.full_name_chirho());
        for import_chirho in &module_chirho.imports_chirho {
            let target_chirho = import_chirho.module_chirho.full_name_chirho();
            let alias_chirho = import_chirho
                .alias_chirho
                .as_ref()
                .unwrap_or(&import_chirho.module_chirho)
                .full_name_chirho();
            self.source_kind_qualifiers_chirho
                .entry(alias_chirho)
                .and_modify(|prior_chirho| {
                    if prior_chirho.as_ref() != Some(&target_chirho) {
                        *prior_chirho = None;
                    }
                })
                .or_insert(Some(target_chirho));
        }
    }

    pub(super) fn canonical_kind_name_chirho(&self, name_chirho: &NameChirho) -> String {
        let full_chirho = name_chirho.full_name_chirho();
        let Some((qualifier_chirho, bare_chirho)) = full_chirho.rsplit_once('.') else {
            return full_chirho;
        };
        if self.local_kind_module_chirho.as_deref() == Some(qualifier_chirho)
            && self.local_kind_decl_names_chirho.contains(bare_chirho)
        {
            return bare_chirho.to_owned();
        }
        match self.source_kind_qualifiers_chirho.get(qualifier_chirho) {
            Some(Some(target_chirho)) => format!("{target_chirho}.{bare_chirho}"),
            _ => full_chirho,
        }
    }

    pub(super) fn named_kind_term_chirho(&self, name_chirho: &NameChirho) -> KindChirho {
        if self.is_type_star_syntax_chirho(name_chirho) {
            return KindChirho::StarChirho;
        }
        let full_chirho = self.canonical_kind_name_chirho(name_chirho);
        if self
            .env_chirho
            .lookup_binding_chirho(&full_chirho)
            .is_none()
            && self
                .env_chirho
                .lookup_promoted_binding_chirho(&full_chirho)
                .is_some()
        {
            return self.named_promoted_kind_term_chirho(name_chirho);
        }
        self.named_term_in_namespace_chirho(name_chirho, &self.local_kind_decl_names_chirho)
    }

    /// StarIsType licenses unqualified syntax, not a same-spelled named
    /// operator. Qualified multiplication and NoStarIsType use name lookup.
    pub(super) fn is_type_star_syntax_chirho(&self, name_chirho: &NameChirho) -> bool {
        self.star_is_type_chirho && name_chirho.full_name_chirho() == "*"
    }

    pub(super) fn named_promoted_kind_term_chirho(&self, name_chirho: &NameChirho) -> KindChirho {
        let full_chirho = self.canonical_kind_name_chirho(name_chirho);
        if self
            .local_promoted_constructor_names_chirho
            .contains(&full_chirho)
        {
            return KindChirho::ConChirho(if full_chirho.contains('.') {
                full_chirho
            } else if let Some(module_chirho) = &self.local_kind_module_chirho {
                format!("{module_chirho}.{full_chirho}")
            } else {
                full_chirho
            });
        }
        self.named_term_in_namespace_chirho(
            name_chirho,
            &self.local_promoted_constructor_names_chirho,
        )
    }

    fn named_term_in_namespace_chirho(
        &self,
        name_chirho: &NameChirho,
        local_names_chirho: &std::collections::HashSet<String>,
    ) -> KindChirho {
        let full_chirho = self.canonical_kind_name_chirho(name_chirho);
        let text_chirho = name_chirho.text_chirho();
        let builtin_scope_chirho = if full_chirho == text_chirho {
            !local_names_chirho.contains(text_chirho)
        } else {
            full_chirho
                .rsplit_once('.')
                .is_some_and(|(module_chirho, _)| {
                    matches!(
                        module_chirho,
                        "GHC.Prim"
                            | "GHC.Types"
                            | "GHC.Exts"
                            | "GHC.Internal.Types"
                            | "GHC.TypeLits"
                            | "GHC.TypeNats"
                            | "Data.Kind"
                    )
                })
        };
        if builtin_scope_chirho && let Some(kind_chirho) = builtin_term_chirho(text_chirho) {
            return kind_chirho;
        }
        KindChirho::ConChirho(full_chirho)
    }

    /// Values may have any TYPE representation; a bare arbitrary kind variable
    /// is not proof of TYPE. Inferred ordinary fields still default to Type.
    pub(super) fn check_runtime_kind_chirho(
        &mut self,
        kind_chirho: &KindChirho,
        context_chirho: &str,
        span_chirho: SpanChirho,
    ) {
        let resolved_chirho = self.subst_chirho.apply_chirho(kind_chirho);
        if matches!(&resolved_chirho, KindChirho::AppChirho(fun_chirho, _)
            if matches!(fun_chirho.as_ref(), KindChirho::ConChirho(name_chirho) if name_chirho == TYPE_CHIRHO))
        {
            return;
        }
        self.unify_chirho(
            &resolved_chirho,
            &KindChirho::StarChirho,
            context_chirho,
            span_chirho,
        );
    }
}

impl KindEnvChirho {
    /// These contracts classify literal terms rather than treating all of them
    /// as Type. Workflow: type-level-character-families-chirho.
    pub(super) fn seed_type_literal_kinds_chirho(&mut self) {
        for name_chirho in ["Nat", "Symbol"] {
            self.bind_generalized_chirho(name_chirho.into(), KindChirho::StarChirho);
        }
        for (names_chirho, arguments_chirho, result_chirho) in [
            (
                &["+", "*", "^", "-", "Div", "Mod"][..],
                &["Nat", "Nat"][..],
                "Nat",
            ),
            (&["<=?"][..], &["Nat", "Nat"][..], "Bool"),
            (&["CmpNat"][..], &["Nat", "Nat"][..], "Ordering"),
            (&["AppendSymbol"][..], &["Symbol", "Symbol"][..], "Symbol"),
            (&["CmpSymbol"][..], &["Symbol", "Symbol"][..], "Ordering"),
            (&["Log2"][..], &["Nat"][..], "Nat"),
            (&["CharToNat"][..], &["Char"][..], "Nat"),
            (&["NatToChar"][..], &["Nat"][..], "Char"),
            (&["KnownNat"][..], &["Nat"][..], "Constraint"),
            (&["KnownSymbol"][..], &["Symbol"][..], "Constraint"),
            (&["KnownChar"][..], &["Char"][..], "Constraint"),
        ] {
            let kind_chirho = KindChirho::arrow_n_chirho(
                arguments_chirho
                    .iter()
                    .map(|name_chirho| builtin_term_chirho(name_chirho).unwrap()),
                builtin_term_chirho(result_chirho).unwrap(),
            );
            for name_chirho in names_chirho {
                self.bind_generalized_chirho((*name_chirho).into(), kind_chirho.clone());
            }
        }
        for name_chirho in ["True", "False"] {
            let kind_chirho = builtin_term_chirho("Bool").unwrap();
            self.bind_runtime_constructor_chirho(name_chirho, kind_chirho);
        }
    }

    pub(super) fn seed_runtime_kinds_chirho(&mut self) {
        for name_chirho in ["Type", "Constraint"] {
            self.bind_generalized_chirho(
                format!("Data.Kind.{name_chirho}"),
                KindChirho::StarChirho,
            );
        }
        // A prefix function arrow has the same representation-polymorphic
        // domains as a syntactic function type. Boxed [] remains Type -> Type.
        self.bind_generalized_chirho(
            "->".into(),
            KindChirho::arrow_n_chirho(
                [10_010, 10_011].map(|identity_chirho| {
                    runtime_type_chirho(KindChirho::VarChirho(KindVarChirho(identity_chirho)))
                }),
                KindChirho::StarChirho,
            ),
        );
        for name_chirho in [
            "Type",
            "Constraint",
            "RuntimeRep",
            "Levity",
            "Multiplicity",
            "UnliftedType",
            "VecCount",
            "VecElem",
        ] {
            self.bind_runtime_aliases_chirho(name_chirho, KindChirho::StarChirho);
        }
        let representation_chirho = builtin_term_chirho("RuntimeRep").unwrap();
        let levity_chirho = builtin_term_chirho("Levity").unwrap();
        self.bind_runtime_aliases_chirho(
            "TYPE",
            KindChirho::arrow_chirho(representation_chirho.clone(), KindChirho::StarChirho),
        );
        self.bind_runtime_constructor_chirho(
            "BoxedRep",
            KindChirho::arrow_chirho(levity_chirho.clone(), representation_chirho.clone()),
        );
        for name_chirho in ["TupleRep", "SumRep"] {
            self.bind_runtime_constructor_chirho(
                name_chirho,
                KindChirho::arrow_chirho(
                    KindChirho::app_chirho(
                        KindChirho::ConChirho("[]".into()),
                        representation_chirho.clone(),
                    ),
                    representation_chirho.clone(),
                ),
            );
        }
        self.bind_runtime_constructor_chirho(
            "VecRep",
            KindChirho::arrow_n_chirho(
                [
                    builtin_term_chirho("VecCount").unwrap(),
                    builtin_term_chirho("VecElem").unwrap(),
                ],
                representation_chirho.clone(),
            ),
        );
        for (names_chirho, classifier_chirho) in [
            (&["One", "Many"][..], "Multiplicity"),
            (
                &["Vec2", "Vec4", "Vec8", "Vec16", "Vec32", "Vec64"][..],
                "VecCount",
            ),
            (
                &[
                    "Int8ElemRep",
                    "Int16ElemRep",
                    "Int32ElemRep",
                    "Int64ElemRep",
                    "Word8ElemRep",
                    "Word16ElemRep",
                    "Word32ElemRep",
                    "Word64ElemRep",
                    "FloatElemRep",
                    "DoubleElemRep",
                ][..],
                "VecElem",
            ),
        ] {
            let kind_chirho = builtin_term_chirho(classifier_chirho).unwrap();
            for name_chirho in names_chirho {
                self.bind_runtime_constructor_chirho(name_chirho, kind_chirho.clone());
            }
        }
        for name_chirho in ["Lifted", "Unlifted"] {
            self.bind_runtime_constructor_chirho(name_chirho, levity_chirho.clone());
        }
        // These two names are type aliases, unlike the representation data
        // constructors below, so a same-spelled value does not hide them.
        for name_chirho in ["LiftedRep", "UnliftedRep"] {
            self.bind_runtime_aliases_chirho(name_chirho, representation_chirho.clone());
        }
        for name_chirho in [
            "IntRep",
            "WordRep",
            "Int8Rep",
            "Word8Rep",
            "Int16Rep",
            "Word16Rep",
            "Int32Rep",
            "Word32Rep",
            "Int64Rep",
            "Word64Rep",
            "AddrRep",
            "FloatRep",
            "DoubleRep",
        ] {
            self.bind_runtime_constructor_chirho(name_chirho, representation_chirho.clone());
        }
        for (primitive_chirho, representation_name_chirho) in [
            ("Int#", "IntRep"),
            ("Word#", "WordRep"),
            ("Int8#", "Int8Rep"),
            ("Word8#", "Word8Rep"),
            ("Int16#", "Int16Rep"),
            ("Word16#", "Word16Rep"),
            ("Int32#", "Int32Rep"),
            ("Word32#", "Word32Rep"),
            ("Int64#", "Int64Rep"),
            ("Word64#", "Word64Rep"),
            ("Addr#", "AddrRep"),
            ("Char#", "WordRep"),
            ("Float#", "FloatRep"),
            ("Double#", "DoubleRep"),
        ] {
            self.bind_runtime_aliases_chirho(
                primitive_chirho,
                runtime_type_chirho(builtin_term_chirho(representation_name_chirho).unwrap()),
            );
        }
    }

    fn bind_runtime_constructor_chirho(&mut self, name_chirho: &str, kind_chirho: KindChirho) {
        self.bind_promoted_generalized_chirho(name_chirho, kind_chirho.clone());
        for module_chirho in ["GHC.Types", "GHC.Prim", "GHC.Exts", "GHC.Internal.Types"] {
            self.bind_promoted_generalized_chirho(
                &format!("{module_chirho}.{name_chirho}"),
                kind_chirho.clone(),
            );
        }
    }

    fn bind_runtime_aliases_chirho(&mut self, name_chirho: &str, kind_chirho: KindChirho) {
        self.bind_generalized_chirho(name_chirho.into(), kind_chirho.clone());
        for module_chirho in ["GHC.Types", "GHC.Prim", "GHC.Exts", "GHC.Internal.Types"] {
            self.bind_generalized_chirho(
                format!("{module_chirho}.{name_chirho}"),
                kind_chirho.clone(),
            );
        }
    }
}
