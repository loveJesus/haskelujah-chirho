// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # LLVM IR code generation
//!
//! Translates Core IR to textual LLVM IR. Currently implements a strict
//! evaluation model for the subset of Core we can handle:
//!
//! - Integer literals → i64 constants
//! - Function definitions → LLVM functions
//! - Function application → direct calls (when function is known)
//! - Case expressions → br/switch on scrutinee
//! - Let bindings → alloca + store/load
//! - Lambdas → lifted to top-level functions (lambda lifting)
//!
//! The full STG machine with lazy evaluation, thunks, closures, and
//! info tables will be added incrementally.

use std::collections::HashSet;
use std::fmt::Write;

use rhasky_core_chirho::{
    AltConChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
    CoreModuleChirho,
};

/// LLVM IR generation context.
pub struct LlvmCodegenChirho {
    /// The LLVM IR output buffer.
    output_chirho: String,
    /// Counter for generating unique LLVM temporaries (%t0, %t1, ...).
    next_tmp_chirho: u32,
    /// Counter for generating unique LLVM labels.
    next_label_chirho: u32,
    /// Lifted lambda functions accumulated during codegen.
    lifted_functions_chirho: Vec<String>,
    /// Track emitted function names to prevent duplicate definitions.
    emitted_names_chirho: HashSet<String>,
    /// Maps CoreId → name for top-level bindings, used to resolve cross-references.
    toplevel_names_chirho: std::collections::HashMap<CoreIdChirho, String>,
    /// Tracks CoreIds that are lambda parameters or let-bound in the current function scope.
    local_scope_chirho: HashSet<CoreIdChirho>,
}

impl LlvmCodegenChirho {
    pub fn new_chirho() -> Self {
        Self {
            output_chirho: String::new(),
            next_tmp_chirho: 0,
            next_label_chirho: 0,
            lifted_functions_chirho: Vec::new(),
            emitted_names_chirho: HashSet::new(),
            toplevel_names_chirho: std::collections::HashMap::new(),
            local_scope_chirho: HashSet::new(),
        }
    }

    /// Generate a fresh LLVM temporary name.
    fn fresh_tmp_chirho(&mut self) -> String {
        let tmp_chirho = format!("%t{}", self.next_tmp_chirho);
        self.next_tmp_chirho += 1;
        tmp_chirho
    }

    /// Generate a fresh LLVM label.
    fn fresh_label_chirho(&mut self, prefix_chirho: &str) -> String {
        let label_chirho = format!("{prefix_chirho}.{}", self.next_label_chirho);
        self.next_label_chirho += 1;
        label_chirho
    }

    /// Compile a Core module to LLVM IR text.
    pub fn compile_module_chirho(&mut self, module_chirho: &CoreModuleChirho) -> String {
        self.output_chirho.clear();
        self.lifted_functions_chirho.clear();

        // Module header
        writeln!(
            self.output_chirho,
            "; ModuleID = '{}'",
            module_chirho.name_chirho
        )
        .unwrap();
        writeln!(
            self.output_chirho,
            "source_filename = \"{}.hs\"",
            module_chirho.name_chirho
        )
        .unwrap();
        // Omit target datalayout/triple to let clang select the native target
        writeln!(self.output_chirho, "; target: native").unwrap();
        writeln!(self.output_chirho).unwrap();

        // Declare external functions we might call
        writeln!(
            self.output_chirho,
            "declare i64 @rhasky_negate_chirho(i64)"
        )
        .unwrap();
        writeln!(self.output_chirho).unwrap();

        // Collect top-level binding CoreIds for cross-reference resolution
        let mut toplevel_names_chirho = std::collections::HashMap::new();
        for binding_chirho in &module_chirho.bindings_chirho {
            toplevel_names_chirho.insert(
                binding_chirho.binder_chirho.id_chirho,
                binding_chirho.binder_chirho.name_chirho.clone(),
            );
        }
        self.toplevel_names_chirho = toplevel_names_chirho;

        // Compile each top-level binding
        for binding_chirho in &module_chirho.bindings_chirho {
            self.compile_binding_chirho(binding_chirho);
        }

        // Append any lifted lambdas
        for lifted_chirho in &self.lifted_functions_chirho.clone() {
            self.output_chirho.push_str(lifted_chirho);
            self.output_chirho.push('\n');
        }

        self.output_chirho.clone()
    }

