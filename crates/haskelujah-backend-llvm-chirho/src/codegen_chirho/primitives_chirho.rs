// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Primitive lowering preserves operation identity; no unknown operation becomes
//! addition or zero. Scalar operands cross the WHNF boundary before use.

use super::{CoreExprChirho, LlvmCodegenChirho};
use std::fmt::Write;

impl LlvmCodegenChirho {
    pub(super) fn compile_primop_chirho(
        &mut self,
        name_chirho: &str,
        args_chirho: &[CoreExprChirho],
    ) -> String {
        if name_chirho == "getLine#" && args_chirho.is_empty() {
            let result_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {result_chirho} = call i64 @haskelujah_get_line_chirho()"
            )
            .unwrap();
            return result_chirho;
        }
        if name_chirho == "isHeapObjectChirho#" && args_chirho.len() == 1 {
            let value_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let value_chirho = self.emit_force_thunk_chirho(&value_chirho);
            let result_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {result_chirho} = call i64 @haskelujah_is_heap_ptr_chirho(i64 {value_chirho})"
            )
            .unwrap();
            return result_chirho;
        }
        if name_chirho == "seq#" && args_chirho.len() == 2 {
            let first_chirho = self.compile_expr_chirho(&args_chirho[0]);
            self.emit_force_thunk_chirho(&first_chirho);
            return self.compile_expr_chirho(&args_chirho[1]);
        }
        if matches!(name_chirho, "error" | "error#") && args_chirho.len() == 1 {
            let message_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let message_chirho = self.emit_force_thunk_chirho(&message_chirho);
            let pointer_chirho = self.fresh_tmp_chirho();
            let result_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {pointer_chirho} = inttoptr i64 {message_chirho} to ptr"
            )
            .unwrap();
            writeln!(
                self.output_chirho,
                "  {result_chirho} = call i64 @raise_error_chirho(ptr {pointer_chirho})"
            )
            .unwrap();
            return result_chirho;
        }

        if args_chirho.len() == 2 {
            let lhs_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let lhs_chirho = self.emit_force_thunk_chirho(&lhs_chirho);
            let rhs_chirho = self.compile_expr_chirho(&args_chirho[1]);
            let rhs_chirho = self.emit_force_thunk_chirho(&rhs_chirho);
            if name_chirho == "++#" {
                let tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {tmp_chirho} = call i64 @haskelujah_append_str_chirho(i64 {lhs_chirho}, i64 {rhs_chirho})"
                )
                .unwrap();
                return tmp_chirho;
            }

            if name_chirho == "eqStr#" {
                return self.emit_string_equal_chirho(&lhs_chirho, &rhs_chirho);
            }
            if matches!(name_chirho, "compare#" | "compareChar#") {
                let less_chirho = self.fresh_tmp_chirho();
                let equal_chirho = self.fresh_tmp_chirho();
                let equal_or_greater_chirho = self.fresh_tmp_chirho();
                let result_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {less_chirho} = icmp slt i64 {lhs_chirho}, {rhs_chirho}"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  {equal_chirho} = icmp eq i64 {lhs_chirho}, {rhs_chirho}"
                )
                .unwrap();
                writeln!(
                    self.output_chirho,
                    "  {equal_or_greater_chirho} = select i1 {equal_chirho}, i64 1, i64 2"
                )
                .unwrap();
                writeln!(self.output_chirho, "  {result_chirho} = select i1 {less_chirho}, i64 0, i64 {equal_or_greater_chirho}").unwrap();
                return result_chirho;
            }
            if matches!(
                name_chirho,
                "+.#" | "-.#" | "*.#" | "/.#" | "eqFloat#" | "<.#" | ">.#"
            ) {
                return self.emit_float_binary_chirho(name_chirho, &lhs_chirho, &rhs_chirho);
            }
            let tmp_chirho = self.fresh_tmp_chirho();
            let op_chirho = match name_chirho {
                "+#" => "add",
                "-#" => "sub",
                "*#" => "mul",
                "div#" | "divInt#" => {
                    return self.emit_floor_div_mod_chirho(&lhs_chirho, &rhs_chirho, false);
                }
                "mod#" | "modInt#" => {
                    return self.emit_floor_div_mod_chirho(&lhs_chirho, &rhs_chirho, true);
                }
                "quot#" | "quotInt#" => "sdiv",
                "rem#" | "remInt#" => "srem",
                "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" => {
                    let cmp_pred_chirho = match name_chirho {
                        "==#" => "eq",
                        "/=#" => "ne",
                        "<#" => "slt",
                        "<=#" => "sle",
                        ">#" => "sgt",
                        ">=#" => "sge",
                        _ => "eq",
                    };
                    let cmp_tmp_chirho = self.fresh_tmp_chirho();
                    writeln!(
                        self.output_chirho,
                        "  {cmp_tmp_chirho} = icmp {cmp_pred_chirho} i64 {lhs_chirho}, {rhs_chirho}"
                    )
                    .unwrap();
                    writeln!(
                        self.output_chirho,
                        "  {tmp_chirho} = zext i1 {cmp_tmp_chirho} to i64"
                    )
                    .unwrap();
                    return tmp_chirho;
                }
                _ => return self.emit_unsupported_primitive_chirho(name_chirho, args_chirho.len()),
            };
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = {op_chirho} i64 {lhs_chirho}, {rhs_chirho}"
            )
            .unwrap();
            tmp_chirho
        } else if args_chirho.len() == 1 && name_chirho == "negate#" {
            let operand_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let operand_chirho = self.emit_force_thunk_chirho(&operand_chirho);
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = sub i64 0, {operand_chirho}"
            )
            .unwrap();
            tmp_chirho
        } else if args_chirho.len() == 1
            && matches!(
                name_chirho,
                "showInt#" | "showBool#" | "showChar#" | "showFloat#" | "readInt#"
            )
        {
            let rts_fn_chirho = match name_chirho {
                "showBool#" => "haskelujah_show_bool_chirho",
                "showChar#" => "haskelujah_show_char_chirho",
                "showFloat#" => "haskelujah_show_float_chirho",
                "readInt#" => "haskelujah_read_int_chirho",
                _ => "haskelujah_show_int_chirho",
            };
            let operand_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let operand_chirho = self.emit_force_thunk_chirho(&operand_chirho);
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = call i64 @{rts_fn_chirho}(i64 {operand_chirho})"
            )
            .unwrap();
            tmp_chirho
        } else if args_chirho.len() == 1 && name_chirho == "not#" {
            let operand_chirho = self.compile_expr_chirho(&args_chirho[0]);
            let operand_chirho = self.emit_force_thunk_chirho(&operand_chirho);
            let cmp_tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {cmp_tmp_chirho} = icmp eq i64 {operand_chirho}, 0"
            )
            .unwrap();
            let tmp_chirho = self.fresh_tmp_chirho();
            writeln!(
                self.output_chirho,
                "  {tmp_chirho} = zext i1 {cmp_tmp_chirho} to i64"
            )
            .unwrap();
            tmp_chirho
        } else {
            self.emit_unsupported_primitive_chirho(name_chirho, args_chirho.len())
        }
    }

    fn emit_unsupported_primitive_chirho(
        &mut self,
        name_chirho: &str,
        arity_chirho: usize,
    ) -> String {
        let description_chirho = format!("{name_chirho}/{arity_chirho}");
        self.unsupported_primitives_chirho
            .insert(description_chirho.clone());
        let global_chirho = self.intern_string_global_name_chirho(&format!(
            "unsupported primitive {description_chirho}"
        ));
        let result_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {result_chirho} = call i64 @raise_error_chirho(ptr @{global_chirho})"
        )
        .unwrap();
        result_chirho
    }

    fn emit_string_equal_chirho(&mut self, left_chirho: &str, right_chirho: &str) -> String {
        let left_ptr_chirho = self.fresh_tmp_chirho();
        let right_ptr_chirho = self.fresh_tmp_chirho();
        let order_chirho = self.fresh_tmp_chirho();
        let equal_chirho = self.fresh_tmp_chirho();
        let result_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {left_ptr_chirho} = inttoptr i64 {left_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {right_ptr_chirho} = inttoptr i64 {right_chirho} to ptr"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {order_chirho} = call i32 @strcmp(ptr {left_ptr_chirho}, ptr {right_ptr_chirho})"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {equal_chirho} = icmp eq i32 {order_chirho}, 0"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {result_chirho} = zext i1 {equal_chirho} to i64"
        )
        .unwrap();
        result_chirho
    }

    fn emit_float_binary_chirho(
        &mut self,
        name_chirho: &str,
        left_chirho: &str,
        right_chirho: &str,
    ) -> String {
        let left_float_chirho = self.fresh_tmp_chirho();
        let right_float_chirho = self.fresh_tmp_chirho();
        let computed_chirho = self.fresh_tmp_chirho();
        let result_chirho = self.fresh_tmp_chirho();
        writeln!(
            self.output_chirho,
            "  {left_float_chirho} = bitcast i64 {left_chirho} to double"
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "  {right_float_chirho} = bitcast i64 {right_chirho} to double"
        )
        .unwrap();
        let (operation_chirho, comparison_chirho) = match name_chirho {
            "+.#" => ("fadd", false),
            "-.#" => ("fsub", false),
            "*.#" => ("fmul", false),
            "/.#" => ("fdiv", false),
            "eqFloat#" => ("fcmp oeq", true),
            "<.#" => ("fcmp olt", true),
            ">.#" => ("fcmp ogt", true),
            _ => unreachable!(),
        };
        writeln!(self.output_chirho, "  {computed_chirho} = {operation_chirho} double {left_float_chirho}, {right_float_chirho}").unwrap();
        if comparison_chirho {
            writeln!(
                self.output_chirho,
                "  {result_chirho} = zext i1 {computed_chirho} to i64"
            )
            .unwrap();
        } else {
            writeln!(
                self.output_chirho,
                "  {result_chirho} = bitcast double {computed_chirho} to i64"
            )
            .unwrap();
        }
        result_chirho
    }
}
