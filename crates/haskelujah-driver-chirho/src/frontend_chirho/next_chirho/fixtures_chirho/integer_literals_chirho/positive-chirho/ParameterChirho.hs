-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RebindableSyntax #-}
module ParameterChirho where
chooseChirho fromInteger = 12
resultChirho = chooseChirho (\ignoredChirho -> 'p')
