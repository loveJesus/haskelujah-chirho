// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Compiler family seeds and positive comparison-head authority.

use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, GenerativeIdentityChirho, family_branches_compatible_chirho,
};
use super::family_definitions_chirho::{
    IntrinsicTypeFamilyChirho, TypeFamilyBranchChirho, TypeFamilyBranchIdChirho,
    TypeFamilyDefinitionChirho, TypeFamilyIdChirho, TypeFamilyReductionChirho,
    TypeFamilyRegistryChirho, is_type_family_key_chirho,
};
use super::family_scope_chirho::TypeFamilyScopeChirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::promotion_chirho::{
    is_promoted_key_chirho, is_unknown_promoted_key_chirho,
};

const FAMILY_SEED_WORK_CHIRHO: usize = 100_000;

fn intrinsic_seeds_chirho() -> Vec<(&'static str, IntrinsicTypeFamilyChirho, Vec<&'static str>)> {
    use IntrinsicTypeFamilyChirho as IChirho;
    let numeric_modules_chirho = vec!["GHC.TypeLits", "GHC.TypeNats"];
    vec![
        ("+", IChirho::AddNatChirho, numeric_modules_chirho.clone()),
        (
            "*",
            IChirho::MultiplyNatChirho,
            numeric_modules_chirho.clone(),
        ),
        ("^", IChirho::PowerNatChirho, numeric_modules_chirho.clone()),
        (
            "-",
            IChirho::SubtractNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Div",
            IChirho::DivideNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Mod",
            IChirho::ModuloNatChirho,
            numeric_modules_chirho.clone(),
        ),
        (
            "Log2",
            IChirho::Log2NatChirho,
            numeric_modules_chirho.clone(),
        ),
        ("<=?", IChirho::LessEqualNatChirho, numeric_modules_chirho),
        (
            "AppendSymbol",
            IChirho::AppendSymbolChirho,
            vec!["GHC.TypeLits"],
        ),
        ("CharToNat", IChirho::CharToNatChirho, vec!["GHC.TypeLits"]),
        ("NatToChar", IChirho::NatToCharChirho, vec!["GHC.TypeLits"]),
        ("If", IChirho::IfChirho, vec!["Data.Type.Bool"]),
        ("Not", IChirho::NotChirho, vec!["Data.Type.Bool"]),
        ("&&", IChirho::AndChirho, vec!["Data.Type.Bool"]),
        ("||", IChirho::OrChirho, vec!["Data.Type.Bool"]),
    ]
}

