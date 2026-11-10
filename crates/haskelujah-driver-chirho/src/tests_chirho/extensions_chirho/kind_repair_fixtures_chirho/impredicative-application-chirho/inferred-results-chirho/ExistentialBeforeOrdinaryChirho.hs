-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ExistentialQuantification #-}
module Main where
data BoxChirho = EmptyChirho | forall aChirho. PackChirho aChirho
higherChirho :: (forall bChirho. bChirho -> bChirho) -> ()
higherChirho _ = ()
badChirho valueChirho = case valueChirho of
  PackChirho _ -> higherChirho
  EmptyChirho -> higherChirho
main :: IO ()
main = return ()
