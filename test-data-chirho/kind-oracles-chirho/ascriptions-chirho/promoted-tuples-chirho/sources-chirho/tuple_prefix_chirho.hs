-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeOperators, StandaloneKindSignatures, KindSignatures #-}
module PromotedTuplesChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
import GHC.TypeLits (Nat)
pairChirho :: Proxy '(Int, Bool) -> Proxy ('(,) Int Bool)
pairChirho xChirho = xChirho
tripleChirho :: Proxy '(Int, Bool, Char) -> Proxy ('(,,) Int Bool Char)
tripleChirho xChirho = xChirho
unitChirho :: Proxy ('() :: ()) -> Proxy '()
unitChirho xChirho = xChirho
