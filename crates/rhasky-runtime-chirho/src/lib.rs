// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionModeChirho {
    BatchChirho,
    ScriptChirho,
    ReplChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePlanChirho {
    pub execution_mode_chirho: ExecutionModeChirho,
    pub entry_module_chirho: String,
    pub incremental_session_chirho: bool,
}

impl RuntimePlanChirho {
    pub fn for_module_chirho(
        execution_mode_chirho: ExecutionModeChirho,
        entry_module_chirho: impl Into<String>,
    ) -> Self {
        let incremental_session_chirho =
            matches!(execution_mode_chirho, ExecutionModeChirho::ScriptChirho | ExecutionModeChirho::ReplChirho);

        Self {
            execution_mode_chirho,
            entry_module_chirho: entry_module_chirho.into(),
            incremental_session_chirho,
        }
    }
}

