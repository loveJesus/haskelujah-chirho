// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Read presence paths from pinned upstream facades; a route supplies no checked declaration.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, OnceLock};

type KeyChirho = (String, NamespaceChirho);
type RoutesChirho = BTreeMap<KeyChirho, Vec<PreludeRouteChirho>>;
static ROUTES_CHIRHO: OnceLock<Result<Arc<RoutesChirho>, String>> = OnceLock::new();

#[derive(Clone, Default)]
struct MembersChirho {
    all_chirho: bool,
    names_chirho: Vec<String>,
}
#[derive(Clone)]
struct ItemChirho {
    name_chirho: String,
    qualifier_chirho: Option<String>,
    namespace_chirho: NamespaceChirho,
    members_chirho: MembersChirho,
    module_chirho: bool,
}
impl ItemChirho {
    fn admits_chirho(&self, key_chirho: &KeyChirho, parent_chirho: Option<&str>) -> bool {
        !self.module_chirho
            && self.qualifier_chirho.is_none()
            && (self.name_chirho == key_chirho.0 && self.namespace_chirho == key_chirho.1
                || parent_chirho == Some(self.name_chirho.as_str())
                    && (self.members_chirho.all_chirho
                        || self.members_chirho.names_chirho.contains(&key_chirho.0)))
    }
}
struct FacadeImportChirho {
    module_chirho: String,
    qualifier_chirho: String,
    usable_chirho: bool,
    hiding_chirho: bool,
    items_chirho: Option<Vec<ItemChirho>>,
}
impl FacadeImportChirho {
    fn admits_chirho(&self, key_chirho: &KeyChirho, parent_chirho: Option<&str>) -> bool {
        self.items_chirho.as_ref().is_none_or(|items_chirho| {
            items_chirho
                .iter()
                .any(|item_chirho| item_chirho.admits_chirho(key_chirho, parent_chirho))
                != self.hiding_chirho
        })
    }
}
struct FacadeChirho {
    exports_chirho: Vec<ItemChirho>,
    imports_chirho: Vec<FacadeImportChirho>,
}
struct ReaderChirho<'source_chirho> {
    tokens_chirho: Vec<(RawTokenKindChirho, &'source_chirho str)>,
    cursor_chirho: usize,
}
fn qualified_name_chirho(written_chirho: &str) -> (Option<String>, String) {
    let mut occurrence_chirho = written_chirho;
    let mut qualifier_len_chirho = 0;
    while let Some((part_chirho, rest_chirho)) = occurrence_chirho.split_once('.') {
        if !part_chirho.starts_with(char::is_uppercase)
            || !part_chirho.chars().all(|character_chirho| {
                character_chirho.is_alphanumeric() || matches!(character_chirho, '_' | '\'')
            })
        {
            break;
        }
        qualifier_len_chirho += part_chirho.len() + 1;
        occurrence_chirho = rest_chirho;
    }
    (
        (qualifier_len_chirho > 0).then(|| written_chirho[..qualifier_len_chirho - 1].to_string()),
        occurrence_chirho.to_string(),
    )
}
fn unsupported_chirho() -> NextErrorChirho {
    NextErrorChirho::new_chirho(
        NextErrorKindChirho::UnsupportedChirho,
        "unmodeled upstream facade header/import region",
    )
}
impl ReaderChirho<'_> {
    fn take_chirho(&mut self, kind_chirho: RawTokenKindChirho) -> bool {
        if self
            .tokens_chirho
            .get(self.cursor_chirho)
            .is_some_and(|token_chirho| token_chirho.0 == kind_chirho)
        {
            self.cursor_chirho += 1;
            true
        } else {
            false
        }
    }
    fn word_chirho(&mut self, word_chirho: &str) -> bool {
        if self
            .tokens_chirho
            .get(self.cursor_chirho)
            .is_some_and(|token_chirho| token_chirho.1 == word_chirho)
        {
            self.cursor_chirho += 1;
            true
        } else {
            false
        }
    }
    fn name_chirho(&mut self) -> Result<String, NextErrorChirho> {
        use RawTokenKindChirho::*;
        let paren_chirho = self.take_chirho(LeftParenChirho);
        let &(kind_chirho, text_chirho) = self
            .tokens_chirho
            .get(self.cursor_chirho)
            .ok_or_else(unsupported_chirho)?;
        if !(matches!(kind_chirho, VarIdChirho | ConIdChirho | QualifiedIdChirho)
            || paren_chirho && matches!(kind_chirho, VarSymChirho | ConSymChirho | TildeChirho))
        {
            return Err(unsupported_chirho());
        }
        self.cursor_chirho += 1;
        if paren_chirho && !self.take_chirho(RightParenChirho) {
            return Err(unsupported_chirho());
        }
        Ok(text_chirho.to_string())
    }
    fn items_chirho(&mut self) -> Result<Vec<ItemChirho>, NextErrorChirho> {
        use RawTokenKindChirho::*;
        let mut result_chirho = Vec::new();
        if self.take_chirho(RightParenChirho) {
            return Ok(result_chirho);
        }
        loop {
            let module_chirho = self.take_chirho(ModuleChirho);
            let explicit_type_chirho = self.take_chirho(TypeChirho);
            let written_chirho = self.name_chirho()?;
            let (qualifier_chirho, name_chirho) = if module_chirho {
                (None, written_chirho)
            } else {
                qualified_name_chirho(&written_chirho)
            };
            let namespace_chirho =
                if explicit_type_chirho || name_chirho.starts_with(char::is_uppercase) {
                    NamespaceChirho::TypeChirho
                } else {
                    NamespaceChirho::ValueChirho
                };
            let members_chirho = if self.take_chirho(LeftParenChirho) {
                if self.take_chirho(DotDotChirho) {
                    if !self.take_chirho(RightParenChirho) {
                        return Err(unsupported_chirho());
                    }
                    MembersChirho {
                        all_chirho: true,
                        ..Default::default()
                    }
                } else {
                    let mut names_chirho = Vec::new();
                    loop {
                        names_chirho.push(self.name_chirho()?);
                        if self.take_chirho(RightParenChirho) {
                            break;
                        }
                        if !self.take_chirho(CommaChirho) {
                            return Err(unsupported_chirho());
                        }
                    }
                    MembersChirho {
                        all_chirho: false,
                        names_chirho,
                    }
                }
            } else {
                MembersChirho::default()
            };
            result_chirho.push(ItemChirho {
                name_chirho,
                qualifier_chirho,
                namespace_chirho,
                members_chirho,
                module_chirho,
            });
            if self.take_chirho(RightParenChirho) {
                break;
            }
            if !self.take_chirho(CommaChirho) {
                return Err(unsupported_chirho());
            }
            if self.take_chirho(RightParenChirho) {
                break;
            }
        }
        Ok(result_chirho)
    }
}
fn read_chirho(source_chirho: &str, module_chirho: &str) -> Result<FacadeChirho, NextErrorChirho> {
    use RawTokenKindChirho::*;
    let mut reader_chirho = ReaderChirho {
        tokens_chirho: LexerChirho::new_chirho(
            source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        )
        .lex_all_chirho()
        .into_iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .map(|token_chirho| {
            Ok((
                token_chirho.kind_chirho,
                token_chirho
                    .span_chirho
                    .text_chirho(source_chirho)
                    .ok_or_else(unsupported_chirho)?,
            ))
        })
        .collect::<Result<_, NextErrorChirho>>()?,
        cursor_chirho: 0,
    };
    while reader_chirho.word_chirho("#") {
        if !reader_chirho.word_chirho("include") || !reader_chirho.take_chirho(StringLitChirho) {
            return Err(unsupported_chirho());
        }
    }
    if !reader_chirho.take_chirho(ModuleChirho)
        || reader_chirho.name_chirho()? != module_chirho
        || !reader_chirho.take_chirho(LeftParenChirho)
    {
        return Err(unsupported_chirho());
    }
    let exports_chirho = reader_chirho.items_chirho()?;
    if !reader_chirho.take_chirho(WhereChirho) {
        return Err(unsupported_chirho());
    }
    let mut imports_chirho = Vec::new();
    loop {
        if reader_chirho.word_chirho("#") {
            // Includes introduce no conditional name scope. Every other directive declines.
            if !reader_chirho.word_chirho("include") || !reader_chirho.take_chirho(StringLitChirho)
            {
                return Err(unsupported_chirho());
            }
            continue;
        }
        if !reader_chirho.take_chirho(ImportChirho) {
            break;
        }
        let qualified_chirho = reader_chirho.word_chirho("qualified");
        let module_chirho = reader_chirho.name_chirho()?;
        let qualified_chirho = reader_chirho.word_chirho("qualified") || qualified_chirho;
        let alias_chirho = reader_chirho.word_chirho("as");
        let qualifier_chirho = if alias_chirho {
            reader_chirho.name_chirho()?
        } else {
            module_chirho.clone()
        };
        let hiding_chirho = reader_chirho.word_chirho("hiding");
        let items_chirho = if reader_chirho.take_chirho(LeftParenChirho) {
            Some(reader_chirho.items_chirho()?)
        } else {
            None
        };
        imports_chirho.push(FacadeImportChirho {
            module_chirho,
            qualifier_chirho,
            usable_chirho: !qualified_chirho && !alias_chirho,
            hiding_chirho,
            items_chirho,
        });
    }
    Ok(FacadeChirho {
        exports_chirho,
        imports_chirho,
    })
}

