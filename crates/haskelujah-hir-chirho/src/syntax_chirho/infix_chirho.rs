// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reassociate only after lexical resolution; explicit parentheses remain atoms.
//! GHC.Rename.HsType.mkOpAppRn and GHC.Types.Fixity.compareFixity.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

enum ChainItemChirho<'source_chirho> {
    AtomChirho(&'source_chirho AstExprChirho),
    OperatorChirho(&'source_chirho NameChirho),
}

impl LowerChirho<'_> {
    pub(super) fn infix_chirho(
        &mut self,
        left_chirho: &AstExprChirho,
        operator_chirho: &NameChirho,
        right_chirho: &AstExprChirho,
        span_chirho: SpanChirho,
        depth_chirho: usize,
    ) -> ResultChirho<ExprChirho> {
        let mut work_chirho = vec![
            ChainItemChirho::AtomChirho(right_chirho),
            ChainItemChirho::OperatorChirho(operator_chirho),
            ChainItemChirho::AtomChirho(left_chirho),
        ];
        let mut atoms_chirho = Vec::new();
        let mut operators_chirho = Vec::new();
        while let Some(item_chirho) = work_chirho.pop() {
            self.charge_names_chirho(1, span_chirho)?;
            match item_chirho {
                ChainItemChirho::AtomChirho(AstExprChirho::InfixChirho {
                    left_chirho,
                    op_chirho,
                    right_chirho,
                    ..
                }) => {
                    work_chirho.extend([
                        ChainItemChirho::AtomChirho(right_chirho),
                        ChainItemChirho::OperatorChirho(op_chirho),
                        ChainItemChirho::AtomChirho(left_chirho),
                    ]);
                }
                ChainItemChirho::AtomChirho(AstExprChirho::NegChirho { .. }) => {
                    return Err(self.error_chirho(
                        "unparenthesized prefix negation needs resolved-name fixity regrouping",
                        span_chirho,
                    ));
                }
                ChainItemChirho::AtomChirho(atom_chirho) => atoms_chirho.push(atom_chirho),
                ChainItemChirho::OperatorChirho(operator_chirho) => {
                    let id_chirho =
                        self.lookup_chirho(operator_chirho, NamespaceChirho::ValueChirho)?;
                    let info_chirho = &self.names_chirho[id_chirho.0 as usize];
                    let fixity_chirho = if info_chirho.local_chirho {
                        // The lossless parser gate rejects scoped fixity declarations
                        // before a local producer can silently lose one.
                        Some(InterfaceFixityChirho {
                            associativity_chirho: AssociativityChirho::LeftChirho,
                            precedence_chirho: 9,
                        })
                    } else {
                        let name_chirho = info_chirho.original_chirho();
                        self.charge_names_chirho(
                            name_chirho.module_chirho.len()
                                + name_chirho.occurrence_chirho.len()
                                + 1,
                            operator_chirho.span_chirho(),
                        )?;
                        self.known_fixities_chirho.get(&name_chirho).copied()
                    };
                    operators_chirho.push((id_chirho, fixity_chirho));
                }
            }
        }
        if operators_chirho.len() > 1
            && operators_chirho
                .iter()
                .any(|(_, fixity_chirho)| fixity_chirho.is_none())
        {
            return Err(self.error_chirho("operator chain needs a checked fixity for every resolved name; a source slice proves no default", span_chirho));
        }
        let mut values_chirho = Vec::new();
        let mut pending_chirho: Vec<(NameIdChirho, InterfaceFixityChirho)> = Vec::new();
        let mut atoms_chirho = atoms_chirho.into_iter();
        let first_chirho = atoms_chirho
            .next()
            .ok_or_else(|| self.error_chirho("empty operator chain", span_chirho))?;
        values_chirho.push((self.expr_chirho(first_chirho, depth_chirho + 1)?, 0usize));
        for ((operator_chirho, fixity_chirho), atom_chirho) in
            operators_chirho.into_iter().zip(atoms_chirho)
        {
            // A single application does not inspect its default or associativity.
            let fixity_chirho = fixity_chirho.unwrap_or(InterfaceFixityChirho {
                associativity_chirho: AssociativityChirho::LeftChirho,
                precedence_chirho: 9,
            });
            while let Some((_, previous_chirho)) = pending_chirho.last() {
                let reduce_chirho = if previous_chirho.precedence_chirho
                    == fixity_chirho.precedence_chirho
                {
                    match (
                        previous_chirho.associativity_chirho,
                        fixity_chirho.associativity_chirho,
                    ) {
                        (AssociativityChirho::LeftChirho, AssociativityChirho::LeftChirho) => true,
                        (AssociativityChirho::RightChirho, AssociativityChirho::RightChirho) => {
                            false
                        }
                        _ => {
                            return Err(
                                if self.unit_chirho == UnitIdentityChirho::CompilerChirho {
                                    self.error_chirho(
                                        "pinned source operator precedence conflicts",
                                        span_chirho,
                                    )
                                } else {
                                    self.invalid_class_chirho(
                                        "operator precedence conflict between resolved fixities",
                                        span_chirho,
                                    )
                                },
                            );
                        }
                    }
                } else {
                    previous_chirho.precedence_chirho > fixity_chirho.precedence_chirho
                };
                if !reduce_chirho {
                    break;
                }
                let (previous_chirho, _) = pending_chirho.pop().unwrap();
                self.reduce_infix_chirho(
                    &mut values_chirho,
                    previous_chirho,
                    span_chirho,
                    depth_chirho,
                )?;
            }
            pending_chirho.push((operator_chirho, fixity_chirho));
            values_chirho.push((self.expr_chirho(atom_chirho, depth_chirho + 1)?, 0));
        }
        while let Some((operator_chirho, _)) = pending_chirho.pop() {
            self.reduce_infix_chirho(
                &mut values_chirho,
                operator_chirho,
                span_chirho,
                depth_chirho,
            )?;
        }
        values_chirho
            .pop()
            .map(|(expression_chirho, _)| expression_chirho)
            .ok_or_else(|| self.error_chirho("operator chain lost its result", span_chirho))
    }

    fn reduce_infix_chirho(
        &mut self,
        values_chirho: &mut Vec<(ExprChirho, usize)>,
        operator_chirho: NameIdChirho,
        span_chirho: SpanChirho,
        outer_depth_chirho: usize,
    ) -> ResultChirho<()> {
        let (right_chirho, right_depth_chirho) = values_chirho.pop().ok_or_else(|| {
            self.error_chirho("operator chain lost its right operand", span_chirho)
        })?;
        let (left_chirho, left_depth_chirho) = values_chirho.pop().ok_or_else(|| {
            self.error_chirho("operator chain lost its left operand", span_chirho)
        })?;
        let depth_chirho = 1 + left_depth_chirho.max(right_depth_chirho);
        self.room_chirho(outer_depth_chirho.saturating_add(depth_chirho), span_chirho)?;
        let expression_chirho = self.apply_name_chirho(
            operator_chirho,
            vec![left_chirho, right_chirho],
            span_chirho,
        )?;
        values_chirho.push((expression_chirho, depth_chirho));
        Ok(())
    }
}
