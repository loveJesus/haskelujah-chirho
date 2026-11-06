// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Resolve Core primitives explicitly. An unsupported operation is an error when
//! demanded, never an invented arithmetic operation.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    ArgSourceChirho, ClosureChirho, CodeChirho, CodePtrChirho, CoreIdChirho, LowerCtxChirho,
    PrimOpKindChirho, ValueChirho,
};

#[cfg(test)]
mod tests_chirho;

#[cfg(test)]
mod binding_ownership_tests_chirho;

impl LowerCtxChirho {
    pub(super) fn emit_named_primitive_chirho(
        &mut self,
        name_chirho: &str,
        arguments_chirho: Vec<ArgSourceChirho>,
    ) -> u32 {
        let instruction_chirho = match primop_name_to_kind_chirho(name_chirho) {
            Some(operation_chirho) => CodeChirho::PrimChirho {
                op_chirho: operation_chirho,
                args_chirho: arguments_chirho,
            },
            None => CodeChirho::PrimChirho {
                op_chirho: PrimOpKindChirho::ErrorChirho,
                args_chirho: vec![ArgSourceChirho::StaticChirho(ValueChirho::StringChirho(
                    format!("unsupported STG primitive: {name_chirho}"),
                ))],
            },
        };
        self.emit_chirho(instruction_chirho)
    }
}

