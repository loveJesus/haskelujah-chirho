// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed type-family equations as written (row 686). Each equation must be
//! complete, be headed by a type constructor, name its own family and give the
//! declared number of arguments; GHC rejects the program otherwise. A head is
//! this family when it is spelled with the family's name, unqualified or
//! qualified by this module, in any written form lowering accepts (prefix,
//! infix, `(op)`, backtick or parenthesized `(F a) b`).
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_ast_chirho::decl_chirho::{TyVarChirho, TypeFamilyEquationChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::DeclarationErrorChirho;

/// Judge every equation of one closed family declaration, in written order.
pub(super) fn check_family_equations_chirho(
    module_name_chirho: &NameChirho,
    family_chirho: &NameChirho,
    type_vars_chirho: &[TyVarChirho],
    equations_chirho: &[TypeFamilyEquationChirho],
    errors_chirho: &mut Vec<DeclarationErrorChirho>,
) {
    let arity_chirho = TyVarChirho::visible_binders_chirho(type_vars_chirho).len();
    errors_chirho.extend(equations_chirho.iter().filter_map(|equation_chirho| {
        equation_error_chirho(
            module_name_chirho,
            family_chirho,
            arity_chirho,
            equation_chirho,
        )
    }));
}

/// The first rule an equation breaks: an incomplete equation is a parse error,
/// then the head's shape, its name and the argument count.
fn equation_error_chirho(
    module_name_chirho: &NameChirho,
    family_chirho: &NameChirho,
    arity_chirho: usize,
    equation_chirho: &TypeFamilyEquationChirho,
) -> Option<DeclarationErrorChirho> {
    let family_text_chirho = family_chirho.text_chirho();
    let (ghc_code_chirho, message_chirho) = if equation_chirho.malformed_chirho {
        (
            "GHC-58481",
            format!(
                "Incomplete equation for type family `{family_text_chirho}`: expected `left side = right side`"
            ),
        )
    } else if let Some(head_chirho) = &equation_chirho.head_chirho {
        if !names_family_chirho(module_name_chirho, family_chirho, head_chirho) {
            (
                "GHC-88221",
                format!(
                    "Mismatched type name in type family instance. Expected: {family_text_chirho}, Actual: {}",
                    head_chirho.full_name_chirho()
                ),
            )
        } else if equation_chirho.lhs_types_chirho.len() != arity_chirho {
            (
                "GHC-12985",
                format!(
                    "Number of parameters must match family declaration; expected {arity_chirho}"
                ),
            )
        } else {
            return None;
        }
    } else {
        (
            "GHC-47568",
            format!(
                "Malformed head of type family equation for `{family_text_chirho}`: no type constructor heads its left side"
            ),
        )
    };
    Some(DeclarationErrorChirho {
        ghc_code_chirho,
        message_chirho,
        span_chirho: equation_chirho.span_chirho,
        related_span_chirho: None,
    })
}

/// Whether a written head names this family: the same name, unqualified or
/// qualified by the module being checked.
fn names_family_chirho(
    module_name_chirho: &NameChirho,
    family_chirho: &NameChirho,
    head_chirho: &NameChirho,
) -> bool {
    head_chirho.text_chirho() == family_chirho.text_chirho()
        && head_chirho
            .qualifier_chirho()
            .is_none_or(|qualifier_chirho| {
                qualifier_chirho == module_name_chirho.full_name_chirho()
            })
}

#[cfg(test)]
#[path = "family_equations_tests_chirho.rs"]
mod tests_chirho;
