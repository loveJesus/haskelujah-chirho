// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::FileIdChirho;

use crate::cst_parser_chirho::parse_to_cst_chirho;
use crate::lower_chirho::lower_module_chirho;

fn lower_source_chirho(source_chirho: &str) -> haskelujah_ast_chirho::module_chirho::ModuleChirho {
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_id_chirho);
    lower_module_chirho(&cst_chirho, file_id_chirho)
}

#[test]
fn data_family_header_survives_ast_lowering_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE DataKinds #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module FamilyHeaderChirho where\n",
        "data family SingChirho (aChirho :: kChirho) :: Type\n",
    ));

    let family_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_kind_chirho,
                equations_chirho,
                ..
            } if name_chirho.text_chirho() == "SingChirho" => {
                Some((type_vars_chirho, result_kind_chirho, equations_chirho))
            }
            _ => None,
        })
        .expect("data-family header should lower to the shared family declaration shape");

    assert!(
        family_decl_chirho
            .0
            .iter()
            .any(|type_var_chirho| { type_var_chirho.name_chirho.text_chirho() == "aChirho" })
    );
    assert!(family_decl_chirho.1.is_some());
    assert!(family_decl_chirho.2.is_empty());
}

#[test]
fn skipped_data_instance_preserves_following_declaration_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE GADTs #-}\n",
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module DataInstanceBoundaryChirho where\n",
        "data family FamilyChirho typeChirho\n",
        "data instance FamilyChirho Int where\n",
        "  FamilyConstructorChirho :: FamilyChirho Int\n",
        "followingChirho :: Int\n",
        "followingChirho = 1\n",
    ));

    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::TypeSigChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "followingChirho"
        )
    }));
}

#[test]
fn skipped_newtype_instance_preserves_following_declaration_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module NewtypeInstanceBoundaryChirho where\n",
        "data family FamilyChirho\n",
        "newtype instance FamilyChirho = FamilyConstructorChirho Int\n",
        "  deriving Eq\n",
        "data FollowingChirho = FollowingChirho\n",
    ));

    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::DataDeclChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "FollowingChirho"
        )
    }));
}

#[test]
fn gadt_record_constructor_preserves_following_type_signature_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE GADTs #-}\n",
        "module GadtRecordBoundaryChirho where\n",
        "data RecordChirho where\n",
        "  RecordConstructorChirho :: { fieldChirho :: Int } -> RecordChirho\n",
        "followingChirho :: Int\n",
        "followingChirho = 1\n",
    ));

    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::TypeSigChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "followingChirho"
        )
    }));
}

#[test]
fn type_family_instance_preserves_following_fundep_class_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE FunctionalDependencies #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE TypeFamilies #-}\n",
        "{-# LANGUAGE TypeOperators #-}\n",
        "module TypeInstanceClassBoundaryChirho where\n",
        "layoutProbeChirho =\n",
        "  case True of { True ->\n",
        "  case True of\n",
        "    False -> True }\n",
        "type family ForallV :: k -> Constraint\n",
        "type instance ForallV = ForallV_\n",
        "class ForallV' p => ForallV_ (p :: k)\n",
        "instance ForallV' p => ForallV_ p\n",
        "\n",
        "-- | Compatibility reduction for the constraints package.\n",
        "class InstV (p :: k) c | k c -> p where\n",
        "  type ForallV' (p :: k) :: Constraint\n",
        "  instV :: ForallV p :- c\n",
    ));

    assert!(module_chirho.decls_chirho.iter().any(|declaration_chirho| {
        matches!(
            declaration_chirho,
            DeclChirho::ClassDeclChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "InstV"
        )
    }));
}

#[test]
fn gadt_newtype_constructor_and_deriving_survive_lowering_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE GADTs #-}\n",
        "module GadtNewtypeChirho where\n",
        "newtype WrapperChirho valueChirho where\n",
        "  WrapChirho :: forall valueChirho. Int -> WrapperChirho valueChirho\n",
        "  deriving Eq\n",
    ));

    let (constructor_chirho, deriving_chirho) = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                deriving_chirho,
                ..
            } if name_chirho.text_chirho() == "WrapperChirho" => {
                Some((constructor_chirho, deriving_chirho))
            }
            _ => None,
        })
        .expect("GADT newtype should survive lowering");

    assert!(
        matches!(
            constructor_chirho,
            ConDeclChirho::GadtChirho { name_chirho, .. }
                if name_chirho.text_chirho() == "WrapChirho"
        ),
        "expected preserved GADT constructor, got {constructor_chirho:?}"
    );
    assert_eq!(deriving_chirho.len(), 1);
    assert_eq!(deriving_chirho[0].text_chirho(), "Eq");
}

