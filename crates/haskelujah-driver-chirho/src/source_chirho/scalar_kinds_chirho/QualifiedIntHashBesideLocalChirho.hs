-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE MagicHash, DataKinds, KindSignatures #-}
module QualifiedIntHashBesideLocalChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
import qualified GHC.Exts as PChirho (Int#)
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
data Int# = UserIntChirho
type LocalChirho = Proxy (Int# :: Type)
type PrimitiveChirho = Proxy (PChirho.Int# :: TYPE 'IntRep)
primitiveChirho :: PrimitiveChirho
primitiveChirho = Proxy
