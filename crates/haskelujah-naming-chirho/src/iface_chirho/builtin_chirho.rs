// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin module interfaces: synthetic interfaces for the modules the compiler provides,
//! from `Prelude` to `GHC.Exts`, which no source file defines.
//!
//! The tables live in family files under `builtin_chirho/` (row 648). Every statement that names
//! a module sits in that module's family file, in the order the tables were written, so each
//! module's merged interface is the same whatever order the families run in. The returned vector
//! follows the family order; consumers address modules by name, never by position.
//! workflow: compiler-pipeline-chirho/builtin-type-environment-chirho

mod base_chirho;
mod ghc_chirho;
mod packages_chirho;
mod platform_chirho;
#[cfg(test)]
mod tests_chirho;

use haskelujah_span_chirho::SpanChirho;

use super::{
    IfaceOriginChirho, IfaceTypeChirho, IfaceValueChirho, ModuleIfaceChirho,
    merge_module_ifaces_chirho,
};
use crate::type_exports_chirho::{
    canonical_value_name_chirho, normalize_builtin_type_exports_chirho,
};

fn builtin_class_methods_chirho(class_name_chirho: &str) -> Option<&'static [&'static str]> {
    match class_name_chirho {
        "Eq" => Some(&["==", "/="]),
        "Ord" => Some(&["compare", "<", "<=", ">", ">=", "max", "min"]),
        "Show" => Some(&["show"]),
        "Read" => Some(&["read"]),
        "Num" => Some(&["+", "*", "-", "negate", "fromInteger", "abs", "signum"]),
        "Fractional" => Some(&["/", "recip", "fromRational"]),
        "Floating" => Some(&[
            "pi", "exp", "log", "sqrt", "sin", "cos", "tan", "asin", "acos", "atan",
        ]),
        "Integral" => Some(&["div", "mod", "quot", "rem", "toInteger"]),
        // GHC's eight Enum methods, so `Enum(..)` imports and hides each (row 671).
        "Enum" => Some(&[
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
        ]),
        "Bounded" => Some(&["minBound", "maxBound"]),
        "IsString" => Some(&["fromString"]),
        "IsList" => Some(&["fromList", "toList"]),
        "Bits" => Some(&[
            ".&.",
            ".|.",
            "xor",
            "complement",
            "shift",
            "shiftL",
            "shiftR",
            "bit",
            "setBit",
            "clearBit",
            "testBit",
            "bitSize",
            "isSigned",
            "popCount",
            "zeroBits",
        ]),
        "FiniteBits" => Some(&["finiteBitSize", "countLeadingZeros", "countTrailingZeros"]),
        "Monad" => Some(&["return", ">>=", ">>"]),
        "MonadPlus" => Some(&["mzero", "mplus"]),
        "MonadFail" => Some(&["fail"]),
        "MonadFix" => Some(&["mfix"]),
        "MonadIO" => Some(&["liftIO"]),
        "MonadTrans" => Some(&["lift"]),
        "MonadZip" => Some(&["mzipWith"]),
        "Functor" => Some(&["fmap", "<$"]),
        "Applicative" => Some(&["pure", "<*>", "*>", "<*"]),
        "Alternative" => Some(&["empty", "<|>", "some", "many", "optional"]),
        "Foldable" => Some(&[
            "foldr", "foldl", "foldMap", "length", "null", "elem", "sum", "product", "maximum",
            "minimum", "toList",
        ]),
        "Traversable" => Some(&["traverse", "sequenceA", "mapM", "sequence"]),
        "Contravariant" => Some(&["contramap"]),
        "Monoid" => Some(&["mempty", "mappend", "mconcat"]),
        "Semigroup" => Some(&["<>"]),
        "NFData" => Some(&["rnf"]),
        "RandomGen" => Some(&["split", "genRange", "next"]),
        "SplitGen" => Some(&["splitGen"]),
        "Arrow" => Some(&["arr", "first", "second", "***", "&&&"]),
        "ArrowChoice" => Some(&["left", "right", "|||", "+++"]),
        "ArrowApply" => Some(&["app"]),
        "ArrowZero" => Some(&["zeroArrow"]),
        "ArrowPlus" => Some(&["<+>"]),
        "ArrowLoop" => Some(&["loop"]),
        "IsOption" => Some(&["defaultValue", "parseValue", "optionName", "optionHelp"]),
        _ => None,
    }
}

fn normalize_builtin_class_exports_chirho(modules_chirho: &mut [ModuleIfaceChirho]) {
    for module_chirho in modules_chirho {
        let class_names_chirho: Vec<String> = module_chirho
            .exports_chirho
            .types_chirho
            .keys()
            .cloned()
            .collect();
        for class_name_chirho in class_names_chirho {
            let Some(methods_chirho) = builtin_class_methods_chirho(&class_name_chirho) else {
                continue;
            };
            if let Some(ty_chirho) = module_chirho
                .exports_chirho
                .types_chirho
                .get_mut(&class_name_chirho)
            {
                for method_chirho in methods_chirho {
                    let method_name_chirho = method_chirho.to_string();
                    if !ty_chirho.methods_chirho.contains(&method_name_chirho) {
                        ty_chirho.methods_chirho.push(method_name_chirho.clone());
                    }
                    module_chirho
                        .exports_chirho
                        .values_chirho
                        .entry(method_name_chirho.clone())
                        .or_insert_with(|| IfaceValueChirho {
                            name_chirho: method_name_chirho,
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                            defining_module_chirho: None,
                        });
                }
            }
        }
    }
}

/// An exported value with no source span, keyed by its canonical name.
fn mk_val_chirho(name_chirho: &str) -> (String, IfaceValueChirho) {
    (
        canonical_value_name_chirho(name_chirho),
        IfaceValueChirho {
            name_chirho: name_chirho.to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: None,
        },
    )
}

/// An exported type or class with these constructors, no methods and no source span.
fn mk_type_chirho(name_chirho: &str, cons_chirho: &[&str]) -> (String, IfaceTypeChirho) {
    (
        name_chirho.to_string(),
        IfaceTypeChirho {
            name_chirho: name_chirho.to_string(),
            constructors_chirho: cons_chirho
                .iter()
                .map(|c_chirho| c_chirho.to_string())
                .collect(),
            methods_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    )
}

/// Build synthetic module interfaces for built-in modules like `Data.Map`,
/// `Data.Set`, `Data.List`, `Data.Char`, `Data.Maybe`, `Data.Either`.
/// These are generated at compile time and don't correspond to source files.
pub fn builtin_module_ifaces_chirho() -> Vec<ModuleIfaceChirho> {
    let mut modules_chirho = Vec::new();
    base_chirho::push_base_chirho(&mut modules_chirho);
    ghc_chirho::push_ghc_chirho(&mut modules_chirho);
    packages_chirho::push_packages_chirho(&mut modules_chirho);
    platform_chirho::push_platform_chirho(&mut modules_chirho);

    normalize_builtin_class_exports_chirho(&mut modules_chirho);
    let mut merged_modules_chirho = merge_module_ifaces_chirho(modules_chirho);
    normalize_builtin_type_exports_chirho(&mut merged_modules_chirho);
    // Row 608: these, and only these, are builtin. workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
    for module_chirho in &mut merged_modules_chirho {
        module_chirho.exports_chirho.origin_chirho = IfaceOriginChirho::BuiltinChirho;
    }
    merged_modules_chirho
}
