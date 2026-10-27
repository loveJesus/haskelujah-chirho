// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Enum's defaults for the methods a program's own instance does not write, as GHC's are
//! (row 669). GHC requires only `toEnum` and `fromEnum`, and every other method defaults
//! through those two with Int's own enumeration in between: `succ` is
//! `toEnum . (+ 1) . fromEnum`, `pred` subtracts 1, and `enumFromThenTo x y z` is
//! `map toEnum [fromEnum x, fromEnum y .. fromEnum z]`, like its three siblings. No default
//! calls another method the instance may write, so an overridden `succ` or `enumFromTo` never
//! reaches a default.
//!
//! The seeded Enum class lays out four methods, so `succ` and `pred` default through their
//! dictionary slots; the four enumerations have no slot, so a row is made for each key a
//! written call to one was proved at. Only a key whose `toEnum` and `fromEnum` rows the module
//! itself binds donates, so builtin instances keep their own rows, and a method the instance
//! writes keeps its own row. Polymorphic enumeration calls are row 671's.
//!
//! A type that derives Enum writes GHC's bounded `enumFrom` and `enumFromThen` instead of
//! taking those two defaults: `enumFrom x = enumFromTo x <last>`, and `enumFromThen x y` is
//! `enumFromThenTo x y` up to the last constructor when `fromEnum y >= fromEnum x` and down
//! to the first otherwise. They are built here from the deriving pass's constructor list, by
//! row ids, constructors and the `>=#` primop only, so no program-owned namesake can capture
//! them; their `enumFromTo` and `enumFromThenTo` are the class defaults above.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::BTreeSet;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use super::DictPassCtxChirho;
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, InlineAnnotationChirho,
};

/// The four enumeration methods, each with the number of arguments it takes.
const ENUMERATIONS_CHIRHO: [(&str, usize); 4] = [
    ("enumFrom", 1),
    ("enumFromThen", 2),
    ("enumFromTo", 2),
    ("enumFromThenTo", 3),
];

/// The Int primop of `succ`'s or `pred`'s step, for those two methods.
fn enum_step_primop_chirho(method_name_chirho: &str) -> Option<&'static str> {
    match method_name_chirho {
        "succ" => Some("+#"),
        "pred" => Some("-#"),
        _ => None,
    }
}

/// The number of arguments of an Enum method that has a default.
fn enum_default_arity_chirho(method_name_chirho: &str) -> Option<usize> {
    if enum_step_primop_chirho(method_name_chirho).is_some() {
        return Some(1);
    }
    ENUMERATIONS_CHIRHO
        .iter()
        .find(|(enumeration_chirho, _)| *enumeration_chirho == method_name_chirho)
        .map(|(_, arity_chirho)| *arity_chirho)
}

