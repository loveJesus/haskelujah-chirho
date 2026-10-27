// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The CST parser's unit tests, moved from its inline `tests_chirho` module in row 640.

use super::*;
use haskelujah_syntax_chirho::green_chirho::GreenElementChirho;

fn parse_chirho(source_chirho: &str) -> Arc<GreenNodeChirho> {
    parse_to_cst_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
}

fn collect_node_kinds_chirho(root_chirho: &GreenNodeChirho) -> Vec<SyntaxKindChirho> {
    let mut kinds_chirho = Vec::new();
    for child_chirho in root_chirho.children_chirho() {
        if let GreenElementChirho::NodeChirho(n_chirho) = child_chirho {
            kinds_chirho.push(n_chirho.kind_chirho());
            kinds_chirho.extend(collect_node_kinds_chirho(n_chirho));
        }
    }
    kinds_chirho
}

#[test]
fn parse_simple_module_chirho() {
    let root_chirho = parse_chirho("module Main where\nmain = putStrLn \"hi\"\n");
    assert_eq!(
        root_chirho.kind_chirho(),
        SyntaxKindChirho::SourceFileChirho
    );

    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ModuleHeaderChirho),
        "should have ModuleHeader: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
        "should have FunBind: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_import_decl_chirho() {
    let source_chirho = "module M where\nimport Data.List\nimport qualified Data.Map as Map\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    let import_count_chirho = kinds_chirho
        .iter()
        .filter(|k_chirho| **k_chirho == SyntaxKindChirho::ImportDeclChirho)
        .count();
    assert_eq!(import_count_chirho, 2, "should parse 2 imports");
}

#[test]
fn parse_data_decl_chirho() {
    let source_chirho = "module M where\ndata Color = Red | Green | Blue\n  deriving (Show, Eq)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::DataDeclChirho),
        "should have DataDecl"
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::DerivingClauseChirho),
        "should have DerivingClause"
    );
}

#[test]
fn parse_type_sig_with_types_chirho() {
    let source_chirho = "module M where\nfoo :: Int -> String -> Bool\nfoo x y = True\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::TypeSigDeclChirho),
        "should have TypeSigDecl"
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::FunTypeChirho),
        "should have FunType for -> : {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
        "should have FunBind"
    );
}

#[test]
fn parse_class_with_where_chirho() {
    let source_chirho = "module M where\nclass Describable a where\n  describe :: a -> String\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ClassDeclChirho),
        "should have ClassDecl"
    );
}

#[test]
fn parse_script_without_module_chirho() {
    let source_chirho = "main = putStrLn \"hello\"\n";
    let root_chirho = parse_chirho(source_chirho);
    assert_eq!(
        root_chirho.kind_chirho(),
        SyntaxKindChirho::SourceFileChirho
    );

    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::FunBindChirho),
        "should have FunBind even without module header"
    );
}

#[test]
fn text_len_matches_source_chirho() {
    let source_chirho = "module Main where\nmain = putStrLn \"hi\"\n";
    let root_chirho = parse_chirho(source_chirho);
    assert_eq!(
        root_chirho.text_len_chirho(),
        source_chirho.len(),
        "green tree text length should match source length"
    );
}

#[test]
fn parse_imports_with_specs_chirho() {
    let source_chirho = "module M where\nimport Data.Maybe (fromMaybe, isJust)\nimport Prelude hiding (map, filter)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    let import_count_chirho = kinds_chirho
        .iter()
        .filter(|k_chirho| **k_chirho == SyntaxKindChirho::ImportDeclChirho)
        .count();
    assert_eq!(import_count_chirho, 2);
}

#[test]
fn parse_expression_nodes_chirho() {
    let source_chirho = "module M where\nresult = f x + g y\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::AppExprChirho)
            || kinds_chirho.contains(&SyntaxKindChirho::InfixExprChirho),
        "should have application or infix expression nodes: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_if_expression_chirho() {
    let source_chirho = "module M where\nx = if True then 1 else 2\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::IfExprChirho),
        "should have IfExpr: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_lambda_expression_chirho() {
    let source_chirho = "module M where\nf = \\x -> x + 1\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::LambdaExprChirho),
        "should have LambdaExpr: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_lambda_expression_with_bang_pattern_arg_chirho() {
    let source_chirho = "module M where\n{-# LANGUAGE BangPatterns #-}\nf = \\x !y -> x\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::LambdaExprChirho),
        "should have LambdaExpr: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
        "should have BangPat: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_fun_binding_bang_as_pattern_arg_chirho() {
    let source_chirho = "module M where\n{-# LANGUAGE BangPatterns, MagicHash #-}\nimport GHC.Exts (Int(I#))\nf !x !_y@(I# y#) = x\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
        "should have BangPat for strict args: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::AsPatChirho),
        "strict as-pattern arg should retain AsPat: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ConPatChirho),
        "strict as-pattern arg should retain inner constructor pattern: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_case_alt_constructor_bang_subpattern_chirho() {
    let source_chirho = "module MChirho where\n{-# LANGUAGE BangPatterns #-}\nfChirho xChirho = case xChirho of\n  Just !yChirho -> yChirho\n  Nothing -> error \"boom\"\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::CaseAltChirho),
        "strict constructor case alt should parse as a case alternative: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ConPatChirho),
        "strict constructor case alt should retain the constructor pattern: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
        "strict constructor case alt should retain the bang subpattern: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_case_alternative_keeps_a_leading_bang_or_minus_chirho() {
    // An alternative begins with its pattern even when that pattern starts
    // with `!` or `-`; losing the `!` made `!_ -> e` force nothing.
    let root_chirho = parse_chirho(
        "module MChirho where\n{-# LANGUAGE BangPatterns #-}\nfChirho x = case x of { !_ -> 5 }\n",
    );
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
        "the alternative keeps its bang: {kinds_chirho:?}"
    );
    let root_chirho =
        parse_chirho("module MChirho where\nfChirho x = case x of { -1 -> 5; _ -> 6 }\n");
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::NegPatChirho),
        "the alternative keeps its negative literal: {kinds_chirho:?}"
    );
}

