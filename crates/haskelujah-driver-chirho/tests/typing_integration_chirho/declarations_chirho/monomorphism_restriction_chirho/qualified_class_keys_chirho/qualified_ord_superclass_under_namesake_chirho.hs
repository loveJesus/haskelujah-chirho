-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Ord, compare)
import qualified Prelude

-- The program's own class, spelled like the Prelude's, with no superclass.
class Ord a where
  compare :: a -> a -> Bool

-- The Prelude's Ord keeps its own superclass, the Prelude's Eq, which serves `==`.
sameChirho :: Prelude.Ord b => b -> b -> Bool
sameChirho leftChirho rightChirho = leftChirho == rightChirho

main :: IO ()
main = do
  print (sameChirho (3 :: Int) 3)
  print (sameChirho 'a' 'b')
