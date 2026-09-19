// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// GHC 9.14.1 scope/instance controls, including retained peer sources verbatim.
[
    ("K5_neg_wrong_kind_use.hs", false, r###"{-# LANGUAGE KindSignatures #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type -> Type) where
  mChirho :: aChirho -> Int
"###),
    ("K3a_kind_var_named_k.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ProbeChirho where
class CChirho (aChirho :: kChirho) where
  mChirho :: proxyChirho aChirho -> Int
"###),
    ("K1_explicit_kind_param.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho kChirho (aChirho :: kChirho) where
  mChirho :: proxyChirho aChirho -> Int
"###),
    ("K4_instances_at_two_kinds.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures, FlexibleInstances #-}
module ProbeChirho where
data ProxyChirho a = ProxyChirho
class CChirho (aChirho :: kChirho) where
  mChirho :: ProxyChirho aChirho -> Int
instance CChirho Int where
  mChirho _ = 1
instance CChirho Maybe where
  mChirho _ = 2
"###),
    ("K3b_kind_var_named_j.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ProbeChirho where
class CChirho (aChirho :: jChirho) where
  mChirho :: proxyChirho aChirho -> Int
"###),
    ("K8_class_kind_in_method.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ProbeChirho where
data ProxyChirho a = ProxyChirho
class CChirho (aChirho :: kChirho) where
  mChirho :: forall (bChirho :: kChirho). ProxyChirho aChirho -> ProxyChirho bChirho -> Int
"###),
    ("K2b_shadow_no_ambiguity.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures, RankNTypes #-}
module ProbeChirho where
data ProxyChirho a = ProxyChirho
class CChirho (aChirho :: kChirho) where
  mChirho :: forall kChirho. ProxyChirho aChirho -> kChirho -> Int
"###),
    ("K7_collision_both_roles.hs", true, r###"{-# LANGUAGE PolyKinds, KindSignatures #-}
module ProbeChirho where
data ProxyChirho a = ProxyChirho
class CChirho kChirho (aChirho :: kChirho) where
  mChirho :: ProxyChirho kChirho -> ProxyChirho aChirho -> Int
"###),
    ("method_parametric_chirho.hs", false, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, PolyKinds, KindSignatures, FlexibleInstances, RankNTypes, AllowAmbiguousTypes #-}
module ProbeChirho where
class CChirho aChirho where
  mChirho :: forall bChirho. aChirho -> bChirho -> bChirho
instance CChirho Int where
  mChirho _ _ = True
"###),
    ("associated_two_kinds_chirho.hs", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, PolyKinds, KindSignatures, FlexibleInstances, RankNTypes, AllowAmbiguousTypes #-}
module ProbeChirho where
class CChirho (aChirho :: kChirho) where
  type TChirho aChirho
  mChirho :: TChirho aChirho
instance CChirho Int where
  type TChirho Int = Bool
  mChirho = True
instance CChirho Maybe where
  type TChirho Maybe = ()
  mChirho = ()
"###),
    ("shadowed_family_kind_chirho.hs", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, PolyKinds, KindSignatures, FlexibleInstances, RankNTypes, AllowAmbiguousTypes #-}
module ProbeChirho where
data ProxyChirho aChirho = ProxyChirho
class CChirho (aChirho :: kChirho) where
  type TChirho aChirho
  mChirho :: forall kChirho. ProxyChirho aChirho -> kChirho -> TChirho aChirho
instance CChirho Int where
  type TChirho Int = Bool
  mChirho _ _ = True
instance CChirho Maybe where
  type TChirho Maybe = ()
  mChirho _ _ = ()
"###),
    ("external_family_two_kinds_chirho.hs", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, PolyKinds, KindSignatures, FlexibleInstances, RankNTypes, AllowAmbiguousTypes #-}
module ProbeChirho where
type family TChirho (aChirho :: kChirho)
type instance TChirho Int = Bool
type instance TChirho Maybe = ()
class CChirho (aChirho :: kChirho) where
  mChirho :: TChirho aChirho
instance CChirho Int where
  mChirho = True
instance CChirho Maybe where
  mChirho = ()
"###),
]
