-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
instance DescribeChirho Int where
  describeChirho _ = "integer"
instance DescribeChirho Bool where
  describeChirho _ = "boolean"
newtype BoxChirho aChirho = BoxChirho aChirho deriving newtype (DescribeChirho)
{-# NOINLINE renderChirho #-}
renderChirho :: DescribeChirho aChirho => aChirho -> String
renderChirho valueChirho = describeChirho valueChirho
main :: IO ()
main = do
  putStrLn (renderChirho (BoxChirho (7 :: Int)))
  putStrLn (renderChirho (BoxChirho True))