    /// Compile a top-level binding to an LLVM function.
    fn compile_binding_chirho(&mut self, binding_chirho: &CoreBindingChirho) {
        let fn_name_chirho = mangle_name_chirho(&binding_chirho.binder_chirho.name_chirho);

        // Skip duplicate function definitions (e.g. Prelude `min` and `$prim_Ord_min_Int`
        // can both mangle to the same name)
        if !self.emitted_names_chirho.insert(fn_name_chirho.clone()) {
            return;
        }

        // Collect lambda parameters
        let (params_chirho, body_chirho) =
            collect_lambda_params_chirho(&binding_chirho.rhs_chirho);

        // Track local scope: lambda parameters are local
        self.local_scope_chirho.clear();
        for id_chirho in &params_chirho {
            self.local_scope_chirho.insert(*id_chirho);
        }

        // Build parameter list
        let params_str_chirho: String = params_chirho
            .iter()
            .enumerate()
            .map(|(_i_chirho, id_chirho)| format!("i64 %v{}", id_chirho.0))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            self.output_chirho,
            "define i64 @{fn_name_chirho}({params_str_chirho}) {{"
        )
        .unwrap();
        writeln!(self.output_chirho, "entry:").unwrap();

        // Reset temporaries for this function
        self.next_tmp_chirho = 0;

        // Compile the body
        let result_chirho = self.compile_expr_chirho(body_chirho);

