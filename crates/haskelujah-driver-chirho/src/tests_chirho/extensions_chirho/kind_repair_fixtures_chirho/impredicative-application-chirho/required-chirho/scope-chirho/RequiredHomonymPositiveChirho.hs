-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables #-}
module RequiredHomonymPositiveChirho where
data KChirho = KChirho
consumeChirho :: forall (aChirho :: KChirho) -> Int
consumeChirho _ = 0
valueChirho :: Int
valueChirho = consumeChirho KChirho