#[test]
fn inline_gadt_newtype_kind_signature_does_not_inflate_head_arity_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE GADTs #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE UnliftedNewtypes #-}\n",
        "module GadtNewtypeKindChirho where\n",
        "import GHC.Exts (TYPE)\n",
        "newtype RuntimeBoxChirho :: forall representationChirho -> TYPE representationChirho where\n",
        "  RuntimeBoxChirho :: RuntimeBoxChirho representationChirho -> RuntimeBoxChirho representationChirho\n",
    ));

    let (type_vars_chirho, kind_sig_chirho, constructor_chirho) = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                kind_sig_chirho,
                constructor_chirho,
                ..
            } if name_chirho.text_chirho() == "RuntimeBoxChirho" => {
                Some((type_vars_chirho, kind_sig_chirho, constructor_chirho))
            }
            _ => None,
        })
        .expect("kind-indexed GADT newtype should survive lowering");

    assert!(type_vars_chirho.is_empty());
    assert!(matches!(
        kind_sig_chirho,
        Some(TypeChirho::RequiredForallChirho { .. })
    ));
    assert!(matches!(
        constructor_chirho,
        ConDeclChirho::GadtChirho { .. }
    ));
}

#[test]
fn operator_kind_signature_keeps_application_order_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE DataKinds #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE TypeOperators #-}\n",
        "module OperatorKindChirho where\n",
        "data TyFunChirho :: Type -> Type -> Type\n",
        "type leftChirho ~> rightChirho = TyFunChirho leftChirho rightChirho -> Type\n",
        "data SymbolChirho :: forall indexChirho. Maybe indexChirho ~> Bool\n",
    ));

    let kind_sig_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                kind_sig_chirho,
                ..
            } if name_chirho.text_chirho() == "SymbolChirho" => kind_sig_chirho.as_ref(),
            _ => None,
        })
        .expect("operator kind signature should survive lowering");

    let TypeChirho::ForallChirho {
        vars_chirho,
        body_chirho,
        ..
    } = kind_sig_chirho
    else {
        panic!("expected explicit forall kind signature, got {kind_sig_chirho:?}");
    };
    assert_eq!(vars_chirho.len(), 1);
    assert_eq!(vars_chirho[0].name_chirho.text_chirho(), "indexChirho");

    let TypeChirho::AppChirho {
        fun_chirho: operator_and_left_chirho,
        arg_chirho: right_chirho,
        ..
    } = body_chirho.as_ref()
    else {
        panic!("expected binary operator application, got {body_chirho:?}");
    };
    let TypeChirho::AppChirho {
        fun_chirho: operator_chirho,
        arg_chirho: left_chirho,
        ..
    } = operator_and_left_chirho.as_ref()
    else {
        panic!("expected operator applied to its left operand");
    };
    assert!(matches!(
        operator_chirho.as_ref(),
        TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "~>"
    ));
    assert!(matches!(
        left_chirho.as_ref(),
        TypeChirho::AppChirho { fun_chirho, .. }
            if matches!(fun_chirho.as_ref(), TypeChirho::ConChirho(name_chirho)
                if name_chirho.text_chirho() == "Maybe")
    ));
    assert!(matches!(
        right_chirho.as_ref(),
        TypeChirho::ConChirho(name_chirho) if name_chirho.text_chirho() == "Bool"
    ));
}

#[test]
fn associated_kind_signature_is_a_type_member_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module AssociatedKindChirho where\n",
        "class ClassChirho kindChirho where\n",
        "  type FamilyChirho :: kindChirho -> Type\n",
    ));
    let class_decl_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho {
                methods_chirho,
                associated_tfs_chirho,
                ..
            } => Some((methods_chirho, associated_tfs_chirho)),
            _ => None,
        })
        .expect("class declaration should survive lowering");

    assert!(class_decl_chirho.0.is_empty());
    assert_eq!(class_decl_chirho.1.len(), 1);
    assert_eq!(
        class_decl_chirho.1[0].name_chirho.text_chirho(),
        "FamilyChirho"
    );
}

