// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing kind controls, grouped by responsibility; assertions are unchanged.
use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, AstKindChirho, ClassMethodChirho, ConDeclChirho, DeclChirho,
    StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

fn mk_name_chirho(s_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        s_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn mk_module_chirho(decls_chirho: Vec<DeclChirho>) -> ModuleChirho {
    ModuleChirho {
        name_chirho: mk_name_chirho("Test"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn mk_fun_chirho(arg_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::FunChirho {
        arg_chirho: Box::new(arg_chirho),
        mult_chirho: None,
        result_chirho: Box::new(result_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn mk_app_chirho(fun_chirho: TypeChirho, arg_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(fun_chirho),
        arg_chirho: Box::new(arg_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

mod applications_chirho;
mod basics_chirho;
mod modules_chirho;
