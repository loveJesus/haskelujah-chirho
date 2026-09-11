-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, StandaloneKindSignatures, TypeFamilies, TypeOperators #-}
module FamilyRequiredRowsChirho where
import Data.Kind (Type)
import Data.Type.Equality
type CastChirho :: forall aChirho bChirho -> (aChirho :~: bChirho) -> (aChirho -> bChirho)
type family CastChirho aChirho bChirho equalityChirho valueChirho where
  CastChirho _ _ Refl valueChirho = valueChirho
