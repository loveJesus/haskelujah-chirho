-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeApplications #-}
module ReflectionValueCorrectChirho where
import qualified Type.Reflection as ReflectionChirho
valueChirho :: ReflectionChirho.TypeRep Int
valueChirho = ReflectionChirho.typeRep @Int
