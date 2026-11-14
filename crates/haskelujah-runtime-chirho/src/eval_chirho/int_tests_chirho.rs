// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::{
    int_arithmetic_chirho::IntOperationChirho,
    integer_arithmetic_chirho::{IntegerArithmeticErrorChirho, IntegerBudgetChirho},
};

#[test]
fn repeated_entries_spend_one_int_allowance_and_stateless_calls_refuse_chirho() {
    let mut machine_chirho = MachineChirho::new_chirho(vec![CodeChirho::PrimChirho {
        op_chirho: PrimOpKindChirho::IntArithmeticChirho(IntOperationChirho::AddChirho),
        args_chirho: vec![
            ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(i64::MAX)),
            ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(1)),
        ],
    }]);
    machine_chirho.integer_budget_chirho = IntegerBudgetChirho::limited_chirho(6, 2);
    for _entry_chirho in 0..2 {
        assert_eq!(
            machine_chirho.run_chirho(0).unwrap(),
            ValueChirho::IntChirho(i64::MIN)
        );
    }
    assert_eq!(
        machine_chirho.run_chirho(0),
        Err(EvalErrorChirho::PrimFailChirho(
            PrimErrorChirho::IntegerArithmeticChirho(IntegerArithmeticErrorChirho::ResourceChirho)
        ))
    );
    assert!(machine_chirho.integer_budget_chirho.exhausted_chirho());
    assert_eq!(
        apply_prim_binop_chirho(
            PrimOpKindChirho::IntArithmeticChirho(IntOperationChirho::AddChirho),
            &ValueChirho::IntChirho(1),
            &ValueChirho::IntChirho(2)
        ),
        Err(PrimErrorChirho::MissingIntegerBudgetChirho)
    );
}

#[test]
fn int_handler_never_enters_an_unevaluated_operand_chirho() {
    let mut machine_chirho =
        MachineChirho::new_chirho(vec![CodeChirho::LitChirho(ValueChirho::IntChirho(7))]);
    let address_chirho = machine_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::thunk_chirho(
            CodePtrChirho(0),
            "pendingIntChirho",
            vec![],
        ));
    let before_chirho = machine_chirho.code_table_chirho.len();
    assert!(matches!(
        machine_chirho.eval_int_chirho(
            IntOperationChirho::NegateChirho,
            &[ValueChirho::HeapPtrChirho(address_chirho)]
        ),
        Err(EvalErrorChirho::TypeErrorChirho { .. })
    ));
    assert_eq!(machine_chirho.steps_chirho, 0);
    assert_eq!(machine_chirho.code_table_chirho.len(), before_chirho);
}
