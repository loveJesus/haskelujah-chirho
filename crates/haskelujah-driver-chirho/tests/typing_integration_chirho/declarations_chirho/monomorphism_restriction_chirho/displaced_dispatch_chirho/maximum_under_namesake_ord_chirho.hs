-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Ord, compare)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance orders backwards.
class Ord a where
  compare :: a -> a -> Ordering

instance Ord Int where
  compare leftChirho rightChirho = Prelude.compare rightChirho leftChirho

-- The Prelude's maximum orders through the compiler's Ord.
main :: IO ()
main = print (maximum [3, 1, 2 :: Int])