/// Only the actual compiler seed producer creates these identities and export aliases.
pub fn seed_type_family_registry_chirho() -> TypeFamilyRegistryChirho {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    for (name_chirho, intrinsic_chirho, modules_chirho) in intrinsic_seeds_chirho() {
        let id_chirho = TypeFamilyIdChirho::IntrinsicChirho(intrinsic_chirho);
        registry_chirho
            .insert_chirho(
                TypeFamilyDefinitionChirho::new_chirho(
                    id_chirho.clone(),
                    intrinsic_chirho.arity_chirho(),
                    TypeFamilyReductionChirho::IntrinsicChirho(intrinsic_chirho),
                    Vec::new(),
                )
                .expect("static intrinsic seed is consistent"),
            )
            .expect("static intrinsic identities are unique");
        for module_chirho in modules_chirho {
            registry_chirho
                .export_alias_chirho(format!("{module_chirho}.{name_chirho}"), &id_chirho)
                .expect("intrinsic seed exists");
        }
    }
    for name_chirho in [
        "Int",
        "Integer",
        "Double",
        "Float",
        "Bool",
        "Char",
        "Ordering",
        "Maybe",
        "Either",
        "IO",
        "ST",
        "RealWorld",
        "Word",
        "Word8",
        "Word16",
        "Word32",
        "Word64",
        "Int8",
        "Int16",
        "Int32",
        "Int64",
        "Natural",
        "Proxy",
        "NonEmpty",
        "True",
        "False",
        ":",
        "'[]",
        "[]",
        "()",
    ] {
        registry_chirho
            .constructor_origins_mut_chirho()
            .insert_compiler_chirho(
                name_chirho.to_owned(),
                format!("compiler-standard:{name_chirho}"),
            );
    }
    // These are real data/newtype heads, also supplied with their constructors
    // by naming's builtin GHC.Generics interface. D1/C1/Rec0 are synonyms and
    // deliberately do not donate nominal authority here.
    for name_chirho in ["Par1", "M1", "V1"] {
        registry_chirho
            .constructor_origins_mut_chirho()
            .insert_compiler_chirho(
                name_chirho.to_owned(),
                format!("compiler:GHC.Generics:{name_chirho}"),
            );
    }
    // Generic's associated family is an extensible declaration, not a spelling
    // fallback. The driver's naming-approved imports admit this export alias.
    let generic_rep_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
        module_chirho: "GHC.Generics".to_owned(),
        name_chirho: "Rep".to_owned(),
    };
    let generic_rep_definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        generic_rep_chirho.clone(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        Vec::new(),
    )
    .expect("static Generic.Rep header is well formed")
    .verify_open_coherence_chirho(&|_, _| true, FAMILY_SEED_WORK_CHIRHO)
    .expect("an empty Generic.Rep header has no overlaps");
    registry_chirho
        .insert_chirho(generic_rep_definition_chirho)
        .expect("static Generic.Rep seed is unique");
    registry_chirho
        .export_alias_chirho("GHC.Generics.Rep".to_owned(), &generic_rep_chirho)
        .expect("Generic.Rep seed exists");
    let id_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
        module_chirho: "Control.Monad.Primitive".to_owned(),
        name_chirho: "PrimState".to_owned(),
    };
    let branch_chirho = |index_chirho, lhs_chirho, rhs_chirho| TypeFamilyBranchChirho {
        id_chirho: TypeFamilyBranchIdChirho {
            family_chirho: id_chirho.clone(),
            producer_module_chirho: "Control.Monad.Primitive".to_owned(),
            declaration_start_chirho: 0,
            equation_index_chirho: index_chirho,
        },
        binders_chirho: vec!["sChirho".to_owned()],
        lhs_chirho: vec![lhs_chirho],
        rhs_chirho,
        blockers_chirho: Vec::new(),
        held_chirho: false,
    };
    let s_chirho = TyChirho::ForallVarChirho("sChirho".to_owned());
    let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        id_chirho.clone(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![
            branch_chirho(
                0,
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ST".to_owned())),
                    Box::new(s_chirho.clone()),
                ),
                s_chirho,
            ),
            branch_chirho(
                1,
                TyChirho::ConChirho("IO".to_owned()),
                TyChirho::ConChirho("RealWorld".to_owned()),
            ),
        ],
    )
    .expect("static PrimState seed is well formed");
    let definition_chirho = definition_chirho
        .verify_open_coherence_chirho(
            &|left_chirho, right_chirho| {
                family_branches_compatible_chirho(
                    &left_chirho.lhs_chirho,
                    &left_chirho.rhs_chirho,
                    &right_chirho.lhs_chirho,
                    &right_chirho.rhs_chirho,
                    &|name_chirho| family_head_role_chirho(&registry_chirho, name_chirho),
                )
            },
            FAMILY_SEED_WORK_CHIRHO,
        )
        .expect("static PrimState rows are apart");
    registry_chirho
        .insert_chirho(definition_chirho)
        .expect("static PrimState seed is unique");
    registry_chirho
        .export_alias_chirho("Control.Monad.Primitive.PrimState".to_owned(), &id_chirho)
        .expect("PrimState seed exists");
    // These real compiler families have no evaluator here. Their sealed empty
    // definitions still prevent treating a stuck application as an injective tycon.
    for (name_chirho, modules_chirho) in [
        ("CmpNat", vec!["GHC.TypeLits", "GHC.TypeNats"]),
        ("CmpSymbol", vec!["GHC.TypeLits"]),
    ] {
        let id_chirho = TypeFamilyIdChirho::CompilerSeedChirho {
            module_chirho: "GHC.TypeLits".to_owned(),
            name_chirho: name_chirho.to_owned(),
        };
        registry_chirho
            .insert_chirho(
                TypeFamilyDefinitionChirho::new_chirho(
                    id_chirho.clone(),
                    2,
                    TypeFamilyReductionChirho::ClosedChirho,
                    Vec::new(),
                )
                .expect("sealed compiler family seed is consistent"),
            )
            .expect("sealed compiler family seed is unique");
        for module_chirho in modules_chirho {
            registry_chirho
                .export_alias_chirho(format!("{module_chirho}.{name_chirho}"), &id_chirho)
                .expect("sealed compiler family exists");
        }
    }
    registry_chirho
}

