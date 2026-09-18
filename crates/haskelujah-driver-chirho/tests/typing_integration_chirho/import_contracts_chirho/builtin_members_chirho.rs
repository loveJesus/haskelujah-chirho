// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class member inventories obey the same selection rules as source interfaces.
//! Workflow: compiler-pipeline-chirho/type-scope-resolution-chirho.
use super::*;

#[test]
fn generic_associated_members_follow_import_selection_chirho() {
    // Exact sources independently checked by GHC 9.14.1. Bare classes and
    // method-only imports do not import associated types; hiding removes them.
    let cases_chirho = [
        (
            "all_members_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic(..))
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "class_only_chirho",
            false,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic)
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "named_member_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic(Rep))
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "all_higher_members_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic1(..))
type ReprChirho aChirho = Rep1 aChirho
"##,
        ),
        (
            "higher_class_only_chirho",
            false,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic1)
type ReprChirho aChirho = Rep1 aChirho
"##,
        ),
        (
            "hidden_members_chirho",
            false,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics hiding (Generic(..))
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "direct_member_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Rep)
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "qualified_members_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import qualified GHC.Generics as GChirho (Generic(..))
type ReprChirho aChirho = GChirho.Rep aChirho
"##,
        ),
        (
            "methods_only_chirho",
            false,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic(from, to))
type ReprChirho aChirho = Rep aChirho
"##,
        ),
        (
            "named_higher_member_chirho",
            true,
            r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
import GHC.Generics (Generic1(Rep1))
type ReprChirho aChirho = Rep1 aChirho
"##,
        ),
    ];
    let mut failures_chirho = Vec::new();
    for (name_chirho, expected_chirho, source_chirho) in cases_chirho {
        let result_chirho = typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ProbeChirho.hs",
        );
        if result_chirho.is_ok() != expected_chirho {
            failures_chirho.push(format!(
                "{name_chirho}: {}",
                result_chirho
                    .err()
                    .map(|error_chirho| error_chirho.to_string())
                    .unwrap_or_else(|| "wrongly accepted".to_string()),
            ));
        } else if !expected_chirho {
            let message_chirho = result_chirho.err().unwrap().to_string();
            assert!(
                message_chirho.contains("type not in scope"),
                "{name_chirho}: {message_chirho}"
            );
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}
