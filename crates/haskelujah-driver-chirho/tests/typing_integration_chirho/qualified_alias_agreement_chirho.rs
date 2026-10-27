// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The generated table behind a qualified builtin alias's scheme (row 608).
//!
//! An entry `(module, name, type)` says that GHC 9.14.1 types `module.name` as `type`, and that
//! the bare builtin scheme of `name` prints to the same `type` under the normalization contract in
//! `test-data-chirho/qualified-alias-agreement-chirho/README-chirho.md`. That one contract is
//! [`normalized_type_chirho`], here. The tests hold the table to it: every entry still agrees with
//! today's bare scheme, names an unkeyed builtin export, and is the whole of Phase 3.9's decision.
//! The two ignored tests are the generator's Rust steps: `generate_chirho.py` runs them, and no
//! gate does.
//! workflow: compiler-pipeline-chirho/builtin-type-environment-chirho

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use haskelujah_driver::qualified_import_alias_scheme_chirho;
use haskelujah_naming_chirho::iface_chirho::{IfaceExportsChirho, merge_module_ifaces_chirho};
use haskelujah_naming_chirho::{
    IfaceOriginChirho, ModuleIfaceChirho, build_iface_with_imports_chirho,
    builtin_module_ifaces_chirho,
};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_typing_chirho::infer_chirho::{
    QUALIFIED_ALIAS_AGREEMENT_CHIRHO, builtin_value_schemes_chirho,
};
use haskelujah_typing_chirho::{SchemeChirho, TyChirho};

const SYMBOL_CHARS_CHIRHO: &str = "!#$%&*+./<=>?@\\^|-~:";

/// A builtin type printed as Haskell type text, ready for the normalizer.
fn ty_text_chirho(ty_chirho: &TyChirho, top_chirho: bool) -> String {
    let wrap_chirho = |text_chirho: String| {
        if top_chirho {
            text_chirho
        } else {
            format!("({text_chirho})")
        }
    };
    match ty_chirho {
        TyChirho::VarChirho(var_chirho) => format!("t{}", var_chirho.0),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            name_chirho.clone()
        }
        TyChirho::ListChirho(inner_chirho) => format!("[{}]", ty_text_chirho(inner_chirho, true)),
        TyChirho::TupleChirho(elements_chirho) => format!(
            "({})",
            elements_chirho
                .iter()
                .map(|element_chirho| ty_text_chirho(element_chirho, true))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
            let arg_text_chirho = match **arg_chirho {
                TyChirho::FunChirho(..) => format!("({})", ty_text_chirho(arg_chirho, true)),
                _ => ty_text_chirho(arg_chirho, true),
            };
            wrap_chirho(format!(
                "{arg_text_chirho} -> {}",
                ty_text_chirho(result_chirho, true)
            ))
        }
        TyChirho::AppChirho(..) => {
            let mut args_chirho = Vec::new();
            let mut head_chirho = ty_chirho;
            while let TyChirho::AppChirho(fun_chirho, arg_chirho) = head_chirho {
                args_chirho.push(ty_text_chirho(arg_chirho, false));
                head_chirho = fun_chirho;
            }
            args_chirho.reverse();
            wrap_chirho(format!(
                "{} {}",
                ty_text_chirho(head_chirho, false),
                args_chirho.join(" ")
            ))
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => format!(
            "(forall {}. {})",
            vars_chirho
                .iter()
                .map(|var_chirho| format!("t{}", var_chirho.0))
                .collect::<Vec<_>>()
                .join(" "),
            ty_text_chirho(body_chirho, true)
        ),
    }
}

