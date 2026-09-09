// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Collect on each allocation, before the interpreter publishes its return or
//! register value. Fresh objects and their captured values must remain usable.

use haskelujah_runtime::{
    ArgSourceChirho, ClosureChirho, CodeChirho, CodePtrChirho, DataConTagChirho, GcConfigChirho,
    MachineChirho, ValueChirho,
};

fn machine_chirho(code_chirho: Vec<CodeChirho>) -> MachineChirho {
    MachineChirho::new_chirho(code_chirho).with_gc_config_chirho(GcConfigChirho {
        alloc_threshold_chirho: 1,
        min_heap_size_chirho: 0,
    })
}

fn captured_chirho() -> Vec<ArgSourceChirho> {
    vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(42))]
}

#[test]
fn newly_returned_constructors_survive_collection_chirho() {
    for allocation_chirho in [
        CodeChirho::ConAppChirho {
            tag_chirho: DataConTagChirho(7),
            name_chirho: "BoxChirho".to_string(),
            fields_chirho: vec![ValueChirho::IntChirho(42)],
        },
        CodeChirho::ConAppFromArgChirho {
            tag_chirho: DataConTagChirho(7),
            name_chirho: "BoxChirho".to_string(),
            fields_chirho: captured_chirho(),
        },
    ] {
        let mut evaluator_chirho = machine_chirho(vec![allocation_chirho]);
        let ValueChirho::HeapPtrChirho(address_chirho) = evaluator_chirho.run_chirho(0).unwrap()
        else {
            panic!("expected a boxed result")
        };
        assert_eq!(
            evaluator_chirho
                .heap_chirho
                .read_chirho(address_chirho)
                .payload_chirho,
            vec![ValueChirho::IntChirho(42)]
        );
        assert!(evaluator_chirho.gc_state_chirho.cycle_count_chirho() > 0);
    }
}

#[test]
fn stored_thunk_retains_its_capture_across_collection_chirho() {
    let mut evaluator_chirho = machine_chirho(vec![
        CodeChirho::StoreAllocThunkChirho {
            code_ptr_chirho: 2,
            name_chirho: "capturedChirho".to_string(),
            captures_chirho: captured_chirho(),
            dest_reg_chirho: 0,
            body_chirho: 1,
        },
        CodeChirho::ArgChirho { index_chirho: 0 },
        CodeChirho::ArgChirho { index_chirho: 0 },
    ]);
    assert_eq!(
        evaluator_chirho.run_chirho(0).unwrap(),
        ValueChirho::IntChirho(42)
    );
    assert!(evaluator_chirho.gc_state_chirho.cycle_count_chirho() > 0);
}

#[test]
fn fresh_function_remains_callable_after_collection_chirho() {
    for allocation_chirho in [
        CodeChirho::AllocFunChirho {
            arity_chirho: 1,
            code_ptr_chirho: 3,
            name_chirho: "capturedChirho".to_string(),
            captures_chirho: captured_chirho(),
        },
        CodeChirho::StoreAllocFunChirho {
            arity_chirho: 1,
            code_ptr_chirho: 3,
            name_chirho: "capturedChirho".to_string(),
            captures_chirho: captured_chirho(),
            dest_reg_chirho: 0,
            body_chirho: 1,
        },
    ] {
        let mut evaluator_chirho = machine_chirho(vec![
            allocation_chirho,
            CodeChirho::ArgChirho { index_chirho: 0 },
            CodeChirho::AppFromArgChirho {
                fun_arg_index_chirho: 0,
                arg_sources_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(9))],
            },
            CodeChirho::ArgChirho { index_chirho: 0 },
        ]);
        let function_chirho = evaluator_chirho.run_chirho(0).unwrap();
        evaluator_chirho.arg_regs_chirho = vec![function_chirho];
        assert_eq!(
            evaluator_chirho.run_chirho(2).unwrap(),
            ValueChirho::IntChirho(42)
        );
        assert!(evaluator_chirho.gc_state_chirho.cycle_count_chirho() > 0);
    }
}

#[test]
fn forced_fresh_thunk_survives_collection_chirho() {
    let mut evaluator_chirho = machine_chirho(vec![
        CodeChirho::ForceChirho {
            thunk_chirho: ClosureChirho::thunk_chirho(
                CodePtrChirho(1),
                "forceChirho",
                vec![ValueChirho::IntChirho(42)],
            ),
        },
        CodeChirho::ArgChirho { index_chirho: 0 },
    ]);
    assert_eq!(
        evaluator_chirho.run_chirho(0).unwrap(),
        ValueChirho::IntChirho(42)
    );
    assert!(evaluator_chirho.gc_state_chirho.cycle_count_chirho() > 0);
}