#[test]
fn parse_tight_as_pattern_binding_is_a_pattern_binding_chirho() {
    // `whole@(a, b) = e` binds a pattern; it is not a function `whole` with
    // a visible type argument `@(a, b)`.
    let root_chirho = parse_chirho(
        "module MChirho where\nfChirho = let whole@(a, b) = (3, 4) in fst whole + a * b\n",
    );
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::PatBindChirho),
        "a tight as-pattern binding is a pattern binding: {kinds_chirho:?}"
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::AsPatChirho),
        "it keeps its as-pattern: {kinds_chirho:?}"
    );
}

#[test]
fn parse_let_banged_pattern_binding_keeps_its_bang_chirho() {
    // The bang at the head of a binding is what makes it strict. It must
    // survive parsing as a BangPat inside the PatBind, not be skipped.
    for source_chirho in [
        "module MChirho where\n{-# LANGUAGE BangPatterns #-}\nfChirho = let !(aChirho, bChirho) = undefined in 5\n",
        "module MChirho where\n{-# LANGUAGE BangPatterns #-}\nfChirho = 5 where !xChirho = undefined\n",
    ] {
        let root_chirho = parse_chirho(source_chirho);
        let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::PatBindChirho),
            "a banged binding is a pattern binding: {kinds_chirho:?}"
        );
        assert!(
            kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
            "a banged binding keeps its bang: {kinds_chirho:?}"
        );
    }
}

#[test]
fn parse_where_strict_tuple_pattern_binding_as_pat_bind_chirho() {
    let source_chirho = "module MChirho where\nfChirho sChirho = r0Chirho where\n  (!q0Chirho, !r0Chirho) = sChirho `divMod` 10\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::PatBindChirho),
        "strict tuple where binding should parse as PatBindChirho: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ParenPatChirho),
        "strict tuple where binding should keep parenthesized pattern: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::BangPatChirho),
        "strict tuple where binding should keep bang patterns: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_do_expression_chirho() {
    let source_chirho = "module M where\nmain = do\n  putStrLn \"hello\"\n  return ()\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::DoExprChirho),
        "should have DoExpr: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_qualified_do_expression_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE QualifiedDo #-}\n",
        "module M where\n",
        "main = FlowChirho.do\n",
        "  valueChirho <- actionChirho\n",
        "  finishChirho valueChirho\n",
    );
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert_eq!(
        kinds_chirho
            .iter()
            .filter(|kind_chirho| **kind_chirho == SyntaxKindChirho::DoExprChirho)
            .count(),
        1
    );
    assert!(kinds_chirho.contains(&SyntaxKindChirho::BindStmtChirho));
}

#[test]
fn parse_recursive_do_statement_group_chirho() {
    let source_chirho = concat!(
        "{-# LANGUAGE RecursiveDo #-}\n",
        "module M where\n",
        "main = do\n",
        "  rec\n",
        "    xs <- pure (1 : ys)\n",
        "    ys <- pure (2 : xs)\n",
        "  print (take 4 xs)\n",
    );
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(kinds_chirho.contains(&SyntaxKindChirho::DoExprChirho));
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::RecStmtChirho),
        "RecursiveDo should preserve an explicit rec statement group: {:?}",
        kinds_chirho
    );
    assert_eq!(
        kinds_chirho
            .iter()
            .filter(|kind_chirho| **kind_chirho == SyntaxKindChirho::BindStmtChirho)
            .count(),
        2
    );
}

#[test]
fn parse_case_expression_chirho() {
    let source_chirho = "module M where\nf x = case x of\n  True -> 1\n  False -> 0\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::CaseExprChirho),
        "should have CaseExpr: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::CaseAltChirho),
        "should have CaseAlt: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_list_and_tuple_chirho() {
    let source_chirho = "module M where\nxs = [1, 2, 3]\np = (1, True)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ListExprChirho),
        "should have ListExpr: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::ParenExprChirho),
        "should have ParenExpr (tuple): {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_type_alias_with_type_nodes_chirho() {
    let source_chirho = "module M where\ntype Name = String\ntype Pair a b = (a, b)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    let alias_count_chirho = kinds_chirho
        .iter()
        .filter(|k_chirho| **k_chirho == SyntaxKindChirho::TypeAliasDeclChirho)
        .count();
    assert_eq!(alias_count_chirho, 2, "should parse 2 type aliases");
}

