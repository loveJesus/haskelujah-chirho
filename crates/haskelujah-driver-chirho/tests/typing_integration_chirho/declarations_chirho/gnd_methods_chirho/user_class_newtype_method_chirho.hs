-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "default"
instance DescribeChirho Int where
  describeChirho valueChirho = show (valueChirho + 1)
newtype AgeChirho = AgeChirho Int deriving newtype DescribeChirho
main :: IO ()
main = putStrLn (describeChirho (AgeChirho 7))
