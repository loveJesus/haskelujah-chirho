// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed compiler identities shared by the existing type/kind normalizers.
//! No public spelling, alias lookup, variable supply or solver lives here.

use super::TyChirho;
use haskelujah_ast_chirho::name_chirho::{
    CompilerRepresentationChirho, CompilerTypeConstructorChirho, TypeConstructorDeclarationChirho,
};
use haskelujah_ast_chirho::promotion_chirho::{
    BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, PromotedKeyChirho,
};
use std::sync::LazyLock;

struct RepresentationIntrinsicsChirho {
    type_key_chirho: String,
    constraint_key_chirho: String,
    lifted_chirho: TyChirho,
    unlifted_chirho: TyChirho,
}

// A fixed pair of closed compiler terms, not a name-indexed registry. Reusing
// them keeps the normalization hot path independent of input/state growth.
static REPRESENTATION_INTRINSICS_CHIRHO: LazyLock<RepresentationIntrinsicsChirho> =
    LazyLock::new(|| {
        let boxed_chirho =
            PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "BoxedRep");
        let boxed_term_chirho = |levity_chirho| {
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(boxed_chirho.as_str_chirho().to_owned())),
                Box::new(TyChirho::ConChirho(
                    PromotedKeyChirho::builtin_chirho(
                        BUILTIN_CONSTRUCTOR_MODULE_CHIRHO,
                        levity_chirho,
                    )
                    .as_str_chirho()
                    .to_owned(),
                )),
            )
        };
        RepresentationIntrinsicsChirho {
            type_key_chirho: TypeConstructorDeclarationChirho::compiler_data_chirho(
                CompilerTypeConstructorChirho::TypeUniverseChirho,
            )
            .type_key_chirho(),
            constraint_key_chirho: TypeConstructorDeclarationChirho::compiler_data_chirho(
                CompilerTypeConstructorChirho::ConstraintUniverseChirho,
            )
            .type_key_chirho(),
            lifted_chirho: boxed_term_chirho("Lifted"),
            unlifted_chirho: boxed_term_chirho("Unlifted"),
        }
    });

pub(crate) fn boxed_representation_chirho(lifted_chirho: bool) -> &'static TyChirho {
    if lifted_chirho {
        &REPRESENTATION_INTRINSICS_CHIRHO.lifted_chirho
    } else {
        &REPRESENTATION_INTRINSICS_CHIRHO.unlifted_chirho
    }
}

/// Inspect one already-resolved application. Opaque `TYPE` and user heads
/// cannot match either private compiler identity.
pub(crate) fn folded_universe_key_chirho(
    function_chirho: &TyChirho,
    argument_chirho: &TyChirho,
) -> Option<&'static str> {
    if argument_chirho != boxed_representation_chirho(true) {
        return None;
    }
    let TyChirho::ConChirho(key_chirho) = function_chirho else {
        return None;
    };
    if key_chirho == CompilerRepresentationChirho::RuntimeTypeChirho.type_key_chirho() {
        Some(&REPRESENTATION_INTRINSICS_CHIRHO.type_key_chirho)
    } else if key_chirho == CompilerRepresentationChirho::RuntimeConstraintChirho.type_key_chirho()
    {
        Some(&REPRESENTATION_INTRINSICS_CHIRHO.constraint_key_chirho)
    } else {
        None
    }
}

fn representation_argument_chirho<'term_chirho>(
    universe_chirho: &TyChirho,
    application_chirho: &'term_chirho TyChirho,
) -> Option<&'term_chirho TyChirho> {
    let (
        TyChirho::ConChirho(universe_key_chirho),
        TyChirho::AppChirho(function_chirho, argument_chirho),
    ) = (universe_chirho, application_chirho)
    else {
        return None;
    };
    let TyChirho::ConChirho(function_key_chirho) = &**function_chirho else {
        return None;
    };
    let matches_chirho = (universe_key_chirho == &REPRESENTATION_INTRINSICS_CHIRHO.type_key_chirho
        && function_key_chirho
            == CompilerRepresentationChirho::RuntimeTypeChirho.type_key_chirho())
        || (universe_key_chirho == &REPRESENTATION_INTRINSICS_CHIRHO.constraint_key_chirho
            && function_key_chirho
                == CompilerRepresentationChirho::RuntimeConstraintChirho.type_key_chirho());
    matches_chirho.then_some(argument_chirho)
}

/// Return an equation for the existing solver, retaining its rigidity, occurs,
/// scope and resource policy. This does not solve or specialize a variable.
pub(crate) fn lifted_representation_equation_chirho<'term_chirho>(
    left_chirho: &'term_chirho TyChirho,
    right_chirho: &'term_chirho TyChirho,
) -> Option<(&'term_chirho TyChirho, &'static TyChirho)> {
    representation_argument_chirho(left_chirho, right_chirho)
        .or_else(|| representation_argument_chirho(right_chirho, left_chirho))
        .map(|argument_chirho| (argument_chirho, boxed_representation_chirho(true)))
}
