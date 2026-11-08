-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ScopedTypeVariables #-}
module ScopedAnnotationShadowChirho where
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho xChirho = let unrelatedChirho = (undefined :: forall aChirho. Maybe aChirho) in (xChirho :: aChirho)
