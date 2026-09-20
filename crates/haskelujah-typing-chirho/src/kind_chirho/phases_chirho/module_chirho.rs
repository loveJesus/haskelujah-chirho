// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One inference context owns declaration closure, GND and instance evidence.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;

/// Kind-check an already elaborated module without adding declarations.
pub fn infer_module_kinds_chirho(module_chirho: &ModuleChirho) -> KindResultChirho {
    infer_module_kinds_with_imports_chirho(module_chirho, &HashMap::new())
}

pub fn infer_module_kinds_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_chirho: &HashMap<String, KindContractChirho>,
) -> KindResultChirho {
    let ctx_chirho = prepare_module_kinds_chirho(module_chirho, imported_chirho);
    finish_module_kinds_chirho(ctx_chirho, module_chirho)
}

/// The frontend requests GND after declaration closure but before the ordinary
/// instance pass. The final module's ordinal space is shared by both phases.
pub fn infer_module_kinds_with_deriving_chirho(
    module_chirho: &mut ModuleChirho,
    imported_chirho: &HashMap<String, KindContractChirho>,
) -> KindResultChirho {
    let mut ctx_chirho = prepare_module_kinds_chirho(module_chirho, imported_chirho);
    // Preparation closes declaration kinds before GND consumes them below.
    if !ctx_chirho.diagnostics_chirho.has_errors_chirho() {
        ctx_chirho.elaborate_newtype_deriving_chirho(module_chirho);
    }
    finish_module_kinds_chirho(ctx_chirho, module_chirho)
}

fn prepare_module_kinds_chirho(
    module_chirho: &ModuleChirho,
    imported_chirho: &HashMap<String, KindContractChirho>,
) -> KindInferCtxChirho {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    ctx_chirho.seed_imported_kind_contracts_chirho(imported_chirho);
    ctx_chirho.record_source_kind_qualifiers_chirho(module_chirho);
    ctx_chirho.cusks_enabled_chirho =
        constructors_chirho::cusks_enabled_chirho(&module_chirho.extensions_chirho);
    let poly_kinds_enabled_chirho =
        constructors_chirho::poly_kinds_enabled_chirho(&module_chirho.extensions_chirho);
    ctx_chirho.poly_kinds_enabled_chirho = poly_kinds_enabled_chirho;
    ctx_chirho.check_kind_declarations_chirho(module_chirho);

    ctx_chirho
}

fn finish_module_kinds_chirho(
    mut ctx_chirho: KindInferCtxChirho,
    module_chirho: &ModuleChirho,
) -> KindResultChirho {
    let poly_kinds_enabled_chirho = ctx_chirho.poly_kinds_enabled_chirho;
    // Phase 1.5: instance heads. Runs as its own pass so every class kind is
    // established regardless of declaration order (an instance may precede its
    // class in the source). Instance heads were never kind-checked at all, so
    // `class MonadReader a b` + `instance MonadReader Int` was accepted.
    //
    // Under-application is only reported when every head form is one our
    // lowering represents faithfully. It is NOT, under these extensions: a head
    // argument may be a type-level literal (`instance A 0`), a promoted list
    // (`instance All c '[]`), an operator type (`instance Category (->)`), or a
    // variable-headed application (`instance MonadReader r (Reader r)`, which
    // lowers to ONE argument instead of two). None of those reach
    // `types_chirho`, so the head looks short and we would report OUR gap as
    // the program's error. Over-application stays sound either way — a dropped
    // argument can only shorten the head.
    let instance_head_forms_representable_chirho =
        !module_chirho.extensions_chirho.iter().any(|e_chirho| {
            e_chirho == "DataKinds"
                || e_chirho == "PolyKinds"
                || e_chirho == "TypeInType"
                || e_chirho == "TypeOperators"
                || e_chirho == "FlexibleInstances"
        });
    let class_defaults_chirho: HashMap<_, _> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|declaration_chirho| {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                type_vars_chirho,
                associated_tfs_chirho,
                ..
            } = declaration_chirho
            {
                Some((
                    name_chirho.text_chirho(),
                    (
                        type_vars_chirho.as_slice(),
                        associated_tfs_chirho.as_slice(),
                    ),
                ))
            } else {
                None
            }
        })
        .collect();
    for (declaration_index_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
        let DeclChirho::InstanceDeclChirho {
            class_chirho,
            types_chirho,
            span_chirho,
            ..
        } = decl_chirho
        else {
            continue;
        };
        ctx_chirho.check_instance_head_arity_chirho(
            class_chirho.text_chirho(),
            types_chirho,
            instance_head_forms_representable_chirho,
            *span_chirho,
        );
        ctx_chirho.check_associated_instance_equations_chirho(
            declaration_index_chirho,
            decl_chirho,
            class_defaults_chirho
                .get(class_chirho.text_chirho())
                .copied(),
        );
    }

    // Phase 2: Finalize — apply substitution and default unconstrained vars.
    ctx_chirho.finalize_chirho(poly_kinds_enabled_chirho);
    ctx_chirho.check_constraint_synonym_licenses_chirho(module_chirho);

    KindResultChirho {
        contracts_chirho: ctx_chirho.export_kind_contracts_chirho(module_chirho),
        associated_contracts_chirho: ctx_chirho
            .export_associated_kind_contracts_chirho(module_chirho),
        elaboration_chirho: ctx_chirho.finish_kind_elaboration_chirho(module_chirho),
        env_chirho: ctx_chirho.env_chirho,
        diagnostics_chirho: ctx_chirho.diagnostics_chirho,
    }
}
