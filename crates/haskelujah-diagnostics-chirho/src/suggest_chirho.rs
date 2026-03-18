// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # "Did you mean?" suggestions
//!
//! Provides edit-distance–based name suggestions for undefined-name diagnostics.
//! Given a misspelled name and a collection of names in scope, returns the
//! closest matches within a configurable edit-distance threshold.

/// Compute the Levenshtein edit distance between two strings.
///
/// Uses an O(min(m,n)) space algorithm with a single row buffer.
pub fn edit_distance_chirho(a_chirho: &str, b_chirho: &str) -> usize {
    let mut a_chars_chirho: Vec<char> = a_chirho.chars().collect();
    let mut b_chars_chirho: Vec<char> = b_chirho.chars().collect();

    // Ensure a is the shorter string to minimize memory usage.
    if a_chars_chirho.len() > b_chars_chirho.len() {
        std::mem::swap(&mut a_chars_chirho, &mut b_chars_chirho);
    }

    let m_chirho = a_chars_chirho.len();
    let n_chirho = b_chars_chirho.len();

    // Previous row of distances.
    let mut prev_chirho: Vec<usize> = (0..=m_chirho).collect();
    let mut curr_chirho: Vec<usize> = vec![0; m_chirho + 1];

    for j_chirho in 1..=n_chirho {
        curr_chirho[0] = j_chirho;
        for i_chirho in 1..=m_chirho {
            let cost_chirho = if a_chars_chirho[i_chirho - 1] == b_chars_chirho[j_chirho - 1] {
                0
            } else {
                1
            };
            curr_chirho[i_chirho] = (prev_chirho[i_chirho] + 1) // deletion
                .min(curr_chirho[i_chirho - 1] + 1) // insertion
                .min(prev_chirho[i_chirho - 1] + cost_chirho); // substitution
        }
        std::mem::swap(&mut prev_chirho, &mut curr_chirho);
    }

    prev_chirho[m_chirho]
}

/// Find the most similar names from a set of candidates.
///
/// Returns up to `max_results_chirho` names whose edit distance from `name_chirho`
/// is within `max_distance_chirho`. Results are sorted by distance (closest first).
pub fn suggest_similar_names_chirho<'a>(
    name_chirho: &str,
    candidates_chirho: impl IntoIterator<Item = &'a str>,
    max_distance_chirho: usize,
    max_results_chirho: usize,
) -> Vec<&'a str> {
    let mut seen_candidates_chirho = std::collections::HashSet::new();
    let mut scored_chirho: Vec<(&str, usize)> = candidates_chirho
        .into_iter()
        .filter(|c_chirho| *c_chirho != name_chirho) // don't suggest the exact same name
        .filter(|c_chirho| seen_candidates_chirho.insert(*c_chirho))
        .map(|c_chirho| (c_chirho, edit_distance_chirho(name_chirho, c_chirho)))
        .filter(|(_, d_chirho)| *d_chirho <= max_distance_chirho && *d_chirho > 0)
        .collect();

    scored_chirho.sort_by(
        |(left_name_chirho, left_distance_chirho), (right_name_chirho, right_distance_chirho)| {
            left_distance_chirho
                .cmp(right_distance_chirho)
                .then_with(|| left_name_chirho.cmp(right_name_chirho))
        },
    );
    scored_chirho.truncate(max_results_chirho);
    scored_chirho
        .into_iter()
        .map(|(name_chirho, _)| name_chirho)
        .collect()
}

/// Compute a reasonable max edit distance for a given name length.
///
/// Short names (1-2 chars) allow distance 1, medium names (3-6) allow 2,
/// longer names allow 3.
pub fn default_max_distance_chirho(name_len_chirho: usize) -> usize {
    match name_len_chirho {
        0..=2 => 1,
        3..=6 => 2,
        _ => 3,
    }
}

