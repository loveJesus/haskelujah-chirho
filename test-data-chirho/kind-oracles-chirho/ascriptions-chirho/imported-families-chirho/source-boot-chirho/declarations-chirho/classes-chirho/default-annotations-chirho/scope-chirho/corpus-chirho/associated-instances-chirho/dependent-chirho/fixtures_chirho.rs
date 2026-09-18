// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
// Generated from independent GHC reference bytes; do not edit the source or verdicts here.
&[
    (true, r########"d01_dependent_default_consumer"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
ok :: Proxy (F Int Bool 'True) -> Proxy 'True
ok = id
"########),
    ]),
    (true, r########"d02_nondependent_default_consumer"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C2 (a :: Type) where
  type G a (b :: Type) :: Type
  type G a b = [b]
instance C2 Int
ok :: Proxy (G Int Bool) -> Proxy [Bool]
ok = id
"########),
    ]),
    (true, r########"d03_two_kinds_same_instance"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
okB :: Proxy (F Int Bool 'False) -> Proxy 'False
okB = id
okO :: Proxy (F Int Ordering 'GT) -> Proxy 'GT
okO = id
"########),
    ]),
    (true, r########"d04_polykinded_class_two_instances"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C3 (a :: j) where
  type H a (k :: Type) (b :: k) :: k
  type H a k b = b
instance C3 Int
instance C3 Maybe
ok1 :: Proxy (H Int Bool 'True) -> Proxy 'True
ok1 = id
ok2 :: Proxy (H Maybe Ordering 'LT) -> Proxy 'LT
ok2 = id
"########),
    ]),
    (false, r########"d05_invisible_dependent_kind"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C4 (a :: Type) where
  type F2 a (b :: k) :: k
  type F2 a b = b
instance C4 Int
ok :: Proxy (F2 Int 'True) -> Proxy 'True
ok = id
okVisible :: Proxy (F2 Int @Bool 'True) -> Proxy 'True
okVisible = id
"########),
    ]),
    (true, r########"d06_explicit_equations_specialise_family_only_kind"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Char where
  type F Char Bool b = 'False
  type F Char Ordering b = 'LT
ok1 :: Proxy (F Char Bool 'True) -> Proxy 'False
ok1 = id
ok2 :: Proxy (F Char Ordering 'GT) -> Proxy 'LT
ok2 = id
"########),
    ]),
    (true, r########"d07_imported_dependent_default"########, &[
        (r########"Provider.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module Provider where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
"########),
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
import Provider
ok :: Proxy (F Int Bool 'True) -> Proxy 'True
ok = id
"########),
    ]),
    (false, r########"d08_contradictory_result"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
bad :: Proxy (F Int Bool 'True) -> Proxy 'False
bad = id
"########),
    ]),
    (false, r########"d09_argument_kind_contradicts_k"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
bad :: Proxy (F Int Bool 'LT) -> Proxy 'LT
bad = id
"########),
    ]),
    (false, r########"d10_result_used_at_wrong_kind"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Int
bad :: F Int Bool 'True
bad = undefined
"########),
    ]),
    (false, r########"d11_default_rhs_contradicts_dependent_result"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = 'True
"########),
    ]),
    (false, r########"d12_default_lhs_specialises_k"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a Bool b = b
"########),
    ]),
    (false, r########"d13_partial_equations_disable_the_default"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Char where
  type F Char Bool b = 'False
bad :: Proxy (F Char () '()) -> Proxy '()
bad = id
"########),
    ]),
    (false, r########"d14_explicit_equation_wrong_kind_rhs"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C (a :: Type) where
  type F a (k :: Type) (b :: k) :: k
  type F a k b = b
instance C Char where
  type F Char Bool b = 'LT
"########),
    ]),
    (true, r########"d05a_invisible_dependent_kind_implicit_use"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C4 (a :: Type) where
  type F2 a (b :: k) :: k
  type F2 a b = b
instance C4 Int
ok :: Proxy (F2 Int 'True) -> Proxy 'True
ok = id
"########),
    ]),
    (false, r########"d05b_invisible_dependent_kind_visible_application"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C4 (a :: Type) where
  type F2 a (b :: k) :: k
  type F2 a b = b
instance C4 Int
okVisible :: Proxy (F2 Int @Bool 'True) -> Proxy 'True
okVisible = id
"########),
    ]),
    (false, r########"d05c_class_kind_signature_does_not_specify_k"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
import Data.Kind (Constraint)
type C5 :: Type -> Constraint
class C5 a where
  type F3 a (b :: k) :: k
  type F3 a b = b
instance C5 Int
okVisible :: Proxy (F3 Int @Bool 'True) -> Proxy 'True
okVisible = id
"########),
    ]),
    (false, r########"d05d_invisible_kind_contradiction"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C4 (a :: Type) where
  type F2 a (b :: k) :: k
  type F2 a b = b
instance C4 Int
bad :: Proxy (F2 Int 'True) -> Proxy 'LT
bad = id
"########),
    ]),
    (true, r########"d05e_invisible_kind_two_kinds"########, &[
        (r########"D.hs"########, r########"{-# LANGUAGE TypeFamilies, DataKinds, PolyKinds, StandaloneKindSignatures #-}
module D where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class C4 (a :: Type) where
  type F2 a (b :: k) :: k
  type F2 a b = b
instance C4 Int
ok1 :: Proxy (F2 Int 'True) -> Proxy 'True
ok1 = id
ok2 :: Proxy (F2 Int 'GT) -> Proxy 'GT
ok2 = id
"########),
    ]),
]
