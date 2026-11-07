-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's.
class Num a where
  fromInteger :: Integer -> a

-- Restricted, with only the compiler's literal Num on its type.
countChirho = 3

main :: IO ()
main = print countChirho