/// Map a primop name from Core to a `PrimOpKindChirho`.
fn primop_name_to_kind_chirho(name_chirho: &str) -> Option<PrimOpKindChirho> {
    Some(match name_chirho {
        "isHeapObjectChirho#" => PrimOpKindChirho::IsHeapObjectChirho,
        "+#" => PrimOpKindChirho::AddIntChirho,
        "enumStepChirho#" => PrimOpKindChirho::EnumStepChirho,
        "-#" => PrimOpKindChirho::SubIntChirho,
        "*#" => PrimOpKindChirho::MulIntChirho,
        "div#" => PrimOpKindChirho::DivIntChirho,
        "mod#" => PrimOpKindChirho::ModIntChirho,
        "quot#" => PrimOpKindChirho::QuotIntChirho,
        "rem#" => PrimOpKindChirho::RemIntChirho,
        "chr#" => PrimOpKindChirho::ChrChirho,
        "ord#" => PrimOpKindChirho::OrdChirho,
        "isDigit#" => PrimOpKindChirho::IsDigitChirho,
        "isAlpha#" => PrimOpKindChirho::IsAlphaChirho,
        "isAlphaNum#" => PrimOpKindChirho::IsAlphaNumChirho,
        "isUpper#" => PrimOpKindChirho::IsUpperChirho,
        "isLower#" => PrimOpKindChirho::IsLowerChirho,
        "isSpace#" => PrimOpKindChirho::IsSpaceChirho,
        "toLower#" => PrimOpKindChirho::ToLowerChirho,
        "toUpper#" => PrimOpKindChirho::ToUpperChirho,
        "digitToInt#" => PrimOpKindChirho::DigitToIntChirho,
        "intToDigit#" => PrimOpKindChirho::IntToDigitChirho,
        "==#" => PrimOpKindChirho::EqIntChirho,
        "/=#" => PrimOpKindChirho::NeIntChirho,
        "<#" => PrimOpKindChirho::LtIntChirho,
        "<=#" => PrimOpKindChirho::LeIntChirho,
        ">#" => PrimOpKindChirho::GtIntChirho,
        ">=#" => PrimOpKindChirho::GeIntChirho,
        "negate#" => PrimOpKindChirho::NegIntChirho,
        "putStrLn" | "putStrLn#" => PrimOpKindChirho::PutStrLnChirho,
        "putStr" | "putStr#" => PrimOpKindChirho::PutStrChirho,
        "putChar" | "putChar#" => PrimOpKindChirho::PutCharChirho,
        "return" | "pure" | "returnIO#" => PrimOpKindChirho::ReturnIOChirho,
        ">>=" | "bindIO#" => PrimOpKindChirho::BindIOChirho,
        ">>" | "thenIO#" => PrimOpKindChirho::ThenIOChirho,
        "getLine" | "getLine#" => PrimOpKindChirho::GetLineChirho,
        "getChar" | "getChar#" => PrimOpKindChirho::GetCharChirho,
        "getContents" | "getContents#" => PrimOpKindChirho::GetContentsChirho,
        "readFile" | "readFile#" => PrimOpKindChirho::ReadFileChirho,
        "writeFile" | "writeFile#" => PrimOpKindChirho::WriteFileChirho,
        "appendFile" | "appendFile#" => PrimOpKindChirho::AppendFileChirho,
        "showInt#" => PrimOpKindChirho::ShowIntChirho,
        "showBool#" => PrimOpKindChirho::ShowBoolChirho,
        "showChar#" => PrimOpKindChirho::ShowCharChirho,
        "not#" => PrimOpKindChirho::NotBoolChirho,
        "++#" => PrimOpKindChirho::AppendStrChirho,
        "eqStr#" => PrimOpKindChirho::EqStrChirho,
        "ltStr#" => PrimOpKindChirho::LtStrChirho,
        "compareStr#" => PrimOpKindChirho::CompareStrChirho,
        "eqFloat#" => PrimOpKindChirho::EqFloatChirho,
        "+.#" => PrimOpKindChirho::AddFloatChirho,
        "*.#" => PrimOpKindChirho::MulFloatChirho,
        "-.#" => PrimOpKindChirho::SubFloatChirho,
        "/.#" => PrimOpKindChirho::DivFloatChirho,
        "recip#" => PrimOpKindChirho::RecipFloatChirho,
        "negateFloat#" => PrimOpKindChirho::NegFloatChirho,
        "showFloat#" => PrimOpKindChirho::ShowFloatChirho,
        "lengthStr#" => PrimOpKindChirho::LengthStrChirho,
        "showStr#" => PrimOpKindChirho::ShowStrChirho,
        "enumFromTo#" => PrimOpKindChirho::EnumFromToChirho,
        "enumFrom#" => PrimOpKindChirho::EnumFromChirho,
        "enumFromThen#" => PrimOpKindChirho::EnumFromThenChirho,
        "enumFromThenTo#" => PrimOpKindChirho::EnumFromThenToChirho,
        "showList#" => PrimOpKindChirho::ShowListChirho,
        "readInt#" => PrimOpKindChirho::ReadIntChirho,
        "readFloat#" => PrimOpKindChirho::ReadFloatChirho,
        "readBool#" => PrimOpKindChirho::ReadBoolChirho,
        "wordsStr#" => PrimOpKindChirho::WordsStrChirho,
        "unwordsStr#" => PrimOpKindChirho::UnwordsStrChirho,
        "takeStr#" => PrimOpKindChirho::TakeStrChirho,
        "dropStr#" => PrimOpKindChirho::DropStrChirho,
        "concatStr#" => PrimOpKindChirho::ConcatStrChirho,
        "intercalateStr#" => PrimOpKindChirho::IntercalateStrChirho,
        "fromIntegral#" => PrimOpKindChirho::FromIntegralChirho,
        "ceiling#" => PrimOpKindChirho::CeilingChirho,
        "floor#" => PrimOpKindChirho::FloorChirho,
        "round#" => PrimOpKindChirho::RoundChirho,
        "truncate#" => PrimOpKindChirho::TruncateChirho,
        "compare#" => PrimOpKindChirho::CompareIntChirho,
        "compareChar#" => PrimOpKindChirho::CompareCharChirho,
        "compareFloat#" => PrimOpKindChirho::CompareFloatChirho,
        "<.#" => PrimOpKindChirho::LtFloatChirho,
        ">.#" => PrimOpKindChirho::GtFloatChirho,
        "<=.#" => PrimOpKindChirho::LeFloatChirho,
        ">=.#" => PrimOpKindChirho::GeFloatChirho,
        // Floating math primops
        "sin#" => PrimOpKindChirho::SinFloatChirho,
        "cos#" => PrimOpKindChirho::CosFloatChirho,
        "tan#" => PrimOpKindChirho::TanFloatChirho,
        "asin#" => PrimOpKindChirho::AsinFloatChirho,
        "acos#" => PrimOpKindChirho::AcosFloatChirho,
        "atan#" => PrimOpKindChirho::AtanFloatChirho,
        "exp#" => PrimOpKindChirho::ExpFloatChirho,
        "log#" => PrimOpKindChirho::LogFloatChirho,
        "sqrt#" => PrimOpKindChirho::SqrtFloatChirho,
        "sinh#" => PrimOpKindChirho::SinhFloatChirho,
        "cosh#" => PrimOpKindChirho::CoshFloatChirho,
        "tanh#" => PrimOpKindChirho::TanhFloatChirho,
        "asinh#" => PrimOpKindChirho::AsinhFloatChirho,
        "acosh#" => PrimOpKindChirho::AcoshFloatChirho,
        "atanh#" => PrimOpKindChirho::AtanhFloatChirho,
        "atan2#" => PrimOpKindChirho::Atan2FloatChirho,
        "exponentFloatChirho#" => PrimOpKindChirho::ExponentFloatChirho,
        "significandFloatChirho#" => PrimOpKindChirho::SignificandFloatChirho,
        "scaleFloatChirho#" => PrimOpKindChirho::ScaleFloatChirho,
        "^#" => PrimOpKindChirho::PowIntChirho,
        "^.#" => PrimOpKindChirho::PowDoubleIntChirho,
        "**#" => PrimOpKindChirho::PowFloatChirho,
        "id#" => PrimOpKindChirho::IdChirho,
        "error" => PrimOpKindChirho::ErrorChirho,
        "undefined" => PrimOpKindChirho::UndefinedChirho,
        "seq" | "deepseq" => PrimOpKindChirho::SeqChirho,
        "evaluate" => PrimOpKindChirho::EvaluateChirho,
        "force" | "force#" => PrimOpKindChirho::ForceChirho,
        "showMaybe#" => PrimOpKindChirho::ShowMaybeChirho,
        "showTuple2#" => PrimOpKindChirho::ShowTuple2Chirho,
        "showEither#" => PrimOpKindChirho::ShowEitherChirho,
        "showOrdering#" => PrimOpKindChirho::ShowOrderingChirho,
        "interact" => PrimOpKindChirho::InteractChirho,
        "print" | "print#" => PrimOpKindChirho::PrintChirho,
        "lines#" => PrimOpKindChirho::LinesChirho,
        "unlines#" => PrimOpKindChirho::UnlinesChirho,
        // IORef primops
        "newIORef" | "newIORef#" => PrimOpKindChirho::NewIORefChirho,
        "readIORef" | "readIORef#" => PrimOpKindChirho::ReadIORefChirho,
        "writeIORef" | "writeIORef#" => PrimOpKindChirho::WriteIORefChirho,
        "modifyIORef" | "modifyIORef#" => PrimOpKindChirho::ModifyIORefChirho,
        // ST monad primops
        "newSTRef" | "newSTRef#" => PrimOpKindChirho::NewSTRefChirho,
        "readSTRef" | "readSTRef#" => PrimOpKindChirho::ReadSTRefChirho,
        "writeSTRef" | "writeSTRef#" => PrimOpKindChirho::WriteSTRefChirho,
        "modifySTRef" | "modifySTRef#" => PrimOpKindChirho::ModifySTRefChirho,
        "runST" | "runST#" => PrimOpKindChirho::RunSTChirho,
        // STM primops
        "newTVar" | "newTVar#" | "newTVarIO" | "newTVarIO#" => PrimOpKindChirho::NewTVarChirho,
        "readTVar" | "readTVar#" | "readTVarIO" | "readTVarIO#" => PrimOpKindChirho::ReadTVarChirho,
        "writeTVar" | "writeTVar#" => PrimOpKindChirho::WriteTVarChirho,
        "atomically" | "atomically#" => PrimOpKindChirho::AtomicallyChirho,
        "retry" | "retry#" => PrimOpKindChirho::RetryChirho,
        "orElse" | "orElse#" => PrimOpKindChirho::OrElseChirho,
        // Exception handling primops
        "catch" | "catch#" => PrimOpKindChirho::CatchChirho,
        "throw" | "throw#" | "throwIO" | "throwIO#" => PrimOpKindChirho::ThrowChirho,
        "try" | "try#" => PrimOpKindChirho::TryChirho,
        "bracket" | "bracket#" => PrimOpKindChirho::BracketChirho,
        "finally" | "finally#" => PrimOpKindChirho::FinallyChirho,
        // Data.Map primops
        "mapEmpty#" | "mapEmpty" => PrimOpKindChirho::MapEmptyChirho,
        "mapSingleton#" | "mapSingleton" => PrimOpKindChirho::MapSingletonChirho,
        "mapInsert#" | "mapInsert" => PrimOpKindChirho::MapInsertChirho,
        "mapLookup#" | "mapLookup" => PrimOpKindChirho::MapLookupChirho,
        "mapDelete#" | "mapDelete" => PrimOpKindChirho::MapDeleteChirho,
        "mapMember#" | "mapMember" => PrimOpKindChirho::MapMemberChirho,
        "mapSize#" | "mapSize" => PrimOpKindChirho::MapSizeChirho,
        "mapFromList#" | "mapFromList" => PrimOpKindChirho::MapFromListChirho,
        "mapToList#" | "mapToList" => PrimOpKindChirho::MapToListChirho,
        "mapKeys#" | "mapKeys" => PrimOpKindChirho::MapKeysChirho,
        "mapElems#" | "mapElems" => PrimOpKindChirho::MapElemsChirho,
        "mapNull#" | "mapNull" => PrimOpKindChirho::MapNullChirho,
        "mapMap#" | "mapMap" => PrimOpKindChirho::MapMapChirho,
        "mapFoldlWithKey#" | "mapFoldlWithKey" => PrimOpKindChirho::MapFoldlWithKeyChirho,
        "mapFoldrWithKey#" | "mapFoldrWithKey" => PrimOpKindChirho::MapFoldrWithKeyChirho,
        "mapUnion#" | "mapUnion" => PrimOpKindChirho::MapUnionChirho,
        "mapDifference#" | "mapDifference" => PrimOpKindChirho::MapDifferenceChirho,
        "mapIntersection#" | "mapIntersection" => PrimOpKindChirho::MapIntersectionChirho,
        "mapIntersectionWith#" | "mapIntersectionWith" => {
            PrimOpKindChirho::MapIntersectionWithChirho
        }
        "mapInsertWith#" | "mapInsertWith" => PrimOpKindChirho::MapInsertWithChirho,
        "mapFindWithDefault#" | "mapFindWithDefault" => PrimOpKindChirho::MapFindWithDefaultChirho,
        "mapAdjust#" | "mapAdjust" => PrimOpKindChirho::MapAdjustChirho,
        "mapUnionWith#" | "mapUnionWith" => PrimOpKindChirho::MapUnionWithChirho,
        "mapFilter#" | "mapFilter" => PrimOpKindChirho::MapFilterChirho,
        "mapFilterWithKey#" | "mapFilterWithKey" => PrimOpKindChirho::MapFilterWithKeyChirho,
        // Data.Set primops
        "setEmpty#" | "setEmpty" => PrimOpKindChirho::SetEmptyChirho,
        "setSingleton#" | "setSingleton" => PrimOpKindChirho::SetSingletonChirho,
        "setInsert#" | "setInsert" => PrimOpKindChirho::SetInsertChirho,
        "setMember#" | "setMember" => PrimOpKindChirho::SetMemberChirho,
        "setDelete#" | "setDelete" => PrimOpKindChirho::SetDeleteChirho,
        "setSize#" | "setSize" => PrimOpKindChirho::SetSizeChirho,
        "setFromList#" | "setFromList" => PrimOpKindChirho::SetFromListChirho,
        "setToList#" | "setToList" => PrimOpKindChirho::SetToListChirho,
        "setUnion#" | "setUnion" => PrimOpKindChirho::SetUnionChirho,
        "setIntersection#" | "setIntersection" => PrimOpKindChirho::SetIntersectionChirho,
        "setDifference#" | "setDifference" => PrimOpKindChirho::SetDifferenceChirho,
        "setNull#" | "setNull" => PrimOpKindChirho::SetNullChirho,
        "setMap#" | "setMap" => PrimOpKindChirho::SetMapChirho,
        "setFilter#" | "setFilter" => PrimOpKindChirho::SetFilterChirho,
        "setFoldr#" | "setFoldr" | "setFold#" | "setFold" => PrimOpKindChirho::SetFoldrChirho,
        _ => return None,
    })
}

