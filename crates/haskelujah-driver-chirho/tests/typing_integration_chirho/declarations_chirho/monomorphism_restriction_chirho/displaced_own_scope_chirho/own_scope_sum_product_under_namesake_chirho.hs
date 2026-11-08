-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance adds one.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

-- The Prelude's sum and product at the binding's own compiler Num dictionary.
totalChirho :: Prelude.Num b => [b] -> b
totalChirho valuesChirho = sum valuesChirho + product valuesChirho

main :: IO ()
main = do
  print (totalChirho [1, 2, 3 :: Int])
  print (totalChirho [1.5, 2.0 :: Double])
