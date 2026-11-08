-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeApplications #-}
module NestedInferredPositiveChirho where
selectChirho :: (forall {aChirho} bChirho. aChirho -> bChirho -> aChirho) -> Bool
selectChirho functionChirho = functionChirho @Bool True False
