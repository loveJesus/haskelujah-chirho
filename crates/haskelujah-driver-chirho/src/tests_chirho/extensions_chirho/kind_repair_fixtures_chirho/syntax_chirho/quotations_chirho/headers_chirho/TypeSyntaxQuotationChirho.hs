-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TemplateHaskellQuotes #-}
module TypeSyntaxQuotationChirho where
import Language.Haskell.TH (Q, Type)
quotationChirho :: Q Type
quotationChirho = [t| Maybe Int Int |]
