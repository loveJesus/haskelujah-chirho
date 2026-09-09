// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Return lazy heap values through continuation frames. See the execution-oracles
//! workflow: laziness is proved by both unused-payload and strict-use controls.

use super::{
    EvalErrorChirho, FrameChirho, HeapAddrChirho, InfoTagChirho, MachineChirho, ReturnActionChirho,
    ValueChirho,
};

impl MachineChirho {
    /// Return a heap pointer through the stack — handles Update frames
    /// with direct indirection and Case frames via `return_con_chirho`.
    pub(super) fn return_heap_ptr_chirho(
        &mut self,
        addr_chirho: HeapAddrChirho,
    ) -> Result<ReturnActionChirho, EvalErrorChirho> {
        let addr_chirho = self.heap_chirho.follow_ind_chirho(addr_chirho);
        let needs_entry_chirho = matches!(
            self.heap_chirho
                .read_chirho(addr_chirho)
                .info_chirho
                .tag_chirho,
            InfoTagChirho::ThunkChirho | InfoTagChirho::BlackholeChirho
        );
        loop {
            match self.stack_chirho.pop_chirho() {
                None => {
                    return Ok(ReturnActionChirho::DoneChirho(ValueChirho::HeapPtrChirho(
                        addr_chirho,
                    )));
                }
                Some(
                    frame_chirho @ (FrameChirho::CaseChirho { .. }
                    | FrameChirho::CaseLitChirho { .. }),
                ) if needs_entry_chirho => {
                    // A lazy primitive (notably returnIO#) can return an unevaluated
                    // payload. A strict consumer must enter it before reading its
                    // constructor tag or literal value. Update/return/catch and
                    // IO sequencing are not demands on that payload. In particular
                    // the bindIO# primitive must pass an unused result on lazily.
                    self.stack_chirho.push_chirho(frame_chirho);
                    let entry_chirho = self.emit_enter_chirho(addr_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(entry_chirho));
                }
                Some(FrameChirho::UpdateChirho { thunk_addr_chirho }) => {
                    // Update thunk to an indirection to the heap object.
                    self.heap_chirho
                        .update_to_ind_chirho(thunk_addr_chirho, addr_chirho);
                    continue;
                }
                Some(FrameChirho::CatchChirho { .. }) => {
                    // Exception handler frame — body succeeded, so the
                    // catch frame is simply discarded and the result
                    // passes through.
                    continue;
                }
                Some(FrameChirho::TryFrameChirho { .. }) => {
                    // try# frame — body succeeded with a heap pointer.
                    // Wrap in Right so the caller receives Either String a.
                    let right_val_chirho =
                        self.make_either_right_chirho(ValueChirho::HeapPtrChirho(addr_chirho));
                    let idx_chirho = self.emit_lit_code_chirho(right_val_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(idx_chirho));
                }
                Some(frame_chirho @ FrameChirho::CaseChirho { .. }) => {
                    // Dispatch via the constructor return path.
                    self.stack_chirho.push_chirho(frame_chirho);
                    return self.return_con_chirho(addr_chirho);
                }
                Some(FrameChirho::PrimOpChirho {
                    op_chirho,
                    mut args_so_far_chirho,
                    mut pending_args_chirho,
                    remaining_chirho,
                }) => {
                    args_so_far_chirho.push(ValueChirho::HeapPtrChirho(addr_chirho));
                    if remaining_chirho <= 1 {
                        let result_chirho =
                            self.eval_prim_chirho(op_chirho, &args_so_far_chirho)?;
                        return self.return_lit_chirho(result_chirho);
                    } else {
                        let next_chirho = pending_args_chirho.remove(0);
                        self.stack_chirho.push_chirho(FrameChirho::PrimOpChirho {
                            op_chirho,
                            args_so_far_chirho,
                            pending_args_chirho,
                            remaining_chirho: remaining_chirho - 1,
                        });
                        match next_chirho {
                            ValueChirho::HeapPtrChirho(a_chirho) => {
                                let enter_idx_chirho = self.emit_enter_chirho(a_chirho);
                                return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                            }
                            other_chirho => {
                                return self.return_lit_chirho(other_chirho);
                            }
                        }
                    }
                }
                Some(FrameChirho::ApplyChirho { args_chirho }) => {
                    // The heap object is being applied to arguments — enter it.
                    self.stack_chirho
                        .push_chirho(FrameChirho::ApplyChirho { args_chirho });
                    let enter_idx_chirho = self.emit_enter_chirho(addr_chirho);
                    return Ok(ReturnActionChirho::ContinueChirho(enter_idx_chirho));
                }
                Some(FrameChirho::CaseLitChirho {
                    alt_entries_chirho,
                    default_entry_chirho,
                    saved_arg_regs_chirho,
                }) => {
                    // Restore arg_regs from the frame.
                    self.arg_regs_chirho = saved_arg_regs_chirho;
                    let entry_chirho = self.dispatch_case_lit_chirho(
                        &ValueChirho::HeapPtrChirho(addr_chirho),
                        &alt_entries_chirho,
                        default_entry_chirho,
                    )?;
                    return Ok(ReturnActionChirho::ContinueChirho(entry_chirho));
                }
            }
        }
    }
}
