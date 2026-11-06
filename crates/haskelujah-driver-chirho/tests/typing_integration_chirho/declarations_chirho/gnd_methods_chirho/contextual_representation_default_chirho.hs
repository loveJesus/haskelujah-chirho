-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- gpt #31791 counterexample for aeeed9f9. GHC 9.14.1 prediction (UNEXECUTED): ACCEPT, prints 78.
-- aeeed9f9 code reading: the newtype's default-filled row is replaced by the
-- "missing method DescribeChirho.describeChirho for [Int]" error stub, because the
-- representation instance is contextual and its ground row is built only after the GND pass.
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "default"

instance DescribeChirho Int where
  describeChirho valueChirho = show valueChirho

instance DescribeChirho aChirho => DescribeChirho [aChirho] where
  describeChirho valuesChirho = concatMap describeChirho valuesChirho

newtype AgesChirho = AgesChirho [Int] deriving newtype DescribeChirho

main :: IO ()
main = putStrLn (describeChirho (AgesChirho [7, 8]))
