-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module SpecializedSigmaArgumentChirho where
data PolyChirho = PolyChirho (forall aChirho. aChirho -> aChirho)
badChirho :: PolyChirho
badChirho = PolyChirho (\valueChirho -> True)
