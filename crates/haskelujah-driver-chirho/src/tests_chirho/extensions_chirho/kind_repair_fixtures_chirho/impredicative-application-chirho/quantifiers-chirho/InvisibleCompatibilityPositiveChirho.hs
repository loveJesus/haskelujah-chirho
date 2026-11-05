-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, RankNTypes, KindSignatures, DataKinds, TypeApplications #-}
module InvisibleCompatibilityPositiveChirho where
acceptChirho :: (forall {aChirho}. aChirho -> aChirho) -> Int
acceptChirho functionChirho = functionChirho (3 :: Int)
candidateChirho :: forall aChirho. aChirho -> aChirho
candidateChirho valueChirho = valueChirho
valueChirho :: Int
valueChirho = acceptChirho candidateChirho
