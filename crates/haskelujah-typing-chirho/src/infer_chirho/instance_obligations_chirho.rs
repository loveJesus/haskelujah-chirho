// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-time obligations of instance declarations: an instance is not
//! merely registered, it has to be allowed to exist. This module holds the
//! duplicate-instance rule (GHC-59692). Superclass obligations (GHC-39999
//! "arising from the superclasses of an instance declaration") are written and
//! tested but wait for faithful context lowering: main's token scan drops
//! constructor arguments and unsplit tuple contexts without leaving a trace in
//! the AST, so no checker-side guard can tell a mangled context from a real one.
//! workflow: language-features-chirho/instance-obligations-chirho

use super::*;

/// An instance declared by the module under inference.
#[derive(Debug, Clone)]
pub(super) struct LocalInstanceChirho {
    pub(super) instance_chirho: InstDeclChirho,
    /// `instance A` of a nullary class: the head type is a placeholder.
    pub(super) nullary_chirho: bool,
    pub(super) span_chirho: SpanChirho,
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
    /// Types and classes are identified by NAME here, so the non-local half
    /// fires only when no local declaration can shadow a seeded name: the class
    /// is not declared in this module, no head mentions a type declared here
    /// (T11552 defines its own `MaybeT`; T10592 its own `Eq`), and the Prelude
    /// is implicit.
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
            .any(|extension_chirho| extension_chirho == "NoImplicitPrelude");
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
                            && equal_to_later_chirho(&earlier_chirho.instance_chirho)
                    });
            let names_unshadowed_chirho = implicit_prelude_chirho
                && !later_chirho.nullary_chirho
                && !local_names_chirho.contains(&later_chirho.instance_chirho.class_name_chirho)
                && !head_tys_chirho(&later_chirho.instance_chirho).any(|ty_chirho| {
                    mentions_constructor_chirho(ty_chirho, &|name_chirho| {
                        local_names_chirho.contains(name_chirho)
                    })
                });
            let non_local_chirho = names_unshadowed_chirho && {
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
