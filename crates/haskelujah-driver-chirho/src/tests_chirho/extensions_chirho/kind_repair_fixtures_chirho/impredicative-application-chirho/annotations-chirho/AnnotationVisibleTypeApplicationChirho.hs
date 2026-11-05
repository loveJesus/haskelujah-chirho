-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeApplications, ScopedTypeVariables #-}
module AnnotationVisibleTypeApplicationChirho where
valueChirho = ((id :: forall aChirho. aChirho -> aChirho) @Int) 3
