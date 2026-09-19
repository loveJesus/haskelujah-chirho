-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, FlexibleContexts #-}
module Main where
class TagChirho aChirho bChirho where
  tagChirho :: aChirho -> bChirho -> String
instance TagChirho Int Bool where
  tagChirho nChirho bChirho = show nChirho ++ "/" ++ show bChirho
instance TagChirho Int Char where
  tagChirho nChirho cChirho = show nChirho ++ "@" ++ [cChirho]
describeChirho :: Int -> String
describeChirho nChirho = tagChirho nChirho 'z'
main :: IO ()
main = putStrLn (describeChirho 3)
