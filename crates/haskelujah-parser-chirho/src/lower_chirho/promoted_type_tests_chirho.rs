// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

// These are syntax identities, the parser's semantic contract, not Core layout.
fn shape_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::ConChirho(name_chirho) | TypeChirho::VarChirho(name_chirho) => {
            name_chirho.text_chirho().to_owned()
        }
        TypeChirho::PromotedConChirho { name_chirho, .. } => {
            format!("'{}", name_chirho.text_chirho())
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            format!(
                "({} {})",
                shape_chirho(fun_chirho),
                shape_chirho(arg_chirho)
            )
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => shape_chirho(inner_chirho),
        TypeChirho::WildcardChirho { .. } => "_".to_owned(),
        other_chirho => panic!("unexpected tuple component {other_chirho:?}"),
    }
}

fn module_chirho(source_chirho: &str) -> haskelujah_ast_chirho::module_chirho::ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    lower_module_chirho(
        &parse_to_cst_chirho(source_chirho, file_chirho),
        file_chirho,
    )
}

const FORMS_CHIRHO: &[(&str, &str)] = &[
    ("'()", "'()"),
    ("'(,)", "'(,)"),
    ("'(,,)", "'(,,)"),
    ("'(Int, Bool)", "(('(,) Int) Bool)"),
    ("'(Int, Bool, Char)", "((('(,,) Int) Bool) Char)"),
    ("'(,) Int Bool", "(('(,) Int) Bool)"),
    ("'( '(Int, Bool), Char)", "(('(,) (('(,) Int) Bool)) Char)"),
];

#[test]
fn structured_promoted_tuples_retain_constructor_and_operands_chirho() {
    for &(source_chirho, expected_chirho) in FORMS_CHIRHO {
        let module_chirho = module_chirho(&format!(
            "module TuplesChirho where\ntype TupleChirho = {source_chirho}\nfollowingChirho :: Int\nfollowingChirho = 1\n"
        ));
        let rhs_chirho = module_chirho
            .decls_chirho
            .iter()
            .find_map(|declaration_chirho| {
                if let DeclChirho::TypeAliasDeclChirho { rhs_chirho, .. } = declaration_chirho {
                    Some(rhs_chirho)
                } else {
                    None
                }
            })
            .expect("tuple alias survives");
        assert_eq!(shape_chirho(rhs_chirho), expected_chirho, "{source_chirho}");
        assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| matches!(declaration_chirho,
            DeclChirho::TypeSigChirho { name_chirho, .. } if name_chirho.text_chirho() == "followingChirho")));
    }
}

#[test]
fn flat_promoted_tuples_retain_constructor_and_operands_chirho() {
    for &(source_chirho, expected_chirho) in FORMS_CHIRHO {
        let module_chirho = module_chirho(&format!(
            "module TuplesChirho where\ndata HolderChirho = HolderChirho {{ fieldChirho :: ProxyChirho ({source_chirho}) }}\n"
        ));
        let field_chirho = module_chirho
            .decls_chirho
            .iter()
            .find_map(|declaration_chirho| {
                let DeclChirho::DataDeclChirho {
                    constructors_chirho,
                    ..
                } = declaration_chirho
                else {
                    return None;
                };
                let ConDeclChirho::RecordChirho { fields_chirho, .. } =
                    constructors_chirho.first()?
                else {
                    return None;
                };
                Some(&fields_chirho.first()?.ty_chirho)
            })
            .expect("record field survives");
        let TypeChirho::AppChirho { arg_chirho, .. } = field_chirho else {
            panic!("{field_chirho:?}");
        };
        assert_eq!(shape_chirho(arg_chirho), expected_chirho, "{source_chirho}");
    }
}

#[test]
fn promoted_tuple_family_patterns_keep_both_binding_positions_chirho() {
    let module_chirho = module_chirho(
        "module TuplesChirho where\ntype family FirstChirho pairChirho where\n  FirstChirho '(leftChirho, _) = leftChirho\ntype family SecondChirho pairChirho where\n  SecondChirho '(_, rightChirho) = rightChirho\n",
    );
    let patterns_chirho: Vec<_> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|declaration_chirho| {
            let DeclChirho::TypeFamilyDeclChirho {
                equations_chirho, ..
            } = declaration_chirho
            else {
                return None;
            };
            Some(shape_chirho(&equations_chirho.first()?.lhs_types_chirho[0]))
        })
        .collect();
    assert_eq!(
        patterns_chirho,
        ["(('(,) leftChirho) _)", "(('(,) _) rightChirho)"]
    );
}
