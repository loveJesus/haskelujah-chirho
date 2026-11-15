// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
use super::*;
use crate::{
    integer_arithmetic_chirho::IntegerBudgetChirho,
    prim_chirho::apply_prim_binop_chirho,
    value_chirho::{ClosureChirho, CodePtrChirho},
};

#[test]
fn formatter_never_reenters_a_thunk_or_accepts_a_lazy_tail_operand_chirho() {
    let mut machine_chirho = MachineChirho::new_chirho(vec![]);
    let address_chirho = machine_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::thunk_chirho(
            CodePtrChirho(0),
            "pendingShowChirho",
            vec![],
        ));
    let before_chirho = machine_chirho.code_table_chirho.len();
    assert!(matches!(
        machine_chirho.eval_show_number_chirho(
            ShowNumberOperationChirho::IntChirho,
            &[
                ValueChirho::IntChirho(0),
                ValueChirho::HeapPtrChirho(address_chirho)
            ]
        ),
        Err(EvalErrorChirho::TypeErrorChirho { .. })
    ));
    assert!(
        machine_chirho
            .eval_show_number_chirho(
                ShowNumberOperationChirho::IntChirho,
                &[
                    ValueChirho::IntChirho(0),
                    ValueChirho::IntChirho(1),
                    ValueChirho::HeapPtrChirho(address_chirho)
                ]
            )
            .is_err()
    );
    assert_eq!(machine_chirho.code_table_chirho.len(), before_chirho);
    assert_eq!(machine_chirho.steps_chirho, 0);
}

#[test]
fn formatting_exhaustion_is_atomic_and_cannot_use_a_stateless_budget_chirho() {
    let mut machine_chirho = MachineChirho::new_chirho(vec![]);
    machine_chirho.integer_budget_chirho = IntegerBudgetChirho::limited_chirho(0, 1000);
    let heap_before_chirho = machine_chirho.heap_chirho.size_chirho();
    assert!(
        machine_chirho
            .eval_show_number_chirho(
                ShowNumberOperationChirho::IntChirho,
                &[ValueChirho::IntChirho(0), ValueChirho::IntChirho(7)]
            )
            .is_err()
    );
    assert!(machine_chirho.integer_budget_chirho.exhausted_chirho());
    assert_eq!(machine_chirho.heap_chirho.size_chirho(), heap_before_chirho);
    assert_eq!(
        apply_prim_binop_chirho(
            PrimOpKindChirho::ShowNumberChirho(ShowNumberOperationChirho::IntChirho),
            &ValueChirho::IntChirho(0),
            &ValueChirho::IntChirho(1)
        ),
        Err(PrimErrorChirho::MissingIntegerBudgetChirho)
    );
}