fn routes_from_chirho(
    module_chirho: &str,
    key_chirho: &KeyChirho,
    parent_chirho: Option<&str>,
    facades_chirho: &BTreeMap<String, FacadeChirho>,
    visited_chirho: &mut BTreeSet<String>,
    output_chirho: &mut Vec<PreludeRouteChirho>,
) {
    let Some(facade_chirho) = facades_chirho.get(module_chirho) else {
        return;
    };
    if !visited_chirho.insert(module_chirho.to_string()) {
        return;
    }
    let named_chirho = facade_chirho
        .exports_chirho
        .iter()
        .any(|item_chirho| item_chirho.admits_chirho(key_chirho, parent_chirho));
    // A self re-export is only a candidate. Installation still requires the exact
    // original name to be exported by this module's independently checked slice.
    let self_chirho = facade_chirho
        .exports_chirho
        .iter()
        .any(|item_chirho| item_chirho.module_chirho && item_chirho.name_chirho == module_chirho);
    if (named_chirho || self_chirho) && manifest_chirho(module_chirho).is_some() {
        let original_chirho = OriginalNameChirho {
            unit_chirho: UnitIdentityChirho::CompilerChirho,
            module_chirho: module_chirho.into(),
            namespace_chirho: key_chirho.1,
            occurrence_chirho: key_chirho.0.clone(),
        };
        let parent_chirho = parent_chirho.map(|parent_chirho| OriginalNameChirho {
            namespace_chirho: NamespaceChirho::TypeChirho,
            occurrence_chirho: parent_chirho.into(),
            ..original_chirho.clone()
        });
        let route_chirho = PreludeRouteChirho {
            original_chirho,
            parent_chirho,
        };
        if !output_chirho.contains(&route_chirho) {
            output_chirho.push(route_chirho);
        }
    }
    for import_chirho in &facade_chirho.imports_chirho {
        let qualified_chirho = facade_chirho.exports_chirho.iter().any(|item_chirho| {
            item_chirho.qualifier_chirho.as_deref() == Some(import_chirho.qualifier_chirho.as_str())
                && item_chirho.name_chirho == key_chirho.0
                && item_chirho.namespace_chirho == key_chirho.1
        });
        if import_chirho.admits_chirho(key_chirho, parent_chirho)
            && (qualified_chirho
                || import_chirho.usable_chirho
                    && (named_chirho
                        || facade_chirho.exports_chirho.iter().any(|item_chirho| {
                            item_chirho.module_chirho
                                && item_chirho.name_chirho == import_chirho.module_chirho
                        })))
        {
            routes_from_chirho(
                &import_chirho.module_chirho,
                key_chirho,
                parent_chirho,
                facades_chirho,
                visited_chirho,
                output_chirho,
            );
        }
    }
    visited_chirho.remove(module_chirho);
}

