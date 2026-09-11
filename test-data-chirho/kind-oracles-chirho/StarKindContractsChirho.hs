-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators, StandaloneKindSignatures #-}
module StarKindContractsChirho where
import Data.Kind (Type)
import qualified GHC.TypeNats as NatChirho

type BoxChirho :: * -> *
data BoxChirho aChirho = BoxChirho aChirho
type ProductChirho (aChirho :: NatChirho.Nat) (bChirho :: NatChirho.Nat) = aChirho NatChirho.* bChirho
