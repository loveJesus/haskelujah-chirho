-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. It is not a standard numeric class,
-- so no default applies to it.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

-- Restricted and never fixed: only Show and the program's own Num constrain it.
sevenChirho = fromInteger 7

main :: IO ()
main = print sevenChirho
