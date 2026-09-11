-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, UndecidableInstances #-}
module ComposedInjectivityChirho where

type family EncodeChirho valueChirho = resultChirho | resultChirho -> valueChirho where
  EncodeChirho Int = Char
  EncodeChirho Bool = Ordering

type family WrapChirho valueChirho = resultChirho | resultChirho -> valueChirho where
  WrapChirho Char = Maybe Int
  WrapChirho Ordering = Maybe Bool

type family EraseChirho valueChirho where
  EraseChirho valueChirho = Char

type family PipelineChirho valueChirho = resultChirho | resultChirho -> valueChirho where
  PipelineChirho valueChirho = WrapChirho (EncodeChirho valueChirho)

checkChirho :: PipelineChirho Int -> Maybe Int
checkChirho valueChirho = valueChirho
