-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance adds one.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

doubleChirho :: Prelude.Num b => b -> b
doubleChirho valueChirho = valueChirho + valueChirho + 0

-- A reference to a compiler-constrained function at the binding's own dictionary.
quadrupleChirho :: Prelude.Num b => b -> b
quadrupleChirho valueChirho = doubleChirho (doubleChirho valueChirho)

main :: IO ()
main = do
  print (quadrupleChirho (5 :: Int))
  print (quadrupleChirho (1.5 :: Double))
