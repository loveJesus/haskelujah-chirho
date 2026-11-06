-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

showBothChirho :: (Show aChirho, Show bChirho) => [aChirho] -> [bChirho] -> IO ()
showBothChirho lettersChirho flagsChirho = do
  mapM_ print lettersChirho
  mapM_ print flagsChirho

main :: IO ()
main = showBothChirho "ab" [True, False]
