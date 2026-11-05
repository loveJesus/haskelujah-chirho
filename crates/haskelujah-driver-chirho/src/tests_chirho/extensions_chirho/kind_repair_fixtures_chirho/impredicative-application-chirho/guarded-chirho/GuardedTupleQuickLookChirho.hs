-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ImpredicativeTypes #-}
module Main where
consumeChirho :: (forall aChirho. aChirho -> aChirho, bChirho) -> ()
consumeChirho = undefined
knownChirho :: (forall aChirho. aChirho -> aChirho, forall aChirho. aChirho -> aChirho)
knownChirho = (id, id)
resultChirho :: ()
resultChirho = consumeChirho knownChirho
main :: IO ()
main = pure ()
