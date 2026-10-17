// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An instance body keeps its method signatures (InstanceSigs) beside the
//! method equations, in written order: the instance-method check reads the
//! signature to scope its binders (T5676), and lowering used to drop it.
//! Instances without signatures, and associated type instances, lower as before.
//! An overlap pragma after `instance` or `deriving instance` is trivia, so the
//! declaration keeps its class, head, context and body. The pragma used to be
//! the first head token and took the class's place (claude_chirho #27375).
//! workflow: language-features-chirho/instance-obligations-chirho
//! workflow: language-features-chirho/flat-type-syntax-chirho

use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;
use haskelujah_ast_chirho::module_chirho::{InlinePragmaChirho, ModuleChirho};
use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho, TypeChirho};
use haskelujah_span_chirho::FileIdChirho;

use super::lower_module_chirho;
use crate::cst_parser_chirho::parse_to_cst_chirho;

fn lower_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let cst_chirho = parse_to_cst_chirho(source_chirho, file_chirho);
    lower_module_chirho(&cst_chirho, file_chirho)
}

/// The methods and associated type instances of the module's only instance.
fn instance_body_chirho(
    module_chirho: &ModuleChirho,
) -> (&[LocalBindChirho], &[AssocTfInstanceChirho]) {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho {
                methods_chirho,
                assoc_tf_instances_chirho,
                ..
            } => Some((
                methods_chirho.as_slice(),
                assoc_tf_instances_chirho.as_slice(),
            )),
            _ => None,
        })
        .expect("an instance declaration")
}

#[test]
fn an_instance_signature_stays_beside_its_equations_chirho() {
    let module_chirho = lower_chirho(
        "{-# LANGUAGE InstanceSigs, ScopedTypeVariables #-}\nmodule M where\nclass CChirho a where\n  fooChirho :: b -> a -> [b]\ninstance CChirho Int where\n  fooChirho :: forall b. b -> Int -> [b]\n  fooChirho x _ = [x]\n",
    );
    let (methods_chirho, _families_chirho) = instance_body_chirho(&module_chirho);
    let [
        LocalBindChirho::TypeSigChirho {
            name_chirho: signed_chirho,
            ty_chirho,
            ..
        },
        LocalBindChirho::FunBindChirho {
            name_chirho: bound_chirho,
            ..
        },
    ] = methods_chirho
    else {
        panic!("expected the signature, then its equations: {methods_chirho:?}");
    };
    assert_eq!(signed_chirho.text_chirho(), "fooChirho");
    assert_eq!(bound_chirho.text_chirho(), "fooChirho");
    // The written `forall` survives: its binder is what scopes over the body.
    let TypeChirho::ForallChirho { vars_chirho, .. } = ty_chirho else {
        panic!("expected the written forall: {ty_chirho:?}");
    };
    let binders_chirho: Vec<&str> = vars_chirho
        .iter()
        .map(|var_chirho| var_chirho.text_chirho())
        .collect();
    assert_eq!(binders_chirho, ["b"]);
}

#[test]
fn instances_without_signatures_lower_as_before_chirho() {
    // A signature-free body is its equations only, and an associated type
    // instance is collected apart from the methods.
    let module_chirho = lower_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule M where\nclass ContainerChirho f where\n  type ElemChirho f\n  emptyChirho :: f\ninstance ContainerChirho [a] where\n  type ElemChirho [a] = a\n  emptyChirho = []\n",
    );
    let (methods_chirho, families_chirho) = instance_body_chirho(&module_chirho);
    let [LocalBindChirho::FunBindChirho { name_chirho, .. }] = methods_chirho else {
        panic!("expected the one equation group: {methods_chirho:?}");
    };
    assert_eq!(name_chirho.text_chirho(), "emptyChirho");
    let [family_chirho] = families_chirho else {
        panic!("expected one associated type instance: {families_chirho:?}");
    };
    assert_eq!(family_chirho.family_name_chirho.text_chirho(), "ElemChirho");
}

