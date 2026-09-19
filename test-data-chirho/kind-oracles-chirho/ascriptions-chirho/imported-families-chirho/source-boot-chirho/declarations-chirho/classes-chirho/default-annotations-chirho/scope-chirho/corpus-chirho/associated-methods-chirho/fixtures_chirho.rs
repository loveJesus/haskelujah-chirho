// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
[
    ("imported_inferred_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho xChirho where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
    ("imported_annotated_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho (xChirho :: Type) where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
    ("imported_polykind_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho (xChirho :: kChirho) where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
    ("local_inferred_chirho", true, &[
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures #-}
module ConsumerChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho xChirho where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
    ("imported_no_default_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho xChirho where
  type TChirho xChirho
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
    ("imported_value_only_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho xChirho where
  type TChirho xChirho
  type TChirho xChirho = Int
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = ()
probeChirho :: TChirho IChirho
probeChirho = ()
"###),
    ]),
    ("contradictory_method_chirho", false, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho (xChirho :: Type) where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import ProviderChirho
data IChirho = IChirho Int
instance CChirho IChirho where
  type TChirho IChirho = Bool
  defChirho = ()
"###),
    ]),
    ("qualified_import_chirho", true, &[
        ("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ProviderChirho where
import Data.Kind (Type)
class Show (TChirho xChirho) => CChirho (xChirho :: Type) where
  type TChirho xChirho
  type TChirho xChirho = Int
  defChirho :: TChirho xChirho
"###),
        ("ConsumerChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, FlexibleContexts, AllowAmbiguousTypes, KindSignatures, PolyKinds #-}
module ConsumerChirho where
import qualified ProviderChirho as PChirho
data IChirho = IChirho Int
instance PChirho.CChirho IChirho where
  type TChirho IChirho = ()
  defChirho = ()
"###),
    ]),
]
