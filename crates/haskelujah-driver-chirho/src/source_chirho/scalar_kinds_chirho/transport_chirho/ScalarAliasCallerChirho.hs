-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MagicHash, DataKinds, KindSignatures #-}
module ScalarAliasCallerChirho where
import ScalarSupplierChirho (ScalarAliasChirho)
import GHC.Exts (TYPE, RuntimeRep(..))
import Data.Proxy (Proxy(..))
data Int# = LocalIntChirho
type WitnessChirho = Proxy (ScalarAliasChirho :: TYPE 'IntRep)
witnessChirho :: WitnessChirho
witnessChirho = Proxy
