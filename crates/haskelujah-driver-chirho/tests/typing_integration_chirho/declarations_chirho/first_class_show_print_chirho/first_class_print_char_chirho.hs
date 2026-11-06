-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

showAllChirho :: Show aChirho => [aChirho] -> IO ()
showAllChirho valuesChirho = mapM_ print valuesChirho

main :: IO ()
main = showAllChirho "ab"
