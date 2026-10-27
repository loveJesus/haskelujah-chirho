// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Small structural operations shared by evidence-key consumers. A separator
//! inside a list or tuple belongs to that nested type, not to the outer key.

/// Split `source_chirho` only at separators outside balanced tuple and list
/// delimiters. An unbalanced key is refused instead of partially interpreted.
pub(super) fn split_outer_type_key_chirho(
    source_chirho: &str,
    separator_chirho: char,
) -> Option<Vec<&str>> {
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

/// Normalize only layout whitespace in a rendered type key. Spaces separating
/// type application (`Maybe Int`) remain meaningful; spaces beside delimiters
/// do not (`(Int, Char)` and `(Int,Char)` name the same row).
pub(super) fn canonical_type_key_chirho(source_chirho: &str) -> String {
    let chars_chirho = source_chirho.chars().collect::<Vec<_>>();
    let mut result_chirho = String::with_capacity(source_chirho.len());
    let mut index_chirho = 0;
    while index_chirho < chars_chirho.len() {
        let char_chirho = chars_chirho[index_chirho];
        if !char_chirho.is_whitespace() {
            result_chirho.push(char_chirho);
            index_chirho += 1;
            continue;
        }
        let previous_chirho = result_chirho.chars().next_back();
        while index_chirho < chars_chirho.len() && chars_chirho[index_chirho].is_whitespace() {
            index_chirho += 1;
        }
        let next_chirho = chars_chirho.get(index_chirho).copied();
        if !matches!(previous_chirho, Some('(' | '[' | ','))
            && !matches!(next_chirho, Some(')' | ']' | ','))
            && !result_chirho.ends_with(' ')
        {
            result_chirho.push(' ');
        }
    }
    result_chirho.trim().to_string()
}

#[cfg(test)]
mod tests_chirho {
    use super::{canonical_type_key_chirho, split_outer_type_key_chirho};

    #[test]
    fn nested_type_keys_split_only_at_the_outer_separator_chirho() {
        assert_eq!(
            split_outer_type_key_chirho("Maybe (Int, Char), [Maybe Double]", ','),
            Some(vec!["Maybe (Int, Char)", "[Maybe Double]"])
        );
        assert_eq!(
            split_outer_type_key_chirho("Either (Int, Char) [Double]", ' '),
            Some(vec!["Either", "(Int, Char)", "[Double]"])
        );
        assert_eq!(split_outer_type_key_chirho("(Int, Char", ','), None);
    }

    #[test]
    fn type_key_layout_does_not_change_the_registry_name_chirho() {
        assert_eq!(
            canonical_type_key_chirho(" (Maybe (Int, Char), [Double]) "),
            "(Maybe (Int,Char),[Double])"
        );
    }
}
