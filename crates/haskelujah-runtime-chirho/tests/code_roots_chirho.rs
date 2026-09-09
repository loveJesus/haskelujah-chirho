// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constants referenced by executable code remain live, without retaining the
//! entire heap. Removing their code reference allows a later collection to retire them.

use haskelujah_runtime::{
    ClosureChirho, CodeChirho, DataConTagChirho, GcConfigChirho, MachineChirho, ValueChirho,
};

#[test]
fn code_only_constant_survives_until_its_reference_is_retired_chirho() {
    let mut evaluator_chirho = MachineChirho::new_chirho(vec![CodeChirho::ConAppChirho {
        tag_chirho: DataConTagChirho(7),
        name_chirho: "allocationChirho".to_string(),
        fields_chirho: vec![],
    }])
    .with_gc_config_chirho(GcConfigChirho {
        alloc_threshold_chirho: 1,
        min_heap_size_chirho: 0,
    });
    let constant_chirho = evaluator_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(8),
            "constantChirho",
            vec![ValueChirho::IntChirho(42)],
        ));
    let garbage_chirho = evaluator_chirho
        .heap_chirho
        .alloc_chirho(ClosureChirho::con_chirho(
            DataConTagChirho(9),
            "unreferencedChirho",
            vec![],
        ));
    evaluator_chirho
        .code_table_chirho
        .push(CodeChirho::EnterChirho(constant_chirho));
    evaluator_chirho.run_chirho(0).unwrap();
    assert!(evaluator_chirho.gc_state_chirho.cycle_count_chirho() > 0);
    assert_eq!(
        evaluator_chirho
            .heap_chirho
            .read_chirho(garbage_chirho)
            .info_chirho
            .name_chirho,
        "$DEAD"
    );
    let value_chirho = evaluator_chirho
        .run_chirho(1)
        .expect("code constant survives collection");
    assert_eq!(value_chirho, ValueChirho::HeapPtrChirho(constant_chirho));
    assert_eq!(
        evaluator_chirho
            .heap_chirho
            .read_chirho(constant_chirho)
            .payload_chirho,
        vec![ValueChirho::IntChirho(42)]
    );

    evaluator_chirho.code_table_chirho.truncate(1);
    evaluator_chirho.arg_regs_chirho.clear();
    evaluator_chirho.run_chirho(0).unwrap();
    assert_eq!(
        evaluator_chirho
            .heap_chirho
            .read_chirho(constant_chirho)
            .info_chirho
            .name_chirho,
        "$DEAD"
    );
}
