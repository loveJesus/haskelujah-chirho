-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TemplateHaskellQuotes, ExplicitForAll, KindSignatures, ScopedTypeVariables #-}
module LocalSignatureQuotationBracesChirho where
import Language.Haskell.TH (Q, Exp)
import Data.Kind (Type)
fChirho :: forall (aChirho :: Type). aChirho -> aChirho
fChirho xChirho = xChirho
quotationChirho :: Q Exp
quotationChirho = [| let { fChirho :: forall (aChirho :: Maybe). aChirho -> aChirho; fChirho xChirho = xChirho } in fChirho |]