/// A builtin scheme printed as `(C1 a, C2 b) => body`.
fn scheme_text_chirho(scheme_chirho: &SchemeChirho) -> String {
    let body_chirho = ty_text_chirho(&scheme_chirho.ty_chirho, true);
    if scheme_chirho.preds_chirho.is_empty() {
        return body_chirho;
    }
    let preds_chirho: Vec<String> = scheme_chirho
        .preds_chirho
        .iter()
        .map(|pred_chirho| {
            std::iter::once(pred_chirho.class_name_chirho.clone())
                .chain(std::iter::once(ty_text_chirho(
                    &pred_chirho.ty_chirho,
                    false,
                )))
                .chain(
                    pred_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(|extra_chirho| ty_text_chirho(extra_chirho, false)),
                )
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    format!("({}) => {body_chirho}", preds_chirho.join(", "))
}

/// Split type text into tokens: brackets, commas, symbol runs, and names. A name keeps its
/// module qualifiers (`Data.Map.Internal.Map`, `Prelude..`) until the contract strips them.
fn tokens_chirho(text_chirho: &str) -> Vec<String> {
    let chars_chirho: Vec<char> = text_chirho.chars().collect();
    let symbol_chirho = |char_chirho: char| SYMBOL_CHARS_CHIRHO.contains(char_chirho);
    let mut tokens_chirho = Vec::new();
    let mut index_chirho = 0;
    while index_chirho < chars_chirho.len() {
        let char_chirho = chars_chirho[index_chirho];
        let start_chirho = index_chirho;
        if char_chirho.is_whitespace() {
            index_chirho += 1;
            continue;
        }
        if "()[],".contains(char_chirho) {
            index_chirho += 1;
        } else if char_chirho.is_alphabetic() || char_chirho == '_' {
            loop {
                let segment_chirho = index_chirho;
                while index_chirho < chars_chirho.len()
                    && (chars_chirho[index_chirho].is_alphanumeric()
                        || "_'".contains(chars_chirho[index_chirho]))
                {
                    index_chirho += 1;
                }
                let qualifier_chirho = chars_chirho[segment_chirho].is_uppercase()
                    && chars_chirho.get(index_chirho) == Some(&'.')
                    && chars_chirho
                        .get(index_chirho + 1)
                        .is_some_and(|next_chirho| {
                            next_chirho.is_uppercase() || symbol_chirho(*next_chirho)
                        });
                if !qualifier_chirho {
                    break;
                }
                index_chirho += 1;
                if symbol_chirho(chars_chirho[index_chirho]) {
                    while index_chirho < chars_chirho.len()
                        && symbol_chirho(chars_chirho[index_chirho])
                    {
                        index_chirho += 1;
                    }
                    break;
                }
            }
        } else if symbol_chirho(char_chirho) {
            while index_chirho < chars_chirho.len() && symbol_chirho(chars_chirho[index_chirho]) {
                index_chirho += 1;
            }
        } else {
            index_chirho += 1;
        }
        tokens_chirho.push(chars_chirho[start_chirho..index_chirho].iter().collect());
    }
    tokens_chirho
}

/// A name with its module qualifiers stripped: `Data.Map.Internal.Map` is `Map`, and
/// `Prelude..` is `.`.
fn unqualified_chirho(token_chirho: &str) -> &str {
    let mut rest_chirho = token_chirho;
    while rest_chirho.starts_with(|char_chirho: char| char_chirho.is_uppercase()) {
        let Some(dot_chirho) = rest_chirho.find('.') else {
            break;
        };
        let segment_chirho = &rest_chirho[..dot_chirho];
        if dot_chirho + 1 >= rest_chirho.len()
            || !segment_chirho
                .chars()
                .all(|char_chirho| char_chirho.is_alphanumeric() || "_'".contains(char_chirho))
        {
            break;
        }
        rest_chirho = &rest_chirho[dot_chirho + 1..];
    }
    rest_chirho
}

/// Index of the first `token` outside any bracket, if any.
fn top_level_index_chirho(tokens_chirho: &[String], token_chirho: &str) -> Option<usize> {
    let mut depth_chirho = 0i32;
    for (index_chirho, current_chirho) in tokens_chirho.iter().enumerate() {
        match current_chirho.as_str() {
            "(" | "[" => depth_chirho += 1,
            ")" | "]" => depth_chirho -= 1,
            other_chirho if depth_chirho == 0 && other_chirho == token_chirho => {
                return Some(index_chirho);
            }
            _ => {}
        }
    }
    None
}

/// Whether the first `(` closes only at the last token, so the parentheses wrap the whole run.
fn encloses_whole_chirho(tokens_chirho: &[String]) -> bool {
    if tokens_chirho.first().map(String::as_str) != Some("(") {
        return false;
    }
    let mut depth_chirho = 0i32;
    for (index_chirho, token_chirho) in tokens_chirho.iter().enumerate() {
        match token_chirho.as_str() {
            "(" | "[" => depth_chirho += 1,
            ")" | "]" => depth_chirho -= 1,
            _ => {}
        }
        if depth_chirho == 0 {
            return index_chirho == tokens_chirho.len() - 1;
        }
    }
    false
}

fn render_chirho(tokens_chirho: &[String]) -> String {
    let mut text_chirho = String::new();
    for (index_chirho, token_chirho) in tokens_chirho.iter().enumerate() {
        let tight_chirho = index_chirho == 0
            || matches!(tokens_chirho[index_chirho - 1].as_str(), "(" | "[")
            || matches!(token_chirho.as_str(), ")" | "]" | ",");
        if !tight_chirho {
            text_chirho.push(' ');
        }
        text_chirho.push_str(token_chirho);
    }
    text_chirho
}

/// The normalization contract, applied alike to GHC's `:type` text and a printed bare scheme:
/// 1. tokenize; strip module qualifiers from names and operators;
/// 2. drop a leading `forall vs.`;
/// 3. expand `String` and `FilePath` to `[Char]`, and nothing else;
/// 4. split at the first top-level `=>`; split a parenthesized context at top-level commas;
/// 5. drop a `HasCallStack` constraint;
/// 6. rename type variables `v0`, `v1`, ... in order of first appearance in the body, then the
///    context;
/// 7. sort the constraints, and print `(C1, C2) => body`, or the body alone, with single spaces.
fn normalized_type_chirho(text_chirho: &str) -> String {
    let mut plain_tokens_chirho: Vec<String> = Vec::new();
    for token_chirho in tokens_chirho(text_chirho) {
        match unqualified_chirho(&token_chirho) {
            "String" | "FilePath" => {
                plain_tokens_chirho.extend(["[", "Char", "]"].map(str::to_string));
            }
            plain_chirho => plain_tokens_chirho.push(plain_chirho.to_string()),
        }
    }
    if plain_tokens_chirho.first().map(String::as_str) == Some("forall")
        && let Some(dot_chirho) = top_level_index_chirho(&plain_tokens_chirho, ".")
    {
        plain_tokens_chirho.drain(..=dot_chirho);
    }
    let (context_chirho, body_chirho) = match top_level_index_chirho(&plain_tokens_chirho, "=>") {
        Some(arrow_chirho) => (
            plain_tokens_chirho[..arrow_chirho].to_vec(),
            plain_tokens_chirho[arrow_chirho + 1..].to_vec(),
        ),
        None => (Vec::new(), plain_tokens_chirho),
    };
    let wrapped_chirho = encloses_whole_chirho(&context_chirho);
    let mut constraints_chirho: Vec<Vec<String>> = Vec::new();
    if wrapped_chirho {
        let mut rest_chirho = &context_chirho[1..context_chirho.len() - 1];
        while let Some(comma_chirho) = top_level_index_chirho(rest_chirho, ",") {
            constraints_chirho.push(rest_chirho[..comma_chirho].to_vec());
            rest_chirho = &rest_chirho[comma_chirho + 1..];
        }
        constraints_chirho.push(rest_chirho.to_vec());
    } else if !context_chirho.is_empty() {
        constraints_chirho.push(context_chirho);
    }
    constraints_chirho.retain(|constraint_chirho| {
        !constraint_chirho.is_empty() && constraint_chirho != &["HasCallStack".to_string()]
    });
    let mut names_chirho: HashMap<String, String> = HashMap::new();
    let mut rename_chirho = |tokens_chirho: &[String]| -> Vec<String> {
        tokens_chirho
            .iter()
            .map(|token_chirho| {
                let variable_chirho = token_chirho != "forall"
                    && token_chirho.starts_with(|char_chirho: char| {
                        char_chirho.is_lowercase() || char_chirho == '_'
                    });
                if !variable_chirho {
                    return token_chirho.clone();
                }
                let next_chirho = format!("v{}", names_chirho.len());
                names_chirho
                    .entry(token_chirho.clone())
                    .or_insert(next_chirho)
                    .clone()
            })
            .collect()
    };
    let body_text_chirho = render_chirho(&rename_chirho(&body_chirho));
    let mut constraint_texts_chirho: Vec<String> = constraints_chirho
        .iter()
        .map(|constraint_chirho| render_chirho(&rename_chirho(constraint_chirho)))
        .collect();
    constraint_texts_chirho.sort();
    if constraint_texts_chirho.is_empty() {
        body_text_chirho
    } else {
        format!(
            "({}) => {body_text_chirho}",
            constraint_texts_chirho.join(", ")
        )
    }
}

/// An operator spelled `(op)` is canonical as `op`; any other name is its own canonical form.
fn canonical_value_name_chirho(name_chirho: &str) -> &str {
    match name_chirho
        .strip_prefix('(')
        .and_then(|inner_chirho| inner_chirho.strip_suffix(')'))
    {
        Some(inner_chirho)
            if !inner_chirho.is_empty()
                && inner_chirho
                    .chars()
                    .all(|char_chirho| !char_chirho.is_alphanumeric() && char_chirho != '_') =>
        {
            inner_chirho
        }
        _ => name_chirho,
    }
}

/// Every value an interface exports by its canonical name: its values, and its types'
/// constructors and class methods.
fn iface_value_names_chirho(iface_chirho: &ModuleIfaceChirho) -> BTreeSet<String> {
    let members_chirho = iface_chirho
        .exports_chirho
        .types_chirho
        .values()
        .flat_map(|type_chirho| {
            type_chirho
                .constructors_chirho
                .iter()
                .chain(&type_chirho.methods_chirho)
        })
        .map(|member_chirho| canonical_value_name_chirho(member_chirho).to_string());
    iface_chirho
        .exports_chirho
        .values_chirho
        .keys()
        .cloned()
        .chain(members_chirho)
        .collect()
}

/// Every builtin export, by module and canonical name, with whether a `module.name` key exists.
fn builtin_exports_chirho() -> BTreeMap<(String, String), bool> {
    let schemes_chirho = builtin_value_schemes_chirho();
    let mut exports_chirho = BTreeMap::new();
    for iface_chirho in builtin_module_ifaces_chirho() {
        for name_chirho in iface_value_names_chirho(&iface_chirho) {
            let keyed_chirho =
                schemes_chirho.contains_key(&format!("{}.{name_chirho}", iface_chirho.name_chirho));
            exports_chirho.insert(
                (iface_chirho.name_chirho.clone(), name_chirho),
                keyed_chirho,
            );
        }
    }
    exports_chirho
}

/// A source module's interface, built the way the loader builds one.
fn source_iface_chirho(
    source_chirho: &str,
    imported_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    let green_chirho =
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
    build_iface_with_imports_chirho(
        &lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO),
        imported_chirho,
    )
}

fn seeded_iface_chirho(name_chirho: &str) -> ModuleIfaceChirho {
    builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == name_chirho)
        .unwrap_or_else(|| panic!("{name_chirho} is a builtin interface"))
}

