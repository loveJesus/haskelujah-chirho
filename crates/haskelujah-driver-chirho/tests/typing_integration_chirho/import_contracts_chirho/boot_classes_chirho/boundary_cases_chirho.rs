// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Independent GHC9.14.1 form and class-parameter-slot controls.
use super::ClassCaseChirho;
pub(super) const CASES_CHIRHO: &[ClassCaseChirho] = &[
    ClassCaseChirho {
        name_chirho: "V1_boot_data_family_impl_type_family",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
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
        reason_chirho: "different kinds or forms",
    },
    ClassCaseChirho {
        name_chirho: "V2_boot_type_family_impl_data_family",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  type family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data family T a :: Type
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
        reason_chirho: "different kinds or forms",
    },
    ClassCaseChirho {
        name_chirho: "V3_data_family_both_explicit_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data family T a :: Type
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
        reason_chirho: "different kinds or forms",
    },
    ClassCaseChirho {
        name_chirho: "V4_data_family_both_omitted_spelling_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data T a :: Type
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
        reason_chirho: "different kinds or forms",
    },
    ClassCaseChirho {
        name_chirho: "V5_data_family_mixed_spellings",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data family T a :: Type
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) where
  data T a :: Type
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
        reason_chirho: "different kinds or forms",
    },
    ClassCaseChirho {
        name_chirho: "V6_method_a_to_b_vs_b_to_a_no_fundep",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  m :: a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  m :: b -> a
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
        reason_chirho: "class method type",
    },
    ClassCaseChirho {
        name_chirho: "V7_method_alpha_renamed_control",
        sources_chirho: &[
            (
                "BootClassChirho.hs-boot",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (a :: Type) (b :: Type) where
  m :: a -> b
"####,
            ),
            (
                "BootClassChirho.hs",
                r####"{-# LANGUAGE KindSignatures, TypeFamilies, MultiParamTypeClasses #-}
module BootClassChirho where
import Data.Kind (Type)
class () => C (x :: Type) (y :: Type) where
  m :: x -> y
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
        reason_chirho: "different kinds or forms",
    },
];
