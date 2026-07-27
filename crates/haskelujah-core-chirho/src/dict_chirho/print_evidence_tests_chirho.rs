// For God so loved the world, that he gave his only begotten Son, that whosoever believeth
// in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::{HashMap, HashSet};

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::env_chirho::TyEnvChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::dict_pass_module_full_with_method_occurrences_chirho;
use crate::expr_chirho::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho, CoreModuleChirho,
    InlineAnnotationChirho,
};

fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn app_spine_head_id_chirho(expr_chirho: &CoreExprChirho) -> Option<CoreIdChirho> {
    let mut current_chirho = expr_chirho;
    while let CoreExprChirho::AppChirho { fun_chirho, .. } = current_chirho {
        current_chirho = fun_chirho.as_ref();
    }
    match current_chirho {
        CoreExprChirho::VarChirho(id_chirho) => Some(*id_chirho),
        _ => None,
    }
}

#[test]
fn evidenced_print_rewrites_to_show_instance_then_put_str_ln_chirho() {
    let mut names_chirho = HashMap::new();
    names_chirho.insert(CoreIdChirho(0), "main".to_string());
    names_chirho.insert(CoreIdChirho(50), "print".to_string());
    names_chirho.insert(CoreIdChirho(60), "print".to_string());
    names_chirho.insert(CoreIdChirho(100), "$prim_Show_show_Bool".to_string());
    names_chirho.insert(CoreIdChirho(101), "putStrLn".to_string());
    let module_chirho = CoreModuleChirho {
        name_chirho: "PrintEvidenceTest".to_string(),
        bindings_chirho: vec![
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("$prim_Show_show_Bool", 100),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(7)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("putStrLn", 101),
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(8)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
            CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(50))),
                    arg_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "True".to_string(),
                        args_chirho: Vec::new(),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            },
        ],
        names_chirho: names_chirho.clone(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
    };
    let mut canonical_occurrences_chirho = HashMap::new();
    canonical_occurrences_chirho.insert(CoreIdChirho(50), ("print".to_string(), CoreIdChirho(60)));
    let mut evidence_chirho = HashMap::new();
    evidence_chirho.insert(CoreIdChirho(50), ("Show".to_string(), "Bool".to_string()));

    let result_chirho = dict_pass_module_full_with_method_occurrences_chirho(
        &module_chirho,
        names_chirho,
        &TyEnvChirho::new_chirho(),
        &ClassEnvChirho::new_chirho(),
        HashMap::new(),
        HashMap::new(),
        HashSet::new(),
        canonical_occurrences_chirho,
        evidence_chirho,
    );
    let main_chirho = result_chirho
        .module_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "main")
        .expect("main binding should exist");
    let shown_arg_chirho = match &main_chirho.rhs_chirho {
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            let put_str_ln_id_chirho = match fun_chirho.as_ref() {
                CoreExprChirho::VarChirho(id_chirho) => *id_chirho,
                other_chirho => panic!("expected putStrLn function, got {other_chirho:?}"),
            };
            assert_eq!(
                result_chirho
                    .names_chirho
                    .get(&put_str_ln_id_chirho)
                    .map(String::as_str),
                Some("putStrLn")
            );
            arg_chirho.as_ref()
        }
        other_chirho => {
            panic!("expected evidenced print to become putStrLn, got {other_chirho:?}")
        }
    };
    let show_head_chirho = app_spine_head_id_chirho(shown_arg_chirho)
        .expect("shown print argument should be an application");
    assert_eq!(
        result_chirho
            .names_chirho
            .get(&show_head_chirho)
            .map(String::as_str),
        Some("$prim_Show_show_Bool")
    );
}
