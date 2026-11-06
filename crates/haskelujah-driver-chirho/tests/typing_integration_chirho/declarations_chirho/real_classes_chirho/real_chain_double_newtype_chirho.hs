-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving #-}
module Main where

newtype MeterChirho = MeterChirho Double
  deriving (Show, Eq, Ord, Num, Real, Fractional, RealFrac, Floating, RealFloat)

realChirho :: Real a => a -> a -> (a, Bool)
realChirho x y = (x + y, x < y)

fracChirho :: RealFrac a => a -> a -> a
fracChirho x y = x / y

floatChirho :: RealFloat a => a -> a
floatChirho x = sqrt x

main :: IO ()
main = do
  print (realChirho (MeterChirho 1.5) (MeterChirho 2))
  print (fracChirho (MeterChirho 3) (MeterChirho 4))
  print (floatChirho (MeterChirho 16))
