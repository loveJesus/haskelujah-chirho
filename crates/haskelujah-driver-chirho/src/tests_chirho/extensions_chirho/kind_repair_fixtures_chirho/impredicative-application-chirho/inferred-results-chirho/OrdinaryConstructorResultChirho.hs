-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

{-# LANGUAGE RankNTypes #-}
module Main where
data ChoiceChirho = FirstChirho | SecondChirho
higherChirho :: (forall aChirho. aChirho -> aChirho) -> (Bool, Int)
higherChirho fChirho = (fChirho True, fChirho 7)
chooseChirho choiceChirho = case choiceChirho of
  FirstChirho -> higherChirho
  SecondChirho -> higherChirho
main = print (chooseChirho FirstChirho id, chooseChirho SecondChirho id)
