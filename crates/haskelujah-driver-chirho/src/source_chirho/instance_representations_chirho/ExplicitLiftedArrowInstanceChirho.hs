-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleInstances, KindSignatures #-}
module ExplicitLiftedArrowInstanceChirho where
import Data.Kind (Type)
class EchoChirho pChirho where
  echoChirho :: pChirho -> pChirho
instance EchoChirho ((aChirho :: Type) -> bChirho) where
  echoChirho fChirho = \xChirho -> fChirho xChirho