/// A project-local `Data.List` whose `reverse` and `lookup` disagree with base's.
const SOURCE_DATA_LIST_CHIRHO: &str = "module Data.List (reverse, lookup) where\nimport Prelude hiding (reverse, lookup)\nreverse :: Int -> Int\nreverse = negate\nlookup :: Int -> Int\nlookup = (+ 1)\n";

/// A source module named `Data.List` that only re-exports builtin names.
const REEXPORTING_DATA_LIST_CHIRHO: &str =
    "module Data.List (reverse, lookup) where\nimport GHC.List (reverse, lookup)\n";

fn generator_dir_chirho(variable_chirho: &str) -> PathBuf {
    PathBuf::from(
        std::env::var(variable_chirho)
            .unwrap_or_else(|_| panic!("generate_chirho.py sets {variable_chirho}")),
    )
}

/// One generated entry, quoted so the table needs no escapes: no exported name or type holds `"`.
fn table_line_chirho(module_chirho: &str, name_chirho: &str, type_chirho: &str) -> String {
    for field_chirho in [module_chirho, name_chirho, type_chirho] {
        assert!(
            !field_chirho.contains('"'),
            "a raw string cannot hold {field_chirho}"
        );
    }
    format!("    (r\"{module_chirho}\", r\"{name_chirho}\", r\"{type_chirho}\"),")
}

