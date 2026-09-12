-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, TypeApplications, TypeAbstractions, StandaloneKindSignatures #-}
module Main where
import Data.Kind (Type)

type TokenChirho :: forall kindChirho. Type
data TokenChirho @kindChirho = TokenChirho

consumeChirho :: TokenChirho @Bool -> Int
consumeChirho _ = 42

main = print (consumeChirho (TokenChirho :: TokenChirho @Bool))
