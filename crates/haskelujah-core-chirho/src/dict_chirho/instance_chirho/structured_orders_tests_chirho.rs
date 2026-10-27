// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

fn binding_chirho(id_chirho: u32, name_chirho: &str) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho: CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::IntChirho(0)),
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

#[test]
fn structured_ord_rows_require_body_backed_components_chirho() {
    let names_chirho = HashMap::from([
        (CoreIdChirho(1), "$prim_Ord_compare_Int".to_string()),
        (CoreIdChirho(2), "$prim_Ord_compare_Char".to_string()),
        (CoreIdChirho(3), "$prim_Ord_compare_FooChirho".to_string()),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    pass_chirho
        .generated_bindings_chirho
        .push(binding_chirho(1, "$prim_Ord_compare_Int"));
    pass_chirho
        .generated_bindings_chirho
        .push(binding_chirho(2, "$prim_Ord_compare_Char"));
    pass_chirho.occurrence_evidence_chirho.insert(
        CoreIdChirho(10),
        ("Ord".to_string(), "Maybe Int".to_string()),
    );
    pass_chirho.occurrence_evidence_chirho.insert(
        CoreIdChirho(11),
        ("Ord".to_string(), "(Maybe Int,Char)".to_string()),
    );
    pass_chirho.occurrence_evidence_chirho.insert(
        CoreIdChirho(12),
        ("Ord".to_string(), "Maybe FooChirho".to_string()),
    );

    pass_chirho.generate_evidenced_ord_bindings_chirho(
        &crate::expr_chirho::CoreModuleChirho {
            name_chirho: "TestChirho".to_string(),
            bindings_chirho: vec![],
            names_chirho: HashMap::new(),
            specialize_pragmas_chirho: HashMap::new(),
            foreign_exports_chirho: vec![],
            provenance_chirho: Default::default(),
        },
        &HashMap::new(),
    );

    for name_chirho in [
        "$prim_Ord_compare_Maybe Int",
        "$prim_Ord_compare_(Maybe Int,Char)",
    ] {
        assert!(
            pass_chirho
                .lookup_dispatch_body_name_id_chirho(name_chirho)
                .is_some(),
            "{name_chirho} must have a real generated body"
        );
    }
    assert!(
        pass_chirho
            .lookup_body_backed_name_id_chirho("$prim_Ord_compare_Maybe FooChirho")
            .is_none(),
        "a dangling component name must not produce a Maybe row"
    );
}
