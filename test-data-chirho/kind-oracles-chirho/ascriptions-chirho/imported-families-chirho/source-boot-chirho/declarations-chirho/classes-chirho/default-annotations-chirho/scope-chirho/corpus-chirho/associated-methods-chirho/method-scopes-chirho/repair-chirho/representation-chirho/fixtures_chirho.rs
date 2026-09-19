// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Calibrated against GHC 9.14.1; the observations retain sources and reasons.
[
    ("runtime_type_family_method_chirho.hs", None, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude, PolyKinds, DataKinds, TypeFamilies, RankNTypes, FlexibleInstances, KindSignatures #-}
module ProbeChirho where
import Data.Kind (Type)
import Prelude (Bool(True))
type family ArrowChirho :: kChirho -> kChirho -> Type
type instance ArrowChirho = (->)
class CategoryChirho kChirho where
  identityChirho :: forall (aChirho :: kChirho). ArrowChirho aChirho aChirho
instance CategoryChirho Type where
  identityChirho valueChirho = valueChirho
"###),
    ("runtime_type_family_method_bad_chirho.hs", Some("E0200"), r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude, PolyKinds, DataKinds, TypeFamilies, RankNTypes, FlexibleInstances, KindSignatures #-}
module ProbeChirho where
import Data.Kind (Type)
import Prelude (Bool(True))
type family ArrowChirho :: kChirho -> kChirho -> Type
type instance ArrowChirho = (->)
class CategoryChirho kChirho where
  identityChirho :: forall (aChirho :: kChirho). ArrowChirho aChirho aChirho
instance CategoryChirho Type where
  identityChirho _ = True
"###),
    ("local_type_is_not_builtin_alias_fixed_chirho.hs", Some("E0200"), r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, MagicHash #-}
module ProbeChirho where
import GHC.Exts (TYPE, LiftedRep)
import Data.Proxy (Proxy)
data Type = LocalChirho
badChirho :: Proxy Type -> Proxy (TYPE LiftedRep)
badChirho = id
"###),
    ("promoted_instance_head_chirho.hs", None, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, FlexibleInstances, KindSignatures #-}
module ProbeChirho where
class ListClassChirho (valuesChirho :: [kindChirho])
instance ListClassChirho '[]
"###),
    ("unpromoted_instance_head_chirho.hs", Some("E0300"), r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, FlexibleInstances, KindSignatures #-}
module ProbeChirho where
class ListClassChirho (valuesChirho :: [kindChirho])
instance ListClassChirho []
"###),
    ("generated_empty_class_instances_chirho.hs", None, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE DeriveAnyClass, GeneralizedNewtypeDeriving #-}
module ProbeChirho where
class MarkerChirho aChirho
newtype FirstChirho aChirho = FirstChirho aChirho deriving MarkerChirho
newtype SecondChirho aChirho = SecondChirho (Maybe aChirho) deriving MarkerChirho
"###),
    ("associated_type_shadow_catchall_chirho.hs", Some("E0200"), r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, FlexibleInstances #-}
module ProbeChirho where
import GHC.Exts
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (representationChirho :: RuntimeRep) where
  type TYPE representationChirho :: Type
instance CChirho representationChirho where
  type TYPE representationChirho = Bool
badChirho :: Proxy Type -> Proxy Bool
badChirho = id
"###),
    ("associated_type_shadow_control_chirho.hs", None, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, FlexibleInstances #-}
module ProbeChirho where
import GHC.Exts
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (representationChirho :: RuntimeRep) where
  type TYPE representationChirho :: Type
instance CChirho representationChirho where
  type TYPE representationChirho = Bool
goodChirho :: Proxy (ProbeChirho.TYPE ('BoxedRep 'Lifted)) -> Proxy Bool
goodChirho = id
"###),
]
