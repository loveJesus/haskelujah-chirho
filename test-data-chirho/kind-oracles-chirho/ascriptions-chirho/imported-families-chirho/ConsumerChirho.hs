-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeOperators #-}
module ConsumerChirho where
import Data.Kind (Type)
import Data.Type.Equality ((:~:)(Refl))
import ProviderChirho
data StateChirho (sChirho :: Type) (mChirho :: Type -> Type) aChirho
data ReaderChirho sChirho
type instance CanDoChirho (StateChirho sChirho mChirho) effChirho = StateCanDoChirho sChirho effChirho
type family StateCanDoChirho sChirho effChirho where
  StateCanDoChirho sChirho (ReaderChirho sChirho) = 'True
  StateCanDoChirho sChirho effChirho = 'False
proofChirho :: CanDoChirho (StateChirho Int Maybe) (ReaderChirho Int) :~: 'True
proofChirho = Refl
