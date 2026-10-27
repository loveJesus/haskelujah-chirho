// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Decode the checker's concrete Show evidence, retaining applied argument boundaries.
//! Unknown instances remain on the dictionary path; they are never guessed as Int.

use crate::dict_chirho::type_keys_chirho::split_outer_type_key_chirho;
use crate::expr_chirho::CoreIdChirho;

/// How deep a Show key may nest: in the parser, and in the derived support check (row 533).
pub(super) const SHOW_SHAPE_DEPTH_LIMIT_CHIRHO: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ShowShapeChirho {
    ScalarChirho(String),
    MaybeChirho(Box<Self>),
    EitherChirho(Box<Self>, Box<Self>),
    TupleChirho(Vec<Self>),
    ListChirho(Box<Self>),
    /// An actual show implementation, admitted only at precedence zero.
    BackedChirho(CoreIdChirho),
    /// A type that derives Show, rendered at any precedence by its key's helper (row 533).
    DerivedChirho(String),
}

impl ShowShapeChirho {
    /// Parse a key rendered at `precedence`: 0 for a whole value, 11 for a prefix
    /// constructor's field, p + 1 for an infix operand. `derived` names the canonical key
    /// when its head derives Show and every field renders.
    pub(super) fn parse_at_chirho(
        key_chirho: &str,
        precedence_chirho: u8,
        resolve_chirho: &impl Fn(&str) -> Option<CoreIdChirho>,
        derived_chirho: &impl Fn(&str) -> Option<String>,
    ) -> Option<Self> {
        Self::parse_at_depth_chirho(
            key_chirho.trim(),
            0,
            precedence_chirho,
            resolve_chirho,
            derived_chirho,
        )
    }

    fn parse_at_depth_chirho(
        key_chirho: &str,
        depth_chirho: usize,
        precedence_chirho: u8,
        resolve_chirho: &impl Fn(&str) -> Option<CoreIdChirho>,
        derived_chirho: &impl Fn(&str) -> Option<String>,
    ) -> Option<Self> {
        if depth_chirho >= SHOW_SHAPE_DEPTH_LIMIT_CHIRHO {
            return None;
        }
        let descend_chirho = |part_chirho: &str, precedence_chirho| {
            Self::parse_at_depth_chirho(
                part_chirho.trim(),
                depth_chirho + 1,
                precedence_chirho,
                resolve_chirho,
                derived_chirho,
            )
        };
        if matches!(
            key_chirho,
            "Int" | "Integer" | "Bool" | "Char" | "Double" | "String" | "[Char]"
        ) {
            return Some(Self::ScalarChirho(key_chirho.to_string()));
        }
        if key_chirho.starts_with('[') && key_chirho.ends_with(']') {
            return Some(Self::ListChirho(Box::new(descend_chirho(
                &key_chirho[1..key_chirho.len() - 1],
                0,
            )?)));
        }
        if key_chirho.starts_with('(') && key_chirho.ends_with(')') {
            let inner_chirho = &key_chirho[1..key_chirho.len() - 1];
            let parts_chirho = split_outer_type_key_chirho(inner_chirho, ',')?;
            return if parts_chirho.len() == 1 {
                descend_chirho(inner_chirho, precedence_chirho)
            } else {
                Some(Self::TupleChirho(
                    parts_chirho
                        .into_iter()
                        .map(|part_chirho| descend_chirho(part_chirho, 0))
                        .collect::<Option<_>>()?,
                ))
            };
        }
        if let Some(inner_chirho) = key_chirho.strip_prefix("Maybe ") {
            return Some(Self::MaybeChirho(Box::new(descend_chirho(
                inner_chirho,
                11,
            )?)));
        }
        if let Some(inner_chirho) = key_chirho.strip_prefix("Either ") {
            let parts_chirho = split_outer_type_key_chirho(inner_chirho, ' ')?;
            let [left_chirho, right_chirho] = parts_chirho.as_slice() else {
                return None;
            };
            return Some(Self::EitherChirho(
                Box::new(descend_chirho(left_chirho, 11)?),
                Box::new(descend_chirho(right_chirho, 11)?),
            ));
        }
        // A type that derives Show renders from its constructor metadata at any
        // precedence, through its own helper (row 533).
        if let Some(derived_key_chirho) = derived_chirho(key_chirho) {
            return Some(Self::DerivedChirho(derived_key_chirho));
        }
        // A hand-written show-only row is not evidence for showsPrec 11: leave
        // those constructor arguments on the dictionary path.
        (precedence_chirho == 0)
            .then(|| resolve_chirho(key_chirho))
            .flatten()
            .map(Self::BackedChirho)
    }
}
