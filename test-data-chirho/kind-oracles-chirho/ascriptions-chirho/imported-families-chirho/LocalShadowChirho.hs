-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeOperators #-}
module LocalShadowChirho where
import Data.Kind (Type)
import Data.Type.Equality ((:~:)(Refl))
import ProviderChirho
data CanDoChirho aChirho = LocalChirho aChirho
type family ProjectChirho (wrappedChirho :: Type) :: Type where
  ProjectChirho (LocalShadowChirho.CanDoChirho aChirho) = aChirho
proofChirho :: ProjectChirho (LocalShadowChirho.CanDoChirho Int) :~: Int
proofChirho = Refl
