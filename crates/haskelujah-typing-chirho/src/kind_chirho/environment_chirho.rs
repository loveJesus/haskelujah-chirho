// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Kind binding storage and builtin contracts. Workflow: declaration-kinds-chirho.
use super::{
    KindBindingChirho, KindChirho, KindSchemeChirho, KindSubstChirho, KindVarChirho,
    is_builtin_typelit_or_typenat_kind_name_chirho,
};
use std::collections::HashMap;

/// Type names carry a monomorphic inference kind or an explicitly quantified scheme.
#[derive(Debug, Clone, Default)]
pub struct KindEnvChirho {
    pub(super) bindings_chirho: HashMap<String, KindBindingChirho>,
    // Promoted data constructors are not type constructors of the same spelling.
    // Only authoritative constructor schemes belong here; missing metadata is
    // instantiated opaquely at a use, never stored as a monomorphic type name.
    promoted_bindings_chirho: HashMap<String, KindBindingChirho>,
    changes_chirho: Vec<(String, Option<KindBindingChirho>)>,
    scopes_chirho: Vec<usize>,
}

impl KindEnvChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }
    /// Seed the environment with built-in type constructor kinds.
    pub fn with_builtins_chirho() -> Self {
        let mut env_chirho = Self::new_chirho();

        // Primitive types: kind *
        for name_chirho in &[
            "Int",
            "Int8",
            "Int16",
            "Int32",
            "Int64",
            "Word",
            "Word8",
            "Word16",
            "Word32",
            "Word64",
            "Bool",
            "Char",
            "Double",
            "Float",
            "Integer",
            "String",
            "Buffer",
            "BufferPool",
        ] {
            env_chirho.bind_generalized_chirho(name_chirho.to_string(), KindChirho::StarChirho);
        }

        // * -> * constructors
        let star_to_star_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        for name_chirho in &["Maybe", "[]", "IO"] {
            env_chirho
                .bind_generalized_chirho(name_chirho.to_string(), star_to_star_chirho.clone());
        }

        // Standard higher-kinded class contracts also constrain superclass
        // arguments before a local class is published. Local declarations
        // replace these entries through the same shadowing path as built-in
        // data constructors; this is not a special case at a use site.
        let unary_class_chirho =
            KindChirho::arrow_chirho(star_to_star_chirho.clone(), KindChirho::ConstraintChirho);
        for name_chirho in &[
            "Functor",
            "Applicative",
            "Monad",
            "MonadFail",
            "Alternative",
            "MonadPlus",
            "Foldable",
            "Traversable",
        ] {
            env_chirho.bind_generalized_chirho(name_chirho.to_string(), unary_class_chirho.clone());
        }

        // * -> * -> * constructors
        let star2_chirho = KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho,
        );
        for name_chirho in &["Either", "(,)", "Map"] {
            env_chirho.bind_generalized_chirho(name_chirho.to_string(), star2_chirho.clone());
        }
        let promoted_cons_elem_kind_chirho = KindVarChirho(10_009);
        let promoted_list_kind_chirho = KindChirho::app_chirho(
            KindChirho::ConChirho("[]".into()),
            KindChirho::VarChirho(promoted_cons_elem_kind_chirho),
        );
        let promoted_cons_kind_chirho = KindChirho::arrow_n_chirho(
            vec![
                KindChirho::VarChirho(promoted_cons_elem_kind_chirho),
                promoted_list_kind_chirho.clone(),
            ],
            promoted_list_kind_chirho,
        );
        for name_chirho in &[":", "':"] {
            env_chirho.bind_generalized_chirho(
                name_chirho.to_string(),
                promoted_cons_kind_chirho.clone(),
            );
        }
        env_chirho.promoted_bindings_chirho.insert(
            ":".to_owned(),
            KindBindingChirho::PolyChirho(KindSchemeChirho::generalize_chirho(
                promoted_cons_kind_chirho,
            )),
        );
        env_chirho.bind_generalized_chirho("ST".to_string(), star2_chirho.clone());
        env_chirho.bind_generalized_chirho(
            "StateT".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    star_to_star_chirho.clone(),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        // Tuple constructors: (,,) :: * -> * -> * -> *, etc.
        for arity_chirho in 3u8..=7 {
            let name_chirho = format!("({})", ",".repeat(arity_chirho as usize - 1));
            let kind_chirho = KindChirho::arrow_n_chirho(
                (0..arity_chirho).map(|_| KindChirho::StarChirho),
                KindChirho::StarChirho,
            );
            env_chirho.bind_generalized_chirho(name_chirho, kind_chirho);
        }

        // (->) :: * -> * -> *
        env_chirho.bind_generalized_chirho("->".to_string(), star2_chirho.clone());

        env_chirho.seed_type_literal_kinds_chirho();

        // Poly-kinded builtins that appear in imported package signatures.
        // We model them with explicitly quantified kind variables so each use site can
        // instantiate them independently via scheme instantiation.
        let typeable_kind_var_chirho = KindVarChirho(10_000);
        env_chirho.bind_generalized_chirho(
            "Typeable".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(typeable_kind_var_chirho),
                KindChirho::ConstraintChirho,
            ),
        );

        let proxy_kind_var_chirho = KindVarChirho(10_001);
        env_chirho.bind_generalized_chirho(
            "Proxy".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(proxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_generalized_chirho(
            "Proxy#".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(proxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let kproxy_kind_var_chirho = KindVarChirho(10_002);
        env_chirho.bind_generalized_chirho(
            "KProxy".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(kproxy_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let type_rep_kind_var_chirho = KindVarChirho(10_003);
        env_chirho.bind_generalized_chirho(
            "TypeRep".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(type_rep_kind_var_chirho),
                KindChirho::StarChirho,
            ),
        );
        let equality_kind_var_chirho = KindVarChirho(10_003_1);
        env_chirho.bind_generalized_chirho(
            ":~:".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(equality_kind_var_chirho),
                    KindChirho::VarChirho(equality_kind_var_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        let hetero_eq_left_kind_chirho = KindVarChirho(10_003_2);
        let hetero_eq_right_kind_chirho = KindVarChirho(10_003_3);
        env_chirho.bind_generalized_chirho(
            ":~~:".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(hetero_eq_left_kind_chirho),
                    KindChirho::VarChirho(hetero_eq_right_kind_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        let const_second_kind_var_chirho = KindVarChirho(10_004);
        env_chirho.bind_generalized_chirho(
            "Const".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::StarChirho,
                    KindChirho::VarChirho(const_second_kind_var_chirho),
                ],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_generalized_chirho(
            "CI".to_string(),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        );
        let tagged_first_kind_var_chirho = KindVarChirho(10_005);
        env_chirho.bind_generalized_chirho(
            "Tagged".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(tagged_first_kind_var_chirho),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );

        // GHC.Generics representation constructors and classes.
        let rep_functor_kind_chirho =
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
        let generic_meta_kind_chirho = KindVarChirho(10_006);
        let generic_meta_kind2_chirho = KindVarChirho(10_007);
        env_chirho.bind_generalized_chirho("V1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_generalized_chirho("U1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_generalized_chirho("Par1".to_string(), rep_functor_kind_chirho.clone());
        env_chirho.bind_generalized_chirho(
            "Rec1".to_string(),
            KindChirho::arrow_chirho(
                rep_functor_kind_chirho.clone(),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_generalized_chirho(
            "K1".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(generic_meta_kind_chirho),
                    KindChirho::StarChirho,
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_generalized_chirho(
            "Rec0".to_string(),
            KindChirho::arrow_n_chirho(
                vec![KindChirho::StarChirho, KindChirho::StarChirho],
                KindChirho::StarChirho,
            ),
        );
        env_chirho.bind_generalized_chirho(
            "M1".to_string(),
            KindChirho::arrow_n_chirho(
                vec![
                    KindChirho::VarChirho(generic_meta_kind_chirho),
                    KindChirho::VarChirho(generic_meta_kind2_chirho),
                    rep_functor_kind_chirho.clone(),
                    KindChirho::StarChirho,
                ],
                KindChirho::StarChirho,
            ),
        );
        for generic_sum_name_chirho in &[":+:", ":*:", ":.:"] {
            env_chirho.bind_generalized_chirho(
                (*generic_sum_name_chirho).to_string(),
                KindChirho::arrow_n_chirho(
                    vec![
                        rep_functor_kind_chirho.clone(),
                        rep_functor_kind_chirho.clone(),
                        KindChirho::StarChirho,
                    ],
                    KindChirho::StarChirho,
                ),
            );
        }
        let rep_arg_kind_chirho = KindVarChirho(10_008);
        env_chirho.bind_generalized_chirho(
            "Rep".to_string(),
            KindChirho::arrow_chirho(
                KindChirho::VarChirho(rep_arg_kind_chirho),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_generalized_chirho(
            "Rep1".to_string(),
            KindChirho::arrow_chirho(
                rep_functor_kind_chirho.clone(),
                rep_functor_kind_chirho.clone(),
            ),
        );
        env_chirho.bind_generalized_chirho(
            "Generic".to_string(),
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho),
        );
        env_chirho.bind_generalized_chirho(
            "Generic1".to_string(),
            KindChirho::arrow_chirho(rep_functor_kind_chirho, KindChirho::ConstraintChirho),
        );

        env_chirho.seed_runtime_kinds_chirho();
        env_chirho
    }

    pub(super) fn bind_promoted_generalized_chirho(
        &mut self,
        name_chirho: &str,
        kind_chirho: KindChirho,
    ) {
        self.promoted_bindings_chirho.insert(
            name_chirho.to_owned(),
            KindBindingChirho::PolyChirho(KindSchemeChirho::generalize_chirho(kind_chirho)),
        );
    }

    pub fn bind_chirho(&mut self, name_chirho: String, kind_chirho: KindChirho) {
        self.bind_entry_chirho(name_chirho, KindBindingChirho::MonoChirho(kind_chirho));
    }

    pub(super) fn bind_generalized_chirho(&mut self, name_chirho: String, kind_chirho: KindChirho) {
        self.bind_entry_chirho(
            name_chirho,
            KindBindingChirho::PolyChirho(KindSchemeChirho::generalize_chirho(kind_chirho)),
        );
    }

    pub(super) fn bind_entry_chirho(
        &mut self,
        name_chirho: String,
        binding_chirho: KindBindingChirho,
    ) {
        let previous_chirho = self
            .bindings_chirho
            .insert(name_chirho.clone(), binding_chirho);
        if !self.scopes_chirho.is_empty() {
            self.changes_chirho.push((name_chirho, previous_chirho));
        }
    }

    pub(super) fn lookup_binding_chirho(&self, name_chirho: &str) -> Option<&KindBindingChirho> {
        self.bindings_chirho.get(name_chirho).or_else(|| {
            name_chirho.rsplit_once('.').and_then(|(_, bare_chirho)| {
                is_builtin_typelit_or_typenat_kind_name_chirho(bare_chirho)
                    .then(|| self.bindings_chirho.get(bare_chirho))
                    .flatten()
            })
        })
    }

    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&KindChirho> {
        self.lookup_binding_chirho(name_chirho)
            .map(KindBindingChirho::body_chirho)
    }

    pub(super) fn lookup_promoted_binding_chirho(
        &self,
        name_chirho: &str,
    ) -> Option<&KindBindingChirho> {
        self.promoted_bindings_chirho.get(name_chirho)
    }

    pub fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        for binding_chirho in self
            .bindings_chirho
            .values_mut()
            .chain(self.promoted_bindings_chirho.values_mut())
        {
            binding_chirho.apply_subst_chirho(subst_chirho);
        }
    }

    /// Constructor-local scopes undo only their writes, not a clone of the module environment.
    pub(super) fn begin_scope_chirho(&mut self) {
        self.scopes_chirho.push(self.changes_chirho.len());
    }

    pub(super) fn hide_chirho(&mut self, name_chirho: &str) {
        let previous_chirho = self.bindings_chirho.remove(name_chirho);
        if !self.scopes_chirho.is_empty() {
            self.changes_chirho
                .push((name_chirho.to_owned(), previous_chirho));
        }
    }

    pub(super) fn end_scope_chirho(&mut self) {
        let start_chirho = self.scopes_chirho.pop().expect("kind scope was opened");
        for (name_chirho, previous_chirho) in self.changes_chirho.drain(start_chirho..).rev() {
            if let Some(binding_chirho) = previous_chirho {
                self.bindings_chirho.insert(name_chirho, binding_chirho);
            } else {
                self.bindings_chirho.remove(&name_chirho);
            }
        }
    }

    /// Save only entries written in this scope for a later checking phase,
    /// then restore the outer environment. Repeated writes are captured once.
    pub(super) fn capture_scope_chirho(&mut self) -> Vec<(String, KindBindingChirho)> {
        let start_chirho = *self.scopes_chirho.last().expect("kind scope was opened");
        let mut seen_chirho = std::collections::HashSet::new();
        let mut entries_chirho = Vec::new();
        for (name_chirho, _) in &self.changes_chirho[start_chirho..] {
            if seen_chirho.insert(name_chirho.as_str())
                && let Some(binding_chirho) = self.bindings_chirho.get(name_chirho)
            {
                entries_chirho.push((name_chirho.clone(), binding_chirho.clone()));
            }
        }
        self.end_scope_chirho();
        entries_chirho
    }
}
