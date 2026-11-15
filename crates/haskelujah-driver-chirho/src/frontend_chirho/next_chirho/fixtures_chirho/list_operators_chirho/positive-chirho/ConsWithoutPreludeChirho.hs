-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module ConsWithoutPreludeChirho where
firstChirho (xChirho:_) = xChirho
secondChirho (_:yChirho:_) = yChirho
resultChirho = firstChirho ('q' : [])
secondResultChirho = secondChirho ('a' : 'b' : [])
constructorChirho = (:)
constructorResultChirho = firstChirho (constructorChirho 'c' [])
