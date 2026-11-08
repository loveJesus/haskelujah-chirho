-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

{-# LANGUAGE TemplateHaskellQuotes #-}
module PatternSpliceQuotesChirho where

import Language.Haskell.TH

exprChirho :: IO Exp
exprChirho = runQ $ do
  nameChirho <- newName "fooChirho"
  [| do $(varP nameChirho) <- pure (); pure () |]
