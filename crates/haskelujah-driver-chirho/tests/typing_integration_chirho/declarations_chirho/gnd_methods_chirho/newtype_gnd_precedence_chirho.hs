-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

instance DescribeChirho Int where
  describeChirho _ = "int"

newtype WrapChirho = WrapChirho Int deriving (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho (WrapChirho 5))
