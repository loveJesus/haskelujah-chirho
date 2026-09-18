// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
[
    ("plain_class_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (Type -> Type) -> Constraint) (fChirho :: Type -> Type). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("quantified_superclass_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (Type -> Type) -> Constraint) (fChirho :: Type -> Type). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("with_instance_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instance (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (Type -> Type) -> Constraint) (fChirho :: Type -> Type). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("star_spelling_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (* -> *) -> Constraint) (fChirho :: * -> *). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("kind_contradiction_chirho", false, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: Type -> Constraint) (fChirho :: Type -> Type). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("prior_polymorphic_consumer_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instAnyChirho :: forall pChirho aChirho. DictChirho (ForallChirho pChirho) -> DictChirho (pChirho aChirho)
instAnyChirho = undefined
instChirho :: forall (pChirho :: (Type -> Type) -> Constraint) (fChirho :: Type -> Type). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("parenthesized_atomic_stars_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: ((*) -> (*)) -> Constraint) (fChirho :: (*) -> (*)). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("nested_star_arrow_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: ((* -> *)) -> Constraint) (fChirho :: * -> *). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("bare_parenthesized_star_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (*) -> Constraint) (fChirho :: (*)). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("no_star_is_type_chirho", false, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoStarIsType, PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: (* -> *) -> Constraint) (fChirho :: * -> *). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
    ("arrow_constructor_chirho", true, r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, QuantifiedConstraints, GADTs, RankNTypes, KindSignatures, FlexibleContexts, FlexibleInstances, UndecidableInstances, UndecidableSuperClasses, TypeOperators, ConstraintKinds #-}
module ProbeChirho where
import Data.Kind (Type, Constraint)
data DictChirho (cChirho :: Constraint) where DictChirho :: cChirho => DictChirho cChirho
class (forall aChirho. pChirho aChirho) => ForallChirho (pChirho :: kChirho -> Constraint)
instChirho :: forall (pChirho :: ((->) Type Type) -> Constraint) (fChirho :: * -> *). DictChirho (ForallChirho pChirho) -> DictChirho (pChirho fChirho)
instChirho = undefined
"###),
]
