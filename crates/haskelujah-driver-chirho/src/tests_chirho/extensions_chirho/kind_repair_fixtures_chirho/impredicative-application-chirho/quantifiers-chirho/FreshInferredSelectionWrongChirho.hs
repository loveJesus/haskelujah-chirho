-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, RankNTypes, KindSignatures, DataKinds, TypeApplications #-}
module FreshInferredSelectionWrongChirho where
valueChirho :: Int
valueChirho = let identityChirho xChirho = xChirho in identityChirho @Int 3
