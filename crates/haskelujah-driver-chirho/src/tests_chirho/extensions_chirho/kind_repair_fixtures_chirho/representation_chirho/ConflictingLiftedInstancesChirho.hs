-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, TypeFamilies #-}
module ConflictingLiftedInstancesChirho where
import Data.Kind (Type)
import GHC.Exts (TYPE, LiftedRep)
type family FChirho (kChirho :: Type) :: Type
type instance FChirho Type = Int
type instance FChirho (TYPE LiftedRep) = Bool
