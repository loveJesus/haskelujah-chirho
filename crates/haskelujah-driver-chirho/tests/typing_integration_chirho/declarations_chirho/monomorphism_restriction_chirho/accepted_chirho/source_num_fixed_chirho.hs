-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance adds one, so the
-- builtin dictionary would print 7 where this program prints 8.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

-- Restricted, and fixed by the use below at Int through the program's own class.
sevenChirho = fromInteger 7

main :: IO ()
main = print (sevenChirho :: Int)
