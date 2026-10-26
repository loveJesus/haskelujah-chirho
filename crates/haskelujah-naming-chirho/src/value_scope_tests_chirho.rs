// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Qualified value scope under explicit import lists (row 536).
//!
//! Each module here resolves against the real builtin interfaces, through the
//! same entry point the driver calls, and every diagnostic it yields is
//! compared whole. A qualifier closed by explicit lists rejects what they do
//! not supply; every way a qualifier stays open keeps today's behavior.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, FieldDeclChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho, ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::OccurrenceRoleChirho;
use haskelujah_ast_chirho::stmt_operation_chirho::SelectedOperationChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{ModuleIfaceChirho, build_iface_chirho, builtin_module_ifaces_chirho};
use crate::resolve_chirho::resolve_module_with_imports_chirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(qualifier_chirho: &str, text_chirho: &str) -> ExprChirho {
    ExprChirho::VarChirho(qualified_chirho(qualifier_chirho, text_chirho))
}

fn con_chirho(qualifier_chirho: &str, text_chirho: &str) -> ExprChirho {
    ExprChirho::ConChirho(qualified_chirho(qualifier_chirho, text_chirho))
}

fn undefined_chirho() -> ExprChirho {
    ExprChirho::VarChirho(name_chirho("undefined"))
}

