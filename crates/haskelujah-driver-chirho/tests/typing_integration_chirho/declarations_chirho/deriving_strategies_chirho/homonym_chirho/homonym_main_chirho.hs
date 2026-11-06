-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified KeysChirho as KeysChirho
data LockChirho = KeyChirho Int Int
sumChirho :: LockChirho -> Int
sumChirho (KeyChirho leftChirho rightChirho) = leftChirho + rightChirho
main :: IO ()
main = do
  putStrLn (KeysChirho.showKeyChirho 7)
  print (sumChirho (KeyChirho 3 4))
