-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude
import qualified Prelude as P (RealFloat(..))

main :: IO ()
main = do
  let x = 6.0 :: Double
  print (P.floatRadix x, P.floatDigits x, P.floatRange x, P.decodeFloat x, P.exponent x, P.significand x)
  print (P.encodeFloat 3 2 :: Double, P.scaleFloat 1 x, P.isNaN x, P.isInfinite x, P.isDenormalized x, P.isNegativeZero x, P.isIEEE x, P.atan2 x 1.0)
