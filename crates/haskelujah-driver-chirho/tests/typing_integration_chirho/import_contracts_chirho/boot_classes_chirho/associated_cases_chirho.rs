// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Unchanged GHC9.14.1 reference sources; evidence JSONL retains hashes and commands.
use super::ClassCaseChirho;
pub(super) const CASES_CHIRHO: &[ClassCaseChirho] = &[
    ClassCaseChirho {
        name_chirho: "N1_nocontext_method_boot_impl_no_method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
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
        accepted_chirho: false,
        reason_chirho: "method",
    },
    ClassCaseChirho {
        name_chirho: "N2_nocontext_method_boot_impl_method_retyped",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> Int
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "method",
    },
    ClassCaseChirho {
        name_chirho: "N3_nocontext_method_boot_impl_extra_method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m2 :: a
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "method",
    },
    ClassCaseChirho {
        name_chirho: "N4_nocontext_method_boot_impl_same_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
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
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "N8_nocontext_method_boot_impl_adds_superclass",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class Eq a => C (a :: Type) where
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
        accepted_chirho: false,
        reason_chirho: "superclass",
    },
    ClassCaseChirho {
        name_chirho: "N5_nocontext_family_boot_impl_no_family",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
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
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "N6_nocontext_family_boot_impl_family_rekinded",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type -> Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "N7_nocontext_family_boot_impl_same_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
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
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "N9_nocontext_family_boot_impl_adds_superclass",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies #-}
module BootClassChirho where
import Data.Kind (Type)
class Eq a => C (a :: Type) where
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
        accepted_chirho: false,
        reason_chirho: "superclass",
    },
    ClassCaseChirho {
        name_chirho: "R1_assoc_family_order_swapped",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type
  type family T2 a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T2 a :: Type
  type family T1 a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R2_assoc_family_order_same_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type
  type family T2 a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type
  type family T2 a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R3_params_alpha_renamed_same_association",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (x :: Type) (y :: Type) where
  type family T x :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R4_params_association_swapped",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type family T b :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R5_default_only_without_family_head",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => D (a :: Type) where
  type T a = Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => D (a :: Type) where
  type T a = Int
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R6_default_spelling_type_vs_type_instance",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => D (a :: Type) where
  type family T a :: Type
  type T a = Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => D (a :: Type) where
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
        accepted_chirho: true,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R7_order_swapped_different_kinds",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type
  type family T2 a :: Type -> Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T2 a :: Type -> Type
  type family T1 a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R8_names_in_place_kinds_exchanged",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type
  type family T2 a :: Type -> Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T1 a :: Type -> Type
  type family T2 a :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "R9_association_swapped_different_param_kinds",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type -> Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type -> Type) where
  type family T b :: Type
"####,
            ),
            (
                "AuxChirho.hs",
                r####"module AuxChirho where
import {-# SOURCE #-} BootClassChirho
"####,
            ),
        ],
        accepted_chirho: false,
        reason_chirho: "associated",
    },
    ClassCaseChirho {
        name_chirho: "A1_assoc_default_boot_only",
        accepted_chirho: false,
        reason_chirho: "associated",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  type family T a :: Type
  type instance T a = Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
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
        name_chirho: "A2_assoc_default_differs",
        accepted_chirho: false,
        reason_chirho: "associated",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  type family T a :: Type
  type instance T a = Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  type family T a :: Type
  type instance T a = Bool
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
        name_chirho: "A3_assoc_family_impl_only",
        accepted_chirho: false,
        reason_chirho: "associated",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  dmeth :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  dmeth :: a -> a
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
        name_chirho: "A4_assoc_family_no_default_both",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class D (a :: Type) where
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
