// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generated activation frames own only their shadow-root suffix. Tokens are
//! current-thread/LIFO authorities, never guessed depths or value classifications.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use std::thread::{ThreadId as ThreadIdChirho, current as current_thread_chirho};

use super::{NativeGcRuntimeChirho, internal_failure_chirho, native_gc_runtime_lock_chirho};

#[derive(Debug)]
struct NativeRootFrameChirho {
    token_chirho: u64,
    start_chirho: usize,
    owner_chirho: ThreadIdChirho,
}

#[derive(Debug, Default)]
pub(super) struct NativeRootFramesChirho {
    frames_chirho: Vec<NativeRootFrameChirho>,
    next_token_chirho: u64,
}

impl NativeRootFramesChirho {
    fn owner_chirho(&self) -> Result<(), &'static str> {
        if self
            .frames_chirho
            .last()
            .is_some_and(|frame_chirho| frame_chirho.owner_chirho != current_thread_chirho().id())
        {
            return Err("native root frame belongs to another thread");
        }
        Ok(())
    }

    fn mark_chirho(&mut self, depth_chirho: usize) -> Result<u64, &'static str> {
        self.owner_chirho()?;
        let token_chirho = self
            .next_token_chirho
            .checked_add(1)
            .ok_or("native root frame token exhausted")?;
        self.frames_chirho.push(NativeRootFrameChirho {
            token_chirho,
            start_chirho: depth_chirho,
            owner_chirho: current_thread_chirho().id(),
        });
        self.next_token_chirho = token_chirho;
        Ok(token_chirho)
    }

    fn start_chirho(&self, token_chirho: u64, depth_chirho: usize) -> Result<usize, &'static str> {
        self.owner_chirho()?;
        let frame_chirho = self
            .frames_chirho
            .last()
            .ok_or("no active native root frame")?;
        if frame_chirho.token_chirho != token_chirho {
            return Err("stale or non-top native root frame token");
        }
        if depth_chirho < frame_chirho.start_chirho {
            return Err("native roots fell below their frame entry");
        }
        Ok(frame_chirho.start_chirho)
    }

    pub(super) fn validate_push_chirho(&self) -> Result<(), &'static str> {
        self.owner_chirho()
    }

    pub(super) fn validate_pop_chirho(&self, depth_chirho: usize) -> Result<(), &'static str> {
        self.owner_chirho()?;
        if self
            .frames_chirho
            .last()
            .is_some_and(|frame_chirho| depth_chirho <= frame_chirho.start_chirho)
        {
            return Err("native root pop crossed its frame entry");
        }
        Ok(())
    }
}

impl NativeGcRuntimeChirho {
    fn mark_root_frame_chirho(&mut self) -> Result<u64, &'static str> {
        self.root_frames_chirho
            .mark_chirho(self.gc_roots_chirho.len())
    }

    fn reset_root_frame_chirho(&mut self, token_chirho: u64) -> Result<(), &'static str> {
        let start_chirho = self
            .root_frames_chirho
            .start_chirho(token_chirho, self.gc_roots_chirho.len())?;
        // Truncation never allocates or invokes native collection. Generated code
        // immediately re-roots the already computed next arguments before allocation.
        self.gc_roots_chirho.truncate(start_chirho);
        Ok(())
    }

    fn restore_root_frame_chirho(&mut self, token_chirho: u64) -> Result<(), &'static str> {
        self.reset_root_frame_chirho(token_chirho)?;
        let _ = self.root_frames_chirho.frames_chirho.pop();
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_frame_mark_chirho() -> u64 {
    native_gc_runtime_lock_chirho()
        .mark_root_frame_chirho()
        .unwrap_or_else(|error_chirho| internal_failure_chirho(error_chirho.as_bytes()))
}

#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_frame_reset_chirho(token_chirho: u64) {
    native_gc_runtime_lock_chirho()
        .reset_root_frame_chirho(token_chirho)
        .unwrap_or_else(|error_chirho| internal_failure_chirho(error_chirho.as_bytes()));
}

#[unsafe(no_mangle)]
pub extern "C" fn haskelujah_gc_root_frame_restore_chirho(token_chirho: u64) {
    native_gc_runtime_lock_chirho()
        .restore_root_frame_chirho(token_chirho)
        .unwrap_or_else(|error_chirho| internal_failure_chirho(error_chirho.as_bytes()));
}

#[cfg(test)]
mod tests_chirho;