        writeln!(self.output_chirho, "  ret i64 {result_chirho}").unwrap();
        writeln!(self.output_chirho, "}}").unwrap();
        writeln!(self.output_chirho).unwrap();
    }

    /// Compile a Core expression, returning the LLVM value name holding the result.
    fn compile_expr_chirho(&mut self, expr_chirho: &CoreExprChirho) -> String {
        match expr_chirho {
            CoreExprChirho::LitChirho(lit_chirho) => self.compile_lit_chirho(lit_chirho),

            CoreExprChirho::VarChirho(id_chirho) => {
                if self.local_scope_chirho.contains(id_chirho) {
                    // Local variable (parameter, let-bound, case binder)
                    format!("%v{}", id_chirho.0)
                } else if let Some(name_chirho) = self.toplevel_names_chirho.get(id_chirho) {
                    // Reference to a top-level binding — call it as a zero-arg function
                    let mangled_chirho = mangle_name_chirho(name_chirho);
                    let tmp_chirho = self.fresh_tmp_chirho();
                    writeln!(
                        self.output_chirho,
                        "  {tmp_chirho} = call i64 @{mangled_chirho}()"
                    )
                    .unwrap();
                    tmp_chirho
                } else {
                    // Unknown variable — use as local (may be from outer scope
                    // like case binder, which we've already bound)
                    format!("%v{}", id_chirho.0)
                }
            }

            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                // Try to detect a direct call pattern: f arg1 arg2 ...
                let (callee_chirho, args_chirho) = flatten_app_chirho(expr_chirho);
                match callee_chirho {
                    CoreExprChirho::VarChirho(id_chirho) => {
                        // Compile all arguments
                        let arg_vals_chirho: Vec<String> = args_chirho
                            .iter()
                            .map(|a_chirho| self.compile_expr_chirho(a_chirho))
                            .collect();
                        let args_str_chirho = arg_vals_chirho
                            .iter()
                            .map(|v_chirho| format!("i64 {v_chirho}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let tmp_chirho = self.fresh_tmp_chirho();
                        // Resolve the function name: top-level binding or local var
                        let fn_ref_chirho = if let Some(name_chirho) =
                            self.toplevel_names_chirho.get(id_chirho)
                        {
                            format!("@{}", mangle_name_chirho(name_chirho))
                        } else {
                            format!("@rhasky_v{}", id_chirho.0)
                        };
                        writeln!(
                            self.output_chirho,
                            "  {tmp_chirho} = call i64 {fn_ref_chirho}({args_str_chirho})"
                        )
                        .unwrap();
                        tmp_chirho
                    }
                    _ => {
                        // Unknown callee — compile as indirect call (simplified)
                        let fun_val_chirho = self.compile_expr_chirho(fun_chirho);
                        let arg_val_chirho = self.compile_expr_chirho(arg_chirho);
                        let tmp_chirho = self.fresh_tmp_chirho();
                        // Placeholder: in a real impl this would be an indirect call
                        // through a function pointer or closure
                        writeln!(
                            self.output_chirho,
                            "  ; indirect apply {fun_val_chirho} {arg_val_chirho}"
                        )
                        .unwrap();
                        writeln!(
                            self.output_chirho,
                            "  {tmp_chirho} = add i64 {fun_val_chirho}, {arg_val_chirho}"
                        )
                        .unwrap();
                        tmp_chirho
                    }
                }
            }

            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                // Lambda that wasn't collected as a top-level function parameter.
                // Lift it to a separate function.
                let lifted_name_chirho = format!(
                    "rhasky_lambda_{}",
                    self.lifted_functions_chirho.len()
                );
                let param_chirho = format!("%v{}", binder_chirho.id_chirho.0);

                let mut inner_codegen_chirho = LlvmCodegenChirho::new_chirho();
                writeln!(
                    inner_codegen_chirho.output_chirho,
                    "define i64 @{lifted_name_chirho}(i64 {param_chirho}) {{"
                )
                .unwrap();
                writeln!(inner_codegen_chirho.output_chirho, "entry:").unwrap();

                let result_chirho = inner_codegen_chirho.compile_expr_chirho(body_chirho);

                writeln!(
                    inner_codegen_chirho.output_chirho,
                    "  ret i64 {result_chirho}"
                )
                .unwrap();
                writeln!(inner_codegen_chirho.output_chirho, "}}").unwrap();

                self.lifted_functions_chirho
                    .push(inner_codegen_chirho.output_chirho);

                // Return a reference to the lifted function (as a function pointer cast)
                let tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {tmp_chirho} = ptrtoint ptr @{lifted_name_chirho} to i64"
                )
                .unwrap();
                tmp_chirho
            }

            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                // Compile each binding, assigning the result to the binder's variable
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    self.local_scope_chirho.insert(binder_chirho.id_chirho);
                    let val_chirho = self.compile_expr_chirho(rhs_chirho);
                    writeln!(
                        self.output_chirho,
                        "  ; let {} = {}",
                        binder_chirho.name_chirho, val_chirho
                    )
                    .unwrap();
                    // We need to alias %v{id} to the computed value.
                    // In LLVM SSA, we use an identity add.
                    writeln!(
                        self.output_chirho,
                        "  %v{} = add i64 0, {val_chirho}",
                        binder_chirho.id_chirho.0
                    )
                    .unwrap();
                }
                self.compile_expr_chirho(body_chirho)
            }

            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                let scrut_val_chirho = self.compile_expr_chirho(scrutinee_chirho);

                // Bind the case binder to the scrutinee value so %v{id} is defined
                self.local_scope_chirho.insert(bind_chirho.id_chirho);
                let case_binder_var_chirho = format!("%v{}", bind_chirho.id_chirho.0);
                writeln!(
                    self.output_chirho,
                    "  {case_binder_var_chirho} = add i64 0, {scrut_val_chirho}"
                )
                .unwrap();

                if alts_chirho.is_empty() {
                    return scrut_val_chirho;
                }

                // For literal case: use switch instruction
                // For data constructor case: use comparison chains
                let result_tmp_chirho = self.fresh_tmp_chirho();
                let end_label_chirho = self.fresh_label_chirho("case.end");

                // Allocate a stack slot for the result
                writeln!(
                    self.output_chirho,
                    "  {result_tmp_chirho}.addr = alloca i64"
                )
                .unwrap();

                // Check if we have literal alts
                let has_lit_alts_chirho = alts_chirho.iter().any(|a_chirho| {
                    matches!(a_chirho.con_chirho, AltConChirho::LitConChirho(_))
                });

                if has_lit_alts_chirho {
                    self.compile_case_lit_chirho(
                        &scrut_val_chirho,
                        alts_chirho,
                        &result_tmp_chirho,
                        &end_label_chirho,
                    );
                } else {
                    // Data constructor or default-only: compile each alt as a branch
                    self.compile_case_data_chirho(
                        &scrut_val_chirho,
                        alts_chirho,
                        &result_tmp_chirho,
                        &end_label_chirho,
                    );
                }

                writeln!(self.output_chirho, "{end_label_chirho}:").unwrap();
                let load_tmp_chirho = self.fresh_tmp_chirho();
                writeln!(
                    self.output_chirho,
                    "  {load_tmp_chirho} = load i64, ptr {result_tmp_chirho}.addr"
                )
                .unwrap();
                load_tmp_chirho
            }

            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                // Type abstraction is erased at runtime
                self.compile_expr_chirho(body_chirho)
            }

            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                // Type application is erased at runtime
                self.compile_expr_chirho(inner_chirho)
            }

            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                if args_chirho.len() == 2 {
                    let lhs_chirho = self.compile_expr_chirho(&args_chirho[0]);
                    let rhs_chirho = self.compile_expr_chirho(&args_chirho[1]);
                    let tmp_chirho = self.fresh_tmp_chirho();
                    let op_chirho = match name_chirho.as_str() {
                        "+#" => "add",
                        "-#" => "sub",
                        "*#" => "mul",
                        "div#" => "sdiv",
                        "mod#" => "srem",
                        "==#" | "/=#" | "<#" | "<=#" | ">#" | ">=#" => {
                            let cmp_pred_chirho = match name_chirho.as_str() {
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
                        _ => "add",
                    };
                    writeln!(
                        self.output_chirho,
                        "  {tmp_chirho} = {op_chirho} i64 {lhs_chirho}, {rhs_chirho}"
                    )
                    .unwrap();
                    tmp_chirho
                } else if args_chirho.len() == 1 && name_chirho == "negate#" {
                    let operand_chirho = self.compile_expr_chirho(&args_chirho[0]);
                    let tmp_chirho = self.fresh_tmp_chirho();
                    writeln!(
                        self.output_chirho,
                        "  {tmp_chirho} = sub i64 0, {operand_chirho}"
                    )
                    .unwrap();
                    tmp_chirho
                } else {
                    // Fallback: return 0
                    "0".to_string()
                }
            }

            CoreExprChirho::ConAppChirho {
                con_name_chirho, ..
            } => {
                // Return the constructor tag. Without heap allocation,
                // fields are not accessible — only tag-based dispatch works.
                let tag_chirho = constructor_tag_chirho(con_name_chirho);
                format!("{tag_chirho}")
            }
        }
    }

    fn compile_lit_chirho(&self, lit_chirho: &CoreLitChirho) -> String {
        match lit_chirho {
            CoreLitChirho::IntChirho(v_chirho) => format!("{v_chirho}"),
            CoreLitChirho::FloatChirho(_) => "0".to_string(), // TODO: float support
            CoreLitChirho::CharChirho(c_chirho) => format!("{}", *c_chirho as i64),
            CoreLitChirho::StringChirho(_) => "0".to_string(), // TODO: string support
        }
    }

    fn compile_case_lit_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
        result_tmp_chirho: &str,
        end_label_chirho: &str,
    ) {
        // Build a switch on the scrutinee for literal alts
        let default_label_chirho = self.fresh_label_chirho("case.default");

        let mut switch_arms_chirho = Vec::new();
        let mut alt_labels_chirho = Vec::new();

        for alt_chirho in alts_chirho {
            match &alt_chirho.con_chirho {
                AltConChirho::LitConChirho(CoreLitChirho::IntChirho(v_chirho)) => {
                    let label_chirho = self.fresh_label_chirho("case.lit");
                    switch_arms_chirho.push(format!("    i64 {v_chirho}, label %{label_chirho}"));
                    alt_labels_chirho.push((label_chirho, alt_chirho));
                }
                AltConChirho::DefaultChirho => {
                    alt_labels_chirho.push((default_label_chirho.clone(), alt_chirho));
                }
                _ => {}
            }
        }

        // Emit switch
        writeln!(
            self.output_chirho,
            "  switch i64 {scrut_val_chirho}, label %{default_label_chirho} ["
        )
        .unwrap();
        for arm_chirho in &switch_arms_chirho {
            writeln!(self.output_chirho, "{arm_chirho}").unwrap();
        }
        writeln!(self.output_chirho, "  ]").unwrap();

        // Emit each alt block
        for (label_chirho, alt_chirho) in &alt_labels_chirho {
            writeln!(self.output_chirho, "{label_chirho}:").unwrap();
            let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
            writeln!(
                self.output_chirho,
                "  store i64 {val_chirho}, ptr {result_tmp_chirho}.addr"
            )
            .unwrap();
            writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
        }

        // If no default alt was present, emit an unreachable default
        if !alts_chirho
            .iter()
            .any(|a_chirho| a_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }
    }

    fn compile_case_data_chirho(
        &mut self,
        scrut_val_chirho: &str,
        alts_chirho: &[CoreAltChirho],
        result_tmp_chirho: &str,
        end_label_chirho: &str,
    ) {
        // For data constructors, we use the tag (encoded as i64) to branch.
        // For now, handle default-only case by just compiling the default RHS.
        if alts_chirho.len() == 1 && alts_chirho[0].con_chirho == AltConChirho::DefaultChirho {
            // Bind any alt binders to scrutinee
            for binder_chirho in &alts_chirho[0].binders_chirho {
                self.local_scope_chirho.insert(binder_chirho.id_chirho);
                let binder_var_chirho = format!("%v{}", binder_chirho.id_chirho.0);
                writeln!(
                    self.output_chirho,
                    "  {binder_var_chirho} = add i64 0, {scrut_val_chirho}"
                )
                .unwrap();
            }
            let val_chirho = self.compile_expr_chirho(&alts_chirho[0].rhs_chirho);
            writeln!(
                self.output_chirho,
                "  store i64 {val_chirho}, ptr {result_tmp_chirho}.addr"
            )
            .unwrap();
            writeln!(self.output_chirho, "  br label %{end_label_chirho}").unwrap();
            return;
        }

        // Multi-alt with data constructors: use tag comparison chain
        // Each constructor gets a tag (True=1, False=0, etc.)
        let default_label_chirho = self.fresh_label_chirho("case.default");

        for (i_chirho, alt_chirho) in alts_chirho.iter().enumerate() {
            match &alt_chirho.con_chirho {
                AltConChirho::DataConChirho(name_chirho) => {
                    let tag_chirho = constructor_tag_chirho(name_chirho);
                    let cmp_tmp_chirho = self.fresh_tmp_chirho();
                    let then_label_chirho = self.fresh_label_chirho("case.con");
                    let else_label_chirho = if i_chirho + 1 < alts_chirho.len() {
                        self.fresh_label_chirho("case.next")
                    } else {
                        default_label_chirho.clone()
                    };

                    writeln!(
                        self.output_chirho,
                        "  {cmp_tmp_chirho} = icmp eq i64 {scrut_val_chirho}, {tag_chirho}"
                    )
                    .unwrap();
                    writeln!(
                        self.output_chirho,
                        "  br i1 {cmp_tmp_chirho}, label %{then_label_chirho}, label %{else_label_chirho}"
                    )
                    .unwrap();

                    writeln!(self.output_chirho, "{then_label_chirho}:").unwrap();
                    // Bind alt binders (constructor field projections).
                    // Without heap layout, single-field constructors get the
                    // scrutinee value; multi-field binders get 0 as a stub.
                    for (field_idx_chirho, binder_chirho) in
                        alt_chirho.binders_chirho.iter().enumerate()
                    {
                        self.local_scope_chirho.insert(binder_chirho.id_chirho);
                        let binder_var_chirho = format!("%v{}", binder_chirho.id_chirho.0);
                        if alt_chirho.binders_chirho.len() == 1 {
                            writeln!(
                                self.output_chirho,
                                "  {binder_var_chirho} = add i64 0, {scrut_val_chirho}"
                            )
                            .unwrap();
                        } else {
                            writeln!(
                                self.output_chirho,
                                "  {binder_var_chirho} = add i64 0, 0 ; stub field {field_idx_chirho}"
                            )
                            .unwrap();
                        }
                    }
                    let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
                    writeln!(
                        self.output_chirho,
                        "  store i64 {val_chirho}, ptr {result_tmp_chirho}.addr"
                    )
                    .unwrap();
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}")
                        .unwrap();

                    if i_chirho + 1 < alts_chirho.len() {
                        writeln!(self.output_chirho, "{else_label_chirho}:").unwrap();
                    }
                }
                AltConChirho::DefaultChirho => {
                    // Bind any alt binders to scrutinee
                    for binder_chirho in &alt_chirho.binders_chirho {
                        self.local_scope_chirho.insert(binder_chirho.id_chirho);
                        let binder_var_chirho = format!("%v{}", binder_chirho.id_chirho.0);
                        writeln!(
                            self.output_chirho,
                            "  {binder_var_chirho} = add i64 0, {scrut_val_chirho}"
                        )
                        .unwrap();
                    }
                    let val_chirho = self.compile_expr_chirho(&alt_chirho.rhs_chirho);
                    writeln!(
                        self.output_chirho,
                        "  store i64 {val_chirho}, ptr {result_tmp_chirho}.addr"
                    )
                    .unwrap();
                    writeln!(self.output_chirho, "  br label %{end_label_chirho}")
                        .unwrap();
                }
                _ => {}
            }
        }

        // If no default, emit unreachable
        if !alts_chirho
            .iter()
            .any(|a_chirho| a_chirho.con_chirho == AltConChirho::DefaultChirho)
        {
            writeln!(self.output_chirho, "{default_label_chirho}:").unwrap();
            writeln!(self.output_chirho, "  unreachable").unwrap();
        }
    }
}

