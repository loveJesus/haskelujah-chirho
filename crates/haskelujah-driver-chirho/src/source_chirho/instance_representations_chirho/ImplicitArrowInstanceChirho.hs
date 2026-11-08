-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleInstances #-}
module ImplicitArrowInstanceChirho where
class EchoChirho pChirho where
  echoChirho :: pChirho -> pChirho
instance EchoChirho (aChirho -> bChirho) where
  echoChirho fChirho = \xChirho -> fChirho xChirho
