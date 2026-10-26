<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Native Strings carry code points Chirho

The minimal row 560 brick (gpt_chirho #29139): a bounded representation correction, not the rest
of row 560. It unblocks the paired 561+557 stack. Six native cells there changed from one silent
wrong answer to another, because a native literal could not hold the decoded value:
- NUL truncated the C-string literal;
- unpack made one cell per UTF-8 byte.

## Contract

- Unpack decodes UTF-8 to one cell per code point; pack encodes code points as UTF-8.
- An evaluated native literal holding NUL fails by a stable named line; so does packing a NUL.
- Invalid UTF-8 and an unencodable cell fail by their own named lines.
- The interpreter is unchanged.

## Checklist

- [x] Measure GHC on the controls: code points through unpack, reverse, `toEnum`, `succ` and
  comparison; a NUL literal; a packed NUL.
- [x] RTS: UTF-8 unpack and pack, and the three named failures in `failure_chirho.rs`.
- [x] LLVM and Cranelift: an evaluated literal holding NUL calls
  `haskelujah_nul_string_literal_chirho` (Cranelift through a required runtime import).
- [x] Controls: the RTS round trip; all-engine code points; NUL failing named on both natives while
  the interpreter keeps it. RTS 61/61, native_error_helpers 40/40.
- [ ] Rebase onto main, transitions, the focused gate, and post.
