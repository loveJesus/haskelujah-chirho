// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Types: type operators, application, atoms, promoted types and kind-annotated binders.
//! Part of the CST parser (`cst_parser_chirho.rs`), split by syntactic category in row 640.

use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;

use super::ParserChirho;
use crate::lexer_chirho::RawTokenKindChirho;

impl ParserChirho<'_> {
    // -----------------------------------------------------------------------
    // Type parsing
    // -----------------------------------------------------------------------

    /// Parse a type. Handles qualified types (context => type) and function
    /// types (a -> b). Uses checkpoints for retroactive wrapping.
    pub(super) fn parse_type_chirho(&mut self) {
        // forall a b . Type
        if self.at_chirho(RawTokenKindChirho::ForallChirho) {
            self.builder_chirho
                .start_node_chirho(SyntaxKindChirho::ForallTypeChirho);
            self.bump_chirho(); // forall
            self.eat_trivia_chirho();
            // Collect type variables until `.` (invisible forall) or `->`
            // (RequiredTypeArguments).
            // Accepts bare VarId and kind-annotated (VarId :: Kind) binders
            // where the kind is a simple kind (*, k, * -> *, etc.)
            while !self.at_dot_chirho()
                && !self.at_chirho(RawTokenKindChirho::RightArrowChirho)
                && !self.at_eof_chirho()
                && !self.at_decl_boundary_chirho()
            {
                if self.at_chirho(RawTokenKindChirho::VarIdChirho) {
                    self.bump_chirho();
                } else if self.at_chirho(RawTokenKindChirho::LeftParenChirho) {
                    // Only parse as a kind-annotated binder if we can
                    // confirm the pattern: ( VarId :: SimpleKind )
                    // Use a lookahead check to avoid consuming complex
                    // type expressions that aren't simple kind annotations.
                    if self.is_simple_kind_annotated_binder_chirho() {
                        self.parse_paren_type_chirho();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                self.eat_trivia_chirho();
            }
            if self.at_dot_chirho() {
                self.bump_chirho(); // .
                self.eat_trivia_chirho();
            } else if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // required-forall ->
                self.eat_trivia_chirho();
            }
            self.parse_type_chirho(); // body type
            self.builder_chirho.finish_node_chirho();
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();

        self.parse_btype_chirho();
        self.eat_trivia_chirho();

        // Check for => (qualified type) — but only if we haven't gone
        // past the declaration boundary.
        if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
            // Retroactively wrap the parsed btype as a context
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
            self.bump_chirho(); // =>
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the actual type
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for ⊸ (linear arrow — LinearTypes)
        if self.at_chirho(RawTokenKindChirho::LinearArrowChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // ⊸
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for %1 -> or %Many -> or %m -> (multiplicity annotation — LinearTypes)
        if self.at_varsym_chirho("%") {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // %
            self.eat_trivia_chirho();
            // Consume multiplicity: integer 1, conid Many/One, or varid (poly)
            if self.at_chirho(RawTokenKindChirho::IntLitChirho)
                || self.at_chirho(RawTokenKindChirho::ConIdChirho)
                || self.at_chirho(RawTokenKindChirho::VarIdChirho)
            {
                self.bump_chirho(); // multiplicity token
                self.eat_trivia_chirho();
            }
            // Expect ->
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
            }
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Check for -> (function type)
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
            self.bump_chirho(); // ->
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // right-recursive
            self.builder_chirho.finish_node_chirho();
        }
        // Type equality constraint: a ~ b (parsed as infix type)
        if self.at_chirho(RawTokenKindChirho::TildeChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_chirho(); // ~
            self.eat_trivia_chirho();
            self.parse_type_operator_rhs_chirho(); // right operand without consuming -> / =>
            self.builder_chirho.finish_node_chirho();
            // After the infix ~ type, check if this is part of a qualified type
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            return;
        }
        // TypeOperators: check for infix operator in type position (e.g. `a :+: b`)
        // VarSym or ConSym that isn't a special symbol like %, !, @
        // Use btype for the right operand so that `->` binds looser than
        // type operators: `a :~: b -> ()` parses as `(a :~: b) -> ()`.
        if self.at_type_operator_chirho() {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_type_operator_chirho();
            self.eat_trivia_chirho();
            self.parse_type_operator_rhs_chirho(); // right operand without consuming -> / =>
            self.builder_chirho.finish_node_chirho();
            // After the infix type, check for -> / => to continue
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            } else if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            return;
        }
        // Backtick infix type constructors: a `Either` b
        if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_chirho(); // `
            self.eat_trivia_chirho();
            // Consume the type constructor name
            if self.at_chirho(RawTokenKindChirho::ConIdChirho)
                || self.at_chirho(RawTokenKindChirho::VarIdChirho)
                || self.at_chirho(RawTokenKindChirho::QualifiedIdChirho)
            {
                self.bump_chirho();
                self.eat_trivia_chirho();
            }
            if self.at_chirho(RawTokenKindChirho::BacktickChirho) {
                self.bump_chirho(); // closing `
                self.eat_trivia_chirho();
            }
            self.parse_type_operator_rhs_chirho(); // right operand without consuming -> / =>
            self.builder_chirho.finish_node_chirho();
            // After backtick infix, check for -> / =>
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::FunTypeChirho);
                self.bump_chirho(); // ->
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            } else if self.at_chirho(RawTokenKindChirho::FatArrowChirho) {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::QualTypeChirho);
                self.bump_chirho(); // =>
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.builder_chirho.finish_node_chirho();
            }
        }
        // Otherwise, just the btype stands as-is (no wrapping needed).
    }

    fn at_type_operator_chirho(&self) -> bool {
        self.at_chirho(RawTokenKindChirho::TildeChirho)
            || self.at_promoted_constructor_operator_chirho()
            || ((self.at_chirho(RawTokenKindChirho::VarSymChirho)
                || self.at_chirho(RawTokenKindChirho::ConSymChirho))
                && !matches!(self.current_text_chirho(), "%" | "!" | "@" | "|"))
    }

    fn at_promoted_constructor_operator_chirho(&self) -> bool {
        self.at_chirho(RawTokenKindChirho::TickChirho)
            && self.peek_after_tick_chirho() == Some(RawTokenKindChirho::ConSymChirho)
    }

    fn bump_type_operator_chirho(&mut self) {
        if self.at_promoted_constructor_operator_chirho() {
            self.bump_chirho(); // tick
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::ConSymChirho) {
                self.bump_chirho(); // promoted constructor symbol
            }
        } else {
            self.bump_chirho();
        }
    }

    fn parse_type_operator_rhs_chirho(&mut self) {
        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_btype_chirho();
        self.eat_trivia_chirho();

        if self.at_type_operator_chirho() {
            self.builder_chirho
                .start_node_at_chirho(cp_chirho, SyntaxKindChirho::InfixTypeChirho);
            self.bump_type_operator_chirho();
            self.eat_trivia_chirho();
            self.parse_type_operator_rhs_chirho();
            self.builder_chirho.finish_node_chirho();
        }
    }

    /// Parse a "btype" — type application (juxtaposition of atomic types).
    fn parse_btype_chirho(&mut self) {
        if !self.can_start_atype_chirho() {
            return;
        }

        let cp_chirho = self.builder_chirho.checkpoint_chirho();
        self.parse_atype_chirho();
        self.eat_trivia_chirho();

        let mut count_chirho = 1u32;
        while self.can_start_atype_chirho() || self.at_visible_type_application_chirho() {
            if self.at_promoted_constructor_operator_chirho() {
                break;
            }
            if self.current_starts_unseparated_value_decl_chirho() {
                break;
            }
            let before_chirho = self.pos_chirho;
            if count_chirho == 1 {
                self.builder_chirho
                    .start_node_at_chirho(cp_chirho, SyntaxKindChirho::AppTypeChirho);
            }
            count_chirho += 1;
            if self.at_visible_type_application_chirho() {
                self.bump_chirho(); // @
                self.eat_trivia_chirho();
                self.parse_atype_chirho();
            } else {
                self.parse_atype_chirho();
            }
            self.eat_trivia_chirho();
            if self.pos_chirho == before_chirho {
                break;
            }
        }

        if count_chirho > 1 {
            self.builder_chirho.finish_node_chirho(); // AppType
        }
    }

    /// Parse an atomic type: variable, constructor, parenthesized, tuple,
    /// list, or unit.
    pub(super) fn parse_atype_chirho(&mut self) {
        match self.current_kind_chirho() {
            Some(RawTokenKindChirho::VarIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::VarTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::ConIdChirho) | Some(RawTokenKindChirho::QualifiedIdChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ConTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                self.parse_paren_type_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                self.parse_list_type_chirho();
            }
            // DataKinds: 'Constructor or '[Type, ...] promoted types
            Some(RawTokenKindChirho::TickChirho) => {
                self.parse_promoted_type_chirho();
            }
            // PartialTypeSignatures: `_` in type position is a wildcard type
            Some(RawTokenKindChirho::UnderscoreChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::WildcardTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            // DataKinds: type-level numeric/string/char literals
            Some(RawTokenKindChirho::IntLitChirho)
            | Some(RawTokenKindChirho::StringLitChirho)
            | Some(RawTokenKindChirho::CharLitChirho) => {
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::LitTypeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Unexpected — wrap in error node
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                if !self.at_eof_chirho() {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Parse a parenthesized, tuple, or function-type-constructor type.
    pub(super) fn parse_paren_type_chirho(&mut self) {
        // Could be: (type), (type, type, ...), (->), ()
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ParenTypeChirho);

        self.bump_chirho(); // (
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            // Unit type ()
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        // Could be (->) or operator as type constructor
        if self.at_chirho(RawTokenKindChirho::RightArrowChirho)
            || self.at_chirho(RawTokenKindChirho::VarSymChirho)
            || self.at_chirho(RawTokenKindChirho::ConSymChirho)
        {
            self.bump_chirho();
            self.eat_trivia_chirho();
            if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
                return;
            }
        }

        // Parse first type
        self.parse_type_chirho();
        self.eat_trivia_chirho();

        // Kind annotation: (a :: k) — consume :: and the kind type
        if self.at_chirho(RawTokenKindChirho::ColonColonChirho) {
            self.bump_chirho(); // ::
            self.eat_trivia_chirho();
            self.parse_type_chirho(); // the kind
            self.eat_trivia_chirho();
        }

        if self.at_chirho(RawTokenKindChirho::CommaChirho) {
            // It's a tuple type — change the node kind would be ideal,
            // but we just keep ParenType and it wraps the contents.
            while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                self.bump_chirho(); // ,
                self.eat_trivia_chirho();
                self.parse_type_chirho();
                self.eat_trivia_chirho();
            }
        }

        if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a list type: [type]
    fn parse_list_type_chirho(&mut self) {
        self.builder_chirho
            .start_node_chirho(SyntaxKindChirho::ListTypeChirho);

        self.bump_chirho(); // [
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            // [] — empty list type constructor
            self.bump_chirho();
            self.builder_chirho.finish_node_chirho();
            return;
        }

        self.parse_type_chirho();
        self.eat_trivia_chirho();

        if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
            self.bump_chirho();
        }

        self.builder_chirho.finish_node_chirho();
    }

    /// Parse a DataKinds promoted type: `'Constructor` or `'[Type, ...]`.
    fn parse_promoted_type_chirho(&mut self) {
        match self.peek_after_tick_chirho() {
            Some(RawTokenKindChirho::ConIdChirho)
            | Some(RawTokenKindChirho::QualifiedIdChirho)
            | Some(RawTokenKindChirho::ConSymChirho) => {
                // Promoted constructor: 'True, 'Just, 'Nothing
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // ConId or constructor symbol
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftBracketChirho) => {
                // Promoted list type: '[], '[Int, Bool]
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedListTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // [
                self.eat_trivia_chirho();

                if !self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.parse_type_chirho();
                    self.eat_trivia_chirho();
                    while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                        self.bump_chirho(); // ,
                        self.eat_trivia_chirho();
                        self.parse_type_chirho();
                        self.eat_trivia_chirho();
                    }
                }

                if self.at_chirho(RawTokenKindChirho::RightBracketChirho) {
                    self.bump_chirho(); // ]
                }
                self.builder_chirho.finish_node_chirho();
            }
            Some(RawTokenKindChirho::LeftParenChirho) => {
                // Promoted tuple or unit: '(), '(,), '(,,)
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::PromotedConTypeChirho);
                self.bump_chirho(); // tick
                self.bump_chirho(); // (
                self.eat_trivia_chirho();
                // Eat commas for promoted tuple constructors
                while self.at_chirho(RawTokenKindChirho::CommaChirho) {
                    self.bump_chirho();
                    self.eat_trivia_chirho();
                }
                if self.at_chirho(RawTokenKindChirho::RightParenChirho) {
                    self.bump_chirho();
                }
                self.builder_chirho.finish_node_chirho();
            }
            _ => {
                // Fallback: just consume the tick as an error
                self.builder_chirho
                    .start_node_chirho(SyntaxKindChirho::ErrorNodeChirho);
                self.bump_chirho();
                self.builder_chirho.finish_node_chirho();
            }
        }
    }

    /// Check if the next non-trivia token after current is a ConId.
    /// Used to distinguish `pattern ConName ...` from a variable named `pattern`.
    pub(super) fn peek_next_is_conid_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if kind_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            return kind_chirho == RawTokenKindChirho::ConIdChirho;
        }
        false
    }

    /// Peek at the token kind after the current tick, skipping trivia.
    fn peek_after_tick_chirho(&self) -> Option<RawTokenKindChirho> {
        let mut i_chirho = self.pos_chirho + 1;
        while i_chirho < self.tokens_chirho.len() {
            let kind_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if !kind_chirho.is_trivia_chirho() {
                return Some(kind_chirho);
            }
            i_chirho += 1;
        }
        None
    }

    /// Lookahead to check if the current `(` starts a simple kind-annotated
    /// binder suitable for forall: `( VarId :: SimpleKind )` where SimpleKind
    /// is `*`, `Type`, `Constraint`, a kind variable, or arrow kinds thereof.
    /// Returns false for complex kinds like `Either x y` or `forall k. k -> Type`.
    pub(super) fn is_simple_kind_annotated_binder_chirho(&self) -> bool {
        let mut i_chirho = self.pos_chirho + 1; // skip `(`
        // Skip trivia after `(`
        while i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
        {
            i_chirho += 1;
        }
        // Must be VarId
        if i_chirho >= self.tokens_chirho.len()
            || self.tokens_chirho[i_chirho].kind_chirho != RawTokenKindChirho::VarIdChirho
        {
            return false;
        }
        i_chirho += 1;
        // Skip trivia
        while i_chirho < self.tokens_chirho.len()
            && self.tokens_chirho[i_chirho].kind_chirho.is_trivia_chirho()
        {
            i_chirho += 1;
        }
        // Must be ::
        if i_chirho >= self.tokens_chirho.len()
            || self.tokens_chirho[i_chirho].kind_chirho != RawTokenKindChirho::ColonColonChirho
        {
            return false;
        }
        i_chirho += 1;
        // Scan ahead for `)` — only simple kind tokens allowed:
        // *, Type, Constraint, VarId (kind var), ConId (named kind), ->
        // Must find `)` before any complex syntax (forall, application).
        // Consecutive name tokens (VarId/ConId) without `->` indicate type
        // application (e.g. `Either x y`), not a simple kind.
        let mut paren_depth_chirho: u32 = 0;
        let mut last_was_name_chirho = false;
        while i_chirho < self.tokens_chirho.len() {
            let k_chirho = self.tokens_chirho[i_chirho].kind_chirho;
            if k_chirho.is_trivia_chirho() {
                i_chirho += 1;
                continue;
            }
            match k_chirho {
                RawTokenKindChirho::RightParenChirho if paren_depth_chirho == 0 => return true,
                RawTokenKindChirho::RightParenChirho => {
                    paren_depth_chirho -= 1;
                    last_was_name_chirho = false;
                }
                RawTokenKindChirho::LeftParenChirho => {
                    paren_depth_chirho += 1;
                    last_was_name_chirho = false;
                }
                RawTokenKindChirho::VarIdChirho | RawTokenKindChirho::ConIdChirho => {
                    // Two consecutive names = type application, not a kind
                    if last_was_name_chirho && paren_depth_chirho == 0 {
                        return false;
                    }
                    last_was_name_chirho = true;
                }
                RawTokenKindChirho::RightArrowChirho => {
                    last_was_name_chirho = false;
                }
                // VarSym covers `*` which is used as a kind
                RawTokenKindChirho::VarSymChirho => {
                    last_was_name_chirho = false;
                }
                // Any other token (forall, etc.) is not simple
                _ => return false,
            }
            i_chirho += 1;
        }
        false
    }

    // -----------------------------------------------------------------------
    // Predicates — what can start various constructs
    // -----------------------------------------------------------------------

    /// Can the current token start an atomic type?
    pub(super) fn can_start_atype_chirho(&self) -> bool {
        self.can_start_atype_idx_chirho(self.pos_chirho)
    }

    fn at_visible_type_application_chirho(&self) -> bool {
        if !self.at_chirho(RawTokenKindChirho::AtChirho) {
            return false;
        }
        let next_idx_chirho = self.skip_trivia_idx_chirho(self.pos_chirho + 1);
        self.can_start_atype_idx_chirho(next_idx_chirho)
    }

    pub(super) fn can_start_atype_idx_chirho(&self, idx_chirho: usize) -> bool {
        matches!(
            self.tokens_chirho
                .get(idx_chirho)
                .map(|token_chirho| token_chirho.kind_chirho),
            Some(RawTokenKindChirho::VarIdChirho)
                | Some(RawTokenKindChirho::ConIdChirho)
                | Some(RawTokenKindChirho::QualifiedIdChirho)
                | Some(RawTokenKindChirho::LeftParenChirho)
                | Some(RawTokenKindChirho::LeftBracketChirho)
                | Some(RawTokenKindChirho::TickChirho)
                // PartialTypeSignatures: `_` as a wildcard type
                | Some(RawTokenKindChirho::UnderscoreChirho)
                // DataKinds: type-level literals
                | Some(RawTokenKindChirho::IntLitChirho)
                | Some(RawTokenKindChirho::StringLitChirho)
                | Some(RawTokenKindChirho::CharLitChirho)
        )
    }
}
