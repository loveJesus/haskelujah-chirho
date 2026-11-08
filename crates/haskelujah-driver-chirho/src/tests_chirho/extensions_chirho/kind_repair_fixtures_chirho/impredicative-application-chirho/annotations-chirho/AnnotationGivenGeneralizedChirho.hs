-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ScopedTypeVariables, NoMonomorphismRestriction #-}
module AnnotationGivenGeneralizedChirho where
fooChirho = ((\xChirho -> xChirho == xChirho) :: forall aChirho. Eq aChirho => aChirho -> Bool)
