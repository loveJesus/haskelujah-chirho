// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Executable code is a root owner, separately from stacks and fresh allocations.
//! Index each instruction once per outer run, then only append new instructions
//! at safepoints. A new outer run rebuilds the index so retired/replaced code does
//! not pin old constants. Workflow: testing-chirho/execution-oracles-chirho.md.

use std::collections::HashSet;

use super::{
    ArgSourceChirho, CodeChirho, EvalErrorChirho, HeapAddrChirho, MachineChirho, ValueChirho,
};

#[derive(Debug, Default)]
pub(super) struct CodeRootsChirho {
    indexed_chirho: usize,
    depth_chirho: usize,
    pub(super) addresses_chirho: HashSet<HeapAddrChirho>,
}

impl CodeRootsChirho {
    pub(super) fn refresh_chirho(&mut self, instructions_chirho: &[CodeChirho]) {
        for instruction_chirho in &instructions_chirho[self.indexed_chirho..] {
            self.instruction_chirho(instruction_chirho);
        }
        self.indexed_chirho = instructions_chirho.len();
    }

    fn value_chirho(&mut self, value_chirho: &ValueChirho) {
        if let ValueChirho::HeapPtrChirho(address_chirho) = value_chirho {
            self.addresses_chirho.insert(*address_chirho);
        }
    }

    fn source_chirho(&mut self, source_chirho: &ArgSourceChirho) {
        match source_chirho {
            ArgSourceChirho::StaticChirho(value_chirho) => self.value_chirho(value_chirho),
            ArgSourceChirho::ThunkCodeChirho {
                captures_chirho, ..
            } => {
                self.sources_chirho(captures_chirho);
            }
            ArgSourceChirho::ArgRegChirho(_) => {}
        }
    }

    fn sources_chirho(&mut self, sources_chirho: &[ArgSourceChirho]) {
        for source_chirho in sources_chirho {
            self.source_chirho(source_chirho);
        }
    }

    fn instruction_chirho(&mut self, instruction_chirho: &CodeChirho) {
        match instruction_chirho {
            CodeChirho::EnterChirho(address_chirho) => {
                self.addresses_chirho.insert(*address_chirho);
            }
            CodeChirho::AppChirho {
                fun_chirho,
                args_chirho,
            } => {
                self.addresses_chirho.insert(*fun_chirho);
                self.sources_chirho(args_chirho);
            }
            CodeChirho::CaseChirho {
                scrutinee_chirho, ..
            } => {
                self.source_chirho(scrutinee_chirho);
            }
            CodeChirho::CaseLitChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                self.source_chirho(scrutinee_chirho);
                for (value_chirho, _) in alts_chirho {
                    self.value_chirho(value_chirho);
                }
            }
            CodeChirho::ConAppChirho { fields_chirho, .. } => {
                for value_chirho in fields_chirho {
                    self.value_chirho(value_chirho);
                }
            }
            CodeChirho::LitChirho(value_chirho) => self.value_chirho(value_chirho),
            CodeChirho::PrimChirho { args_chirho, .. }
            | CodeChirho::AppFromArgChirho {
                arg_sources_chirho: args_chirho,
                ..
            }
            | CodeChirho::ConAppFromArgChirho {
                fields_chirho: args_chirho,
                ..
            }
            | CodeChirho::AllocFunChirho {
                captures_chirho: args_chirho,
                ..
            }
            | CodeChirho::StoreAllocFunChirho {
                captures_chirho: args_chirho,
                ..
            }
            | CodeChirho::StoreAllocThunkChirho {
                captures_chirho: args_chirho,
                ..
            } => {
                self.sources_chirho(args_chirho);
            }
            CodeChirho::LetChirho {
                closures_chirho, ..
            } => {
                for closure_chirho in closures_chirho {
                    for value_chirho in &closure_chirho.payload_chirho {
                        self.value_chirho(value_chirho);
                    }
                }
            }
            CodeChirho::ForceChirho { thunk_chirho } => {
                for value_chirho in &thunk_chirho.payload_chirho {
                    self.value_chirho(value_chirho);
                }
            }
            CodeChirho::StoreAllocRecGroupChirho {
                bindings_chirho, ..
            } => {
                for binding_chirho in bindings_chirho {
                    self.sources_chirho(&binding_chirho.captures_chirho);
                }
            }
            CodeChirho::ArgChirho { .. } => {}
        }
    }
}

impl MachineChirho {
    /// Run a code entry to WHNF. Nested evaluator calls share this run's code
    /// roots; a later outer invocation may replace its public code table.
    pub fn run_chirho(&mut self, entry_chirho: u32) -> Result<ValueChirho, EvalErrorChirho> {
        if self.code_roots_chirho.depth_chirho == 0 {
            self.code_roots_chirho.indexed_chirho = 0;
            self.code_roots_chirho.addresses_chirho.clear();
        }
        self.code_roots_chirho.depth_chirho += 1;
        let result_chirho = self.run_loop_chirho(entry_chirho);
        self.code_roots_chirho.depth_chirho -= 1;
        result_chirho
    }
}