/// Collect all lambda parameters from a nested chain of Lam nodes.
fn collect_lambda_params_chirho(expr_chirho: &CoreExprChirho) -> (Vec<CoreIdChirho>, &CoreExprChirho) {
    let mut params_chirho = Vec::new();
    let mut current_chirho = expr_chirho;

    while let CoreExprChirho::LamChirho {
        binder_chirho,
        body_chirho,
    } = current_chirho
    {
        params_chirho.push(binder_chirho.id_chirho);
        current_chirho = body_chirho;
    }

    (params_chirho, current_chirho)
}

/// Flatten nested applications into callee + argument list.
fn flatten_app_chirho(expr_chirho: &CoreExprChirho) -> (&CoreExprChirho, Vec<&CoreExprChirho>) {
    let mut args_chirho = Vec::new();
    let mut current_chirho = expr_chirho;

    while let CoreExprChirho::AppChirho {
        fun_chirho,
        arg_chirho,
    } = current_chirho
    {
        args_chirho.push(arg_chirho.as_ref());
        current_chirho = fun_chirho;
    }

    args_chirho.reverse();
    (current_chirho, args_chirho)
}

/// Get a numeric tag for a data constructor name.
fn constructor_tag_chirho(name_chirho: &str) -> i64 {
    match name_chirho {
        "False" => 0,
        "True" => 1,
        "Nothing" => 0,
        "Just" => 1,
        "Left" => 0,
        "Right" => 1,
        "LT" => 0,
        "EQ" => 1,
        "GT" => 2,
        _ => {
            // Hash the name to get a tag (placeholder strategy)
            let mut hash_chirho: i64 = 0;
            for byte_chirho in name_chirho.bytes() {
                hash_chirho = hash_chirho.wrapping_mul(31).wrapping_add(byte_chirho as i64);
            }
            hash_chirho.abs()
        }
    }
}

