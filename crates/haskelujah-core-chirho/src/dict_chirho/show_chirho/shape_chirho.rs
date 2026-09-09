// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Decode the checker's concrete Show evidence, retaining applied argument boundaries.
//! Unknown instances remain on the dictionary path; they are never guessed as Int.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ShowShapeChirho {
    ScalarChirho(String),
    MaybeChirho(Box<Self>),
    EitherChirho(Box<Self>, Box<Self>),
    TupleChirho(Vec<Self>),
}

impl ShowShapeChirho {
    pub(super) fn parse_chirho(key_chirho: &str) -> Option<Self> {
        Self::parse_at_depth_chirho(key_chirho.trim(), 0)
    }

    fn parse_at_depth_chirho(key_chirho: &str, depth_chirho: usize) -> Option<Self> {
        if depth_chirho >= 64 {
            return None;
        }
        let descend_chirho =
            |part_chirho: &str| Self::parse_at_depth_chirho(part_chirho.trim(), depth_chirho + 1);
        if key_chirho.starts_with('(') && key_chirho.ends_with(')') {
            let inner_chirho = &key_chirho[1..key_chirho.len() - 1];
            let parts_chirho = split_outer_chirho(inner_chirho, ',')?;
            return if parts_chirho.len() == 1 {
                descend_chirho(inner_chirho)
            } else {
                Some(Self::TupleChirho(
                    parts_chirho
                        .into_iter()
                        .map(descend_chirho)
                        .collect::<Option<_>>()?,
                ))
            };
        }
        if let Some(inner_chirho) = key_chirho.strip_prefix("Maybe ") {
            return Some(Self::MaybeChirho(Box::new(descend_chirho(inner_chirho)?)));
        }
        if let Some(inner_chirho) = key_chirho.strip_prefix("Either ") {
            let parts_chirho = split_outer_chirho(inner_chirho, ' ')?;
            let [left_chirho, right_chirho] = parts_chirho.as_slice() else {
                return None;
            };
            return Some(Self::EitherChirho(
                Box::new(descend_chirho(left_chirho)?),
                Box::new(descend_chirho(right_chirho)?),
            ));
        }
        matches!(
            key_chirho,
            "Int" | "Integer" | "Bool" | "Char" | "Double" | "String" | "[Char]"
        )
        .then(|| Self::ScalarChirho(key_chirho.to_string()))
    }
}

fn split_outer_chirho(source_chirho: &str, separator_chirho: char) -> Option<Vec<&str>> {
    let mut closing_chirho = Vec::new();
    let mut parts_chirho = Vec::new();
    let mut start_chirho = 0;
    for (index_chirho, char_chirho) in source_chirho.char_indices() {
        match char_chirho {
            '(' => closing_chirho.push(')'),
            '[' => closing_chirho.push(']'),
            ')' | ']' if closing_chirho.pop() != Some(char_chirho) => return None,
            _ => {}
        }
        if char_chirho == separator_chirho && closing_chirho.is_empty() {
            let part_chirho = source_chirho[start_chirho..index_chirho].trim();
            if !part_chirho.is_empty() {
                parts_chirho.push(part_chirho);
            }
            start_chirho = index_chirho + char_chirho.len_utf8();
        }
    }
    if !closing_chirho.is_empty() {
        return None;
    }
    let part_chirho = source_chirho[start_chirho..].trim();
    if !part_chirho.is_empty() {
        parts_chirho.push(part_chirho);
    }
    Some(parts_chirho)
}
