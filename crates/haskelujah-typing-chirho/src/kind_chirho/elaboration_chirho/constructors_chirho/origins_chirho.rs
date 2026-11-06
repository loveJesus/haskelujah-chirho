// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Preserve the actual parent-template versus checked-signature result origin.

use std::sync::Arc;

/// The source producer states whether its result still needs its parent
/// header instantiated. An already checked GADT spine keeps every argument,
/// including applications of arrows in the parent's result kind.
#[derive(Debug, Clone)]
pub(in crate::kind_chirho::elaboration_chirho) enum ConstructorResultOriginChirho {
    ParentHeaderTemplateChirho {
        parent_chirho: Arc<str>,
    },
    SignatureApplicationChirho {
        parent_chirho: Arc<str>,
        checking_chirho: SignatureCheckingChirho,
    },
}

#[derive(Debug, Clone, Copy)]
pub(in crate::kind_chirho::elaboration_chirho) enum SignatureCheckingChirho {
    ProvisionalChirho,
    CompleteChirho,
}
