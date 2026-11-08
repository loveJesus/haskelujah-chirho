-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import qualified Data.Foldable as F

-- A qualified class from a module other than the Prelude: `F.Foldable` names the class whose
-- method `F.foldr` the body uses.
totalChirho :: (F.Foldable t, Num a) => t a -> a
totalChirho = F.foldr (+) 0

main :: IO ()
main = print (totalChirho [1, 2, 3 :: Int])