#[test]
#[ignore = "generator step 1; run by test-data-chirho/qualified-alias-agreement-chirho/generate_chirho.py"]
fn dump_qualified_alias_queries_chirho() {
    let dir_chirho = generator_dir_chirho("QUALIFIED_ALIAS_WORK_CHIRHO");
    let rows_chirho: Vec<String> = builtin_exports_chirho()
        .into_iter()
        .filter(|(_, keyed_chirho)| !keyed_chirho)
        .map(|((module_chirho, name_chirho), _)| format!("{module_chirho}\t{name_chirho}\n"))
        .collect();
    std::fs::write(dir_chirho.join("queries-chirho.tsv"), rows_chirho.concat()).unwrap();
}

#[test]
#[ignore = "generator step 2; run by test-data-chirho/qualified-alias-agreement-chirho/generate_chirho.py"]
fn write_qualified_alias_table_chirho() {
    let work_chirho = generator_dir_chirho("QUALIFIED_ALIAS_WORK_CHIRHO");
    let evidence_chirho = generator_dir_chirho("QUALIFIED_ALIAS_EVIDENCE_CHIRHO");
    let schemes_chirho = builtin_value_schemes_chirho();
    let answers_chirho = std::fs::read_to_string(work_chirho.join("ghc-types-chirho.tsv")).unwrap();
    let mut counts_chirho: BTreeMap<&str, usize> = BTreeMap::new();
    let mut table_chirho = Vec::new();
    let mut disagreements_chirho = Vec::new();
    for line_chirho in answers_chirho.lines() {
        let mut fields_chirho = line_chirho.splitn(3, '\t');
        let (Some(module_chirho), Some(name_chirho), Some(ghc_chirho)) = (
            fields_chirho.next(),
            fields_chirho.next(),
            fields_chirho.next(),
        ) else {
            panic!("malformed answer row: {line_chirho}");
        };
        let verdict_chirho = match (ghc_chirho.is_empty(), schemes_chirho.get(name_chirho)) {
            (true, _) => "not typed by GHC",
            (false, None) => "no bare scheme",
            (false, Some(scheme_chirho)) => {
                let ghc_type_chirho = normalized_type_chirho(ghc_chirho);
                let bare_type_chirho = normalized_type_chirho(&scheme_text_chirho(scheme_chirho));
                if ghc_type_chirho == bare_type_chirho {
                    table_chirho.push(table_line_chirho(
                        module_chirho,
                        name_chirho,
                        &ghc_type_chirho,
                    ));
                    "agree"
                } else {
                    disagreements_chirho.push(format!(
                        "{module_chirho}\t{name_chirho}\t{ghc_type_chirho}\t{bare_type_chirho}\n"
                    ));
                    "disagree"
                }
            }
        };
        *counts_chirho.entry(verdict_chirho).or_default() += 1;
    }
    let table_path_chirho = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../haskelujah-typing-chirho/src/infer_chirho/builtins_chirho/qualified_alias_agreement_chirho.rs");
    let header_chirho = std::fs::read_to_string(&table_path_chirho).unwrap();
    let (prefix_chirho, _) = header_chirho
        .split_once("= &[\n")
        .expect("the generated table keeps its header");
    std::fs::write(
        &table_path_chirho,
        format!("{prefix_chirho}= &[\n{}\n];\n", table_chirho.join("\n")),
    )
    .unwrap();
    std::fs::write(
        evidence_chirho.join("disagreements-chirho.tsv"),
        format!(
            "# module\tname\tGHC 9.14.1 type, normalized\tbare builtin scheme, normalized\n{}",
            disagreements_chirho.concat()
        ),
    )
    .unwrap();
    let total_chirho: usize = counts_chirho.values().sum();
    let mut summary_chirho = format!("queried pairs\t{total_chirho}\n");
    for (verdict_chirho, count_chirho) in &counts_chirho {
        summary_chirho.push_str(&format!("{verdict_chirho}\t{count_chirho}\n"));
    }
    std::fs::write(evidence_chirho.join("counts-chirho.txt"), summary_chirho).unwrap();
}

