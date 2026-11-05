-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
module AgesChirho (AgeChirho (..), bumpChirho) where
newtype AgeChirho = AgeChirho { unAgeChirho :: Int }
  deriving newtype (Show, Num)
{-# NOINLINE bumpChirho #-}
bumpChirho :: AgeChirho -> AgeChirho
bumpChirho (AgeChirho nChirho) = AgeChirho (nChirho + 1)
