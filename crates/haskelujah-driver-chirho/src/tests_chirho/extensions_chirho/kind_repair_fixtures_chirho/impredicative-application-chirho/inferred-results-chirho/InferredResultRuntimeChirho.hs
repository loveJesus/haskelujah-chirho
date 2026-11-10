-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
applyBothChirho :: (forall aChirho. aChirho -> aChirho) -> (Bool, Int)
applyBothChirho functionChirho = (functionChirho True, functionChirho 7)
chooseChirho flagChirho = case flagChirho of
  True -> applyBothChirho
  False -> applyBothChirho
answerChirho = chooseChirho True id
main :: IO ()
main = print (answerChirho, chooseChirho False (\valueChirho -> valueChirho))
