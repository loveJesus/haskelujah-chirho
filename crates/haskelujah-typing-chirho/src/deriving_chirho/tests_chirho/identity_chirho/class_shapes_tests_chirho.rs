// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which local classes GND derives with written class arguments (#31897): T3955's
//! `class (Monad m) => MonadReader r m` derives as `MonadReader a` with its last parameter
//! taking one argument; every other shape stays refused.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ClassMethodChirho, DerivedClassChirho};
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassChirho, BuiltinClassScopeChirho, CompilerClassChirho, NameChirho, RawNameChirho,
};
use haskelujah_span_chirho::SpanChirho;

use super::*;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

/// `class <context> => <class> rChirho <last>` with these methods' names.
fn class_chirho(
    class_name_chirho: &str,
    context_chirho: Vec<ConstraintChirho>,
    last_chirho: TyVarChirho,
    methods_chirho: &[&str],
) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho,
        name_chirho: name_chirho(class_name_chirho),
        type_vars_chirho: vec![
            TyVarChirho::plain_chirho(name_chirho("rChirho")),
            last_chirho,
        ],
        methods_chirho: methods_chirho
            .iter()
            .map(|method_chirho| ClassMethodChirho {
                name_chirho: name_chirho(method_chirho),
                ty_chirho: var_chirho("rChirho"),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .collect(),
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `<class> mChirho`, a superclass constraint on the last parameter.
fn on_last_chirho(class_name_chirho: &str) -> ConstraintChirho {
    ConstraintChirho::ClassChirho {
        kind_args_chirho: Vec::new(),
        class_chirho: name_chirho(class_name_chirho),
        args_chirho: vec![var_chirho("mChirho")],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

/// K01 proves `Monad` (one argument), `Show` (none) and `MonadTrans`, whose parameter takes a
/// constructor and then a type.
fn scope_chirho() -> BuiltinClassScopeChirho {
    BuiltinClassScopeChirho::new_chirho(HashMap::from([
        (
            "Monad".to_string(),
            BuiltinClassChirho {
                identity_chirho: CompilerClassChirho::MonadChirho,
                module_chirho: "GHC.Base",
                name_chirho: "Monad",
            },
        ),
        (
            "Show".to_string(),
            BuiltinClassChirho {
                identity_chirho: CompilerClassChirho::ShowChirho,
                module_chirho: "GHC.Show",
                name_chirho: "Show",
            },
        ),
        (
            "MonadTrans".to_string(),
            BuiltinClassChirho {
                identity_chirho: CompilerClassChirho::MonadTransChirho,
                module_chirho: "Control.Monad.Trans.Class",
                name_chirho: "MonadTrans",
            },
        ),
    ]))
}

/// The arity GND reads for `derived_chirho` in a module of these declarations.
fn arity_chirho(
    decls_chirho: Vec<DeclChirho>,
    derived_chirho: &DerivedClassChirho,
) -> Option<usize> {
    let module_chirho = module_chirho(decls_chirho);
    let scope_chirho = scope_chirho();
    let identity_chirho = StockIdentityChirho::of_module_chirho(&module_chirho, &scope_chirho);
    LocalClassShapesChirho::of_module_chirho(&module_chirho, &identity_chirho)
        .class_argument_arity_chirho(derived_chirho, &identity_chirho)
}

/// `deriving (<class> aChirho)`.
fn applied_chirho(class_chirho: NameChirho) -> DerivedClassChirho {
    DerivedClassChirho::applied_chirho(class_chirho, vec![var_chirho("aChirho")], None)
}

fn plain_last_chirho() -> TyVarChirho {
    TyVarChirho::plain_chirho(name_chirho("mChirho"))
}

#[test]
fn t3955s_methodless_reader_class_takes_its_written_argument_chirho() {
    let reader_chirho = || {
        class_chirho(
            "MonadReaderChirho",
            vec![on_last_chirho("Monad")],
            plain_last_chirho(),
            &[],
        )
    };
    assert_eq!(
        arity_chirho(
            vec![reader_chirho()],
            &applied_chirho(name_chirho("MonadReaderChirho"))
        ),
        Some(1)
    );
    // A kind annotation proves the arity as well.
    let kinded_chirho = TyVarChirho {
        kind_annotation_chirho: Some(Box::new(
            AstKindChirho::ArrowChirho(
                Box::new(AstKindChirho::StarChirho),
                Box::new(AstKindChirho::StarChirho),
            )
            .into(),
        )),
        ..plain_last_chirho()
    };
    assert_eq!(
        arity_chirho(
            vec![class_chirho("KindedChirho", vec![], kinded_chirho, &[])],
            &applied_chirho(name_chirho("KindedChirho"))
        ),
        Some(1)
    );
}

#[test]
fn every_other_class_argument_shape_stays_refused_chirho() {
    let derived_chirho = applied_chirho(name_chirho("ClassChirho"));
    // A method needs its own coercion and dictionary route.
    assert_eq!(
        arity_chirho(
            vec![class_chirho(
                "ClassChirho",
                vec![on_last_chirho("Monad")],
                plain_last_chirho(),
                &["askChirho"]
            )],
            &derived_chirho
        ),
        None
    );
    // Nothing proves the last parameter's arity.
    assert_eq!(
        arity_chirho(
            vec![class_chirho(
                "ClassChirho",
                vec![],
                plain_last_chirho(),
                &[]
            )],
            &derived_chirho
        ),
        None
    );
    // Two proofs disagree: `Monad m` takes one argument, `Show m` none.
    assert_eq!(
        arity_chirho(
            vec![class_chirho(
                "ClassChirho",
                vec![on_last_chirho("Monad"), on_last_chirho("Show")],
                plain_last_chirho(),
                &[]
            )],
            &derived_chirho
        ),
        None
    );
    let local_chirho = || {
        vec![class_chirho(
            "ClassChirho",
            vec![on_last_chirho("Monad")],
            plain_last_chirho(),
            &[],
        )]
    };
    // Two written arguments for one leading parameter.
    let two_chirho = DerivedClassChirho::applied_chirho(
        name_chirho("ClassChirho"),
        vec![var_chirho("aChirho"), var_chirho("bChirho")],
        None,
    );
    assert_eq!(arity_chirho(local_chirho(), &two_chirho), None);
    // No written argument: not this shape.
    assert_eq!(
        arity_chirho(
            local_chirho(),
            &DerivedClassChirho::unspecified_chirho(name_chirho("ClassChirho"))
        ),
        None
    );
    // A foreign spelling never borrows the local class.
    let foreign_chirho = applied_chirho(NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        "F",
        "ClassChirho",
        SpanChirho::DUMMY_CHIRHO,
    )));
    assert_eq!(arity_chirho(local_chirho(), &foreign_chirho), None);
}

#[test]
fn a_transformer_superclass_proves_no_type_argument_count_chirho() {
    // `class MonadTrans m => LiftChirho r m`: `m` takes a constructor and then a type, so no
    // count of `Type` arguments describes it, and its two outer arrows never stand in for one.
    assert_eq!(
        arity_chirho(
            vec![class_chirho(
                "LiftChirho",
                vec![on_last_chirho("MonadTrans")],
                plain_last_chirho(),
                &[]
            )],
            &applied_chirho(name_chirho("LiftChirho"))
        ),
        None
    );
}

#[test]
fn written_universe_projection_preserves_opacity_and_one_depth_allowance_chirho() {
    let universe_chirho = TypeChirho::from(AstKindChirho::StarChirho);
    let annotation_arity_chirho = |annotation_chirho| {
        arity_chirho(
            vec![class_chirho(
                "AnnotatedChirho",
                vec![],
                TyVarChirho::annotated_chirho(name_chirho("mChirho"), annotation_chirho),
                &[],
            )],
            &applied_chirho(name_chirho("AnnotatedChirho")),
        )
    };
    assert_eq!(annotation_arity_chirho(universe_chirho.clone()), Some(0));
    let TypeChirho::ConChirho(mut opaque_chirho) = universe_chirho.clone() else {
        panic!("the AST universe producer retains its constructor");
    };
    opaque_chirho.mark_type_constructor_opaque_chirho();
    assert_eq!(
        annotation_arity_chirho(TypeChirho::ConChirho(opaque_chirho)),
        None
    );

    let mut deep_chirho = universe_chirho.clone();
    for _index_chirho in 0
        ..(haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho::default().depth_chirho
            / 2
            + 1)
    {
        deep_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(universe_chirho.clone()),
            mult_chirho: None,
            result_chirho: Box::new(deep_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    }
    assert_eq!(annotation_arity_chirho(deep_chirho), None);
    assert_eq!(annotation_arity_chirho(universe_chirho), Some(0));
}
