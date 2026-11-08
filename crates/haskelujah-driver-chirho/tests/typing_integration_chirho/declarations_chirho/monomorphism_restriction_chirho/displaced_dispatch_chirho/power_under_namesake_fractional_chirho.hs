-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Fractional, (/))
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Double instance divides to zero.
class Fractional a where
  (/) :: a -> a -> a

instance Fractional Double where
  _ / _ = 0

-- The Prelude's (^^) divides through the compiler's Fractional.
main :: IO ()
main = print (2 ^^ (-1 :: Int) :: Double)
