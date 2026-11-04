// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type families and aliases, `class` and `instance` declarations, fixity, standalone deriving,
//! `default`, pattern synonyms and foreign declarations.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Type alias declarations
    // -----------------------------------------------------------------------

    /// Dispatch `type` keyword: check if followed by `family` or `instance`. An
    /// instance body's own `type` member is an associated instance in either
    /// spelling, parsed by `parse_associated_type_instance_chirho`.
    pub(super) fn parse_type_or_family_decl_chirho(&mut self, instance_member_chirho: bool) {
        // Peek ahead past trivia to see if next token is "family" or "instance"
        let mut look_chirho = self.pos_chirho + 1;
        while look_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[look_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            look_chirho += 1;
        }
        let next_text_chirho = if look_chirho < self.tokens_chirho.len() {
            self.token_text_chirho(&self.tokens_chirho[look_chirho])
        } else {
            ""
        };
        match next_text_chirho {
            "family" => self.parse_type_family_decl_chirho(),
            "instance" if instance_member_chirho => self.parse_associated_type_instance_chirho(),
            "instance" => self.parse_type_family_instance_decl_chirho(),
            "role" => {
                // RoleAnnotations: `type role T nominal phantom representational`
                // Just consume the entire declaration
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);
                self.eat_until_decl_end_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // StandaloneKindSignatures: `type T :: Kind`
                // Detect pattern: `type ConId ::` or `type (op) ::` (no `=` before `::`)
                // by scanning ahead for `::` before `=` or decl boundary.
                let mut scan_chirho = look_chirho;
                let mut is_kind_sig_chirho = false;
                // Skip past the declared name, a ConId or `(op)`, to check for ::
                if scan_chirho < self.tokens_chirho.len() {
                    scan_chirho = self.past_declared_type_name_chirho(scan_chirho);
                    if scan_chirho < self.tokens_chirho.len()
                        && self.tokens_chirho[scan_chirho].kind_chirho
                            == RawTokenKindChirho::ColonColonChirho
                    {
                        is_kind_sig_chirho = true;
                    }
                }
                if is_kind_sig_chirho {
                    // Standalone kind signature — consume as TypeSigDecl (skipped)
                    self.builder_chirho
                        .start_node_chirho(SyntaxKindChirho::TypeSigDeclChirho);
                    self.eat_until_decl_end_chirho();
                    self.builder_chirho.finish_node_chirho();
                } else if instance_member_chirho {
                    self.parse_associated_type_instance_chirho();
                } else {
                    self.parse_type_alias_decl_chirho();
                }
            }
        }
    }

    /// The index after a declared type's name and the trivia that follows it.
    /// The name is one token, or an operator in parentheses: `type (:==) :: k`
    /// is a standalone kind signature, not a synonym (row 686, T15079, SplitWD).
    /// workflow: language-features-chirho/declaration-kinds-chirho
    fn past_declared_type_name_chirho(&self, name_chirho: usize) -> usize {
        let kind_at_chirho = |index_chirho: usize| {
            self.tokens_chirho
                .get(index_chirho)
                .map(|token_chirho| token_chirho.kind_chirho)
        };
        if kind_at_chirho(name_chirho) == Some(RawTokenKindChirho::LeftParenChirho) {
            let operator_chirho = self.past_raw_trivia_chirho(name_chirho + 1);
            let close_chirho = self.past_raw_trivia_chirho(operator_chirho + 1);
            if matches!(
                kind_at_chirho(operator_chirho),
                Some(RawTokenKindChirho::VarSymChirho | RawTokenKindChirho::ConSymChirho)
            ) && kind_at_chirho(close_chirho) == Some(RawTokenKindChirho::RightParenChirho)
            {
                return self.past_raw_trivia_chirho(close_chirho + 1);
            }
        }
        self.past_raw_trivia_chirho(name_chirho + 1)
    }

    /// The first index at or after `index_chirho` that is not trivia.
    fn past_raw_trivia_chirho(&self, mut index_chirho: usize) -> usize {
        while index_chirho < self.tokens_chirho.len()
            && matches!(
                self.tokens_chirho[index_chirho].kind_chirho,
                RawTokenKindChirho::WhitespaceChirho
                    | RawTokenKindChirho::LineCommentChirho
                    | RawTokenKindChirho::BlockCommentChirho
            )
        {
            index_chirho += 1;
        }
        index_chirho
    }

    pub(super) fn parse_type_family_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyDeclChirho);

        if self.at_chirho(RawTokenKindChirho::TypeChirho) {
            self.bump_chirho(); // type
        } else {
            self.expect_chirho(RawTokenKindChirho::DataChirho); // data
        }
        self.eat_trivia_chirho();
        self.bump_chirho(); // family
        self.eat_trivia_chirho();

        // Family name and type variables until `where`, `::`, or end of decl
        self.eat_until_any_top_level_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::ColonColonChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        // Optional result kind annotation `:: *`
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // kind
        }

        // Check for `where` (closed type family)
        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.bump_chirho(); // where
            self.eat_trivia_chirho();
            // Parse equations: each is `[forall bs.] lhs = rhs`
            if self.at_chirho(RawTokenKindChirho::VirtualLeftBraceChirho)
                || self.at_chirho(RawTokenKindChirho::LeftBraceChirho)
            {
                self.bump_chirho();
            }
            while !self.at_eof_chirho()
                && !self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                && !self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.eat_trivia_chirho();
                if self.at_chirho(RawTokenKindChirho::VirtualSemicolonChirho)
                    || self.at_chirho(RawTokenKindChirho::SemicolonChirho)
                {
                    self.bump_chirho();
                    continue;
                }
                if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                    || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
                {
                    break;
                }
                self.parse_family_equation_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::VirtualRightBraceChirho)
                || self.at_chirho(RawTokenKindChirho::RightBraceChirho)
            {
                self.bump_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a declaration's right side with its optional kind annotation
    /// (`F a = Int :: Type`), kept as one kind-annotated type. A `::` that ends the
    /// declaration stays a bare token in its node, so an incomplete family equation
    /// remains incomplete, as GHC rejects it (row 686, KindSigs).
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    fn parse_type_with_optional_kind_chirho(&mut self) {
        let rhs_start_chirho = self.pos_chirho;
        let rhs_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_type_chirho();
        if self.pos_chirho == rhs_start_chirho
            || self.next_non_trivia_kind_chirho(0) != Some(RawTokenKindChirho::ColonColonChirho)
        {
            return;
        }
        self.eat_trivia_chirho();
        let kind_follows_chirho = !matches!(
            self.next_non_trivia_kind_chirho(1),
            None | Some(
                RawTokenKindChirho::VirtualSemicolonChirho
                    | RawTokenKindChirho::SemicolonChirho
                    | RawTokenKindChirho::VirtualRightBraceChirho
                    | RawTokenKindChirho::RightBraceChirho
                    | RawTokenKindChirho::EofChirho
            )
        );
        if !kind_follows_chirho {
            self.bump_chirho(); // a `::` with no kind
            return;
        }
        self.builder_chirho
            .start_node_at_chirho(rhs_chirho, SyntaxKindChirho::KindAnnotTypeChirho);
        self.bump_chirho(); // ::
        self.eat_trivia_chirho();
        self.parse_type_chirho(); // the kind
        self.builder_chirho.finish_node_chirho();
    }

    /// The kind of the non-trivia token `skip_chirho` places past the next one at or after
    /// the cursor: 0 is the next non-trivia token itself.
    fn next_non_trivia_kind_chirho(&self, skip_chirho: usize) -> Option<RawTokenKindChirho> {
        self.tokens_chirho[self.pos_chirho.min(self.tokens_chirho.len())..]
            .iter()
            .map(|token_chirho| token_chirho.kind_chirho)
            .filter(|kind_chirho| !kind_chirho.is_trivia_chirho())
            .nth(skip_chirho)
    }

    /// Parse a closed equation's left side as one type. A continuation line
    /// (`F a⏎    b = …`) is still this left side: layout already separates the
    /// equations, so the value-binding guard in `parse_btype_chirho` stays off.
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    fn parse_family_equation_lhs_chirho(&mut self) {
        let outer_chirho = std::mem::replace(&mut self.family_equation_lhs_chirho, true);
        self.parse_type_chirho();
        self.family_equation_lhs_chirho = outer_chirho;
    }

    /// One equation node, `[forall bs.] lhs = rhs`, for a closed family's `where`
    /// block and an instance body's associated instance alike. The left side is a
    /// type; whatever that grammar leaves before `=` stays in the node, so lowering
    /// reports an incomplete equation instead of dropping it.
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    fn parse_family_equation_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyEquationChirho);
        self.parse_family_equation_lhs_chirho();
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::SemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
            RawTokenKindChirho::RightBraceChirho,
        ]);
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_with_optional_kind_chirho();
        }
        self.builder_chirho.finish_node_chirho();
    }

    /// An instance body's associated type instance, `type F args = rhs` or
    /// `type instance F args = rhs`. Its equation is parsed like a closed family's,
    /// so lowering reads the written arguments in order: wildcards, and arguments
    /// that are not class parameters, keep their places (row 686: V03, T11401).
    /// workflow: compiler-pipeline-chirho/declaration-validity-chirho
    fn parse_associated_type_instance_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyInstanceDeclChirho);
        self.expect_chirho(RawTokenKindChirho::TypeChirho); // type
        self.eat_trivia_chirho();
        if self.at_chirho(RawTokenKindChirho::InstanceChirho) {
            self.bump_chirho(); // instance
            self.eat_trivia_chirho();
        }
        self.parse_family_equation_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_family_instance_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeFamilyInstanceDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho); // type
        self.eat_trivia_chirho();
        self.bump_chirho(); // instance
        self.eat_trivia_chirho();

        // LHS patterns until =
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // RHS type
        }

        self.builder_chirho.finish_node_chirho();
    }

    fn parse_type_alias_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::TypeAliasDeclChirho);

        self.expect_chirho(RawTokenKindChirho::TypeChirho);
        self.eat_trivia_chirho();

        // Type name and variables until =
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_type_with_optional_kind_chirho();
            // `= r | r -> a` is an injectivity annotation. It belongs to this
            // declaration, an associated family's result binder, never to the
            // class body's next member (row 686: T6018, T26255a).
            if self.next_non_trivia_kind_chirho(0) == Some(RawTokenKindChirho::PipeChirho) {
                self.eat_trivia_chirho();
                self.eat_until_any_chirho(&[
                    RawTokenKindChirho::VirtualSemicolonChirho,
                    RawTokenKindChirho::SemicolonChirho,
                    RawTokenKindChirho::VirtualRightBraceChirho,
                    RawTokenKindChirho::RightBraceChirho,
                ]);
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Class / Instance declarations
    // -----------------------------------------------------------------------

    pub(super) fn parse_class_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ClassDeclChirho);

        self.expect_chirho(RawTokenKindChirho::ClassChirho);
        self.eat_trivia_chirho();

        // Eat until where or end of decl
        self.eat_until_any_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            self.parse_where_block_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    pub(super) fn parse_instance_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::InstanceDeclChirho);

        self.expect_chirho(RawTokenKindChirho::InstanceChirho);
        self.eat_trivia_chirho();

        self.eat_until_any_chirho(&[
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::VirtualSemicolonChirho,
            RawTokenKindChirho::VirtualRightBraceChirho,
        ]);

        if self.at_chirho(RawTokenKindChirho::WhereChirho) {
            // The block's own members are instance members; see `parse_decl_chirho`.
            let outer_chirho = std::mem::replace(&mut self.instance_body_chirho, true);
            self.parse_where_block_chirho();
            self.instance_body_chirho = outer_chirho;
        }

        self.builder_chirho.finish_node_chirho();
    }

    // -----------------------------------------------------------------------
    // Fixity / Default / Foreign declarations
    // -----------------------------------------------------------------------

    pub(super) fn parse_fixity_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::FixityDeclChirho);

        // infix | infixl | infixr
        self.bump_chirho();
        self.eat_trivia_chirho();

        self.eat_until_decl_end_chirho();

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse: `deriving instance [context =>] ClassName Type`
    pub(super) fn parse_standalone_deriving_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::StandaloneDerivingDeclChirho);
        self.bump_chirho(); // deriving
        self.eat_trivia_chirho();

        // Expect "instance"
        if self.at_chirho(RawTokenKindChirho::InstanceChirho) {
            self.bump_chirho(); // instance
            self.eat_trivia_chirho();
        }

        // Consume the rest of the declaration (context => Class Type)
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    pub(super) fn parse_default_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::DefaultDeclChirho);
        self.bump_chirho(); // default
        self.eat_trivia_chirho();
        self.eat_until_decl_end_chirho();
        self.builder_chirho.finish_node_chirho();
    }

    /// Parse: foreign (import|export) callconv [safety] [string] name :: type
    /// Parse a pattern synonym declaration:
    /// `pattern ConName args = pat`   (implicitly bidirectional)
    /// `pattern ConName args <- pat`  (unidirectional)
    /// `pattern ConName args <- pat where ConName args = expr`  (explicitly bidirectional)
    pub(super) fn parse_pat_syn_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::PatSynDeclChirho);

        self.bump_chirho(); // "pattern" (VarId)
        self.eat_trivia_chirho();

        // Pattern synonym name (ConId)
        if self.at_chirho(RawTokenKindChirho::ConIdChirho) {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

        // Pattern variables (VarIds before = or <-)
        while self.at_chirho(RawTokenKindChirho::VarIdChirho) && !self.at_varid_text_chirho("where")
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Direction: = (implicitly bidirectional) or <- (unidirectional/explicitly bidirectional)
        if self.at_chirho(RawTokenKindChirho::EqualsChirho) {
            self.bump_chirho(); // =
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
        } else if self.at_chirho(RawTokenKindChirho::LeftArrowChirho) {
            self.bump_chirho(); // <-
            self.eat_trivia_chirho();
            self.parse_pat_chirho();
            // Check for explicitly bidirectional where clause
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::WhereChirho) {
                self.parse_where_block_chirho();
            }
        }

        self.builder_chirho.finish_node_chirho();
    }

    pub(super) fn parse_foreign_decl_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ForeignDeclChirho);
        self.bump_chirho(); // foreign
        self.eat_trivia_chirho();

        // import or export
        if self.at_chirho(RawTokenKindChirho::ImportChirho) || self.at_varid_text_chirho("export") {
            self.bump_chirho();
        }
        self.eat_trivia_chirho();

        // calling convention: ccall, capi, stdcall, prim, javascript (all VarId tokens)
        if self.at_chirho(RawTokenKindChirho::VarIdChirho) {
            let text_chirho = self.current_text_chirho();
            if matches!(
                text_chirho,
                "ccall" | "capi" | "stdcall" | "prim" | "javascript" | "cplusplus"
            ) {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
        }

        // optional safety: safe, unsafe, interruptible
        if self.at_varid_text_chirho("safe")
            || self.at_varid_text_chirho("unsafe")
            || self.at_varid_text_chirho("interruptible")
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // optional C foreign name as string literal
        if self.at_chirho(RawTokenKindChirho::StringLitChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // Haskell name (VarId or ConId or operator in parens)
        if self.at_chirho(RawTokenKindChirho::VarIdChirho)
            || self.at_chirho(RawTokenKindChirho::ConIdChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
        }

        // :: and type signature
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho();
            self.eat_trivia_chirho();
            // Parse the rest as individual tokens until decl end
            self.eat_until_decl_end_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Consume an unsupported data/newtype-family instance without swallowing
    /// a sibling declaration after a GADT-style `where` layout block.
    pub(super) fn eat_until_unrepresented_instance_end_chirho(&mut self) {
        let mut paren_depth_chirho = 0usize;
        let mut bracket_depth_chirho = 0usize;
        let mut brace_depth_chirho = 0usize;
        let mut saw_top_level_where_chirho = false;
        let mut where_block_depth_chirho = None;

        while let Some(kind_chirho) = self.current_kind_chirho() {
            if kind_chirho == RawTokenKindChirho::EofChirho {
                break;
            }
            let at_top_level_chirho =
                paren_depth_chirho == 0 && bracket_depth_chirho == 0 && brace_depth_chirho == 0;
            if at_top_level_chirho
                && matches!(
                    kind_chirho,
                    RawTokenKindChirho::VirtualSemicolonChirho
                        | RawTokenKindChirho::VirtualRightBraceChirho
                        | RawTokenKindChirho::SemicolonChirho
                        | RawTokenKindChirho::RightBraceChirho
                )
            {
                break;
            }

            match kind_chirho {
                RawTokenKindChirho::WhereChirho if at_top_level_chirho => {
                    saw_top_level_where_chirho = true;
                }
                RawTokenKindChirho::LeftParenChirho => paren_depth_chirho += 1,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho = paren_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBracketChirho => bracket_depth_chirho += 1,
                RawTokenKindChirho::RightBracketChirho => {
                    bracket_depth_chirho = bracket_depth_chirho.saturating_sub(1);
                }
                RawTokenKindChirho::LeftBraceChirho
                | RawTokenKindChirho::VirtualLeftBraceChirho => {
                    brace_depth_chirho += 1;
                    if saw_top_level_where_chirho && where_block_depth_chirho.is_none() {
                        where_block_depth_chirho = Some(brace_depth_chirho);
                    }
                }
                RawTokenKindChirho::RightBraceChirho
                | RawTokenKindChirho::VirtualRightBraceChirho => {
                    let closing_where_block_chirho = where_block_depth_chirho
                        .is_some_and(|depth_chirho| depth_chirho == brace_depth_chirho);
                    brace_depth_chirho = brace_depth_chirho.saturating_sub(1);
                    self.bump_chirho();
                    if closing_where_block_chirho {
                        break;
                    }
                    continue;
                }
                _ => {}
            }
            self.bump_chirho();
        }
    }
}
