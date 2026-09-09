// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Setup dependencies belong to the custom-setup component, not to ordinary
//! library/executable build dependencies. Reuse the common dependency parser.

use super::{BuildInfoChirho, FieldChirho, parse_build_info_chirho, parse_deps_chirho};

pub(super) fn parse_setup_chirho(fields_chirho: &[&FieldChirho]) -> BuildInfoChirho {
    let mut setup_chirho = parse_build_info_chirho(fields_chirho);
    for field_chirho in fields_chirho {
        if field_chirho.key_chirho == "setup-depends" {
            setup_chirho
                .build_depends_chirho
                .extend(parse_deps_chirho(&field_chirho.value_chirho));
        }
    }
    setup_chirho
}