/// Mangle a Haskell name for use as an LLVM symbol.
fn mangle_name_chirho(name_chirho: &str) -> String {
    let mut mangled_chirho = String::with_capacity(name_chirho.len() + 8);
    mangled_chirho.push_str("rhasky_");
    for ch_chirho in name_chirho.chars() {
        match ch_chirho {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => mangled_chirho.push(ch_chirho),
            '\'' => mangled_chirho.push_str("_prime"),
            '.' => mangled_chirho.push('_'),
            _ => {
                mangled_chirho.push_str(&format!("_u{:04x}", ch_chirho as u32));
            }
        }
    }
    mangled_chirho
}

/// Compile a Core module to LLVM IR text.
pub fn compile_core_to_llvm_chirho(module_chirho: &CoreModuleChirho) -> String {
    let mut codegen_chirho = LlvmCodegenChirho::new_chirho();
    codegen_chirho.compile_module_chirho(module_chirho)
}

/// Compile a Core module to LLVM IR text with a C-compatible `main()` entry
/// point that calls `rhasky_main()` and prints the i64 result via `printf`.
/// The resulting IR can be compiled with `clang -o output file.ll` to produce
/// a native executable.
///
/// Currently emits only bindings transitively reachable from `main` since the
/// LLVM backend doesn't yet support closures/heap needed by the full Prelude.
pub fn compile_core_to_llvm_executable_chirho(module_chirho: &CoreModuleChirho) -> String {
    // Dictionary elision + reachability filtering via shared Core utility
    let filtered_module_chirho =
        rhasky_core_chirho::elide_dicts_and_filter_chirho(module_chirho);

    let mut codegen_chirho = LlvmCodegenChirho::new_chirho();
    let mut ir_chirho = codegen_chirho.compile_module_chirho(&filtered_module_chirho);

    // Check if there's a binding named "main" — that's the Haskell entry point
    let has_main_chirho = module_chirho.bindings_chirho.iter().any(|b_chirho| {
        b_chirho.binder_chirho.name_chirho == "main"
    });

    if has_main_chirho {
        // Add printf/puts declarations and a C main() entry point
        writeln!(ir_chirho).unwrap();
        writeln!(ir_chirho, "; ── RTS entry point ──").unwrap();
        writeln!(ir_chirho, "@.fmt_int = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\"").unwrap();
        writeln!(ir_chirho, "declare i32 @printf(ptr, ...)").unwrap();
        writeln!(ir_chirho, "declare i32 @puts(ptr)").unwrap();
        writeln!(ir_chirho).unwrap();
        writeln!(ir_chirho, "define i32 @main() {{").unwrap();
        writeln!(ir_chirho, "entry:").unwrap();
        writeln!(ir_chirho, "  %result = call i64 @rhasky_main()").unwrap();
        writeln!(ir_chirho, "  call i32 (ptr, ...) @printf(ptr @.fmt_int, i64 %result)").unwrap();
        writeln!(ir_chirho, "  %exitcode = trunc i64 %result to i32").unwrap();
        writeln!(ir_chirho, "  ret i32 %exitcode").unwrap();
        writeln!(ir_chirho, "}}").unwrap();
    }

    ir_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_core_chirho::{BinderChirho, CoreIdChirho, InlineAnnotationChirho};
    use rhasky_typing_chirho::ty_chirho::TyChirho;
    use rhasky_core_chirho::CoreBindingChirho;

    fn dummy_binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
        BinderChirho {
            id_chirho: CoreIdChirho(id_chirho),
            name_chirho: name_chirho.to_string(),
            ty_chirho: TyChirho::int_chirho(),
            span_chirho: rhasky_span_chirho::SpanChirho::DUMMY_CHIRHO,
        }
    }

    fn int_lit_chirho(v_chirho: i64) -> CoreExprChirho {
        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(v_chirho))
    }

    #[test]
    fn compile_simple_constant_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: int_lit_chirho(42),
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("; ModuleID = 'Test'"));
        assert!(ir_chirho.contains("define i64 @rhasky_main()"));
        assert!(ir_chirho.contains("ret i64 42"));
    }

    #[test]
    fn compile_identity_function_chirho() {
        // f x = x
        let module_chirho = CoreModuleChirho {
            name_chirho: "Identity".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("f", 10),
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: dummy_binder_chirho("x", 0),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @rhasky_f(i64 %v0)"));
        assert!(ir_chirho.contains("ret i64 %v0"));
    }

    #[test]
    fn compile_let_binding_chirho() {
        // main = let x = 42 in x
        let module_chirho = CoreModuleChirho {
            name_chirho: "LetTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(
                        dummy_binder_chirho("x", 0),
                        int_lit_chirho(42),
                    )],
                    body_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("define i64 @rhasky_main()"));
        assert!(ir_chirho.contains("; let x = 42"));
    }

    #[test]
    fn compile_case_literal_chirho() {
        // main = case 1 of { 1 -> 10; _ -> 20 }
        let module_chirho = CoreModuleChirho {
            name_chirho: "CaseTest".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(int_lit_chirho(1)),
                    bind_chirho: dummy_binder_chirho("wild", 99),
                    result_ty_chirho: TyChirho::int_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::LitConChirho(CoreLitChirho::IntChirho(1)),
                            binders_chirho: vec![],
                            rhs_chirho: int_lit_chirho(10),
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: int_lit_chirho(20),
                        },
                    ],
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_chirho(&module_chirho);
        assert!(ir_chirho.contains("switch i64"));
        assert!(ir_chirho.contains("i64 1, label"));
    }

    #[test]
    fn mangle_name_basic_chirho() {
        assert_eq!(mangle_name_chirho("main"), "rhasky_main");
        assert_eq!(mangle_name_chirho("f'"), "rhasky_f_prime");
        assert_eq!(mangle_name_chirho("Data.Map"), "rhasky_Data_Map");
    }

    #[test]
    fn constructor_tags_chirho() {
        assert_eq!(constructor_tag_chirho("False"), 0);
        assert_eq!(constructor_tag_chirho("True"), 1);
        assert_eq!(constructor_tag_chirho("Nothing"), 0);
        assert_eq!(constructor_tag_chirho("Just"), 1);
    }

    #[test]
    fn compile_executable_with_main_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Main".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 0),
                rhs_chirho: int_lit_chirho(42),
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        // Should contain the rhasky_main function
        assert!(ir_chirho.contains("define i64 @rhasky_main()"));
        // Should contain the C main entry point
        assert!(ir_chirho.contains("define i32 @main()"));
        assert!(ir_chirho.contains("call i64 @rhasky_main()"));
        assert!(ir_chirho.contains("@printf"));
        assert!(ir_chirho.contains("ret i32 %exitcode"));
    }

    #[test]
    fn compile_executable_without_main_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Lib".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("helper", 0),
                rhs_chirho: int_lit_chirho(99),
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        // Without a `main` binding, no user bindings are reachable,
        // so no functions should be emitted (only module header)
        assert!(!ir_chirho.contains("define i64 @rhasky_helper()"));
        // Should NOT contain C main entry point since no "main" binding
        assert!(!ir_chirho.contains("define i32 @main()"));
    }

    #[test]
    fn compile_executable_with_arithmetic_chirho() {
        // main = 2 + 3
        let module_chirho = CoreModuleChirho {
            name_chirho: "Arith".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: dummy_binder_chirho("main", 10),
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![int_lit_chirho(2), int_lit_chirho(3)],
                },
                is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
            }],
            names_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
        };

        let ir_chirho = compile_core_to_llvm_executable_chirho(&module_chirho);
        assert!(ir_chirho.contains("add i64 2, 3"));
        assert!(ir_chirho.contains("define i32 @main()"));
        assert!(ir_chirho.contains("call i64 @rhasky_main()"));
    }

    #[test]
    fn flatten_app_basic_chirho() {
        // f 1 2 → (f, [1, 2])
        let expr_chirho = CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(0))),
                arg_chirho: Box::new(int_lit_chirho(1)),
            }),
            arg_chirho: Box::new(int_lit_chirho(2)),
        };
        let (callee_chirho, args_chirho) = flatten_app_chirho(&expr_chirho);
        assert!(matches!(callee_chirho, CoreExprChirho::VarChirho(CoreIdChirho(0))));
        assert_eq!(args_chirho.len(), 2);
    }
}
