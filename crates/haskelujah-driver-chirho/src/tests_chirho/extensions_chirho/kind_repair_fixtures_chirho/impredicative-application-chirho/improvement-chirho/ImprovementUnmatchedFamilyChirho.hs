-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ScopedTypeVariables, FlexibleContexts #-}
module ImprovementUnmatchedFamilyChirho where
type family FChirho aChirho
type family GChirho aChirho
type family UnFChirho aChirho
fChirho :: forall bChirho. (UnFChirho (FChirho bChirho) ~ bChirho) => GChirho bChirho -> ()
fChirho _ = ()