/// The IO facade every reference path of the lowering consults, moved from `stg_lower_chirho.rs`
/// (#32355).
impl LowerCtxChirho {
    /// Check if a CoreId refers to a known I/O primop and return the name if so. A spelling is
    /// consulted only for an id the program does not own (`program_owns_chirho`): every one of
    /// the lowering's four reference paths asks here, so each keeps a bound value's own id and
    /// lookup (#32355).
    /// workflow: testing-chirho/execution-oracles-chirho
    pub(super) fn is_io_primop_chirho(&self, id_chirho: CoreIdChirho) -> Option<&str> {
        if self.program_owns_chirho(id_chirho) {
            return None;
        }
        let name_chirho = self.id_names_chirho.get(&id_chirho)?;
        match name_chirho.as_str() {
            "putStrLn" | "putStr" | "putChar" | "print" | "interact" | "getLine" | "getChar"
            | "getContents" | "readFile" | "writeFile" | "appendFile" | "return" | "pure"
            | ">>=" | ">>" | "newIORef" | "readIORef" | "writeIORef" | "modifyIORef"
            | "newSTRef" | "readSTRef" | "writeSTRef" | "modifySTRef" | "runST" | "newTVar"
            | "readTVar" | "writeTVar" | "atomically" | "retry" | "orElse" | "newTVarIO"
            | "error" | "undefined" | "seq" | "deepseq" | "evaluate" | "force" | "catch"
            | "throw" | "throwIO" | "try" | "bracket" | "finally" => Some(name_chirho.as_str()),
            _ => None,
        }
    }