impl DictPassCtxChirho {
    /// GHC's default row `$prim_Enum_<method>_<key>` for an Enum method the program's own
    /// instance at `donor_key_chirho` does not write: `toEnum (fromEnum x ± 1)` for `succ` and
    /// `pred`, and `map toEnum (m (fromEnum a) …)` for each enumeration `m`, whose Int body is
    /// the compiler's own. None for any other class or method, for a key whose `toEnum` or
    /// `fromEnum` row the module does not bind, or when a Prelude function the default calls
    /// is absent.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn enum_default_row_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        donor_key_chirho: &str,
        prim_name_chirho: &str,
    ) -> Option<CoreIdChirho> {
        if class_name_chirho != "Enum" {
            return None;
        }
        let arity_chirho = enum_default_arity_chirho(method_name_chirho)?;
        let to_enum_row_chirho = self.program_enum_row_chirho("toEnum", donor_key_chirho)?;
        let from_enum_row_chirho = self.program_enum_row_chirho("fromEnum", donor_key_chirho)?;
        let parameters_chirho = self.enum_parameters_chirho(arity_chirho);
        let ints_chirho: Vec<CoreExprChirho> = parameters_chirho
            .iter()
            .map(|parameter_chirho| {
                Self::apply_all_chirho(
                    from_enum_row_chirho,
                    vec![Self::var_expr_chirho(parameter_chirho)],
                )
            })
            .collect();
        let (body_chirho, result_ty_chirho) =
            if let Some(primop_chirho) = enum_step_primop_chirho(method_name_chirho) {
                let stepped_chirho = CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![
                        ints_chirho[0].clone(),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                };
                (
                    Self::apply_all_chirho(to_enum_row_chirho, vec![stepped_chirho]),
                    TyChirho::int_chirho(),
                )
            } else {
                let map_id_chirho = self.lookup_compiler_owned_name_id_chirho("map")?;
                let int_enumeration_chirho =
                    self.lookup_compiler_owned_name_id_chirho(method_name_chirho)?;
                (
                    Self::apply_two_chirho(
                        map_id_chirho,
                        CoreExprChirho::VarChirho(to_enum_row_chirho),
                        Self::apply_all_chirho(int_enumeration_chirho, ints_chirho),
                    ),
                    Self::enum_list_ty_chirho(),
                )
            };
        Some(self.register_enum_row_chirho(
            prim_name_chirho,
            parameters_chirho,
            body_chirho,
            result_ty_chirho,
        ))
    }

    /// The program's own `toEnum` or `fromEnum` row at `key_chirho`: a body-backed row the
    /// module itself binds, never a builtin instance's row or a missing-method stub.
    fn program_enum_row_chirho(
        &self,
        method_name_chirho: &str,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.existing_keyed_row_chirho(&format!("$prim_Enum_{method_name_chirho}_"), key_chirho)
            .filter(|row_chirho| self.module_bound_ids_chirho.contains(row_chirho))
    }

    /// A default row for each enumeration method a written call was proved at, at a key whose
    /// instance does not write it. The four have no dictionary slot, so these rows are what
    /// the call's key finds. Runs after the instance dictionaries and the derived rows.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_evidenced_enum_default_rows_chirho(&mut self) {
        let wanted_chirho: BTreeSet<(String, String)> = self
            .occurrence_evidence_chirho
            .iter()
            .filter(|(_, (class_name_chirho, key_chirho))| {
                class_name_chirho == "Enum" && !is_own_dictionary_key_chirho(key_chirho)
            })
            .filter_map(|(id_chirho, (_, key_chirho))| {
                let (name_chirho, _) = self.method_occurrence_canon_chirho.get(id_chirho)?;
                let method_chirho = name_chirho.rsplit('.').next()?;
                ENUMERATIONS_CHIRHO
                    .iter()
                    .any(|(enumeration_chirho, _)| *enumeration_chirho == method_chirho)
                    .then(|| (method_chirho.to_string(), key_chirho.clone()))
            })
            .collect();
        for (method_chirho, key_chirho) in wanted_chirho {
            let row_name_chirho = format!("$prim_Enum_{method_chirho}_{key_chirho}");
            if self
                .lookup_body_backed_name_id_chirho(&row_name_chirho)
                .is_none()
            {
                self.enum_default_row_chirho("Enum", &method_chirho, &key_chirho, &row_name_chirho);
            }
        }
    }

    /// GHC's bounded `enumFrom` and `enumFromThen` for each local type that derives Enum, made
    /// before the evidenced default rows so a derived key's own pair wins. A type with no
    /// constructor is never recorded, and is skipped here as well.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(in crate::dict_chirho) fn generate_derived_enum_rows_chirho(&mut self) {
        let mut derived_chirho: Vec<(String, Vec<String>)> = self
            .derived_enums_chirho
            .iter()
            .map(|(key_chirho, constructors_chirho)| {
                (key_chirho.clone(), constructors_chirho.clone())
            })
            .collect();
        derived_chirho.sort();
        for (key_chirho, constructors_chirho) in derived_chirho {
            let (Some(first_chirho), Some(last_chirho)) =
                (constructors_chirho.first(), constructors_chirho.last())
            else {
                continue;
            };
            self.derived_enum_from_row_chirho(&key_chirho, last_chirho);
            self.derived_enum_from_then_row_chirho(&key_chirho, first_chirho, last_chirho);
        }
    }

    /// `$prim_Enum_enumFrom_<key> = \x -> enumFromTo_<key> x <last>`, over the key's own
    /// `enumFromTo` row.
    fn derived_enum_from_row_chirho(
        &mut self,
        key_chirho: &str,
        last_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let row_name_chirho = format!("$prim_Enum_enumFrom_{key_chirho}");
        if self
            .lookup_body_backed_name_id_chirho(&row_name_chirho)
            .is_some()
        {
            return None;
        }
        let enum_from_to_chirho = self.key_enum_row_chirho("enumFromTo", key_chirho)?;
        let parameters_chirho = self.enum_parameters_chirho(1);
        let body_chirho = Self::apply_two_chirho(
            enum_from_to_chirho,
            Self::var_expr_chirho(&parameters_chirho[0]),
            Self::nullary_con_chirho(last_chirho),
        );
        Some(self.register_enum_row_chirho(
            &row_name_chirho,
            parameters_chirho,
            body_chirho,
            Self::enum_list_ty_chirho(),
        ))
    }

    /// `$prim_Enum_enumFromThen_<key> = \x y -> enumFromThenTo_<key> x y (case fromEnum_<key> y
    /// >=# fromEnum_<key> x of True -> <last>; _ -> <first>)`, over the key's own rows.
    fn derived_enum_from_then_row_chirho(
        &mut self,
        key_chirho: &str,
        first_chirho: &str,
        last_chirho: &str,
    ) -> Option<CoreIdChirho> {
        let row_name_chirho = format!("$prim_Enum_enumFromThen_{key_chirho}");
        if self
            .lookup_body_backed_name_id_chirho(&row_name_chirho)
            .is_some()
        {
            return None;
        }
        let enum_from_then_to_chirho = self.key_enum_row_chirho("enumFromThenTo", key_chirho)?;
        let from_enum_chirho = self.program_enum_row_chirho("fromEnum", key_chirho)?;
        let parameters_chirho = self.enum_parameters_chirho(2);
        let tag_chirho = |parameter_chirho: &BinderChirho| {
            Self::apply_all_chirho(
                from_enum_chirho,
                vec![Self::var_expr_chirho(parameter_chirho)],
            )
        };
        let ascending_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: ">=#".to_string(),
            args_chirho: vec![
                tag_chirho(&parameters_chirho[1]),
                tag_chirho(&parameters_chirho[0]),
            ],
        };
        let limit_chirho = CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ascending_chirho),
            bind_chirho: self.fresh_binder_chirho("$derived_enum_up", TyChirho::bool_chirho()),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![
                CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("True".to_string()),
                    binders_chirho: vec![],
                    rhs_chirho: Self::nullary_con_chirho(last_chirho),
                },
                CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: Self::nullary_con_chirho(first_chirho),
                },
            ],
        };
        let body_chirho = Self::apply_all_chirho(
            enum_from_then_to_chirho,
            vec![
                Self::var_expr_chirho(&parameters_chirho[0]),
                Self::var_expr_chirho(&parameters_chirho[1]),
                limit_chirho,
            ],
        );
        Some(self.register_enum_row_chirho(
            &row_name_chirho,
            parameters_chirho,
            body_chirho,
            Self::enum_list_ty_chirho(),
        ))
    }

    /// The key's own body-backed `$prim_Enum_<method>_<key>` row, or, for an enumeration its
    /// instance does not write, the class default made now.
    fn key_enum_row_chirho(
        &mut self,
        method_name_chirho: &str,
        key_chirho: &str,
    ) -> Option<CoreIdChirho> {
        self.existing_keyed_row_chirho(&format!("$prim_Enum_{method_name_chirho}_"), key_chirho)
            .or_else(|| {
                self.enum_default_row_chirho(
                    "Enum",
                    method_name_chirho,
                    key_chirho,
                    &format!("$prim_Enum_{method_name_chirho}_{key_chirho}"),
                )
            })
    }

    /// `arity_chirho` fresh parameters for a generated Enum row.
    fn enum_parameters_chirho(&mut self, arity_chirho: usize) -> Vec<BinderChirho> {
        (0..arity_chirho)
            .map(|index_chirho| {
                self.fresh_binder_chirho(
                    &format!("$enum_row_{index_chirho}"),
                    TyChirho::int_chirho(),
                )
            })
            .collect()
    }

    fn enum_list_ty_chirho() -> TyChirho {
        TyChirho::ListChirho(Box::new(TyChirho::int_chirho()))
    }

    /// Bind `prim_name_chirho` to `\parameters -> body` as a generated row, returning its id.
    fn register_enum_row_chirho(
        &mut self,
        prim_name_chirho: &str,
        parameters_chirho: Vec<BinderChirho>,
        body_chirho: CoreExprChirho,
        result_ty_chirho: TyChirho,
    ) -> CoreIdChirho {
        let parameter_tys_chirho: Vec<TyChirho> = parameters_chirho
            .iter()
            .map(|parameter_chirho| parameter_chirho.ty_chirho.clone())
            .collect();
        let rhs_chirho = parameters_chirho.into_iter().rev().fold(
            body_chirho,
            |inner_chirho, parameter_chirho| CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(inner_chirho),
            },
        );
        let row_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho: row_id_chirho,
                name_chirho: prim_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_n_chirho(parameter_tys_chirho, result_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
        row_id_chirho
    }
}
