// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The piped UTF-8/LF stdout profile: bounded spare chars, committed text and sticky budget.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
pub const WORLD_KEY_CHIRHO: &str = "%next:io-world-chirho";
pub const RESULT_KEY_CHIRHO: &str = "%next:io-result-chirho";
pub const STDOUT_KEY_CHIRHO: &str = "%next:io-stdout-chirho";
// Pinned IO/Handle/Internals.hs:489-490; verified by the source driver.
pub const CHAR_BUFFER_SIZE_CHIRHO: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IoOperationChirho {
    CharacterChirho(CharacterOperationChirho),
    FinishChirho,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CharacterOperationChirho {
    PrepareChirho,
    AppendChirho,
}
impl IoOperationChirho {
    pub const PREPARE_CHIRHO: Self = Self::CharacterChirho(CharacterOperationChirho::PrepareChirho);
    pub const APPEND_CHIRHO: Self = Self::CharacterChirho(CharacterOperationChirho::AppendChirho);
    pub fn primitive_chirho(self) -> &'static str {
        match self {
            Self::PREPARE_CHIRHO => "%next:io-prepare-char-chirho",
            Self::APPEND_CHIRHO => "%next:io-append-char-chirho",
            Self::FinishChirho => "%next:io-finish-chirho",
        }
    }
    pub fn from_primitive_chirho(name_chirho: &str) -> Option<Self> {
        [
            Self::PREPARE_CHIRHO,
            Self::APPEND_CHIRHO,
            Self::FinishChirho,
        ]
        .into_iter()
        .find(|operation_chirho| operation_chirho.primitive_chirho() == name_chirho)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoErrorChirho {
    BudgetChirho,
    ContractChirho,
}
impl std::fmt::Display for IoErrorChirho {
    fn fmt(&self, formatter_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter_chirho.write_str(match self {
            Self::BudgetChirho => "next output byte allowance exhausted",
            Self::ContractChirho => "next IO primitive contract violated",
        })
    }
}
#[derive(Debug)]
pub struct OutputStateChirho {
    limit_chirho: usize,
    used_chirho: usize,
    exhausted_chirho: bool,
    pending_chirho: String,
    characters_chirho: usize,
}
impl Default for OutputStateChirho {
    fn default() -> Self {
        Self::with_limit_chirho(1 << 20)
    }
}
impl OutputStateChirho {
    pub fn with_limit_chirho(limit_chirho: usize) -> Self {
        Self {
            limit_chirho,
            used_chirho: 0,
            exhausted_chirho: false,
            pending_chirho: String::new(),
            characters_chirho: 0,
        }
    }
    pub fn exhausted_chirho(&self) -> bool {
        self.exhausted_chirho
    }
    fn available_chirho(&self) -> Result<(), IoErrorChirho> {
        if self.exhausted_chirho {
            Err(IoErrorChirho::BudgetChirho)
        } else {
            Ok(())
        }
    }
    fn commit_chirho(&mut self, output_chirho: &mut String) {
        output_chirho.push_str(&self.pending_chirho);
        self.pending_chirho.clear();
        self.characters_chirho = 0;
    }
    /// Called only after a cons cell is WHNF, before its head is demanded.
    pub fn prepare_chirho(&mut self, output_chirho: &mut String) -> Result<(), IoErrorChirho> {
        self.available_chirho()?;
        if self.characters_chirho + 1 >= CHAR_BUFFER_SIZE_CHIRHO {
            self.commit_chirho(output_chirho);
        }
        Ok(())
    }
    pub fn append_chirho(&mut self, character_chirho: char) -> Result<(), IoErrorChirho> {
        self.available_chirho()?;
        let Some(next_chirho) = self
            .used_chirho
            .checked_add(character_chirho.len_utf8())
            .filter(|next_chirho| *next_chirho <= self.limit_chirho)
        else {
            self.exhausted_chirho = true;
            return Err(IoErrorChirho::BudgetChirho);
        };
        if self.characters_chirho >= CHAR_BUFFER_SIZE_CHIRHO {
            return Err(IoErrorChirho::ContractChirho);
        }
        self.used_chirho = next_chirho;
        self.pending_chirho.push(character_chirho);
        self.characters_chirho += 1;
        Ok(())
    }
    /// GHC's [] branch appends the final LF without a capacity preparation.
    pub fn finish_chirho(
        &mut self,
        output_chirho: &mut String,
        newline_chirho: bool,
    ) -> Result<(), IoErrorChirho> {
        self.available_chirho()?;
        if newline_chirho {
            self.append_chirho('\n')?;
        }
        self.commit_chirho(output_chirho);
        Ok(())
    }
    pub fn discard_pending_chirho(&mut self) {
        self.pending_chirho.clear();
        self.characters_chirho = 0;
    }
}

#[cfg(test)]
mod tests_chirho;
