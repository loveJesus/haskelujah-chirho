-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeOperators, StandaloneKindSignatures, KindSignatures #-}
module PromotedTuplesChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
import GHC.TypeLits (Nat)
data HolderChirho (pairChirho :: (Bool, Nat))
type GoodChirho = HolderChirho '( 'True, 3)