#[test]
fn the_contract_normalizes_both_sides_alike_chirho() {
    // GHC's spelling and the printer's spelling of one type meet in one normal form.
    for (ghc_chirho, bare_chirho) in [
        ("[a] -> [a]", "[t3080] -> [t3080]"),
        (
            "Foldable t => t a -> GHC.Types.Int",
            "(Foldable t30) => t30 t31 -> Int",
        ),
        (
            "(GHC.Classes.Ord b, GHC.Base.Monad m) => (a -> m b) -> [a] -> m [b]",
            "(Monad t2, Ord t3) => (t1 -> t2 t3) -> [t1] -> t2 [t3]",
        ),
        ("GHC.Stack.HasCallStack => [a] -> a", "[t9] -> t9"),
        (
            "Data.String.String -> [Data.String.String]",
            "[Char] -> [[Char]]",
        ),
        (
            "(a -> b) -> (c -> a) -> c -> b",
            "(t1 -> t2) -> (t3 -> t1) -> t3 -> t2",
        ),
        ("forall a. a -> a", "t4 -> t4"),
    ] {
        assert_eq!(
            normalized_type_chirho(ghc_chirho),
            normalized_type_chirho(bare_chirho),
            "{ghc_chirho}"
        );
    }
    // And nothing more: a container, a missing constraint, and an unexpanded Rational stay apart.
    for (ghc_chirho, bare_chirho) in [
        (
            "(a -> Bool) -> Data.Map.Internal.Map k a -> Data.Map.Internal.Map k a",
            "(t1 -> Bool) -> [t1] -> [t1]",
        ),
        ("GHC.Base.Applicative f => f a -> f b", "t1 t2 -> t1 t3"),
        ("GHC.Real.Rational -> a", "Ratio Integer -> t1"),
    ] {
        assert_ne!(
            normalized_type_chirho(ghc_chirho),
            normalized_type_chirho(bare_chirho),
            "{ghc_chirho}"
        );
    }
}