#[test]
fn parse_where_clause_chirho() {
    let source_chirho = "module M where\nf x = y + z\n  where\n    y = x + 1\n    z = x * 2\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);

    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::WhereClauseChirho),
        "should have WhereClause: {:?}",
        kinds_chirho
    );
}

#[test]
fn roundtrip_text_len_various_chirho() {
    let sources_chirho = [
        "main = putStrLn \"hello\"\n",
        "module M where\nf :: Int -> Int\nf x = x + 1\n",
        "module M where\nimport Data.List\ndata T = A | B\n",
        "x = if True then 1 else 2\n",
        "f = \\x y -> x + y\n",
        "main = do\n  putStrLn \"hi\"\n  return 42\n",
    ];
    for src_chirho in &sources_chirho {
        let root_chirho = parse_chirho(src_chirho);
        assert_eq!(
            root_chirho.text_len_chirho(),
            src_chirho.len(),
            "text_len mismatch for: {}",
            src_chirho.trim()
        );
    }
}

#[test]
fn parse_open_type_family_chirho() {
    let source_chirho = "module M where\ntype family F a :: Type\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyDeclChirho),
        "should have TypeFamilyDecl: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_closed_type_family_chirho() {
    let source_chirho = "module M where\ntype family F a where\n  F Int = Bool\n  F Char = Int\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyDeclChirho),
        "should have TypeFamilyDecl: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_type_family_instance_chirho() {
    let source_chirho = "module M where\ntype instance F Int = Bool\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::TypeFamilyInstanceDeclChirho),
        "should have TypeFamilyInstanceDecl: {:?}",
        kinds_chirho
    );
}

// -------------------------------------------------------------------
// Template Haskell parsing tests
// -------------------------------------------------------------------

#[test]
fn parse_th_splice_name_expr_chirho() {
    // $name as expression inside a binding
    let source_chirho = "module M where\nx = $foo\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::SpliceExprChirho),
        "should have SpliceExpr for $foo: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_splice_parens_expr_chirho() {
    // $(expr) as expression inside a binding
    let source_chirho = "module M where\nx = $(makeLenses foo)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::SpliceExprChirho),
        "should have SpliceExpr for $(makeLenses foo): {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_typed_splice_expr_chirho() {
    // $$name as expression
    let source_chirho = "module M where\nx = $$foo\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::TypedSpliceExprChirho),
        "should have TypedSpliceExpr for $$foo: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_splice_decl_chirho() {
    // $(expr) at top level parsed as SpliceDeclChirho
    let source_chirho = "module M where\n$(makeLenses foo)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::SpliceDeclChirho),
        "should have SpliceDecl for top-level $(makeLenses foo): {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_expr_quote_chirho() {
    // [| expr |] expression quotation
    let source_chirho = "module M where\nx = [| foo |]\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::QuoteExprChirho),
        "should have QuoteExpr for [| foo |]: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_decl_quote_chirho() {
    // [d| decls |] declaration quotation
    let source_chirho = "module M where\nx = [d| y = 1 |]\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::QuoteDeclChirho),
        "should have QuoteDecl for [d| y = 1 |]: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_type_quote_chirho() {
    // [t| type |] type quotation
    let source_chirho = "module M where\nx = [t| Int |]\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::QuoteTypeChirho),
        "should have QuoteType for [t| Int |]: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_pat_quote_chirho() {
    // [p| pat |] pattern quotation
    let source_chirho = "module M where\nx = [p| y |]\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::QuotePatChirho),
        "should have QuotePat for [p| y |]: {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_splice_decl_make_lenses_chirho() {
    // $(makeLenses ''Foo) — typical TH usage at top level
    let source_chirho = "module M where\ndata Foo = Foo\n$(makeLenses ''Foo)\n";
    let root_chirho = parse_chirho(source_chirho);
    let kinds_chirho = collect_node_kinds_chirho(&root_chirho);
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::DataDeclChirho),
        "should have DataDecl: {:?}",
        kinds_chirho
    );
    assert!(
        kinds_chirho.contains(&SyntaxKindChirho::SpliceDeclChirho),
        "should have SpliceDecl for $(makeLenses ''Foo): {:?}",
        kinds_chirho
    );
}

#[test]
fn parse_th_text_len_matches_chirho() {
    // Green tree text length must match source length for TH sources
    let source_chirho = "module M where\nx = $foo\n";
    let root_chirho = parse_chirho(source_chirho);
    assert_eq!(
        root_chirho.text_len_chirho(),
        source_chirho.len(),
        "TH green tree text length should match source length"
    );
}
