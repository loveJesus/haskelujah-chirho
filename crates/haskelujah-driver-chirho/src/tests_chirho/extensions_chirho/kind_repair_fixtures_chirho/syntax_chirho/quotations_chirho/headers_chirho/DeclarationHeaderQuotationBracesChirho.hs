-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TemplateHaskellQuotes #-}
module DeclarationHeaderQuotationBracesChirho where
import Language.Haskell.TH (Q, Dec)
quotationChirho :: Q [Dec]
quotationChirho = [d| { fChirho :: Maybe; fChirho = undefined } |]
