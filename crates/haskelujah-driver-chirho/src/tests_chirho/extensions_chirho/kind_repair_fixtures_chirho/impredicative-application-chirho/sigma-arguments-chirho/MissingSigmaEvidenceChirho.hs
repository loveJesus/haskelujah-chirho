-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module MissingSigmaEvidenceChirho where
data PolyChirho = PolyChirho (forall aChirho. Eq aChirho => aChirho -> String)
badChirho :: PolyChirho
badChirho = PolyChirho (\valueChirho -> show valueChirho)