#[test]
fn the_table_is_sorted_and_unique_chirho() {
    for pair_chirho in QUALIFIED_ALIAS_AGREEMENT_CHIRHO.windows(2) {
        assert!(
            (pair_chirho[0].0, pair_chirho[0].1) < (pair_chirho[1].0, pair_chirho[1].1),
            "{:?} must sort before {:?}",
            pair_chirho[0],
            pair_chirho[1]
        );
    }
}

#[test]
fn every_entry_is_an_unkeyed_builtin_export_that_still_agrees_chirho() {
    // A later edit to a bare builtin scheme fails here instead of mistyping an alias.
    let schemes_chirho = builtin_value_schemes_chirho();
    let exports_chirho = builtin_exports_chirho();
    for (module_chirho, name_chirho, ghc_type_chirho) in QUALIFIED_ALIAS_AGREEMENT_CHIRHO {
        assert_eq!(
            exports_chirho.get(&(module_chirho.to_string(), name_chirho.to_string())),
            Some(&false),
            "{module_chirho}.{name_chirho} must be a builtin export with no module key"
        );
        let scheme_chirho = schemes_chirho
            .get(*name_chirho)
            .unwrap_or_else(|| panic!("{name_chirho} has no bare builtin scheme"));
        assert_eq!(
            normalized_type_chirho(&scheme_text_chirho(scheme_chirho)),
            *ghc_type_chirho,
            "{module_chirho}.{name_chirho}: the bare scheme no longer prints to GHC 9.14.1's type; \
             regenerate with generate_chirho.py"
        );
    }
}

#[test]
fn phase_3_9_reads_the_table_and_nothing_else_chirho() {
    // Over every unkeyed export of every builtin interface, Phase 3.9's decision gives the bare
    // scheme exactly for the table's entries, so no curated exception list stands beside it.
    let schemes_chirho = builtin_value_schemes_chirho();
    let table_chirho: BTreeSet<(&str, &str)> = QUALIFIED_ALIAS_AGREEMENT_CHIRHO
        .iter()
        .map(|(module_chirho, name_chirho, _)| (*module_chirho, *name_chirho))
        .collect();
    let mut decided_chirho = 0;
    for iface_chirho in builtin_module_ifaces_chirho() {
        let module_chirho = iface_chirho.name_chirho.clone();
        for name_chirho in iface_value_names_chirho(&iface_chirho) {
            if schemes_chirho.contains_key(&format!("{module_chirho}.{name_chirho}")) {
                continue;
            }
            let decision_chirho =
                qualified_import_alias_scheme_chirho(&iface_chirho, &name_chirho, &schemes_chirho);
            let listed_chirho =
                table_chirho.contains(&(module_chirho.as_str(), name_chirho.as_str()));
            assert_eq!(
                decision_chirho.is_some(),
                listed_chirho,
                "{module_chirho}.{name_chirho}"
            );
            if let Some(scheme_chirho) = decision_chirho {
                assert_eq!(Some(&scheme_chirho), schemes_chirho.get(&name_chirho));
            }
            decided_chirho += 1;
        }
    }
    assert!(decided_chirho > table_chirho.len());
}

