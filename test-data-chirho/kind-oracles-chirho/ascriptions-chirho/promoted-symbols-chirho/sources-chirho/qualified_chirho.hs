-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators #-}
module PromotedQualifiedChirho where
import Data.Proxy (Proxy)
data PairChirho aChirho bChirho = aChirho :*: bChirho
qualifiedChirho :: Proxy ('(PromotedQualifiedChirho.:*:) Int Bool) -> Proxy (Int ':*: Bool)
qualifiedChirho valueChirho = valueChirho
