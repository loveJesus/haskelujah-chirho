-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
higherChirho :: (forall aChirho. aChirho -> aChirho) -> ()
higherChirho _ = ()
cycleChirho = let ignoredChirho = cycleChirho in higherChirho
main :: IO ()
main = return ()