#[test]
fn every_builtin_interface_is_builtin_and_the_default_is_source_chirho() {
    // A later default copy of builtin exports would downgrade them and fail here.
    for iface_chirho in builtin_module_ifaces_chirho() {
        assert_eq!(
            iface_chirho.exports_chirho.origin_chirho,
            IfaceOriginChirho::BuiltinChirho,
            "{}",
            iface_chirho.name_chirho
        );
    }
    assert_eq!(
        IfaceExportsChirho::default().origin_chirho,
        IfaceOriginChirho::SourceChirho
    );
    for source_chirho in [SOURCE_DATA_LIST_CHIRHO, REEXPORTING_DATA_LIST_CHIRHO] {
        let iface_chirho = source_iface_chirho(source_chirho, &[seeded_iface_chirho("GHC.List")]);
        assert_eq!(
            iface_chirho.exports_chirho.origin_chirho,
            IfaceOriginChirho::SourceChirho
        );
    }
}

#[test]
fn a_merge_stays_builtin_only_when_every_copy_is_chirho() {
    let seeded_chirho = seeded_iface_chirho("Data.List");
    let source_chirho = source_iface_chirho(SOURCE_DATA_LIST_CHIRHO, &[]);
    let merged_origin_chirho = |copies_chirho: Vec<ModuleIfaceChirho>| {
        let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
        assert_eq!(merged_chirho.len(), 1);
        merged_chirho[0].exports_chirho.origin_chirho
    };
    assert_eq!(
        merged_origin_chirho(vec![seeded_chirho.clone(), seeded_chirho.clone()]),
        IfaceOriginChirho::BuiltinChirho
    );
    assert_eq!(
        merged_origin_chirho(vec![seeded_chirho.clone(), source_chirho.clone()]),
        IfaceOriginChirho::SourceChirho
    );
    assert_eq!(
        merged_origin_chirho(vec![source_chirho, seeded_chirho]),
        IfaceOriginChirho::SourceChirho
    );
}

#[test]
fn only_a_builtin_interface_reads_the_table_chirho() {
    // The seeded Data.List hits `reverse`. A source Data.List, alone or merged with the seed in
    // either order, and a source module re-exporting base's names, miss `reverse` and `lookup`
    // although `(Data.List, reverse)` and `(Data.List, lookup)` are both table entries.
    let schemes_chirho = builtin_value_schemes_chirho();
    let seeded_chirho = seeded_iface_chirho("Data.List");
    assert_eq!(
        qualified_import_alias_scheme_chirho(&seeded_chirho, "reverse", &schemes_chirho).as_ref(),
        schemes_chirho.get("reverse")
    );
    let source_chirho = source_iface_chirho(SOURCE_DATA_LIST_CHIRHO, &[]);
    let sourced_chirho = [
        merge_module_ifaces_chirho(vec![seeded_chirho.clone(), source_chirho.clone()]).remove(0),
        merge_module_ifaces_chirho(vec![source_chirho.clone(), seeded_chirho]).remove(0),
        source_chirho,
        source_iface_chirho(
            REEXPORTING_DATA_LIST_CHIRHO,
            &[seeded_iface_chirho("GHC.List")],
        ),
    ];
    for iface_chirho in &sourced_chirho {
        assert_eq!(iface_chirho.name_chirho, "Data.List");
        for name_chirho in ["reverse", "lookup"] {
            assert!(
                QUALIFIED_ALIAS_AGREEMENT_CHIRHO
                    .iter()
                    .any(
                        |(module_chirho, entry_chirho, _)| *module_chirho == "Data.List"
                            && *entry_chirho == name_chirho
                    ),
                "Data.List.{name_chirho} is a table entry"
            );
            assert_eq!(
                qualified_import_alias_scheme_chirho(iface_chirho, name_chirho, &schemes_chirho),
                None,
                "a source Data.List must not read the table for {name_chirho}"
            );
        }
    }
}
