// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Unchanged GHC9.14.1 reference sources; evidence JSONL retains hashes and commands.
use super::ClassCaseChirho;
pub(super) const CASES_CHIRHO: &[ClassCaseChirho] = &[
    ClassCaseChirho {
        name_chirho: "M0_T20588d_verbatim",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
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
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
class D (a :: Type) where
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
        name_chirho: "M1_method_type_differs",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> Int
  meth _ = 0
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
        name_chirho: "M2_impl_extra_method",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "M3_boot_default_impl_none",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
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
        name_chirho: "M4_boot_none_impl_default",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  meth :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  meth :: a -> a
  meth = id
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
        name_chirho: "M5_minimal_boot_only",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "M6_minimal_both",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE TypeFamilies #-}
module BootClassChirho where
import Data.Kind
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "O1_method_order_swapped",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m2 :: a -> Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m2 :: a -> Int
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
        name_chirho: "O2_method_order_same_control",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m2 :: a -> Int
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m2 :: a -> Int
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
        name_chirho: "O3_superclass_order_swapped",
        accepted_chirho: false,
        reason_chirho: "superclass",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class (Eq a, Show a) => C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class (Show a, Eq a) => C (a :: Type) where
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
        name_chirho: "O4_superclass_order_same_control",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class (Eq a, Show a) => C (a :: Type) where
  m1 :: a -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class (Eq a, Show a) => C (a :: Type) where
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
        name_chirho: "D1_default_body_text_differs",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m1 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m1 x = x
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
        name_chirho: "D2_default_body_semantically_different",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m1 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  m1 = undefined
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
        name_chirho: "D3_default_signature_differs",
        accepted_chirho: false,
        reason_chirho: "method",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  default m1 :: Show a => a -> a
  m1 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  default m1 :: Eq a => a -> a
  m1 = id
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
        name_chirho: "D4_default_signature_same_control",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  default m1 :: Show a => a -> a
  m1 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, DefaultSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  m1 :: a -> a
  default m1 :: Show a => a -> a
  m1 = id
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
        name_chirho: "Q1_boot_minimal_weaker_than_impl",
        accepted_chirho: false,
        reason_chirho: "MINIMAL",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "Q2_boot_minimal_stronger_disjunction",
        accepted_chirho: false,
        reason_chirho: "MINIMAL",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth | meth2 #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "Q3_boot_minimal_stronger_conjunction",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth, meth2 #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
        name_chirho: "Q4_minimal_equal_disjunction_control",
        accepted_chirho: true,
        reason_chirho: "boot contract",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth | meth2 #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures #-}
module BootClassChirho where
import Data.Kind (Type)
class C (a :: Type) where
  {-# MINIMAL meth | meth2 #-}
  meth :: a -> a
  meth = id
  meth2 :: a -> a
  meth2 = id
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
