-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators #-}
module PromotedPrefixChirho where
import Data.Proxy (Proxy)
prefixChirho :: Proxy ('(:) Int '[]) -> Proxy '[Int]
prefixChirho valueChirho = valueChirho
followingChirho :: Int
followingChirho = 1
