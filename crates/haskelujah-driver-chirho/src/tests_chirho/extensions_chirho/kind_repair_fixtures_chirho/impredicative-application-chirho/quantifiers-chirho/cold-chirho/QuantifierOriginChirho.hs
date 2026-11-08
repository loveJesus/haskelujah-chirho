-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module QuantifierOriginChirho (exposeChirho) where
exposeChirho :: () -> (forall {aChirho} bChirho. aChirho -> bChirho -> aChirho)
exposeChirho () valueChirho _ = valueChirho
