// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Unchanged GHC9.14.1 reference sources; evidence JSONL retains hashes and commands.
use super::ClassCaseChirho;
pub(super) const CASES_CHIRHO: &[ClassCaseChirho] = &[
    ClassCaseChirho {
        name_chirho: "C0_T20661_verbatim",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class () => C a b | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C1_abstract_boot_with_fundep_and_instance",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C2_concrete_empty_boot_impl_adds_method",
        accepted_chirho: false,
        reason_chirho: "kind",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class () => C a b | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b where
  meth :: a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C3_abstract_boot_impl_adds_method",
        accepted_chirho: false,
        reason_chirho: "kind",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b | a -> b where
  meth :: a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C4_concrete_boot_fundep_impl_without",
        accepted_chirho: false,
        reason_chirho: "functional dependencies",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class () => C a b | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C5_concrete_boot_impl_adds_superclass",
        accepted_chirho: false,
        reason_chirho: "kind",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class () => C a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class Eq a => C a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "C6_abstract_boot_impl_superclass_and_method",
        accepted_chirho: false,
        reason_chirho: "kind",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class C a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module BootClassChirho where
class Eq a => C a where
  meth :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K1_concrete_empty_annotated_impl_adds_method",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type) | a -> b where
  meth :: a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K2_abstract_annotated_impl_adds_method",
        accepted_chirho: true,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type) | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type) | a -> b where
  meth :: a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K3_abstract_annotated_impl_superclass_method",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class Eq a => C (a :: Type) where
  meth :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K4_concrete_empty_annotated_impl_superclass",
        accepted_chirho: false,
        reason_chirho: "superclass",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class Eq a => C (a :: Type)
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K5_abstract_fundep_boot_impl_no_fundep",
        accepted_chirho: false,
        reason_chirho: "functional dependencies",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type) | a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type)
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K6_abstract_no_fundep_boot_impl_fundep",
        accepted_chirho: false,
        reason_chirho: "functional dependencies",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) (b :: Type) | a -> b
"####,
            ),
            (
                "AuxChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies #-}
module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int Bool
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K7_abstract_annotated_instance_of_abstract",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  meth :: a -> a
  meth = id
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "K8_abstract_annotated_instance_missing_method",
        accepted_chirho: true,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE FunctionalDependencies, KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  meth :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
instance C Int
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "W1_empty_where_no_context_impl_adds_method",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where {}
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "W2_empty_where_explicit_context_impl_adds_method",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where {}
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "W3_empty_where_vs_absent_body_both_empty",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where {}
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "W4_absent_body_no_context_impl_adds_method_control",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "F1_abstract_boot_impl_adds_assoc_family",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "F2_abstract_boot_impl_adds_assoc_family_with_default",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
  type instance T a = Int
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
    ClassCaseChirho {
        name_chirho: "F3_concrete_empty_boot_impl_adds_assoc_family",
        accepted_chirho: false,
        reason_chirho: "associated",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type)
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
    },
];