    /// Whether the program owns `id_chirho`, so that no spelling may make it an IO primop. Each
    /// is read from what the lowering already holds and from the module's provenance, never from
    /// a name:
    /// - a lambda parameter or a value captured into an argument slot;
    /// - a let, where or case binder held in the environment;
    /// - a top-level binding the compiler did not generate. The module must record what it
    ///   generated (`CoreProvenanceChirho::compiler_bindings_chirho`, the only bindings a backend
    ///   may recognize by spelling). A module that arrives without that record, such as the
    ///   multi-module interpreter merge, keeps the legacy spelling route at top level; the IO
    ///   preparation's own wrappers never give it one (`run_lowered_chirho`);
    /// - a reference to another program module's export (`imported_references_chirho`). Unlinked
    ///   here, it stays a missing binding.
    ///
    /// A free id that no program binding owns keeps its builtin spelling, as before.
    /// workflow: testing-chirho/execution-oracles-chirho
    fn program_owns_chirho(&self, id_chirho: CoreIdChirho) -> bool {
        let generated_chirho = &self.provenance_chirho.compiler_bindings_chirho;
        let owns_binding_chirho = if self.top_level_ids_chirho.contains(&id_chirho) {
            !generated_chirho.is_empty() && !generated_chirho.contains(&id_chirho)
        } else {
            self.env_chirho.contains_key(&id_chirho)
        };
        owns_binding_chirho
            || self.arg_param_indices_chirho.contains_key(&id_chirho)
            || self
                .provenance_chirho
                .imported_references_chirho
                .contains_key(&id_chirho)
    }

