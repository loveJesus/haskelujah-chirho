-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, GADTs, PatternSynonyms, ExplicitForAll #-}
module ExistentialArgumentChirho where

data SomeChirho where
  MkSomeChirho :: forall valueChirho. valueChirho -> SomeChirho

pattern SomePatternChirho :: () => forall valueChirho. valueChirho -> SomeChirho
pattern SomePatternChirho valueChirho <- MkSomeChirho valueChirho
