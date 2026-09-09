// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pinned upstream sources keep parser regressions independent of package caches
//! and the host C preprocessor. See test-data-chirho/package-fixtures-chirho/README-chirho.md.

pub(crate) const CONTRAVARIANT_REP_SOURCE_CHIRHO: &str = include_str!(
    "../../../test-data-chirho/package-fixtures-chirho/adjunctions-4.4.4/src/Data/Functor/Contravariant/Rep.hs"
);
pub(crate) const TH_DATATYPE_PREPROCESSED_SOURCE_CHIRHO: &str = include_str!(
    "../../../test-data-chirho/package-fixtures-chirho/th-abstraction-0.7.2.0/preprocessed-chirho/datatype_chirho.hs"
);
pub(crate) const INTSET_RAW_SOURCE_CHIRHO: &str = include_str!(
    "../../../test-data-chirho/package-fixtures-chirho/containers-0.8/src/Data/IntSet/Internal.hs"
);
pub(crate) const INTSET_PREPROCESSED_SOURCE_CHIRHO: &str = include_str!(
    "../../../test-data-chirho/package-fixtures-chirho/containers-0.8/preprocessed-chirho/intset_chirho.hs"
);
pub(crate) const BYTEARRAY_PREPROCESSED_SOURCE_CHIRHO: &str = include_str!(
    "../../../test-data-chirho/package-fixtures-chirho/primitive-0.9.1.0/preprocessed-chirho/bytearray_chirho.hs"
);
