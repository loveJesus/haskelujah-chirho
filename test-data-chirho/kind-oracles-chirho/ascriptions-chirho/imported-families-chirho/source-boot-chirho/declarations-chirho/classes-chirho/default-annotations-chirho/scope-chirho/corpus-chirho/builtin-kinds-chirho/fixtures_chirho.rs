// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
[
    BuiltinKindCaseChirho {
        name_chirho: "monoid_associated_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid (First)
class WrappedChirho sChirho where
  type UnwrappedChirho sChirho
instance WrappedChirho (First aChirho) where
  type UnwrappedChirho (First aChirho) = Maybe aChirho
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "qualified_associated_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import qualified Data.Monoid as MChirho
class WrappedChirho sChirho where
  type UnwrappedChirho sChirho
instance WrappedChirho (MChirho.First aChirho) where
  type UnwrappedChirho (MChirho.First aChirho) = Maybe aChirho
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "semigroup_associated_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Semigroup (First)
class WrappedChirho sChirho where
  type UnwrappedChirho sChirho
instance WrappedChirho (First aChirho) where
  type UnwrappedChirho (First aChirho) = Maybe aChirho
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "monoid_wrong_argument_chirho",
        accepts_chirho: false,
        error_marker_chirho: "kind mismatch",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid (First)
type BadChirho = First Maybe
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "semigroup_wrong_argument_chirho",
        accepts_chirho: false,
        error_marker_chirho: "kind mismatch",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import qualified Data.Semigroup as SChirho
type BadChirho = SChirho.First 'True
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "hidden_first_chirho",
        accepts_chirho: false,
        error_marker_chirho: "type not in scope",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid hiding (First)
type BadChirho = First Int
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "local_shadow_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import qualified Data.Monoid as MChirho
import Data.Kind (Type)
data First (fChirho :: Type -> Type)
type GoodChirho = First Maybe
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "unrelated_provider_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("OtherChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module OtherChirho (First) where
import Data.Kind (Type)
data First (fChirho :: Type -> Type)
"##),
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import OtherChirho (First)
type GoodChirho = First Maybe
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "source_provider_shadow_chirho",
        accepts_chirho: true,
        error_marker_chirho: "",
        files_chirho: &[
            ("Data/Monoid.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Monoid (First) where
import Data.Kind (Type)
data First (fChirho :: Type -> Type)
"##),
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid (First)
type GoodChirho = First Maybe
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "source_provider_wrong_argument_chirho",
        accepts_chirho: false,
        error_marker_chirho: "kind mismatch",
        files_chirho: &[
            ("Data/Monoid.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures #-}
module Data.Monoid (First) where
import Data.Kind (Type)
data First (fChirho :: Type -> Type)
"##),
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid (First)
type BadChirho = First Int
"##),
        ],
    },
    BuiltinKindCaseChirho {
        name_chirho: "contradictory_associated_chirho",
        accepts_chirho: false,
        error_marker_chirho: "kind mismatch",
        files_chirho: &[
            ("ProbeChirho.hs", r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleInstances, KindSignatures, PolyKinds, DataKinds #-}
module ProbeChirho where
import Data.Monoid (First)
class WrappedChirho sChirho where
  type UnwrappedChirho sChirho
instance WrappedChirho (First aChirho) where
  type UnwrappedChirho (First aChirho) = First Maybe
"##),
        ],
    },
]
