// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
    InlineAnnotationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

fn binding_chirho(
    id_chirho: u32,
    name_chirho: &str,
    rhs_chirho: CoreExprChirho,
) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        rhs_chirho,
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

/// A pass with body-backed `==` rows for Int (id 1) and Bool (id 2), the generated
/// missing-method stub for `()` (id 3), and Eq evidence at `keys_chirho` on occurrences of
/// `consumer_chirho`.
fn pass_chirho(consumer_chirho: &str, keys_chirho: &[&str]) -> DictPassCtxChirho {
    let names_chirho = HashMap::from([
        (CoreIdChirho(1), "$prim_Eq_==_Int".to_string()),
        (CoreIdChirho(2), "$prim_Eq_==_Bool".to_string()),
        (CoreIdChirho(3), "$prim_Eq_==_()".to_string()),
    ]);
    let mut pass_chirho = DictPassCtxChirho::new_chirho(100, names_chirho, HashMap::new());
    let row_chirho = CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0));
    let stub_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "error".to_string(),
        args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(
            "missing method Eq.== for ()".to_string(),
        ))],
    };
    pass_chirho.generated_bindings_chirho.extend([
        binding_chirho(1, "$prim_Eq_==_Int", row_chirho.clone()),
        binding_chirho(2, "$prim_Eq_==_Bool", row_chirho),
        binding_chirho(3, "$prim_Eq_==_()", stub_chirho),
    ]);
    for (index_chirho, key_chirho) in keys_chirho.iter().enumerate() {
        let occurrence_chirho = CoreIdChirho(10 + index_chirho as u32);
        pass_chirho.occurrence_evidence_chirho.insert(
            occurrence_chirho,
            ("Eq".to_string(), key_chirho.to_string()),
        );
        pass_chirho.method_occurrence_canon_chirho.insert(
            occurrence_chirho,
            (consumer_chirho.to_string(), CoreIdChirho(9)),
        );
    }
    pass_chirho.generate_evidenced_eq_bindings_chirho();
    pass_chirho
}

fn reaches_chirho(expr_chirho: &CoreExprChirho, id_chirho: CoreIdChirho) -> bool {
    let mut found_chirho =
        matches!(expr_chirho, CoreExprChirho::VarChirho(seen_chirho) if *seen_chirho == id_chirho);
    crate::transform_chirho::children_chirho(expr_chirho, &mut |child_chirho| {
        found_chirho = found_chirho || reaches_chirho(child_chirho, id_chirho);
    });
    found_chirho
}

/// Row 603: structured Eq keys compose rows from their components' rows, and only from
/// real bodies. A `()` component is a missing-method stub, so its keys stay unserved.
#[test]
fn structured_eq_rows_compose_only_from_body_backed_components_chirho() {
    let pass_chirho = pass_chirho(
        "==",
        &[
            "(Int,Bool)",
            "Maybe Int",
            "[Maybe Bool]",
            "(Int,())",
            "Maybe ()",
        ],
    );
    for served_chirho in ["(Int,Bool)", "Maybe Int", "[Maybe Bool]", "Maybe Bool"] {
        assert!(
            pass_chirho.existing_eq_row_chirho(served_chirho).is_some(),
            "{served_chirho}"
        );
    }
    for unserved_chirho in ["(Int,())", "Maybe ()"] {
        assert!(
            pass_chirho
                .lookup_body_backed_name_id_chirho(&format!("$prim_Eq_==_{unserved_chirho}"))
                .is_none(),
            "{unserved_chirho}"
        );
    }
    let tuple_row_chirho = pass_chirho
        .existing_eq_row_chirho("(Int,Bool)")
        .expect("served above");
    let tuple_binding_chirho = pass_chirho
        .generated_bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.id_chirho == tuple_row_chirho)
        .expect("the row is generated");
    for field_row_chirho in [CoreIdChirho(1), CoreIdChirho(2)] {
        assert!(reaches_chirho(
            &tuple_binding_chirho.rhs_chirho,
            field_row_chirho
        ));
    }
}

/// Row 603: a consumer at a routable key gets its compiler-owned By-helper with a real
/// body; one at an Int-shaped key gets none.
#[test]
fn routable_consumers_get_compiler_owned_helpers_chirho() {
    let routed_chirho = pass_chirho("isSuffixOf", &["Maybe Int"]);
    for helper_chirho in ["$isSuffixOfBy", "$isPrefixOfBy"] {
        assert!(
            routed_chirho
                .lookup_dispatch_body_name_id_chirho(helper_chirho)
                .is_some(),
            "{helper_chirho}"
        );
    }
    let int_shaped_chirho = pass_chirho("lookup", &["Int"]);
    assert!(
        int_shaped_chirho
            .lookup_body_backed_name_id_chirho("$lookupBy")
            .is_none()
    );
}
