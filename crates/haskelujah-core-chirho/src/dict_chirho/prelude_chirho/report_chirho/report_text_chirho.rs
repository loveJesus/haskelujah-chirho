// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The report's `lines`, `words`, `unlines` and `unwords`, and the String `intercalate`, as lazy
//! Core (row 579). Each was a strict text primop (`lines#`, `wordsStr#`, `unlines#`,
//! `unwordsStr#`, `intercalateStr#`) that read only a literal: a String built at run time read
//! as "" or lost its run-time elements, and so did an infinite one. These follow base, so each
//! line or word comes out before the next is read, on every engine.
//! - `lines` and `words` split with `takeWhile` and `dropWhile`, the report's meaning of
//!   `break`. The generated `span` forces its recursion, so it cannot serve an infinite line.
//! - `words` and the public `isSpace` share one generated test of GHC's `isSpace` code points;
//!   `isSpace` is a guarded alias of it. Its negation is built from the same table, not by
//!   running it. The interpreter's `isSpace#` also counted U+0085, U+2028 and U+2029, and
//!   neither native backend has `isSpace#`.
//! - The joins use the generated lazy `append`. Natively, the String `++#` packs both operands.
//!
//! workflow: compiler-pipeline-chirho/runtime-primitive-evaluation-chirho

use super::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreExprChirho, CoreLitChirho, DictPassCtxChirho,
    TyChirho, alt_chirho, apply_chirho, lambdas_chirho, var_chirho,
};

#[cfg(test)]
#[path = "report_text_tests_chirho.rs"]
mod report_text_tests_chirho;

/// The code points GHC 9.14.1's `isSpace` accepts, as inclusive ranges, measured over every
/// `Char`. The first `LOW_SPACE_RANGES_CHIRHO` end at the space; the rest start at U+00A0.
const GHC_SPACE_RANGES_CHIRHO: [(i64, i64); 8] = [
    (9, 13),
    (32, 32),
    (160, 160),
    (5760, 5760),
    (8192, 8202),
    (8239, 8239),
    (8287, 8287),
    (12288, 12288),
];

/// How many of `GHC_SPACE_RANGES_CHIRHO` lie at or below the space.
const LOW_SPACE_RANGES_CHIRHO: usize = 2;

/// The code point that ends a line.
const NEWLINE_CODE_POINT_CHIRHO: i64 = 10;

// The Char tests the text functions share, as generated bindings. The public `isSpace` is a
// guarded alias of the first, so `words` and `isSpace` agree on every Char.
const IS_SPACE_ROW_CHIRHO: &str = "$is_space_chirho";
const IS_NOT_SPACE_ROW_CHIRHO: &str = "$is_not_space_chirho";
const IS_NOT_NEWLINE_ROW_CHIRHO: &str = "$is_not_newline_chirho";

fn char_ty_chirho() -> TyChirho {
    TyChirho::ConChirho("Char".to_string())
}

fn strings_ty_chirho() -> TyChirho {
    TyChirho::ListChirho(Box::new(TyChirho::string_chirho()))
}

fn con_chirho(name_chirho: &str, args_chirho: Vec<CoreExprChirho>) -> CoreExprChirho {
    CoreExprChirho::ConAppChirho {
        con_name_chirho: name_chirho.to_string(),
        args_chirho,
    }
}

fn nil_chirho() -> CoreExprChirho {
    con_chirho("[]", vec![])
}

fn cons_chirho(head_chirho: CoreExprChirho, tail_chirho: CoreExprChirho) -> CoreExprChirho {
    con_chirho(":", vec![head_chirho, tail_chirho])
}

fn bool_chirho(value_chirho: bool) -> CoreExprChirho {
    con_chirho(if value_chirho { "True" } else { "False" }, vec![])
}

fn char_lit_chirho(value_chirho: char) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::CharChirho(value_chirho))
}

/// `ord# c <op> bound`.
fn code_point_test_chirho(
    c_chirho: &BinderChirho,
    op_chirho: &str,
    bound_chirho: i64,
) -> CoreExprChirho {
    let code_point_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "ord#".to_string(),
        args_chirho: vec![var_chirho(c_chirho)],
    };
    CoreExprChirho::PrimOpChirho {
        name_chirho: op_chirho.to_string(),
        args_chirho: vec![
            code_point_chirho,
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(bound_chirho)),
        ],
    }
}

