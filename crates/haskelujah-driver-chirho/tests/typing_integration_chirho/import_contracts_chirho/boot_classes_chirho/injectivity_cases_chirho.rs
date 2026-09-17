// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Exact GHC9.14.1 sources for associated injectivity validity and set agreement.
use super::ClassCaseChirho;
pub(super) static CASES_CHIRHO: &[ClassCaseChirho] = &[
    ClassCaseChirho {
        name_chirho: "J1_unknown_rhs_variable",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> missing
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> missing
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
        reason_chirho: "injectivity names an unbound family parameter",
    },
    ClassCaseChirho {
        name_chirho: "J2_wrong_lhs_variable",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | other -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | other -> a
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
        reason_chirho: "injectivity must name the declared result binder",
    },
    ClassCaseChirho {
        name_chirho: "J3_duplicate_position",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a a
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
        reason_chirho: "",
    },
    ClassCaseChirho {
        name_chirho: "J4_valid_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a
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
        reason_chirho: "",
    },
    ClassCaseChirho {
        name_chirho: "J5_positions_order_swapped",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> a b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> b a
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
        reason_chirho: "",
    },
    ClassCaseChirho {
        name_chirho: "J6_duplicate_vs_single",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a a
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type F a = (r :: Type) | r -> a
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
        reason_chirho: "",
    },
    ClassCaseChirho {
        name_chirho: "J7_identical_two_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> a b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> a b
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
        reason_chirho: "",
    },
    ClassCaseChirho {
        name_chirho: "J8_subset_negative",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> a b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, TypeFamilyDependencies #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  type F a b = (r :: Type) | r -> a
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
        reason_chirho: "associated type defaults or dependencies differ",
    },
];
