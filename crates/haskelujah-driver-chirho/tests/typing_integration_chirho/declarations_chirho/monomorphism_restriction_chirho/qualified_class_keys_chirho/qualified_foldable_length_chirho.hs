-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import qualified Data.Foldable as F

-- `F.length` carries its own Foldable constraint, so the body asks for `Foldable t`, which only
-- the qualified given `F.Foldable t` can serve.
countChirho :: F.Foldable t => t a -> Int
countChirho = F.length

main :: IO ()
main = print (countChirho [True, False, True])