#[test]
fn associated_data_header_is_a_type_member_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module AssociatedDataChirho where\n",
        "class ClassChirho typeChirho where\n",
        "  data FamilyChirho typeChirho\n",
    ));
    let associated_names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho {
                associated_tfs_chirho,
                ..
            } => Some(
                associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| family_chirho.name_chirho.text_chirho())
                    .collect(),
            ),
            _ => None,
        })
        .expect("class declaration should survive lowering");

    assert_eq!(associated_names_chirho, ["FamilyChirho"]);
}

#[test]
fn operator_class_name_survives_ast_lowering_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeOperators #-}\n",
        "module OperatorClassChirho where\n",
        "class leftChirho <+> rightChirho where\n",
        "  combineChirho :: leftChirho -> rightChirho\n",
    ));
    let class_name_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .expect("operator class should survive lowering");

    assert_eq!(class_name_chirho, "<+>");
}

#[test]
fn explicit_empty_closed_family_keeps_its_sibling_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module ClosedFamiliesChirho where\n",
        "type family EmptyChirho :: kindChirho where {}\n",
        "type family SiblingChirho typeChirho where\n",
        "  SiblingChirho typeChirho = typeChirho\n",
    ));
    let family_names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .collect();

    assert_eq!(family_names_chirho, ["EmptyChirho", "SiblingChirho"]);
}

#[test]
fn operator_type_family_names_survive_ast_lowering_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE TypeFamilies #-}\n",
        "{-# LANGUAGE TypeOperators #-}\n",
        "module OperatorFamiliesChirho where\n",
        "type family (^.) leftChirho rightChirho where\n",
        "  leftChirho ^. rightChirho = leftChirho\n",
        "type family (*.) leftChirho rightChirho where\n",
        "  leftChirho *. rightChirho = rightChirho\n",
    ));
    let family_names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .filter_map(|decl_chirho| match decl_chirho {
            DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .collect();

    assert_eq!(family_names_chirho, ["^.", "*."]);
}

#[test]
fn datatype_context_does_not_replace_the_type_name_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE DatatypeContexts #-}\n",
        "module DatatypeContextChirho where\n",
        "data (Show typeChirho) => ObservedChirho typeChirho = ObservedChirho typeChirho\n",
    ));
    let type_name_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::DataDeclChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
            _ => None,
        })
        .expect("data declaration should survive lowering");

    assert_eq!(type_name_chirho, "ObservedChirho");
}

#[test]
fn instance_equality_keeps_the_equality_as_constraint_head_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE DataKinds #-}\n",
        "{-# LANGUAGE TypeOperators #-}\n",
        "module InstanceEqualityChirho where\n",
        "data ShapeChirho = TypeChirho :~> TypeChirho\n",
        "class ClassChirho typeChirho where\n",
        "instance resultChirho ~ (leftChirho ':~> rightChirho) => ClassChirho resultChirho where\n",
    ));
    let constraint_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho { context_chirho, .. } => context_chirho.first(),
            _ => None,
        })
        .expect("instance equality should survive lowering");

    assert!(matches!(
        constraint_chirho,
        haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } if class_chirho.text_chirho() == "~" && args_chirho.len() == 2
    ));
}

#[test]
fn nested_rank_n_context_does_not_replace_existential_constructor_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "module NestedForallChirho where\n",
        "class leftChirho <= rightChirho where\n",
        "data FreeChirho fChirho uChirho aChirho\n",
        "  = forall xChirho. FreeChirho\n",
        "      (fChirho uChirho xChirho)\n",
        "      (forall nextChirho. uChirho <= nextChirho =>\n",
        "         nextChirho xChirho -> FreeChirho fChirho nextChirho xChirho)\n",
    ));
    let constructors_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } if name_chirho.text_chirho() == "FreeChirho" => Some(constructors_chirho),
            _ => None,
        })
        .expect("FreeChirho declaration should survive lowering");
    let ConDeclChirho::OrdinaryChirho {
        name_chirho,
        fields_chirho,
        ..
    } = &constructors_chirho[0]
    else {
        panic!("existential constructor should remain ordinary")
    };

    assert_eq!(name_chirho.text_chirho(), "FreeChirho");
    assert_eq!(fields_chirho.len(), 2);
    assert!(matches!(
        &fields_chirho[1].1,
        TypeChirho::ParenChirho { inner_chirho, .. }
            if matches!(inner_chirho.as_ref(), TypeChirho::ForallChirho { vars_chirho, .. }
                if vars_chirho.len() == 1
                    && vars_chirho[0].name_chirho.text_chirho() == "nextChirho")
    ));
}