/// A type as written, without its parentheses: `BoxChirho (Maybe a)`.
fn render_chirho(ty_chirho: &TypeChirho) -> String {
    match ty_chirho {
        TypeChirho::VarChirho(name_chirho) | TypeChirho::ConChirho(name_chirho) => {
            name_chirho.text_chirho().to_string()
        }
        TypeChirho::ParenChirho { inner_chirho, .. } => render_chirho(inner_chirho),
        TypeChirho::ListChirho { element_chirho, .. } => {
            format!("[{}]", render_chirho(element_chirho))
        }
        TypeChirho::AppChirho {
            fun_chirho,
            arg_chirho,
            ..
        } => {
            let arg_text_chirho = render_chirho(arg_chirho);
            if matches!(unparen_chirho(arg_chirho), TypeChirho::AppChirho { .. }) {
                format!("{} ({arg_text_chirho})", render_chirho(fun_chirho))
            } else {
                format!("{} {arg_text_chirho}", render_chirho(fun_chirho))
            }
        }
        other_chirho => panic!("unexpected head type: {other_chirho:?}"),
    }
}

fn unparen_chirho(ty_chirho: &TypeChirho) -> &TypeChirho {
    match ty_chirho {
        TypeChirho::ParenChirho { inner_chirho, .. } => unparen_chirho(inner_chirho),
        other_chirho => other_chirho,
    }
}

fn constraint_text_chirho(constraint_chirho: &ConstraintChirho) -> String {
    match constraint_chirho {
        ConstraintChirho::ClassChirho {
            class_chirho,
            args_chirho,
            ..
        } => std::iter::once(class_chirho.text_chirho().to_string())
            .chain(args_chirho.iter().map(render_chirho))
            .collect::<Vec<_>>()
            .join(" "),
        other_chirho => panic!("unexpected constraint: {other_chirho:?}"),
    }
}

fn member_text_chirho(member_chirho: &LocalBindChirho) -> String {
    match member_chirho {
        LocalBindChirho::TypeSigChirho { name_chirho, .. } => {
            format!("sig {}", name_chirho.text_chirho())
        }
        LocalBindChirho::FunBindChirho { name_chirho, .. } => {
            format!("fun {}", name_chirho.text_chirho())
        }
        other_chirho => panic!("unexpected instance body member: {other_chirho:?}"),
    }
}

/// The module's only instance, as text: (context, class, head, body).
type InstanceShapeChirho = (Vec<String>, String, Vec<String>, Vec<String>);

fn instance_shape_chirho(module_chirho: &ModuleChirho) -> InstanceShapeChirho {
    module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::InstanceDeclChirho {
                context_chirho,
                class_chirho,
                types_chirho,
                methods_chirho,
                ..
            } => Some((
                context_chirho.iter().map(constraint_text_chirho).collect(),
                class_chirho.text_chirho().to_string(),
                types_chirho.iter().map(render_chirho).collect(),
                methods_chirho.iter().map(member_text_chirho).collect(),
            )),
            _ => None,
        })
        .expect("an instance declaration")
}

const TAG_CLASS_CHIRHO: &str =
    "class TagChirho a where\n  tagChirho :: a -> String\ndata BoxChirho a = BoxChirho a\n";

fn shape_of_chirho(extensions_chirho: &str, instance_chirho: &str) -> InstanceShapeChirho {
    instance_shape_chirho(&lower_chirho(&format!(
        "{{-# LANGUAGE {extensions_chirho} #-}}\nmodule M where\n{TAG_CLASS_CHIRHO}{instance_chirho}"
    )))
}

fn texts_chirho(items_chirho: &[&str]) -> Vec<String> {
    items_chirho
        .iter()
        .map(|item_chirho| item_chirho.to_string())
        .collect()
}

#[test]
fn every_overlap_pragma_keeps_the_instance_whole_chirho() {
    // Each source is accepted by GHC 9.14.1. The pragma used to take the
    // class's place, so the instance belonged to no class: an Eq instance
    // written this way reported E0204 alone and vanished silently beside a
    // generic one.
    for spelling_chirho in ["OVERLAPPING", "OVERLAPPABLE", "OVERLAPS", "INCOHERENT"] {
        let shape_chirho = shape_of_chirho(
            "FlexibleInstances, InstanceSigs",
            &format!(
                "instance {{-# {spelling_chirho} #-}} TagChirho (BoxChirho Int) where\n  tagChirho :: BoxChirho Int -> String\n  tagChirho _ = \"int\"\n"
            ),
        );
        assert_eq!(
            shape_chirho,
            (
                Vec::new(),
                "TagChirho".to_string(),
                texts_chirho(&["BoxChirho Int"]),
                texts_chirho(&["sig tagChirho", "fun tagChirho"]),
            ),
            "{spelling_chirho}"
        );
    }
}

