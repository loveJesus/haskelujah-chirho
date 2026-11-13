-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module UnknownInlineChirho where
class EchoChirho aChirho where
 {-# MINIMAL echoChirho #-}
 echoChirho :: aChirho -> Char
 {-# INLINE missingChirho #-}
 echoChirho _ = 'p'
 laterChirho :: aChirho -> Char
 laterChirho xChirho = echoChirho xChirho
instance EchoChirho Bool
resultChirho = laterChirho True
