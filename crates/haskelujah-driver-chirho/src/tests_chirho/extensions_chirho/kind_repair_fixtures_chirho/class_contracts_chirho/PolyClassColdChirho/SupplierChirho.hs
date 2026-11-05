-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, StandaloneKindSignatures #-}
module SupplierChirho (CChirho, proxyChirho) where
import Data.Kind (Type, Constraint)
import Data.Proxy (Proxy)
type CChirho :: forall kChirho. kChirho -> Constraint
class CChirho aChirho where
  proxyChirho :: Proxy aChirho