fn default_alt_chirho(rhs_chirho: CoreExprChirho) -> CoreAltChirho {
    CoreAltChirho {
        con_chirho: AltConChirho::DefaultChirho,
        binders_chirho: vec![],
        rhs_chirho,
    }
}

impl DictPassCtxChirho {
    /// Generate the report's `lines`, `words`, `unlines` and `unwords`, the String
    /// `intercalate`, and `isSpace`, at the types the checker already gives them.
    /// workflow: compiler-pipeline-chirho/runtime-primitive-evaluation-chirho
    pub(in crate::dict_chirho::prelude_chirho) fn generate_report_text_prelude_chirho(&mut self) {
        self.generate_char_tests_chirho();
        let split_ty_chirho = TyChirho::fun_chirho(TyChirho::string_chirho(), strings_ty_chirho());
        let join_ty_chirho = TyChirho::fun_chirho(strings_ty_chirho(), TyChirho::string_chirho());
        self.generate_lines_chirho(split_ty_chirho.clone());
        self.generate_words_chirho(split_ty_chirho);
        self.generate_unlines_chirho(join_ty_chirho.clone());
        self.generate_unwords_chirho(join_ty_chirho.clone());
        self.generate_intercalate_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            join_ty_chirho,
        ));
    }

    // lines s = case s of
    //   [] -> []
    //   _ -> takeWhile (/= '\n') s : case dropWhile (/= '\n') s of { [] -> []; _ : rest -> lines rest }
    fn generate_lines_chirho(&mut self, ty_chirho: TyChirho) {
        let lines_id_chirho = self.resolve_or_fresh_id_chirho("lines");
        let s_chirho = self.fresh_binder_chirho("s", TyChirho::string_chirho());
        let newline_chirho = self.fresh_binder_chirho("nl", char_ty_chirho());
        let rest_chirho = self.fresh_binder_chirho("rest", TyChirho::string_chirho());
        let line_test_chirho = self.char_test_chirho(IS_NOT_NEWLINE_ROW_CHIRHO);
        let line_chirho = self.while_chirho("takeWhile", line_test_chirho, var_chirho(&s_chirho));
        let end_test_chirho = self.char_test_chirho(IS_NOT_NEWLINE_ROW_CHIRHO);
        let after_chirho = self.while_chirho("dropWhile", end_test_chirho, var_chirho(&s_chirho));
        let next_chirho = apply_chirho(
            CoreExprChirho::VarChirho(lines_id_chirho),
            vec![var_chirho(&rest_chirho)],
        );
        let more_chirho = self.case_list_chirho(
            after_chirho,
            TyChirho::string_chirho(),
            strings_ty_chirho(),
            alt_chirho(":", vec![newline_chirho, rest_chirho], next_chirho),
        );
        let body_chirho = self.case_list_chirho(
            var_chirho(&s_chirho),
            TyChirho::string_chirho(),
            strings_ty_chirho(),
            default_alt_chirho(cons_chirho(line_chirho, more_chirho)),
        );
        self.push_report_binding_chirho(
            lines_id_chirho,
            "lines",
            ty_chirho,
            lambdas_chirho(vec![s_chirho], body_chirho),
            true,
        );
    }

    // words s = case dropWhile isSpace s of
    //   [] -> []
    //   h : t -> (h : takeWhile (not . isSpace) t) : words (dropWhile (not . isSpace) t)
    // `h` is no space, so the word's split begins after it.
    fn generate_words_chirho(&mut self, ty_chirho: TyChirho) {
        let words_id_chirho = self.resolve_or_fresh_id_chirho("words");
        let s_chirho = self.fresh_binder_chirho("s", TyChirho::string_chirho());
        let h_chirho = self.fresh_binder_chirho("h", char_ty_chirho());
        let t_chirho = self.fresh_binder_chirho("t", TyChirho::string_chirho());
        let space_chirho = self.char_test_chirho(IS_SPACE_ROW_CHIRHO);
        let start_chirho = self.while_chirho("dropWhile", space_chirho, var_chirho(&s_chirho));
        let letter_chirho = self.char_test_chirho(IS_NOT_SPACE_ROW_CHIRHO);
        let word_tail_chirho = self.while_chirho("takeWhile", letter_chirho, var_chirho(&t_chirho));
        let letter_chirho = self.char_test_chirho(IS_NOT_SPACE_ROW_CHIRHO);
        let after_chirho = self.while_chirho("dropWhile", letter_chirho, var_chirho(&t_chirho));
        let word_chirho = cons_chirho(var_chirho(&h_chirho), word_tail_chirho);
        let next_chirho = apply_chirho(
            CoreExprChirho::VarChirho(words_id_chirho),
            vec![after_chirho],
        );
        let body_chirho = self.case_list_chirho(
            start_chirho,
            TyChirho::string_chirho(),
            strings_ty_chirho(),
            alt_chirho(
                ":",
                vec![h_chirho, t_chirho],
                cons_chirho(word_chirho, next_chirho),
            ),
        );
        self.push_report_binding_chirho(
            words_id_chirho,
            "words",
            ty_chirho,
            lambdas_chirho(vec![s_chirho], body_chirho),
            true,
        );
    }

    // unlines ls = case ls of { [] -> []; l : rest -> l ++ '\n' : unlines rest }
    fn generate_unlines_chirho(&mut self, ty_chirho: TyChirho) {
        let unlines_id_chirho = self.resolve_or_fresh_id_chirho("unlines");
        let ls_chirho = self.fresh_binder_chirho("ls", strings_ty_chirho());
        let l_chirho = self.fresh_binder_chirho("l", TyChirho::string_chirho());
        let rest_chirho = self.fresh_binder_chirho("rest", strings_ty_chirho());
        let next_chirho = apply_chirho(
            CoreExprChirho::VarChirho(unlines_id_chirho),
            vec![var_chirho(&rest_chirho)],
        );
        let joined_chirho = self.append_chirho(
            var_chirho(&l_chirho),
            cons_chirho(char_lit_chirho('\n'), next_chirho),
        );
        let body_chirho = self.case_list_chirho(
            var_chirho(&ls_chirho),
            strings_ty_chirho(),
            TyChirho::string_chirho(),
            alt_chirho(":", vec![l_chirho, rest_chirho], joined_chirho),
        );
        self.push_report_binding_chirho(
            unlines_id_chirho,
            "unlines",
            ty_chirho,
            lambdas_chirho(vec![ls_chirho], body_chirho),
            true,
        );
    }

    // unwords ws = case ws of
    //   [] -> []
    //   w : rest -> w ++ case rest of { [] -> []; _ -> ' ' : unwords rest }
    // The test of `rest` sits in `++`'s second argument, so `w` comes out before it runs.
    fn generate_unwords_chirho(&mut self, ty_chirho: TyChirho) {
        let unwords_id_chirho = self.resolve_or_fresh_id_chirho("unwords");
        let ws_chirho = self.fresh_binder_chirho("ws", strings_ty_chirho());
        let w_chirho = self.fresh_binder_chirho("w", TyChirho::string_chirho());
        let rest_chirho = self.fresh_binder_chirho("rest", strings_ty_chirho());
        let next_chirho = apply_chirho(
            CoreExprChirho::VarChirho(unwords_id_chirho),
            vec![var_chirho(&rest_chirho)],
        );
        let tail_chirho = self.case_list_chirho(
            var_chirho(&rest_chirho),
            strings_ty_chirho(),
            TyChirho::string_chirho(),
            default_alt_chirho(cons_chirho(char_lit_chirho(' '), next_chirho)),
        );
        let joined_chirho = self.append_chirho(var_chirho(&w_chirho), tail_chirho);
        let body_chirho = self.case_list_chirho(
            var_chirho(&ws_chirho),
            strings_ty_chirho(),
            TyChirho::string_chirho(),
            alt_chirho(":", vec![w_chirho, rest_chirho], joined_chirho),
        );
        self.push_report_binding_chirho(
            unwords_id_chirho,
            "unwords",
            ty_chirho,
            lambdas_chirho(vec![ws_chirho], body_chirho),
            true,
        );
    }

    // intercalate sep ws = case ws of
    //   [] -> []
    //   w : rest -> w ++ case rest of { [] -> []; _ -> sep ++ intercalate sep rest }
    fn generate_intercalate_chirho(&mut self, ty_chirho: TyChirho) {
        let intercalate_id_chirho = self.resolve_or_fresh_id_chirho("intercalate");
        let sep_chirho = self.fresh_binder_chirho("sep", TyChirho::string_chirho());
        let ws_chirho = self.fresh_binder_chirho("ws", strings_ty_chirho());
        let w_chirho = self.fresh_binder_chirho("w", TyChirho::string_chirho());
        let rest_chirho = self.fresh_binder_chirho("rest", strings_ty_chirho());
        let next_chirho = apply_chirho(
            CoreExprChirho::VarChirho(intercalate_id_chirho),
            vec![var_chirho(&sep_chirho), var_chirho(&rest_chirho)],
        );
        let separated_chirho = self.append_chirho(var_chirho(&sep_chirho), next_chirho);
        let tail_chirho = self.case_list_chirho(
            var_chirho(&rest_chirho),
            strings_ty_chirho(),
            TyChirho::string_chirho(),
            default_alt_chirho(separated_chirho),
        );
        let joined_chirho = self.append_chirho(var_chirho(&w_chirho), tail_chirho);
        let body_chirho = self.case_list_chirho(
            var_chirho(&ws_chirho),
            strings_ty_chirho(),
            TyChirho::string_chirho(),
            alt_chirho(":", vec![w_chirho, rest_chirho], joined_chirho),
        );
        self.push_report_binding_chirho(
            intercalate_id_chirho,
            "intercalate",
            ty_chirho,
            lambdas_chirho(vec![sep_chirho, ws_chirho], body_chirho),
            true,
        );
    }

    /// The shared Char tests and the guarded `isSpace` alias. Both space tests come from
    /// `GHC_SPACE_RANGES_CHIRHO`, and neither runs the other.
    fn generate_char_tests_chirho(&mut self) {
        let test_ty_chirho = TyChirho::fun_chirho(char_ty_chirho(), TyChirho::bool_chirho());
        for (name_chirho, space_chirho) in [
            (IS_SPACE_ROW_CHIRHO, true),
            (IS_NOT_SPACE_ROW_CHIRHO, false),
        ] {
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho());
            let body_chirho = self.space_test_chirho(&c_chirho, space_chirho);
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            self.push_report_binding_chirho(
                id_chirho,
                name_chirho,
                test_ty_chirho.clone(),
                lambdas_chirho(vec![c_chirho], body_chirho),
                false,
            );
        }
        let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho());
        let body_chirho = code_point_test_chirho(&c_chirho, "/=#", NEWLINE_CODE_POINT_CHIRHO);
        let id_chirho = self.resolve_or_fresh_id_chirho(IS_NOT_NEWLINE_ROW_CHIRHO);
        self.push_report_binding_chirho(
            id_chirho,
            IS_NOT_NEWLINE_ROW_CHIRHO,
            test_ty_chirho,
            lambdas_chirho(vec![c_chirho], body_chirho),
            false,
        );
        self.push_prelude_alias_chirho("isSpace", IS_SPACE_ROW_CHIRHO);
    }

    /// A shared Char test, by name.
    fn char_test_chirho(&mut self, name_chirho: &str) -> CoreExprChirho {
        CoreExprChirho::VarChirho(self.resolve_or_fresh_id_chirho(name_chirho))
    }

    /// GHC 9.14.1's `isSpace c`, or its negation built from the same table with each leaf
    /// flipped. Most text lies strictly between the space and U+00A0, where two comparisons
    /// settle it.
    fn space_test_chirho(&mut self, c_chirho: &BinderChirho, space_chirho: bool) -> CoreExprChirho {
        let (low_chirho, high_chirho) = GHC_SPACE_RANGES_CHIRHO.split_at(LOW_SPACE_RANGES_CHIRHO);
        let low_test_chirho = self.ranges_test_chirho(c_chirho, low_chirho, space_chirho);
        let high_test_chirho = self.ranges_test_chirho(c_chirho, high_chirho, space_chirho);
        let above_low_chirho =
            code_point_test_chirho(c_chirho, ">#", low_chirho[low_chirho.len() - 1].1);
        let below_high_chirho = code_point_test_chirho(c_chirho, "<#", high_chirho[0].0);
        let upper_chirho = self.if_chirho(
            below_high_chirho,
            bool_chirho(!space_chirho),
            high_test_chirho,
        );
        self.if_chirho(above_low_chirho, upper_chirho, low_test_chirho)
    }

    /// Whether `c`'s code point lies in one of `ranges` (`inside`) or in none of them, tested
    /// in order.
    fn ranges_test_chirho(
        &mut self,
        c_chirho: &BinderChirho,
        ranges_chirho: &[(i64, i64)],
        inside_chirho: bool,
    ) -> CoreExprChirho {
        let Some((&last_chirho, earlier_chirho)) = ranges_chirho.split_last() else {
            return bool_chirho(!inside_chirho);
        };
        let mut test_chirho = self.range_test_chirho(c_chirho, last_chirho, inside_chirho);
        for &range_chirho in earlier_chirho.iter().rev() {
            let here_chirho = self.range_test_chirho(c_chirho, range_chirho, true);
            test_chirho = self.if_chirho(here_chirho, bool_chirho(inside_chirho), test_chirho);
        }
        test_chirho
    }

    /// Whether `c`'s code point lies in `low..=high` (`inside`) or outside it.
    fn range_test_chirho(
        &mut self,
        c_chirho: &BinderChirho,
        (low_chirho, high_chirho): (i64, i64),
        inside_chirho: bool,
    ) -> CoreExprChirho {
        if low_chirho == high_chirho {
            let op_chirho = if inside_chirho { "==#" } else { "/=#" };
            return code_point_test_chirho(c_chirho, op_chirho, low_chirho);
        }
        let from_low_chirho = code_point_test_chirho(c_chirho, ">=#", low_chirho);
        let high_op_chirho = if inside_chirho { "<=#" } else { ">#" };
        let to_high_chirho = code_point_test_chirho(c_chirho, high_op_chirho, high_chirho);
        self.if_chirho(from_low_chirho, to_high_chirho, bool_chirho(!inside_chirho))
    }

    /// `case cond of { True -> yes; False -> no }`, a Bool.
    fn if_chirho(
        &mut self,
        cond_chirho: CoreExprChirho,
        yes_chirho: CoreExprChirho,
        no_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(cond_chirho),
            bind_chirho: self.fresh_binder_chirho("$wb", TyChirho::bool_chirho()),
            result_ty_chirho: TyChirho::bool_chirho(),
            alts_chirho: vec![
                alt_chirho("True", vec![], yes_chirho),
                alt_chirho("False", vec![], no_chirho),
            ],
        }
    }

    /// `case list of { [] -> []; <on_cons> }` at `result_ty`.
    fn case_list_chirho(
        &mut self,
        list_chirho: CoreExprChirho,
        list_ty_chirho: TyChirho,
        result_ty_chirho: TyChirho,
        on_cons_chirho: CoreAltChirho,
    ) -> CoreExprChirho {
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(list_chirho),
            bind_chirho: self.fresh_binder_chirho("$w", list_ty_chirho),
            result_ty_chirho,
            alts_chirho: vec![alt_chirho("[]", vec![], nil_chirho()), on_cons_chirho],
        }
    }

    /// The generated `takeWhile` or `dropWhile` applied to a test and a list.
    fn while_chirho(
        &mut self,
        name_chirho: &str,
        test_chirho: CoreExprChirho,
        list_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
        apply_chirho(
            CoreExprChirho::VarChirho(id_chirho),
            vec![test_chirho, list_chirho],
        )
    }

    /// The generated lazy list `append`.
    fn append_chirho(
        &mut self,
        left_chirho: CoreExprChirho,
        right_chirho: CoreExprChirho,
    ) -> CoreExprChirho {
        let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
        apply_chirho(
            CoreExprChirho::VarChirho(append_id_chirho),
            vec![left_chirho, right_chirho],
        )
    }
}
