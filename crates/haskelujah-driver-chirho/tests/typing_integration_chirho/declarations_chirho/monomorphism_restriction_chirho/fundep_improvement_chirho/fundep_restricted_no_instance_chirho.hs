-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FunctionalDependencies #-}
module Main where

class DescribeChirho aChirho bChirho | aChirho -> bChirho where
  describeChirho :: aChirho -> bChirho

instance DescribeChirho Bool Int where
  describeChirho flagChirho = if flagChirho then 42 else 0

-- No instance has a Double in the determining position, so nothing fixes the result.
missingChirho = describeChirho (1.5 :: Double)

main :: IO ()
main = print (missingChirho + 1)
