<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# WI-008 Onfn STG Print Parity

- [x] Reproduce the CLI `onfn` residual and confirm explicit `:: Int` annotation makes `print (((+) `on` abs) (-3) 3)` produce `6`.
- [x] Add body-backed `Product` Functor/Applicative/Monad row for erased payload newtype dispatch.
- [x] Add body-backed `Enum Integer` methods used by numeric defaulting and `fromEnum`.
- [x] Remove invalid `Integral Float` seed while preserving lawful Float class instances.
- [x] Harden dict-param dispatch key inference so higher-order applications do not inherit arbitrary argument keys.
- [x] Specialize ambiguous numeric class-method references under `print` to `Int` without rewriting constructor-headed values.
- [x] Add targeted regression tests for exact onfn print output, Product dispatch, and Integer enum conversion.
- [x] Run targeted gates only under the resource guard and leave the full probe to Claude for acceptance.
- [x] Update PRD/progress records and release the builder slot.
