-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeAbstractions #-}
module ExplicitTypeSigmaArgumentChirho where
data PolyChirho = PolyChirho (forall aChirho. aChirho -> aChirho)
polyChirho :: PolyChirho
polyChirho = PolyChirho ((\ @aChirho valueChirho -> valueChirho))
