// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::MAX_TUPLE_INSTANCE_ARITY_CHIRHO;
use crate::class_chirho::{ClassEnvChirho, PredChirho};
use crate::ty_chirho::TyChirho;

fn ints_chirho(arity_chirho: usize) -> TyChirho {
    TyChirho::TupleChirho(vec![TyChirho::int_chirho(); arity_chirho])
}

/// Row 604: `Eq`, `Show` and `Ord` hold for every tuple of Ints GHC's base covers, pairs
/// through 15-tuples, and for none larger: GHC rejects `print` of a 16-tuple too.
#[test]
fn tuple_instances_cover_ghcs_arities_and_no_more_chirho() {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.seed_standard_chirho();
    for class_chirho in ["Eq", "Show", "Ord"] {
        for arity_chirho in 2..=MAX_TUPLE_INSTANCE_ARITY_CHIRHO {
            assert!(
                env_chirho.entails_chirho(&PredChirho::new_chirho(
                    class_chirho,
                    ints_chirho(arity_chirho)
                )),
                "{class_chirho} at arity {arity_chirho}"
            );
        }
        let beyond_chirho = MAX_TUPLE_INSTANCE_ARITY_CHIRHO + 1;
        assert!(
            !env_chirho.entails_chirho(&PredChirho::new_chirho(
                class_chirho,
                ints_chirho(beyond_chirho)
            )),
            "{class_chirho} at arity {beyond_chirho}"
        );
    }
}
