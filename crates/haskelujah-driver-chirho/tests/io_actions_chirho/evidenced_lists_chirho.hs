-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data OpChirho = AddChirho | MulChirho deriving (Eq, Show)
data TokenChirho = TokenChirho
instance Show TokenChirho where
  show _ = "custom-chirho"
stepsChirho = [(AddChirho,2),(MulChirho,3)]
main = do
  print [1,2,3]
  print stepsChirho
  print ([] :: [Bool])
  print [True,False]
  print [[1],[2,3]]
  print [Just (-42 :: Int),Nothing]
  print [1.5,-2.25]
  print [TokenChirho,TokenChirho]
