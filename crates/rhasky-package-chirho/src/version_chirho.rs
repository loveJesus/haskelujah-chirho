// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Version types and constraint parsing
//!
//! Implements PVP-style version numbers (e.g. `1.2.3.4`) and version
//! constraints (`>=1.0 && <2.0`, `^>=1.4`, `==1.2.*`).

use std::fmt;

// ---------------------------------------------------------------------------
// Version
// ---------------------------------------------------------------------------

/// A PVP version number (e.g. `1.2.3.4`).
///
/// Equality and ordering treat trailing zeros as absent:
/// `1.0` == `1.0.0` == `1.0.0.0`.
#[derive(Debug, Clone)]
pub struct VersionChirho {
    pub components_chirho: Vec<u32>,
}

impl PartialEq for VersionChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.cmp_chirho(other_chirho) == std::cmp::Ordering::Equal
    }
}

impl Eq for VersionChirho {}

impl std::hash::Hash for VersionChirho {
    fn hash<H: std::hash::Hasher>(&self, state_chirho: &mut H) {
        // Normalize: strip trailing zeros for consistent hashing
        let trimmed_chirho: Vec<u32> = {
            let mut v_chirho = self.components_chirho.clone();
            while v_chirho.last() == Some(&0) {
                v_chirho.pop();
            }
            v_chirho
        };
        trimmed_chirho.hash(state_chirho);
    }
}

impl VersionChirho {
    pub fn new_chirho(components_chirho: Vec<u32>) -> Self {
        Self { components_chirho }
    }

    /// Compare two versions component-wise (missing trailing components = 0).
    fn cmp_chirho(&self, other_chirho: &Self) -> std::cmp::Ordering {
        let max_len_chirho = self
            .components_chirho
            .len()
            .max(other_chirho.components_chirho.len());
        for i_chirho in 0..max_len_chirho {
            let a_chirho = self.components_chirho.get(i_chirho).copied().unwrap_or(0);
            let b_chirho = other_chirho
                .components_chirho
                .get(i_chirho)
                .copied()
                .unwrap_or(0);
            match a_chirho.cmp(&b_chirho) {
                std::cmp::Ordering::Equal => continue,
                ord_chirho => return ord_chirho,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for VersionChirho {
    fn partial_cmp(&self, other_chirho: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp_chirho(other_chirho))
    }
}

impl Ord for VersionChirho {
    fn cmp(&self, other_chirho: &Self) -> std::cmp::Ordering {
        self.cmp_chirho(other_chirho)
    }
}

impl fmt::Display for VersionChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts_chirho: Vec<String> = self
            .components_chirho
            .iter()
            .map(|c_chirho| c_chirho.to_string())
            .collect();
        write!(f_chirho, "{}", parts_chirho.join("."))
    }
}

/// Parse a version string like `"1.2.3"`.
pub fn parse_version_chirho(input_chirho: &str) -> Option<VersionChirho> {
    let trimmed_chirho = input_chirho.trim();
    if trimmed_chirho.is_empty() {
        return None;
    }
    let mut components_chirho = Vec::new();
    for part_chirho in trimmed_chirho.split('.') {
        let part_chirho = part_chirho.trim();
        if part_chirho == "*" {
            // Wildcard — not a version component, bail
            return None;
        }
        components_chirho.push(part_chirho.parse::<u32>().ok()?);
    }
    if components_chirho.is_empty() {
        return None;
    }
    Some(VersionChirho::new_chirho(components_chirho))
}

// ---------------------------------------------------------------------------
// Version constraint
// ---------------------------------------------------------------------------

/// A version constraint expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraintChirho {
    /// Any version.
    AnyChirho,
    /// `==1.2.3`
    ExactChirho(VersionChirho),
    /// `==1.2.*` (prefix match)
    PrefixChirho(Vec<u32>),
    /// `>=1.0`
    GeChirho(VersionChirho),
    /// `>1.0`
    GtChirho(VersionChirho),
    /// `<=1.0`
    LeChirho(VersionChirho),
    /// `<2.0`
    LtChirho(VersionChirho),
    /// `^>=1.4` (major bound: >=1.4 && <1.5 for 2-component, >=1.4 && <2 for 1-component)
    CaretChirho(VersionChirho),
    /// `a && b`
    AndChirho(Box<VersionConstraintChirho>, Box<VersionConstraintChirho>),
    /// `a || b`
    OrChirho(Box<VersionConstraintChirho>, Box<VersionConstraintChirho>),
}

