-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MagicHash, DataKinds, KindSignatures #-}
module ExtsIntHashKindChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
import qualified GHC.Exts as PChirho (Int#)
import Data.Proxy (Proxy(..))
type CheckedChirho = Proxy (PChirho.Int# :: TYPE 'IntRep)
witnessChirho :: CheckedChirho
witnessChirho = Proxy
