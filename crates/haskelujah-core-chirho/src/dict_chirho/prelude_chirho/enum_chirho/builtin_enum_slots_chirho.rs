// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The four enumeration rows of every builtin Enum key, which fill the enumeration slots of
//! its dictionary now that the class lays out all eight methods (row 671).
//!
//! - Int owns the shared bodies (`integral_sequence_chirho.rs`). Every other key a value of
//!   which is an integer at run time, and Char, reaches them by CoreId through a row of its
//!   own: these are the bodies its written calls reached before this row. The bounded keys'
//!   `enumFrom` and `enumFromThen` therefore still run past `maxBound`, where GHC stops; that
//!   wrong answer is measured and pre-existing, not a failure by name.
//! - Bool and Ordering take GHC's derived enumerations over their constructors, through
//!   their own `toEnum` and `fromEnum` rows.
//! - Float and `()` have no rows: their slots take the stub that fails by the method's name.
//!   Float's f64 representation is row 676's.
//!
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, DictPassCtxChirho, InlineAnnotationChirho,
};

/// The four enumeration methods every Enum dictionary carries a slot for.
const ENUMERATION_METHODS_CHIRHO: [&str; 4] =
    ["enumFrom", "enumFromThen", "enumFromTo", "enumFromThenTo"];

/// The builtin keys whose values are integers at run time, with Char, which the shared bodies
/// serve through `enumStepChirho#`.
const SHARED_BODY_KEYS_CHIRHO: [&str; 18] = [
    "Integer", "Char", "Word", "Word8", "Word16", "Word32", "Word64", "Int8", "Int16", "Int32",
    "Int64", "Natural", "CSize", "CInt", "CChar", "CLong", "CUInt", "CULong",
];

/// The builtin keys base derives Enum for, with their constructors in order.
const CONSTRUCTOR_KEYS_CHIRHO: [(&str, &[&str]); 2] = [
    ("Bool", &["False", "True"]),
    ("Ordering", &["LT", "EQ", "GT"]),
];

impl DictPassCtxChirho {
    /// Give each builtin Enum key its four enumeration rows (row 671). Runs after Int's rows
    /// and every key's `toEnum` and `fromEnum` rows exist, and before any dictionary is filled.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn generate_builtin_enum_slot_rows_chirho(&mut self) {
        for method_chirho in ENUMERATION_METHODS_CHIRHO {
            let Some(int_row_chirho) = self
                .generated_bindings_chirho
                .iter()
                .find(|binding_chirho| {
                    binding_chirho.binder_chirho.name_chirho
                        == format!("$prim_Enum_{method_chirho}_Int")
                })
                .map(|binding_chirho| binding_chirho.binder_chirho.clone())
            else {
                continue;
            };
            for key_chirho in SHARED_BODY_KEYS_CHIRHO {
                self.shared_body_row_chirho(method_chirho, key_chirho, &int_row_chirho);
            }
        }
        for (key_chirho, constructors_chirho) in CONSTRUCTOR_KEYS_CHIRHO {
            self.generate_builtin_constructor_enum_rows_chirho(key_chirho, constructors_chirho);
        }
    }

    /// `$prim_Enum_<method>_<key> = $prim_Enum_<method>_Int`, by Int's CoreId, unless the key
    /// already has a body for the method.
    fn shared_body_row_chirho(
        &mut self,
        method_chirho: &str,
        key_chirho: &str,
        int_row_chirho: &BinderChirho,
    ) {
        let row_name_chirho = format!("$prim_Enum_{method_chirho}_{key_chirho}");
        if self
            .lookup_body_backed_name_id_chirho(&row_name_chirho)
            .is_some()
        {
            return;
        }
        let row_id_chirho = self.resolve_or_fresh_id_chirho(&row_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: row_id_chirho,
                name_chirho: row_name_chirho,
                ty_chirho: int_row_chirho.ty_chirho.clone(),
                span_chirho: int_row_chirho.span_chirho,
            },
            rhs_chirho: CoreExprChirho::VarChirho(int_row_chirho.id_chirho),
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
