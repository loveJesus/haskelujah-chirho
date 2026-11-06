-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude
import qualified Prelude as P (RealFrac(..))

main :: IO ()
main = do
  let x = 2.5 :: Double
  print (P.properFraction x :: (Int, Double))
  print (P.truncate x :: Int, P.round x :: Int, P.ceiling x :: Int, P.floor x :: Int)
