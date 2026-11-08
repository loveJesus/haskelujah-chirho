-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables #-}
module RequiredLexicalWrongChirho where
idRequiredChirho :: forall aChirho -> aChirho -> aChirho
idRequiredChirho tChirho (xChirho :: tChirho) = True :: tChirho
valueChirho :: Int
valueChirho = idRequiredChirho Int 3
