-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Eq, (==))
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Double instance never agrees.
class Eq a where
  (==) :: a -> a -> Bool

instance Eq Double where
  _ == _ = False

-- The Prelude's elem compares through the compiler's Eq.
main :: IO ()
main = print (elem 2.5 [1.5, 2.5 :: Double])
