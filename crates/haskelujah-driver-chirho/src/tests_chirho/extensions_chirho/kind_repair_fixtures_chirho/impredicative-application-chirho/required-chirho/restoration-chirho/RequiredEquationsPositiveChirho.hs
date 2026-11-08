-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables, GADTs, FlexibleInstances #-}
module RequiredEquationsPositiveChirho where
chooseChirho :: forall aChirho -> Bool -> aChirho -> aChirho
chooseChirho tChirho True (xChirho :: tChirho) = xChirho :: tChirho
chooseChirho uChirho False (yChirho :: uChirho) = yChirho :: uChirho
valueChirho :: Int
valueChirho = chooseChirho Int False 3