#[test]
fn an_overlap_pragma_beside_a_context_or_forall_chirho() {
    // A context: the pragma used to sit among the context tokens instead.
    assert_eq!(
        shape_of_chirho(
            "FlexibleInstances",
            "instance {-# OVERLAPPABLE #-} Show a => TagChirho (BoxChirho [a]) where\n  tagChirho _ = \"list\"\n",
        ),
        (
            texts_chirho(&["Show a"]),
            "TagChirho".to_string(),
            texts_chirho(&["BoxChirho [a]"]),
            texts_chirho(&["fun tagChirho"]),
        )
    );
    // A forall telescope: the pragma used to stand before it, so the
    // telescope was not stripped and the class was lost again.
    assert_eq!(
        shape_of_chirho(
            "FlexibleInstances, ExplicitForAll",
            "instance {-# INCOHERENT #-} forall a. TagChirho (BoxChirho a) where\n  tagChirho _ = \"any\"\n",
        ),
        (
            Vec::new(),
            "TagChirho".to_string(),
            texts_chirho(&["BoxChirho a"]),
            texts_chirho(&["fun tagChirho"]),
        )
    );
    // Both: the telescope, then the context, then the head.
    assert_eq!(
        shape_of_chirho(
            "FlexibleInstances, ExplicitForAll",
            "instance {-# OVERLAPS #-} forall a. Show a => TagChirho (BoxChirho (Maybe a)) where\n  tagChirho _ = \"maybe\"\n",
        ),
        (
            texts_chirho(&["Show a"]),
            "TagChirho".to_string(),
            texts_chirho(&["BoxChirho (Maybe a)"]),
            texts_chirho(&["fun tagChirho"]),
        )
    );
}

#[test]
fn a_standalone_deriving_keeps_its_class_beside_a_pragma_chirho() {
    // It shares the instance head parser, so on 3ece4a33 its class fell back
    // to the placeholder `Unknown` (measured with Eq in place of Show: E0101
    // "type not in scope: `Unknown`"). GHC 9.14.1 accepts it.
    let module_chirho = lower_chirho(
        "{-# LANGUAGE FlexibleInstances, StandaloneDeriving #-}\nmodule M where\ndata BoxChirho a = BoxChirho a\nderiving instance {-# OVERLAPPABLE #-} Show (BoxChirho Int)\n",
    );
    let (class_chirho, types_chirho) = module_chirho
        .decls_chirho
        .iter()
        .find_map(|decl_chirho| match decl_chirho {
            DeclChirho::StandaloneDerivingDeclChirho {
                class_chirho,
                types_chirho,
                ..
            } => Some((
                class_chirho.text_chirho().to_string(),
                types_chirho.iter().map(render_chirho).collect::<Vec<_>>(),
            )),
            _ => None,
        })
        .expect("a standalone deriving declaration");
    assert_eq!(class_chirho, "Show");
    assert_eq!(types_chirho, texts_chirho(&["BoxChirho Int"]));
}

#[test]
fn pragmas_read_on_purpose_still_reach_their_readers_chirho() {
    // LANGUAGE, INLINE and SPECIALIZE are read from raw children, which the
    // trivia filter does not touch. Keep the metadata pragmas before nested
    // declaration bodies: today a trailing pragma can attach to that CST body
    // and is then invisible to these root-only readers on both trivia rules.
    let module_chirho = lower_chirho(&format!(
        "{{-# LANGUAGE FlexibleInstances #-}}\nmodule M where\n{{-# INLINE doubleChirho #-}}\n{{-# SPECIALIZE doubleChirho :: Int -> Int #-}}\n{TAG_CLASS_CHIRHO}instance {{-# OVERLAPPING #-}} TagChirho [Char] where\n  tagChirho _ = \"string\"\ndoubleChirho :: Num a => a -> a\ndoubleChirho x = x + x\n"
    ));
    assert!(
        module_chirho
            .extensions_chirho
            .iter()
            .any(|extension_chirho| extension_chirho == "FlexibleInstances"),
        "{:?}",
        module_chirho.extensions_chirho
    );
    assert_eq!(
        module_chirho.inline_pragmas_chirho.get("doubleChirho"),
        Some(&InlinePragmaChirho::InlineChirho)
    );
    assert_eq!(
        module_chirho.specialize_pragmas_chirho.get("doubleChirho"),
        Some(&vec!["Int -> Int".to_string()])
    );
    assert_eq!(
        instance_shape_chirho(&module_chirho),
        (
            Vec::new(),
            "TagChirho".to_string(),
            texts_chirho(&["[Char]"]),
            texts_chirho(&["fun tagChirho"]),
        )
    );
}
