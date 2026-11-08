-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Eq, (==))
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Double instance never agrees.
class Eq a where
  (==) :: a -> a -> Bool

instance Eq Double where
  _ == _ = False

-- The compiler's Ord, whose Eq superclass serves `Prelude.==`.
sameChirho :: Ord a => a -> a -> Bool
sameChirho leftChirho rightChirho = leftChirho Prelude.== rightChirho

-- The use demands the compiler's `Ord [Double]` dictionary itself.
main :: IO ()
main = print (sameChirho [1.5] [1.5 :: Double])
