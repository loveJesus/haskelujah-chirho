// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin kind terms and runtime-type consumers; workflow: declaration-kinds-chirho.
use super::{KindChirho, KindEnvChirho, KindInferCtxChirho, SpanChirho};
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
        "Type" | "*" => KindChirho::StarChirho,
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
        | "False" => KindChirho::ConChirho(format!("GHC.Types.{name_chirho}")),
        _ => return None,
    })
}

impl KindInferCtxChirho {
    pub(super) fn named_kind_term_chirho(&self, name_chirho: &NameChirho) -> KindChirho {
        let full_chirho = name_chirho.full_name_chirho();
        let text_chirho = name_chirho.text_chirho();
        let builtin_scope_chirho = if full_chirho == text_chirho {
            !self.local_kind_decl_names_chirho.contains(text_chirho)
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
            self.bind_runtime_aliases_chirho(name_chirho, kind_chirho.clone());
            self.bind_promoted_generalized_chirho(name_chirho, kind_chirho);
        }
    }

    pub(super) fn seed_runtime_kinds_chirho(&mut self) {
        for name_chirho in [
            "Type",
            "Constraint",
            "RuntimeRep",
            "Levity",
            "Multiplicity",
            "UnliftedType",
        ] {
            self.bind_runtime_aliases_chirho(name_chirho, KindChirho::StarChirho);
        }
        let representation_chirho = builtin_term_chirho("RuntimeRep").unwrap();
        let levity_chirho = builtin_term_chirho("Levity").unwrap();
        self.bind_runtime_aliases_chirho(
            "TYPE",
            KindChirho::arrow_chirho(representation_chirho.clone(), KindChirho::StarChirho),
        );
        self.bind_runtime_aliases_chirho(
            "BoxedRep",
            KindChirho::arrow_chirho(levity_chirho.clone(), representation_chirho.clone()),
        );
        for name_chirho in ["Lifted", "Unlifted"] {
            self.bind_runtime_aliases_chirho(name_chirho, levity_chirho.clone());
            self.bind_promoted_generalized_chirho(name_chirho, levity_chirho.clone());
        }
        for name_chirho in [
            "LiftedRep",
            "UnliftedRep",
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
            self.bind_runtime_aliases_chirho(name_chirho, representation_chirho.clone());
            self.bind_promoted_generalized_chirho(name_chirho, representation_chirho.clone());
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
