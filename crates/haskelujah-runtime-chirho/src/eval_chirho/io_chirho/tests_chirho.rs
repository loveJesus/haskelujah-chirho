// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::{ClosureChirho, CodeChirho, CodePtrChirho, PrimOpKindChirho, apply_prim_binop_chirho};

fn token_chirho(machine_chirho: &mut MachineChirho, name_chirho: &str) -> ValueChirho {
    let tag_chirho = machine_chirho.lookup_con_tag_chirho(name_chirho, 0);
    ValueChirho::HeapPtrChirho(
        machine_chirho
            .heap_chirho
            .alloc_chirho(ClosureChirho::con_chirho(
                tag_chirho,
                name_chirho,
                Vec::new(),
            )),
    )
}

#[test]
fn only_runner_tokens_and_evaluated_scalars_reach_output_chirho() {
    let mut machine_chirho =
        MachineChirho::new_chirho(vec![CodeChirho::LitChirho(ValueChirho::CharChirho('x'))]);
    let handle_chirho = token_chirho(&mut machine_chirho, STDOUT_KEY_CHIRHO);
    let world_chirho = token_chirho(&mut machine_chirho, WORLD_KEY_CHIRHO);
    let pending_chirho = machine_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::thunk_chirho(
            CodePtrChirho(0),
            "pendingCharacterChirho",
            Vec::new(),
        ));
    let code_count_chirho = machine_chirho.code_table_chirho.len();
    assert!(
        machine_chirho
            .eval_next_io_chirho(
                IoOperationChirho::APPEND_CHIRHO,
                &[
                    handle_chirho.clone(),
                    ValueChirho::HeapPtrChirho(pending_chirho),
                    world_chirho.clone()
                ]
            )
            .is_err()
    );
    assert_eq!(machine_chirho.steps_chirho, 0);
    assert_eq!(machine_chirho.code_table_chirho.len(), code_count_chirho);
    assert!(
        machine_chirho
            .eval_next_io_chirho(
                IoOperationChirho::PREPARE_CHIRHO,
                &[ValueChirho::IntChirho(0), world_chirho.clone()]
            )
            .is_err()
    );
    assert!(
        machine_chirho
            .eval_next_io_chirho(
                IoOperationChirho::PREPARE_CHIRHO,
                &[handle_chirho.clone(), ValueChirho::IntChirho(0)]
            )
            .is_err()
    );
    machine_chirho
        .eval_next_io_chirho(
            IoOperationChirho::APPEND_CHIRHO,
            &[
                handle_chirho.clone(),
                ValueChirho::CharChirho('λ'),
                world_chirho.clone(),
            ],
        )
        .unwrap();
    assert!(machine_chirho.io_output_chirho.is_empty());
    assert_eq!(
        machine_chirho
            .eval_next_io_chirho(
                IoOperationChirho::FinishChirho,
                &[
                    handle_chirho,
                    ValueChirho::BoolChirho(true),
                    world_chirho.clone()
                ]
            )
            .unwrap(),
        world_chirho
    );
    assert_eq!(machine_chirho.io_output_chirho, "λ\n");
}

#[test]
fn stateless_io_cannot_bypass_the_machine_output_owner_chirho() {
    assert!(
        apply_prim_binop_chirho(
            PrimOpKindChirho::NextIoChirho(IoOperationChirho::PREPARE_CHIRHO),
            &ValueChirho::IntChirho(0),
            &ValueChirho::IntChirho(0)
        )
        .is_err()
    );
}
