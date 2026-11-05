-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import AgesChirho
describeChirho :: AgeChirho -> Int
describeChirho (AgeChirho nChirho) = nChirho * 10
main :: IO ()
main = do
  print (AgeChirho 6)
  print (bumpChirho (AgeChirho 6))
  print (unAgeChirho (AgeChirho 8))
  print (describeChirho (AgeChirho 9))
  print (AgeChirho 3 + 4)
