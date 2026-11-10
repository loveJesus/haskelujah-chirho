-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
higherChirho :: (forall aChirho. aChirho -> aChirho) -> ()
higherChirho _ = ()
badChirho flagChirho = case flagChirho of
  True -> higherChirho
  False -> \functionChirho -> higherChirho functionChirho
main :: IO ()
main = return ()
