-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
consumeChirho :: (forall aChirho. aChirho -> aChirho) -> Bool
consumeChirho functionChirho = functionChirho True
badChirho functionChirho = consumeChirho functionChirho
main :: IO ()
main = return ()
