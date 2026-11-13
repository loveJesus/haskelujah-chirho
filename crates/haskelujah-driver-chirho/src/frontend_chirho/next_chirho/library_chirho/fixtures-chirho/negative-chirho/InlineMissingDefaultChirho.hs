-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module InlineMissingDefaultChirho where
class EchoChirho aChirho where
 {-# MINIMAL echoChirho #-}
 echoChirho :: aChirho -> Char
 {-# INLINE echoChirho #-}
 laterChirho :: aChirho -> Char
 laterChirho xChirho = echoChirho xChirho
instance EchoChirho Bool
resultChirho = laterChirho True