pub(in crate::next_chirho) fn routes_chirho() -> Result<Arc<RoutesChirho>, NextErrorChirho> {
    ROUTES_CHIRHO
        .get_or_init(|| {
            let build_chirho = || -> Result<RoutesChirho, NextErrorChirho> {
                let exports_chirho = prelude_exports_chirho()?;
                let mut facades_chirho = BTreeMap::from([(
                    "Prelude".into(),
                    read_chirho(prelude_scope_chirho::SOURCE_CHIRHO, "Prelude")?,
                )]);
                for manifest_chirho in
                    [&NUM_CHIRHO, &INTEGER_CHIRHO, &TUPLE_CHIRHO, &CLASSES_CHIRHO]
                {
                    verify_source_chirho(
                        manifest_chirho.bytes_chirho,
                        manifest_chirho.sha256_chirho,
                        manifest_chirho.upstream_path_chirho,
                        manifest_chirho.ghc_version_chirho,
                    )?;
                    if let Ok(facade_chirho) =
                        read_chirho(manifest_chirho.bytes_chirho, manifest_chirho.module_chirho)
                    {
                        facades_chirho.insert(manifest_chirho.module_chirho.into(), facade_chirho);
                    }
                }
                verify_source_chirho(
                    SHOW_CHIRHO.bytes_chirho,
                    SHOW_CHIRHO.sha256_chirho,
                    SHOW_CHIRHO.upstream_path_chirho,
                    SHOW_CHIRHO.ghc_version_chirho,
                )?;
                facades_chirho.insert(
                    SHOW_MODULE_CHIRHO.into(),
                    read_chirho(
                        prologue_chirho::validate_chirho(&SHOW_CHIRHO)?,
                        SHOW_MODULE_CHIRHO,
                    )?,
                );
                let base_chirho = include_str!("vendor-chirho/ghc-internal-chirho/Base.hs");
                verify_source_chirho(
                    base_chirho,
                    "bc38ea9356f90aeb38298ef1269fbdc2dee433528374948375da112b273a89e2",
                    "libraries/ghc-internal/src/GHC/Internal/Base.hs",
                    "9.14.1",
                )?;
                facades_chirho.insert(
                    "GHC.Internal.Base".into(),
                    read_chirho(base_chirho, "GHC.Internal.Base")?,
                );
                for (module_chirho, path_chirho, bytes_chirho, sha_chirho) in [
                    (
                        "GHC.Internal.Data.List",
                        "libraries/ghc-internal/src/GHC/Internal/Data/List.hs",
                        include_str!("vendor-chirho/ghc-internal-chirho/data-chirho/List.hs"),
                        "9f8f3d2b317b06c1ea65b03d0cd95b50a6125642d6836847e3a755ff1c91d79b",
                    ),
                    (
                        "GHC.Internal.Data.OldList",
                        "libraries/ghc-internal/src/GHC/Internal/Data/OldList.hs",
                        include_str!("vendor-chirho/ghc-internal-chirho/data-chirho/OldList.hs"),
                        "4b519356f7bcbd705866551e26552c84b08d527fe93b060cc89b27882aec6248",
                    ),
                ] {
                    verify_source_chirho(bytes_chirho, sha_chirho, path_chirho, "9.14.1")?;
                    facades_chirho.insert(
                        module_chirho.into(),
                        read_chirho(bytes_chirho, module_chirho)?,
                    );
                }
                let mut result_chirho = BTreeMap::new();
                for item_chirho in &facades_chirho["Prelude"].exports_chirho {
                    let key_chirho = (
                        item_chirho.name_chirho.clone(),
                        item_chirho.namespace_chirho,
                    );
                    let mut queries_chirho = vec![(key_chirho, None)];
                    queries_chirho.extend(item_chirho.members_chirho.names_chirho.iter().map(
                        |name_chirho| {
                            (
                                (name_chirho.clone(), NamespaceChirho::ValueChirho),
                                Some(item_chirho.name_chirho.as_str()),
                            )
                        },
                    ));
                    for (key_chirho, parent_chirho) in queries_chirho {
                        // Qualified facade spellings are not ordinary occurrences. A
                        // candidate must belong to the independently read export account.
                        if !exports_chirho.contains(&key_chirho) {
                            continue;
                        }
                        let mut routes_chirho = Vec::new();
                        routes_from_chirho(
                            "Prelude",
                            &key_chirho,
                            parent_chirho,
                            &facades_chirho,
                            &mut BTreeSet::new(),
                            &mut routes_chirho,
                        );
                        if !routes_chirho.is_empty() {
                            result_chirho.insert(key_chirho, routes_chirho);
                        }
                    }
                }
                Ok(result_chirho)
            };
            build_chirho()
                .map(Arc::new)
                .map_err(|error_chirho| error_chirho.message_chirho)
        })
        .clone()
        .map_err(|message_chirho| {
            NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
        })
}

#[cfg(test)]
#[path = "facade_tests_chirho.rs"]
mod tests_chirho;
