-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
data CompareChirho = CompareChirho (forall aChirho. Ord aChirho => aChirho -> aChirho -> Bool)
mkChirho :: Ord bChirho => bChirho -> CompareChirho
mkChirho ignoredChirho = CompareChirho (\leftChirho rightChirho -> leftChirho < rightChirho)
main :: IO ()
main = case mkChirho 'x' of
  CompareChirho lessChirho -> print (lessChirho (-2.5 :: Double) 1.0, lessChirho 'a' 'b')