pub(super) fn legacy_family_scope_chirho(
    registry_chirho: &TypeFamilyRegistryChirho,
) -> TypeFamilyScopeChirho {
    let mut scope_chirho = TypeFamilyScopeChirho::new_chirho();
    for (name_chirho, _intrinsic_chirho, modules_chirho) in intrinsic_seeds_chirho() {
        for module_chirho in modules_chirho {
            let exported_chirho = format!("{module_chirho}.{name_chirho}");
            scope_chirho.import_export_chirho(
                name_chirho.to_owned(),
                &exported_chirho,
                registry_chirho,
            );
            scope_chirho.import_export_chirho(
                exported_chirho.clone(),
                &exported_chirho,
                registry_chirho,
            );
        }
    }
    for (name_chirho, export_chirho) in [
        ("Rep", "GHC.Generics.Rep"),
        ("PrimState", "Control.Monad.Primitive.PrimState"),
        ("CmpNat", "GHC.TypeLits.CmpNat"),
        ("CmpSymbol", "GHC.TypeLits.CmpSymbol"),
    ] {
        scope_chirho.import_export_chirho(name_chirho.to_owned(), export_chirho, registry_chirho);
        scope_chirho.import_export_chirho(export_chirho.to_owned(), export_chirho, registry_chirho);
    }
    scope_chirho
}

pub(super) fn family_head_role_chirho(
    registry_chirho: &TypeFamilyRegistryChirho,
    name_chirho: &str,
) -> FamilyHeadRoleChirho {
    if let Some(definition_chirho) = registry_chirho.definition_chirho(name_chirho) {
        return FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: definition_chirho.id_chirho().key_chirho(),
            arity_chirho: definition_chirho.arity_chirho(),
        };
    }
    if let Some(scheme_chirho) = registry_chirho
        .declaration_schemes_chirho()
        .get_chirho(name_chirho)
    {
        if scheme_chirho.owner_chirho.is_none()
            && !scheme_chirho.held_chirho
            && scheme_chirho.promoted_chirho
            && is_promoted_key_chirho(name_chirho)
            && !is_unknown_promoted_key_chirho(name_chirho)
        {
            return FamilyHeadRoleChirho::GenerativeChirho {
                identity_chirho: GenerativeIdentityChirho::NominalChirho(
                    scheme_chirho.identity_chirho.to_string(),
                ),
            };
        }
        if scheme_chirho.owner_chirho.is_none() && !scheme_chirho.held_chirho && scheme_chirho.declaration_chirho.as_ref().is_some_and(|declaration_chirho| declaration_chirho.is_nominally_generative_chirho() || declaration_chirho.role_chirho == haskelujah_ast_chirho::name_chirho::TypeConstructorRoleChirho::UniverseChirho) {
            return FamilyHeadRoleChirho::GenerativeChirho { identity_chirho: GenerativeIdentityChirho::NominalChirho(scheme_chirho.identity_chirho.to_string()) };
        }
        return FamilyHeadRoleChirho::UnknownChirho;
    }
    if is_type_family_key_chirho(name_chirho) || is_unknown_promoted_key_chirho(name_chirho) {
        return FamilyHeadRoleChirho::UnknownChirho;
    }
    if is_promoted_key_chirho(name_chirho) {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(name_chirho.to_owned()),
        };
    }
    if let Some(identity_chirho) = registry_chirho
        .constructor_origins_chirho()
        .identity_chirho(name_chirho)
    {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(identity_chirho),
        };
    }
    if name_chirho.parse::<u128>().is_ok()
        || (name_chirho.starts_with('"') && name_chirho.ends_with('"'))
        || (name_chirho.starts_with('\'') && name_chirho.ends_with('\'') && name_chirho.len() >= 3)
    {
        return FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(format!(
                "literal:{name_chirho}"
            )),
        };
    }
    FamilyHeadRoleChirho::UnknownChirho
}
