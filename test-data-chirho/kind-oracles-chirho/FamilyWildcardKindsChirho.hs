-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeFamilies #-}
module FamilyWildcardKindsChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)

type family PickChirho (aChirho :: Bool) (bChirho :: Type) :: Type where
  PickChirho 'True bChirho = bChirho
  PickChirho _ bChirho = Bool

type family LookupChirho (aChirho :: Type) :: Type where
  LookupChirho (_ Char) = Bool
  LookupChirho _ = Int

proofChirho :: Proxy (LookupChirho (Maybe Char)) -> Proxy Bool
proofChirho xChirho = xChirho

type family SeparateChirho (aChirho :: Type) (bChirho :: Type) :: Type where
  SeparateChirho _ _ = Int

separateChirho :: Proxy (SeparateChirho Bool Char) -> Proxy Int
separateChirho xChirho = xChirho