impl VersionConstraintChirho {
    /// Check whether a version satisfies this constraint.
    pub fn satisfied_by_chirho(&self, ver_chirho: &VersionChirho) -> bool {
        match self {
            Self::AnyChirho => true,
            Self::ExactChirho(v_chirho) => ver_chirho == v_chirho,
            Self::PrefixChirho(prefix_chirho) => {
                for (i_chirho, p_chirho) in prefix_chirho.iter().enumerate() {
                    let c_chirho = ver_chirho
                        .components_chirho
                        .get(i_chirho)
                        .copied()
                        .unwrap_or(0);
                    if c_chirho != *p_chirho {
                        return false;
                    }
                }
                true
            }
            Self::GeChirho(v_chirho) => ver_chirho >= v_chirho,
            Self::GtChirho(v_chirho) => ver_chirho > v_chirho,
            Self::LeChirho(v_chirho) => ver_chirho <= v_chirho,
            Self::LtChirho(v_chirho) => ver_chirho < v_chirho,
            Self::CaretChirho(v_chirho) => {
                if ver_chirho < v_chirho {
                    return false;
                }
                // Upper bound: bump the first component after the major version.
                // For ^>=A.B, upper bound is A.(B+1).
                // For ^>=A, upper bound is (A+1).
                let mut upper_chirho = v_chirho.components_chirho.clone();
                if upper_chirho.len() >= 2 {
                    upper_chirho[1] += 1;
                    upper_chirho.truncate(2);
                } else if !upper_chirho.is_empty() {
                    upper_chirho[0] += 1;
                    upper_chirho.truncate(1);
                }
                let upper_ver_chirho = VersionChirho::new_chirho(upper_chirho);
                ver_chirho < &upper_ver_chirho
            }
            Self::AndChirho(a_chirho, b_chirho) => {
                a_chirho.satisfied_by_chirho(ver_chirho)
                    && b_chirho.satisfied_by_chirho(ver_chirho)
            }
            Self::OrChirho(a_chirho, b_chirho) => {
                a_chirho.satisfied_by_chirho(ver_chirho)
                    || b_chirho.satisfied_by_chirho(ver_chirho)
            }
        }
    }
}

impl fmt::Display for VersionConstraintChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnyChirho => write!(f_chirho, "-any"),
            Self::ExactChirho(v_chirho) => write!(f_chirho, "=={}", v_chirho),
            Self::PrefixChirho(p_chirho) => {
                let parts_chirho: Vec<String> =
                    p_chirho.iter().map(|c_chirho| c_chirho.to_string()).collect();
                write!(f_chirho, "=={}.*", parts_chirho.join("."))
            }
            Self::GeChirho(v_chirho) => write!(f_chirho, ">={}", v_chirho),
            Self::GtChirho(v_chirho) => write!(f_chirho, ">{}", v_chirho),
            Self::LeChirho(v_chirho) => write!(f_chirho, "<={}", v_chirho),
            Self::LtChirho(v_chirho) => write!(f_chirho, "<{}", v_chirho),
            Self::CaretChirho(v_chirho) => write!(f_chirho, "^>={}", v_chirho),
            Self::AndChirho(a_chirho, b_chirho) => {
                write!(f_chirho, "{} && {}", a_chirho, b_chirho)
            }
            Self::OrChirho(a_chirho, b_chirho) => {
                write!(f_chirho, "{} || {}", a_chirho, b_chirho)
            }
        }
    }
}

/// Parse a single version constraint atom (no `&&` / `||`).
fn parse_constraint_atom_chirho(input_chirho: &str) -> Option<VersionConstraintChirho> {
    let s_chirho = input_chirho.trim();

    if s_chirho == "-any" || s_chirho.is_empty() {
        return Some(VersionConstraintChirho::AnyChirho);
    }

    // ^>=
    if let Some(rest_chirho) = s_chirho.strip_prefix("^>=") {
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::CaretChirho(v_chirho));
    }
    // ==X.Y.*
    if let Some(rest_chirho) = s_chirho.strip_prefix("==") {
        let rest_chirho = rest_chirho.trim();
        if rest_chirho.ends_with(".*") {
            let prefix_str_chirho = &rest_chirho[..rest_chirho.len() - 2];
            let parts_chirho: Vec<u32> = prefix_str_chirho
                .split('.')
                .map(|p_chirho| p_chirho.trim().parse::<u32>())
                .collect::<Result<_, _>>()
                .ok()?;
            return Some(VersionConstraintChirho::PrefixChirho(parts_chirho));
        }
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::ExactChirho(v_chirho));
    }
    // >=
    if let Some(rest_chirho) = s_chirho.strip_prefix(">=") {
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::GeChirho(v_chirho));
    }
    // >
    if let Some(rest_chirho) = s_chirho.strip_prefix('>') {
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::GtChirho(v_chirho));
    }
    // <=
    if let Some(rest_chirho) = s_chirho.strip_prefix("<=") {
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::LeChirho(v_chirho));
    }
    // <
    if let Some(rest_chirho) = s_chirho.strip_prefix('<') {
        let v_chirho = parse_version_chirho(rest_chirho)?;
        return Some(VersionConstraintChirho::LtChirho(v_chirho));
    }

    None
}

