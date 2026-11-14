-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module MonomorphicNumericChirho where
import GHC.Internal.Num ((+))
restrictedChirho = \xChirho -> xChirho + 1
resultChirho = restrictedChirho 2
