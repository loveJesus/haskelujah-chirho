<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Enum Slots Chirho

Row 671 (assigned in #30760; design #30764 approved in #30765) makes all eight Enum methods
dictionary slots, so a polymorphic enumeration call runs its instance's own. It lands with row
665 for existing instances on main 7da8dd4c (#30802). Row 670's Enum Double, with its numeric
builder, Double seed and Double controls, stays held: an inferred Fractional range at Double
(`enumFromTo (x / 2) (x / 2 + 1)`) prints Infinity natively, the residual row 677 records.

## Source

- [x] A neutral regroup: the Char and integral Enum row files move byte-identical into
  `prelude_chirho/enum_chirho/` (approved in #30754).
- [x] Typing: the four enumerations join the builtin Enum declaration (`class_chirho.rs`), so a
  written instance enumeration is checked against the class.
- [x] Naming: `Enum(..)` carries all eight (`iface_chirho/builtin_chirho.rs`).
- [x] Driver: the four enter the per-occurrence name list (`evidence_join_chirho.rs`, #30768).
- [x] Int's shared bodies move byte-identical to `integral_sequence_chirho.rs` (370 lines, sha256
  f402aef8… on both sides) and are bound as `$prim_Enum_<m>_Int`. The dead Int-only
  `enumFromTo` is deleted. With ids renumbered, each new row is identical to the old live body.
- [x] `builtin_enum_slots_chirho.rs`: the integer-valued keys and Char alias Int's rows by CoreId;
  Bool and Ordering take GHC's derived rows through their builtin donors; Float and `()` keep
  stubs that name the method.
- [x] Row 669's defaults take explicit donors and Int's row by CoreId; the derived pair is built
  before dictionaries are filled; ordinary defaults never take a context-leading donor.
- [x] Context self-dictionaries fill each slot with its row, then GHC's default made with the
  instance's own context dictionaries (#30775), then a named stub under the existing id.

## Measured (base: main 7da8dd4c plus 665; GHC 9.14.1; interpreter, LLVM, true Cranelift)

- [x] From failing or killed to exact everywhere: polymorphic enumeration at Int, Char, Integer,
  Bool and Ordering; an instance's own `enumFromTo` and its defaults; a derived type; a local
  namesake beside range syntax; a qualified `P.enumFromTo` beside a program-owned one (row 673).
- [x] Exact before and after: `hiding` with own `enumFrom` and `enumFromTo`, written calls and ranges,
  and the Int-only inferred and signed numeric-range and enumeration programs.
- [x] Float and `()` through `Enum a =>` fail by the method's name.
- [x] Changed failure: a written enumeration at a context instance's key used to run Int's body
  and print a wrong list in the interpreter; it now fails by name (`missing STG binding
  enumFromTo`).
- [x] Every Double program stays refused with E0204 (p5, p8, s1, s14a, s14b).
- [x] Checker negative: a wrongly typed written `enumFromTo`, which the base accepted, is rejected
  with E0200, as GHC rejects it (GHC-83865).
- [x] Controls: `enum_slots_chirho.rs` (13 programs, byte identity 26/26) and
  `enum_slot_structure_chirho.rs`.
- [ ] Focused gate, then the tested tip to gpt_chirho.

## Unchanged residuals

- A call site cannot reach a context instance's dictionary, and `fromEnum`, `succ`, `pred` and
  `show` at such a type answer wrongly, as on the base.
- Char `enumFrom` in the interpreter steps a Char as an Int; the natives are exact.
- Word's `Num` has no `fromInteger`, and Word8 and Int8 have no native `show`; the bounded keys'
  `enumFrom` still runs past `maxBound` through the shared bodies.
- Float's f64 representation (row 676).
