-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, GADTs #-}
module Main where
data GChirho aChirho where
  GIChirho :: GChirho Int
higherChirho :: (forall bChirho. bChirho -> bChirho) -> ()
higherChirho _ = ()
badChirho = case GIChirho of
  GIChirho -> higherChirho
main :: IO ()
main = return ()
