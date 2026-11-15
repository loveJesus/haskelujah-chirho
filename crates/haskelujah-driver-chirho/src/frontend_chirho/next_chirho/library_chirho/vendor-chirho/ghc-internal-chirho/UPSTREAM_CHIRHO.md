<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# GHC 9.14.1 source

Whole, unmodified files from GHC commit 902339d332fb4ce2b3c87dcac1ee6495d41ad886. The BSD-style upstream LICENSE contains all three origin notices and accompanies these files and the embedded binary notice. Rust manifest spans select checked fragments; no whole-module completeness follows. The developer reference checkout is not a product input.

- libraries/ghc-internal/src/GHC/Internal/Num.hs | 5200 bytes | SHA256 a35a0fac5e44dde385bc98024d85a82924dac33c59de0996c9978e4295e4077a
- libraries/ghc-internal/src/GHC/Internal/Data/Tuple.hs | 1816 bytes | SHA256 97658f1c5b9be7910c23480641fb8f1aedbdbe96d0e2d68b5c6106aa48e7a7ad
- libraries/ghc-internal/src/GHC/Internal/Bignum/Integer.hs | 60857 bytes | SHA256 1f8ec2a8e12ecbab7eb0b59f249fae663d8066f2fb14224177a4538653bb17f4
- libraries/ghc-internal/LICENSE | 3881 bytes | SHA256 768c070bd0b7d820d169ee8153d5487acfc262cbbc10dfce18d05c0bb2d2800d

The sibling base-chirho/Prelude.hs is vendored whole (6040 bytes, SHA256 02f4309be195717f29c307f5041784c62a08297a68169bf4760cfe95abd1fa3c) from libraries/base/src/Prelude.hs at the same commit. Only its explicit complete export name set is read, to detect scope clashes and distinguish unsupported present names from absence. Its body, types and instances are not claimed checked. The base LICENSE is byte-identical to this ghc-internal LICENSE, and both are retained. The binary notice covers both source origins.

- libraries/ghc-internal/src/GHC/Internal/Classes.hs | 78107 bytes | SHA256 9f07eab1df0c2c9892ba5b8e35236e45868f738cb6a755234a7c49e70fb150cd | actual Eq/Ord/not and shared fixity spans checked
- libraries/ghc-internal/src/GHC/Internal/Base.hs | 79928 bytes | SHA256 bc38ea9356f90aeb38298ef1269fbdc2dee433528374948375da112b273a89e2 | facade header only, no body/type checking claim

- libraries/ghc-internal/src/GHC/Internal/Types.hs | 316020 bytes | SHA256 a8fe6ab5c7a84be9b0710078b549aa63e511e7cb12d056cef91403d05ca6d2bb | wired cons fixity from unconditional bytes 4497..4508 only; no Types body claimed.

Unit 761 facade-only whole files, GHC 9.14.1, original BSD licence retained: `src/GHC/Internal/Data/List.hs` (5161 bytes, SHA256 `9f8f3d2b317b06c1ea65b03d0cd95b50a6125642d6836847e3a755ff1c91d79b`) and `src/GHC/Internal/Data/OldList.hs` (71857 bytes, SHA256 `4b519356f7bcbd705866551e26552c84b08d527fe93b060cc89b27882aec6248`). Only headers/imports prove the qualified Prelude `(List.++)` path through List/OldList to the checked Base original; no bodies from either facade are claimed.
