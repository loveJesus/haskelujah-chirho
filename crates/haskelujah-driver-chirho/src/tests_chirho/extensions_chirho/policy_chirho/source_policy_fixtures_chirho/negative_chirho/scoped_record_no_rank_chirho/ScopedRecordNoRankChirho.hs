-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
{-# OPTIONS_GHC -XScopedTypeVariables #-}
module ScopedRecordNoRankChirho where

newtype RecordChirho = MkRecordChirho
  { runRecordChirho :: forall valueChirho. valueChirho -> valueChirho }
