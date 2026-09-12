-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies #-}
module ProviderChirho where
import Data.Kind (Type)
type family CanDoChirho (mChirho :: Type -> Type) (effChirho :: kChirho) :: Bool