/// `undefined Q.op undefined`.
fn infix_chirho(qualifier_chirho: &str, op_chirho: &str) -> ExprChirho {
    ExprChirho::InfixChirho {
        left_chirho: Box::new(undefined_chirho()),
        op_chirho: qualified_chirho(qualifier_chirho, op_chirho),
        right_chirho: Box::new(undefined_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// `import [qualified] module [as alias] [spec]`.
fn import_chirho(
    module_text_chirho: &str,
    qualified_import_chirho: bool,
    alias_chirho: Option<&str>,
    spec_chirho: Option<ImportSpecChirho>,
) -> ImportDeclChirho {
    ImportDeclChirho {
        module_chirho: name_chirho(module_text_chirho),
        qualified_chirho: qualified_import_chirho,
        alias_chirho: alias_chirho.map(name_chirho),
        spec_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn listing_chirho(items_chirho: Vec<ImportItemChirho>) -> Option<ImportSpecChirho> {
    Some(ImportSpecChirho {
        hiding_chirho: false,
        items_chirho,
    })
}

fn hiding_chirho(items_chirho: Vec<ImportItemChirho>) -> Option<ImportSpecChirho> {
    Some(ImportSpecChirho {
        hiding_chirho: true,
        items_chirho,
    })
}

fn item_chirho(text_chirho: &str) -> ImportItemChirho {
    ImportItemChirho::VarChirho(name_chirho(text_chirho))
}

fn all_of_chirho(type_text_chirho: &str) -> ImportItemChirho {
    ImportItemChirho::TyConChirho {
        name_chirho: name_chirho(type_text_chirho),
        members_chirho: ExportMembersChirho::AllChirho,
    }
}

/// `valueN = expression`, one top-level binding per expression.
fn bindings_chirho(expressions_chirho: Vec<ExprChirho>) -> Vec<DeclChirho> {
    expressions_chirho
        .into_iter()
        .enumerate()
        .map(
            |(index_chirho, expression_chirho)| DeclChirho::FunBindChirho {
                name_chirho: name_chirho(&format!("value{index_chirho}Chirho")),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(expression_chirho),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        )
        .collect()
}

fn module_chirho(
    module_text_chirho: &str,
    imports_chirho: Vec<ImportDeclChirho>,
    decls_chirho: Vec<DeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho(module_text_chirho),
        exports_chirho: None,
        imports_chirho,
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

/// Every diagnostic message name resolution yields for `Main`, resolved
/// against the builtin interfaces and then `extra_ifaces_chirho`.
fn messages_chirho(
    imports_chirho: Vec<ImportDeclChirho>,
    decls_chirho: Vec<DeclChirho>,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
) -> Vec<String> {
    let mut ifaces_chirho = builtin_module_ifaces_chirho();
    ifaces_chirho.extend(extra_ifaces_chirho);
    let main_chirho = module_chirho("Main", imports_chirho, decls_chirho);
    resolve_module_with_imports_chirho(&main_chirho, &ifaces_chirho)
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .map(|diagnostic_chirho| diagnostic_chirho.message_chirho.clone())
        .collect()
}

/// I2's own import: `import qualified Data.List.NonEmpty as NE (head, NonEmpty(..))`.
fn non_empty_head_import_chirho() -> ImportDeclChirho {
    import_chirho(
        "Data.List.NonEmpty",
        true,
        Some("NE"),
        listing_chirho(vec![item_chirho("head"), all_of_chirho("NonEmpty")]),
    )
}

#[test]
fn i2_rejects_the_unlisted_to_list_and_keeps_the_listed_names_chirho() {
    // GHC 9.14.1: `NE.toList` is GHC-76037, outside the import list.
    // `NE.head` is listed, and `NE.:|` comes with `NonEmpty(..)`, whose builtin
    // entry is complete.
    let messages_chirho = messages_chirho(
        vec![non_empty_head_import_chirho()],
        bindings_chirho(vec![
            var_chirho("NE", "toList"),
            var_chirho("NE", "head"),
            infix_chirho("NE", ":|"),
            con_chirho("NE", ":|"),
        ]),
        vec![],
    );
    assert_eq!(messages_chirho, ["variable not in scope: `NE.toList`"]);
}

#[test]
fn every_written_expression_form_is_checked_chirho() {
    // A constructor (GHC module/mod88: `import qualified Prelude (map)` and
    // `Prelude.Left`), an operator, a section's operator and a record
    // construction's constructor: each is a written occurrence.
    let messages_chirho = messages_chirho(
        vec![
            import_chirho(
                "Prelude",
                true,
                None,
                listing_chirho(vec![item_chirho("map")]),
            ),
            import_chirho(
                "Data.Map",
                true,
                Some("M"),
                listing_chirho(vec![item_chirho("insert")]),
            ),
        ],
        bindings_chirho(vec![
            con_chirho("Prelude", "Left"),
            infix_chirho("M", "!"),
            ExprChirho::LeftSectionChirho {
                op_chirho: qualified_chirho("M", "\\\\"),
                arg_chirho: Box::new(undefined_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            ExprChirho::RecordConChirho {
                con_chirho: qualified_chirho("Prelude", "Just"),
                fields_chirho: vec![],
                has_wildcard_chirho: false,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            var_chirho("Prelude", "map"),
            var_chirho("M", "insert"),
        ]),
        vec![],
    );
    assert_eq!(
        messages_chirho,
        [
            "data constructor not in scope: `Prelude.Left`",
            "variable not in scope: `M.!`",
            "variable not in scope: `M.\\\\`",
            "data constructor not in scope: `Prelude.Just`",
        ]
    );
}

#[test]
fn explicit_lists_close_their_qualifier_in_every_spelling_chirho() {
    let messages_chirho = messages_chirho(
        vec![
            // GHC rename/should_fail/T19843i: an unqualified import with an alias.
            import_chirho(
                "Data.Monoid",
                false,
                Some("M"),
                listing_chirho(vec![item_chirho("getAll")]),
            ),
            // An empty list supplies nothing.
            import_chirho("Data.Map", true, Some("E"), listing_chirho(vec![])),
            // An unqualified list closes the full module name.
            import_chirho(
                "Data.List",
                false,
                None,
                listing_chirho(vec![item_chirho("sort")]),
            ),
            // Two lists under one alias supply their union.
            import_chirho(
                "Data.List",
                true,
                Some("L"),
                listing_chirho(vec![item_chirho("sortOn")]),
            ),
            import_chirho(
                "Data.List.NonEmpty",
                true,
                Some("L"),
                listing_chirho(vec![item_chirho("toList")]),
            ),
        ],
        bindings_chirho(vec![
            var_chirho("M", "getSum"),
            var_chirho("M", "getAll"),
            var_chirho("E", "size"),
            var_chirho("Data.List", "nub"),
            var_chirho("Data.List", "sort"),
            var_chirho("L", "head"),
            var_chirho("L", "sortOn"),
            var_chirho("L", "toList"),
        ]),
        vec![],
    );
    assert_eq!(
        messages_chirho,
        [
            "variable not in scope: `M.getSum`",
            "variable not in scope: `E.size`",
            "variable not in scope: `Data.List.nub`",
            "variable not in scope: `L.head`",
        ]
    );
}

#[test]
fn every_open_qualifier_keeps_todays_behavior_chirho() {
    // Each of these leaves its qualifier open, so nothing here is reported.
    let messages_chirho = messages_chirho(
        vec![
            // An explicit list beside an open import of the same qualifier.
            import_chirho(
                "Data.Map",
                true,
                Some("M"),
                listing_chirho(vec![item_chirho("insert")]),
            ),
            import_chirho("Data.Map", true, Some("M"), None),
            // `hiding`.
            import_chirho(
                "Data.List.NonEmpty",
                true,
                Some("NE"),
                hiding_chirho(vec![item_chirho("head")]),
            ),
            // An item lowering could not represent (`(data ..)`).
            import_chirho(
                "Data.Proxy",
                true,
                Some("D"),
                listing_chirho(vec![ImportItemChirho::UnrepresentedChirho(
                    SpanChirho::DUMMY_CHIRHO,
                )]),
            ),
            // Partial builtin entries: `Semigroup(..)` lists only `<>`, and
            // `RWST(..)` only its constructor.
            import_chirho(
                "Data.Semigroup",
                true,
                Some("S"),
                listing_chirho(vec![all_of_chirho("Semigroup")]),
            ),
            import_chirho(
                "Control.Monad.Trans.RWS.Lazy",
                true,
                Some("Lazy"),
                listing_chirho(vec![all_of_chirho("RWST")]),
            ),
        ],
        bindings_chirho(vec![
            var_chirho("M", "lookup"),
            var_chirho("NE", "toList"),
            con_chirho("D", "Proxy"),
            var_chirho("S", "sconcat"),
            var_chirho("Lazy", "runRWST"),
        ]),
        vec![],
    );
    assert!(messages_chirho.is_empty(), "{messages_chirho:?}");
}

#[test]
fn a_listed_name_is_supplied_even_where_the_interface_omits_it_chirho() {
    // The builtin Data.List.NonEmpty lists six values; `reverse` is not one of
    // them, but the written list is the authority.
    let messages_chirho = messages_chirho(
        vec![import_chirho(
            "Data.List.NonEmpty",
            true,
            Some("NE"),
            listing_chirho(vec![item_chirho("reverse"), all_of_chirho("NonEmpty")]),
        )],
        bindings_chirho(vec![
            var_chirho("NE", "reverse"),
            infix_chirho("NE", ":|"),
            var_chirho("NE", "toList"),
        ]),
        vec![],
    );
    assert_eq!(messages_chirho, ["variable not in scope: `NE.toList`"]);
}

#[test]
fn a_source_derived_type_closes_on_its_complete_members_chirho() {
    // GHC rename/should_compile/T17853's shape: a sibling module's record,
    // imported as `X(..)`, brings its constructor and its field.
    let records_chirho = build_iface_chirho(&module_chirho(
        "RecordsChirho",
        vec![],
        vec![DeclChirho::DataDeclChirho {
            name_chirho: name_chirho("X"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![ConDeclChirho::RecordChirho {
                name_chirho: name_chirho("X"),
                fields_chirho: vec![FieldDeclChirho {
                    names_chirho: vec![name_chirho("name")],
                    ty_chirho: TypeChirho::ConChirho(name_chirho("String")),
                    strictness_chirho: StrictnessChirho::LazyChirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    ));
    let messages_chirho = messages_chirho(
        vec![import_chirho(
            "RecordsChirho",
            true,
            Some("R"),
            listing_chirho(vec![all_of_chirho("X")]),
        )],
        bindings_chirho(vec![
            var_chirho("R", "name"),
            con_chirho("R", "X"),
            var_chirho("R", "other"),
        ]),
        vec![records_chirho],
    );
    assert_eq!(messages_chirho, ["variable not in scope: `R.other`"]);
}

#[test]
fn the_module_name_still_qualifies_its_own_top_level_values_chirho() {
    // Even where an import list also binds the module's own name: the module's
    // own `value0Chirho` stays in scope, the listed `head` is supplied, and
    // only the unlisted `toList` is reported.
    let messages_chirho = messages_chirho(
        vec![import_chirho(
            "Data.List.NonEmpty",
            true,
            Some("Main"),
            listing_chirho(vec![item_chirho("head")]),
        )],
        bindings_chirho(vec![
            var_chirho("Main", "value0Chirho"),
            var_chirho("Main", "head"),
            var_chirho("Main", "toList"),
        ]),
        vec![],
    );
    assert_eq!(messages_chirho, ["variable not in scope: `Main.toList`"]);
}

#[test]
fn a_do_statements_selected_operations_are_not_written_occurrences_chirho() {
    // `M.do { x <- undefined; M.lookup }` selects `M.>>=`, which the list does
    // not name; only the written `M.lookup` is checked.
    let bind_chirho = StmtChirho::BindChirho {
        pat_chirho: PatChirho::VarChirho(name_chirho("x")),
        expr_chirho: undefined_chirho(),
        bind_chirho: Some(SelectedOperationChirho::new_chirho(
            qualified_chirho("M", ">>="),
            OccurrenceRoleChirho::Bind,
        )),
        fail_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let do_block_chirho = ExprChirho::DoChirho {
        qualifier_chirho: Some("M".to_string()),
        stmts_chirho: vec![
            bind_chirho,
            StmtChirho::expr_stmt_chirho(var_chirho("M", "lookup")),
        ],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let messages_chirho = messages_chirho(
        vec![import_chirho(
            "Data.Map",
            true,
            Some("M"),
            listing_chirho(vec![item_chirho("insert")]),
        )],
        bindings_chirho(vec![do_block_chirho]),
        vec![],
    );
    assert_eq!(messages_chirho, ["variable not in scope: `M.lookup`"]);
}
