-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeApplications #-}
module ReflectionValueWrongChirho where
import qualified Type.Reflection as ReflectionChirho
valueChirho :: ReflectionChirho.TypeRep Bool
valueChirho = ReflectionChirho.typeRep @Int
