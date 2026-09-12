-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators #-}
module PromotedFieldChirho where
import Data.Proxy (Proxy)
data HolderChirho = HolderChirho { fieldChirho :: Proxy ('(:) Int '[]) }
readChirho :: HolderChirho -> Proxy '[Int]
readChirho valueChirho = fieldChirho valueChirho