    pub(super) fn io_primop_value_arity_chirho(name_chirho: &str) -> Option<u16> {
        match name_chirho {
            "getLine" | "getLine#" | "getChar" | "getContents" | "getContents#" | "retry"
            | "retry#" | "undefined" => Some(0),
            "putStrLn" | "putStrLn#" | "putStr" | "putStr#" | "putChar" | "putChar#" | "print"
            | "interact" | "readFile" | "readFile#" | "return" | "pure" | "returnIO#"
            | "newIORef" | "newIORef#" | "readIORef" | "readIORef#" | "newSTRef" | "newSTRef#"
            | "readSTRef" | "readSTRef#" | "runST" | "runST#" | "newTVar" | "newTVar#"
            | "newTVarIO" | "newTVarIO#" | "readTVar" | "readTVar#" | "readTVarIO"
            | "readTVarIO#" | "atomically" | "atomically#" | "error" | "seq" | "deepseq"
            | "evaluate" | "force" | "force#" | "throw" | "throw#" | "throwIO" | "throwIO#"
            | "try" | "try#" => Some(1),
            "writeFile" | "writeFile#" | "appendFile" | "appendFile#" | ">>=" | "bindIO#"
            | ">>" | "thenIO#" | "writeIORef" | "writeIORef#" | "modifyIORef" | "modifyIORef#"
            | "writeSTRef" | "writeSTRef#" | "modifySTRef" | "modifySTRef#" | "writeTVar"
            | "writeTVar#" | "orElse" | "orElse#" | "catch" | "catch#" | "finally" | "finally#" => {
                Some(2)
            }
            "bracket" | "bracket#" => Some(3),
            _ => None,
        }
    }

    /// Lower an IO primop name used as a value into a real heap closure.
    ///
    /// WI-001/002 PRD note: the strict monad dispatcher intentionally
    /// preserves unknown IO method names for the STG fallback. If such a name
    /// appears in argument position (`getLine >>= putStrLn`), it still needs a
    /// runtime value instead of the missing-binding `Int 0` placeholder.
    pub(super) fn lower_io_primop_value_chirho(
        &mut self,
        name_chirho: &str,
    ) -> Option<ValueChirho> {
        let arity_chirho = Self::io_primop_value_arity_chirho(name_chirho)?;
        let args_chirho: Vec<ArgSourceChirho> = (0..usize::from(arity_chirho))
            .map(ArgSourceChirho::ArgRegChirho)
            .collect();
        let entry_chirho = self.emit_named_primitive_chirho(name_chirho, args_chirho);
        let closure_chirho = if arity_chirho == 0 {
            ClosureChirho::thunk_chirho(CodePtrChirho(entry_chirho), name_chirho, vec![])
        } else {
            ClosureChirho::fun_chirho(
                arity_chirho,
                CodePtrChirho(entry_chirho),
                name_chirho,
                vec![],
            )
        };
        let addr_chirho = self.heap_chirho.alloc_chirho(closure_chirho);
        Some(ValueChirho::HeapPtrChirho(addr_chirho))
    }
}