/// Format a "did you mean?" note string from a list of suggestions.
///
/// Returns `None` if there are no suggestions.
pub fn format_did_you_mean_chirho(suggestions_chirho: &[&str]) -> Option<String> {
    match suggestions_chirho.len() {
        0 => None,
        1 => Some(format!("did you mean `{}`?", suggestions_chirho[0])),
        2 => Some(format!(
            "did you mean `{}` or `{}`?",
            suggestions_chirho[0], suggestions_chirho[1]
        )),
        _ => {
            let init_chirho: Vec<String> = suggestions_chirho[..suggestions_chirho.len() - 1]
                .iter()
                .map(|s_chirho| format!("`{s_chirho}`"))
                .collect();
            Some(format!(
                "did you mean {}, or `{}`?",
                init_chirho.join(", "),
                suggestions_chirho.last().unwrap()
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn edit_distance_identical_chirho() {
        assert_eq!(edit_distance_chirho("hello", "hello"), 0);
    }

    #[test]
    fn edit_distance_empty_chirho() {
        assert_eq!(edit_distance_chirho("", "abc"), 3);
        assert_eq!(edit_distance_chirho("abc", ""), 3);
        assert_eq!(edit_distance_chirho("", ""), 0);
    }

    #[test]
    fn edit_distance_substitution_chirho() {
        assert_eq!(edit_distance_chirho("kitten", "sitten"), 1);
    }

    #[test]
    fn edit_distance_insertion_deletion_chirho() {
        assert_eq!(edit_distance_chirho("abc", "ab"), 1);
        assert_eq!(edit_distance_chirho("ab", "abc"), 1);
    }

    #[test]
    fn edit_distance_full_example_chirho() {
        assert_eq!(edit_distance_chirho("kitten", "sitting"), 3);
    }

    #[test]
    fn edit_distance_symmetric_chirho() {
        assert_eq!(
            edit_distance_chirho("abc", "xyz"),
            edit_distance_chirho("xyz", "abc")
        );
    }

    #[test]
    fn edit_distance_handles_unicode_scalar_values_chirho() {
        assert_eq!(edit_distance_chirho("cafe", "cafe"), 0);
        assert_eq!(edit_distance_chirho("cafe", "cafe\u{301}"), 1);
        assert_eq!(edit_distance_chirho("cafe", "café"), 1);
    }

    #[test]
    fn suggest_similar_finds_close_matches_chirho() {
        let candidates_chirho = ["map", "filter", "fold", "mop", "fmap", "max"];
        let suggestions_chirho =
            suggest_similar_names_chirho("mp", candidates_chirho.into_iter(), 2, 3);
        // "map" has distance 1 from "mp"
        assert!(suggestions_chirho.contains(&"map"));
    }

    #[test]
    fn suggest_similar_returns_empty_for_no_match_chirho() {
        let candidates_chirho = ["zzzzz", "yyyyy"];
        let suggestions_chirho =
            suggest_similar_names_chirho("abc", candidates_chirho.into_iter(), 1, 3);
        assert!(suggestions_chirho.is_empty());
    }

    #[test]
    fn suggest_similar_excludes_exact_match_chirho() {
        let candidates_chirho = ["foo", "fop", "bar"];
        let suggestions_chirho =
            suggest_similar_names_chirho("foo", candidates_chirho.into_iter(), 2, 3);
        // Should not suggest "foo" itself
        assert!(!suggestions_chirho.contains(&"foo"));
    }

    #[test]
    fn suggest_sorted_by_distance_chirho() {
        let candidates_chirho = ["foobar", "foobaR", "foob", "f"];
        let suggestions_chirho =
            suggest_similar_names_chirho("foobar", candidates_chirho.into_iter(), 3, 4);
        // "foobaR" (dist 1) should come before "foob" (dist 2)
        if suggestions_chirho.len() >= 2 {
            let idx_r_chirho = suggestions_chirho
                .iter()
                .position(|s_chirho| *s_chirho == "foobaR")
                .unwrap();
            let idx_b_chirho = suggestions_chirho
                .iter()
                .position(|s_chirho| *s_chirho == "foob")
                .unwrap();
            assert!(idx_r_chirho < idx_b_chirho);
        }
    }

    #[test]
    fn suggest_sorted_deterministically_for_equal_distance_chirho() {
        let candidates_chirho = ["may", "map", "max"];
        let suggestions_chirho =
            suggest_similar_names_chirho("maz", candidates_chirho.into_iter(), 1, 3);
        assert_eq!(suggestions_chirho, vec!["map", "max", "may"]);
    }

    #[test]
    fn suggest_similar_deduplicates_candidates_chirho() {
        let candidates_chirho = ["map", "map", "max", "mop"];
        let suggestions_chirho =
            suggest_similar_names_chirho("maz", candidates_chirho.into_iter(), 2, 4);
        assert_eq!(suggestions_chirho, vec!["map", "max", "mop"]);
    }

    #[test]
    fn default_max_distance_short_names_chirho() {
        assert_eq!(default_max_distance_chirho(1), 1);
        assert_eq!(default_max_distance_chirho(2), 1);
    }

    #[test]
    fn default_max_distance_medium_names_chirho() {
        assert_eq!(default_max_distance_chirho(4), 2);
        assert_eq!(default_max_distance_chirho(6), 2);
    }

    #[test]
    fn default_max_distance_long_names_chirho() {
        assert_eq!(default_max_distance_chirho(10), 3);
    }

    #[test]
    fn format_did_you_mean_none_chirho() {
        assert_eq!(format_did_you_mean_chirho(&[]), None);
    }

    #[test]
    fn format_did_you_mean_one_chirho() {
        assert_eq!(
            format_did_you_mean_chirho(&["map"]),
            Some("did you mean `map`?".to_string())
        );
    }

    #[test]
    fn format_did_you_mean_two_chirho() {
        assert_eq!(
            format_did_you_mean_chirho(&["map", "max"]),
            Some("did you mean `map` or `max`?".to_string())
        );
    }

    #[test]
    fn format_did_you_mean_three_chirho() {
        let result_chirho = format_did_you_mean_chirho(&["map", "max", "mop"]);
        assert_eq!(
            result_chirho,
            Some("did you mean `map`, `max`, or `mop`?".to_string())
        );
    }
}
