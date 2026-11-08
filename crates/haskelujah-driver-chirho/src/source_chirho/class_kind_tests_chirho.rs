// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fresh GHC-paired source controls for actual selected builtin class producers.

use super::language_options_tests_chirho::check_chirho;

#[test]
fn selected_higher_order_class_headers_share_the_checked_catalogue_chirho() {
    let source_chirho = r#"
module ProvenClassKindsChirho where
import Control.Applicative (Alternative)
import Control.Monad.Fix (MonadFix)
import Control.Monad.IO.Class (MonadIO)
import Control.Monad.Trans.Class (MonadTrans)
alternativeChirho :: Alternative f => f a -> f a
alternativeChirho x = x
fixChirho :: MonadFix f => f a -> f a
fixChirho x = x
ioChirho :: MonadIO m => m a -> m a
ioChirho x = x
transformerChirho :: MonadTrans t => t m a -> t m a
transformerChirho x = x
"#;
    assert!(check_chirho(source_chirho, &[]).is_ok());
}

#[test]
fn ordinary_types_cannot_fill_higher_order_class_parameters_chirho() {
    for (module_chirho, class_chirho) in [
        ("Control.Applicative", "Alternative"),
        ("Control.Monad.Fix", "MonadFix"),
        ("Control.Monad.IO.Class", "MonadIO"),
        ("Control.Monad.Trans.Class", "MonadTrans"),
    ] {
        let source_chirho = format!(
            "module M where\nimport {module_chirho} ({class_chirho})\nvalueChirho :: {class_chirho} Int => ()\nvalueChirho = ()\n"
        );
        assert!(
            check_chirho(&source_chirho, &[]).is_err(),
            "{module_chirho}.{class_chirho}"
        );
    }
}

#[test]
fn prelude_does_not_export_alternative_chirho() {
    let source_chirho = "module PreludeAlternativeAbsentChirho where\nimport qualified Prelude as PChirho\nvalueChirho :: PChirho.Alternative f => f a -> f a\nvalueChirho x = x\n";
    assert!(check_chirho(source_chirho, &[]).is_err());
}
