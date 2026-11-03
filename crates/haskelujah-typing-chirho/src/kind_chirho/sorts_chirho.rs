// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Sort judgments (K01 reject frontier): a constraint where a type is required, and a
//! type where a constraint is required. Only a sort that has positively
//! resolved is reported. The global Type/Constraint unification stays as it
//! is, so an unknown kind never becomes a rejection.
//! workflow: language-features-chirho/declaration-kinds-chirho

use haskelujah_ast_chirho::name_chirho::NameChirho;

use super::{
    DiagnosticChirho, ErrorCodeChirho, KIND_MISMATCH_CODE_CHIRHO, KindChirho, KindInferCtxChirho,
    SpanChirho, TypeChirho,
};
use crate::validity_chirho::render_type_chirho;

/// Homogeneous equality, ASCII and UnicodeSyntax spellings (the same names
/// naming treats as builtin constraints).
const HOMOGENEOUS_EQUALITY_NAMES_CHIRHO: &[&str] = &["~", "∼"];
/// Heterogeneous equality.
const HETEROGENEOUS_EQUALITY_NAME_CHIRHO: &str = "~~";

impl KindInferCtxChirho {
    /// The kind of an equality constructor at one use: `~` is
    /// `k -> k -> Constraint` and `~~` is `k1 -> k2 -> Constraint`, with fresh
    /// variables every time. A module that declares a type of that name keeps
    /// its own.
    pub(super) fn equality_constraint_kind_chirho(
        &mut self,
        name_chirho: &str,
    ) -> Option<KindChirho> {
        let homogeneous_chirho = HOMOGENEOUS_EQUALITY_NAMES_CHIRHO.contains(&name_chirho);
        if (!homogeneous_chirho && name_chirho != HETEROGENEOUS_EQUALITY_NAME_CHIRHO)
            || self.local_kind_decl_names_chirho.contains(name_chirho)
        {
            return None;
        }
        let left_chirho = self.fresh_kind_chirho();
        let right_chirho = if homogeneous_chirho {
            left_chirho.clone()
        } else {
            self.fresh_kind_chirho()
        };
        Some(KindChirho::arrow_chirho(
            left_chirho,
            KindChirho::arrow_chirho(right_chirho, KindChirho::ConstraintChirho),
        ))
    }

    /// Report `ty_chirho`, written where a type is required (a signature, a
    /// method signature, either side of an arrow, a constructor field), when
    /// it is headed by a known constraint former and its kind has resolved to
    /// `Constraint`. Called after that position's own unification with `*`,
    /// which the Type/Constraint workaround lets pass.
    pub(super) fn report_constraint_at_type_position_chirho(
        &mut self,
        ty_chirho: &TypeChirho,
        kind_chirho: &KindChirho,
    ) {
        if !self.has_known_constraint_head_chirho(ty_chirho)
            || self.subst_chirho.apply_chirho(kind_chirho) != KindChirho::ConstraintChirho
        {
            return;
        }
        self.push_sort_error_chirho(
            format!(
                "expected a type, but `{}` is a constraint",
                render_type_chirho(ty_chirho)
            ),
            ty_chirho.span_chirho(),
        );
    }

    /// Report a constraint headed by a data or newtype declared in this module
    /// whose kind, after the written arguments, is `*`. Synonyms and imported
    /// names are never judged: a constraint-tuple synonym over classes of
    /// unknown kind can read as `*` without being a type.
    pub(super) fn report_type_at_constraint_position_chirho(
        &mut self,
        class_chirho: &NameChirho,
        args_chirho: &[TypeChirho],
        span_chirho: SpanChirho,
    ) {
        let head_chirho = class_chirho.full_name_chirho();
        if !self.local_data_type_names_chirho.contains(&head_chirho) {
            return;
        }
        let Some(head_kind_chirho) = self.env_chirho.lookup_chirho(&head_chirho).cloned() else {
            return;
        };
        let mut result_chirho = self.subst_chirho.apply_chirho(&head_kind_chirho);
        for _ in args_chirho {
            let KindChirho::ArrowChirho(_, rest_chirho) = result_chirho else {
                return;
            };
            result_chirho = *rest_chirho;
        }
        if result_chirho != KindChirho::StarChirho {
            return;
        }
        let mut words_chirho = vec![head_chirho];
        for arg_chirho in args_chirho {
            let text_chirho = render_type_chirho(arg_chirho);
            words_chirho.push(if text_chirho.contains(' ') {
                format!("({text_chirho})")
            } else {
                text_chirho
            });
        }
        self.push_sort_error_chirho(
            format!(
                "expected a constraint, but `{}` is a type",
                words_chirho.join(" ")
            ),
            span_chirho,
        );
    }

    /// Whether `ty_chirho` is headed by an equality or by a name this module
    /// declares, the heads whose kinds this pass knows. A type variable or an
    /// imported name is never judged: an imported constructor keeps the kind
    /// of its first use, so in `P (C Int) -> P a -> a` the variable `a`
    /// resolves to `Constraint` although GHC gives `P` a polymorphic kind.
    fn has_known_constraint_head_chirho(&self, ty_chirho: &TypeChirho) -> bool {
        match ty_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.full_name_chirho();
                self.local_kind_decl_names_chirho.contains(&text_chirho)
                    || HOMOGENEOUS_EQUALITY_NAMES_CHIRHO.contains(&text_chirho.as_str())
                    || text_chirho == HETEROGENEOUS_EQUALITY_NAME_CHIRHO
            }
            TypeChirho::AppChirho { fun_chirho, .. } => {
                self.has_known_constraint_head_chirho(fun_chirho)
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.has_known_constraint_head_chirho(inner_chirho)
            }
            TypeChirho::ForallChirho { body_chirho, .. }
            | TypeChirho::RequiredForallChirho { body_chirho, .. }
            | TypeChirho::QualChirho { body_chirho, .. } => {
                self.has_known_constraint_head_chirho(body_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .any(|element_chirho| self.has_known_constraint_head_chirho(element_chirho)),
            _ => false,
        }
    }

    fn push_sort_error_chirho(&mut self, message_chirho: String, span_chirho: SpanChirho) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }
}