#[test]
fn injectivity_annotation_does_not_inflate_family_arity_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE TypeFamilies #-}\n",
        "{-# LANGUAGE TypeFamilyDependencies #-}\n",
        "{-# LANGUAGE TypeOperators #-}\n",
        "module InjectiveFamilyChirho where\n",
        "type family DimChirho valueChirho\n",
        "type family valueChirho `OfDimChirho` (dimensionChirho :: DimChirho valueChirho) = resultChirho | resultChirho -> dimensionChirho\n",
    ));
    let parameter_names_chirho: Vec<&str> = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } if name_chirho.text_chirho() == "OfDimChirho" => Some(
                type_vars_chirho
                    .iter()
                    .map(|type_var_chirho| type_var_chirho.name_chirho.text_chirho())
                    .collect(),
            ),
            _ => None,
        })
        .expect("operator family should survive lowering");

    assert_eq!(parameter_names_chirho, ["valueChirho", "dimensionChirho"]);
}

#[test]
fn unsupported_applied_binder_kind_remains_inferred_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE KindSignatures #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "module AppliedKindChirho where\n",
        "type CatChirho indexChirho = indexChirho -> indexChirho -> Type\n",
        "newtype WrappedChirho (categoryChirho :: CatChirho indexChirho) leftChirho rightChirho = WrappedChirho (categoryChirho rightChirho leftChirho)\n",
    ));
    let category_binder_chirho = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                type_vars_chirho,
                ..
            } if name_chirho.text_chirho() == "WrappedChirho" => {
                type_vars_chirho.iter().find(|type_var_chirho| {
                    type_var_chirho.name_chirho.text_chirho() == "categoryChirho"
                })
            }
            _ => None,
        })
        .expect("kind-annotated category binder should survive lowering");

    assert!(
        category_binder_chirho.kind_annotation_chirho.is_none(),
        "an unrepresented named kind application must defer to kind inference"
    );
}

#[test]
fn visible_kind_binders_do_not_become_family_arguments_chirho() {
    let module_chirho = lower_source_chirho(concat!(
        "{-# LANGUAGE KindSignatures #-}\n",
        "{-# LANGUAGE PolyKinds #-}\n",
        "{-# LANGUAGE TypeAbstractions #-}\n",
        "{-# LANGUAGE TypeFamilies #-}\n",
        "module VisibleFamilyBindersChirho where\n",
        "class ClassChirho @kindChirho (typeChirho :: kindChirho) where\n",
        "  type FamilyChirho @kindChirho typeChirho\n",
        "  data DataFamilyChirho @kindChirho typeChirho\n",
    ));
    let (class_parameters_chirho, family_parameters_chirho) = module_chirho
        .decls_chirho
        .iter()
        .find_map(|declaration_chirho| match declaration_chirho {
            DeclChirho::ClassDeclChirho {
                type_vars_chirho,
                associated_tfs_chirho,
                ..
            } => Some((
                type_vars_chirho
                    .iter()
                    .map(|type_var_chirho| type_var_chirho.name_chirho.text_chirho().to_string())
                    .collect::<Vec<_>>(),
                associated_tfs_chirho
                    .iter()
                    .map(|family_chirho| {
                        (
                            family_chirho.name_chirho.text_chirho().to_string(),
                            family_chirho
                                .type_vars_chirho
                                .iter()
                                .map(|type_var_chirho| type_var_chirho.text_chirho().to_string())
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )),
            _ => None,
        })
        .expect("class declaration should survive lowering");

    assert_eq!(class_parameters_chirho, ["typeChirho"]);
    assert_eq!(
        family_parameters_chirho,
        [
            ("FamilyChirho".to_string(), vec!["typeChirho".to_string()]),
            (
                "DataFamilyChirho".to_string(),
                vec!["typeChirho".to_string()]
            ),
        ]
    );
}
