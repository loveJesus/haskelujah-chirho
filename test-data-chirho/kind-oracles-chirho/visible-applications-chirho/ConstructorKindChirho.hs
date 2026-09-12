-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures, TypeApplications, DataKinds #-}
module Main where
data BoxChirho (valueChirho :: kindChirho) = BoxChirho Int
madeChirho :: BoxChirho @Bool True
madeChirho = BoxChirho 42
getChirho :: BoxChirho @Bool True -> Int
getChirho (BoxChirho numberChirho) = numberChirho
identityChirho :: BoxChirho valueChirho -> BoxChirho valueChirho
identityChirho valueChirho = valueChirho
main = print (getChirho (identityChirho madeChirho))