/// Parse a version constraint expression (supports `&&` and `||`).
pub fn parse_version_constraint_chirho(
    input_chirho: &str,
) -> Option<VersionConstraintChirho> {
    let s_chirho = input_chirho.trim();
    if s_chirho.is_empty() {
        return Some(VersionConstraintChirho::AnyChirho);
    }

    // Split on || first (lower precedence), then &&
    if let Some(idx_chirho) = s_chirho.find("||") {
        let left_chirho = parse_version_constraint_chirho(&s_chirho[..idx_chirho])?;
        let right_chirho = parse_version_constraint_chirho(&s_chirho[idx_chirho + 2..])?;
        return Some(VersionConstraintChirho::OrChirho(
            Box::new(left_chirho),
            Box::new(right_chirho),
        ));
    }

    if let Some(idx_chirho) = s_chirho.find("&&") {
        let left_chirho = parse_version_constraint_chirho(&s_chirho[..idx_chirho])?;
        let right_chirho = parse_version_constraint_chirho(&s_chirho[idx_chirho + 2..])?;
        return Some(VersionConstraintChirho::AndChirho(
            Box::new(left_chirho),
            Box::new(right_chirho),
        ));
    }

    parse_constraint_atom_chirho(s_chirho)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn parse_simple_version_chirho() {
        let v_chirho = parse_version_chirho("1.2.3").unwrap();
        assert_eq!(v_chirho.components_chirho, vec![1, 2, 3]);
        assert_eq!(v_chirho.to_string(), "1.2.3");
    }

    #[test]
    fn parse_single_component_version_chirho() {
        let v_chirho = parse_version_chirho("4").unwrap();
        assert_eq!(v_chirho.components_chirho, vec![4]);
    }

    #[test]
    fn version_ordering_chirho() {
        let v1_chirho = parse_version_chirho("1.2.3").unwrap();
        let v2_chirho = parse_version_chirho("1.2.4").unwrap();
        let v3_chirho = parse_version_chirho("1.3").unwrap();
        let v4_chirho = parse_version_chirho("2.0").unwrap();

        assert!(v1_chirho < v2_chirho);
        assert!(v2_chirho < v3_chirho);
        assert!(v3_chirho < v4_chirho);
        assert_eq!(
            parse_version_chirho("1.0").unwrap(),
            parse_version_chirho("1.0.0").unwrap()
        );
    }

    #[test]
    fn constraint_exact_chirho() {
        let c_chirho = parse_version_constraint_chirho("==1.2.3").unwrap();
        let v_chirho = parse_version_chirho("1.2.3").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&v_chirho));
        let v2_chirho = parse_version_chirho("1.2.4").unwrap();
        assert!(!c_chirho.satisfied_by_chirho(&v2_chirho));
    }

    #[test]
    fn constraint_prefix_chirho() {
        let c_chirho = parse_version_constraint_chirho("==1.2.*").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.2.0").unwrap()));
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.2.9").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("1.3.0").unwrap()));
    }

    #[test]
    fn constraint_range_chirho() {
        let c_chirho =
            parse_version_constraint_chirho(">=1.0 && <2.0").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.0").unwrap()));
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.5.3").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("2.0").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("0.9").unwrap()));
    }

    #[test]
    fn constraint_caret_chirho() {
        // ^>=1.4 means >=1.4 && <1.5
        let c_chirho = parse_version_constraint_chirho("^>=1.4").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.4").unwrap()));
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.4.2").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("1.5").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("1.3").unwrap()));
    }

    #[test]
    fn constraint_or_chirho() {
        let c_chirho =
            parse_version_constraint_chirho("==1.0 || ==2.0").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("1.0").unwrap()));
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("2.0").unwrap()));
        assert!(!c_chirho.satisfied_by_chirho(&parse_version_chirho("1.5").unwrap()));
    }

    #[test]
    fn constraint_any_chirho() {
        let c_chirho = parse_version_constraint_chirho("-any").unwrap();
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("0.0.0").unwrap()));
        assert!(c_chirho.satisfied_by_chirho(&parse_version_chirho("99.99").unwrap()));
    }

    #[test]
    fn constraint_display_chirho() {
        let c_chirho =
            parse_version_constraint_chirho(">=1.0 && <2.0").unwrap();
        assert_eq!(c_chirho.to_string(), ">=1.0 && <2.0");
    }

    #[test]
    fn empty_version_fails_chirho() {
        assert!(parse_version_chirho("").is_none());
    }

    #[test]
    fn wildcard_version_fails_chirho() {
        assert!(parse_version_chirho("1.2.*").is_none());
    }
}
