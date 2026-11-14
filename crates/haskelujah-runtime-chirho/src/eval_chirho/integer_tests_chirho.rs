// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::integer_arithmetic_chirho::{
    IntegerArithmeticErrorChirho, IntegerBudgetChirho, IntegerOperationChirho,
};

fn integer_chirho(source_chirho: &str) -> ValueChirho {
    ValueChirho::IntegerChirho(
        crate::integer_literal_chirho::IntegerValueChirho::from_literal_chirho(source_chirho)
            .unwrap(),
    )
}

#[test]
fn arithmetic_allowance_survives_repeated_machine_entries_chirho() {
    let mut machine_chirho = MachineChirho::new_chirho(vec![CodeChirho::PrimChirho {
        op_chirho: PrimOpKindChirho::IntegerArithmeticChirho(IntegerOperationChirho::AddChirho),
        args_chirho: vec![
            ArgSourceChirho::StaticChirho(integer_chirho("1")),
            ArgSourceChirho::StaticChirho(integer_chirho("1")),
        ],
    }]);
    machine_chirho.integer_budget_chirho = IntegerBudgetChirho::limited_chirho(6, 2);
    for _entry_chirho in 0..2 {
        assert_eq!(machine_chirho.run_chirho(0).unwrap().to_string(), "2");
    }
    assert_eq!(
        machine_chirho.run_chirho(0),
        Err(EvalErrorChirho::PrimFailChirho(
            PrimErrorChirho::IntegerArithmeticChirho(IntegerArithmeticErrorChirho::ResourceChirho)
        ))
    );
    assert!(machine_chirho.integer_budget_chirho.exhausted_chirho());
}

#[test]
fn opaque_operations_never_accept_word_values_or_wrong_arity_chirho() {
    for arguments_chirho in [
        vec![ArgSourceChirho::StaticChirho(integer_chirho("1"))],
        vec![
            ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(1)),
            ArgSourceChirho::StaticChirho(integer_chirho("1")),
        ],
    ] {
        let mut machine_chirho = MachineChirho::new_chirho(vec![CodeChirho::PrimChirho {
            op_chirho: PrimOpKindChirho::IntegerArithmeticChirho(IntegerOperationChirho::AddChirho),
            args_chirho: arguments_chirho,
        }]);
        assert!(matches!(
            machine_chirho.run_chirho(0),
            Err(EvalErrorChirho::PrimFailChirho(_))
        ));
        assert!(!machine_chirho.integer_budget_chirho.exhausted_chirho());
    }
    assert_eq!(
        apply_prim_binop_chirho(
            PrimOpKindChirho::IntegerArithmeticChirho(IntegerOperationChirho::AddChirho),
            &integer_chirho("1"),
            &integer_chirho("1")
        ),
        Err(PrimErrorChirho::MissingIntegerBudgetChirho)
    );
}

#[test]
fn multiplying_by_zero_does_not_swallow_either_operand_failure_chirho() {
    for failing_position_chirho in 0..2 {
        let mut arguments_chirho = vec![ArgSourceChirho::StaticChirho(integer_chirho("0")); 2];
        arguments_chirho[failing_position_chirho] = ArgSourceChirho::ThunkCodeChirho {
            code_ptr_chirho: 0,
            captures_chirho: vec![],
        };
        let mut machine_chirho = MachineChirho::new_chirho(vec![
            CodeChirho::PrimChirho {
                op_chirho: PrimOpKindChirho::ErrorChirho,
                args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::StringChirho(
                    "integer operand failure chirho".into(),
                ))],
            },
            CodeChirho::PrimChirho {
                op_chirho: PrimOpKindChirho::IntegerArithmeticChirho(
                    IntegerOperationChirho::MultiplyChirho,
                ),
                args_chirho: arguments_chirho,
            },
        ]);
        assert!(
            matches!(machine_chirho.run_chirho(1), Err(EvalErrorChirho::RuntimeErrorChirho(message_chirho)) if message_chirho == "integer operand failure chirho")
        );
    }
}

#[test]
fn the_integer_handler_refuses_an_unevaluated_operand_instead_of_nesting_the_evaluator_chirho() {
    let mut machine_chirho =
        MachineChirho::new_chirho(vec![CodeChirho::LitChirho(integer_chirho("7"))]);
    let address_chirho = machine_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::thunk_chirho(
            CodePtrChirho(0),
            "pendingIntegerChirho",
            vec![],
        ));
    let before_chirho = machine_chirho.code_table_chirho.len();
    let result_chirho = machine_chirho.eval_integer_chirho(
        IntegerOperationChirho::NegateChirho,
        &[ValueChirho::HeapPtrChirho(address_chirho)],
    );
    assert!(matches!(
        result_chirho,
        Err(EvalErrorChirho::TypeErrorChirho { .. })
    ));
    assert_eq!(machine_chirho.steps_chirho, 0);
    assert_eq!(machine_chirho.code_table_chirho.len(), before_chirho);
}
