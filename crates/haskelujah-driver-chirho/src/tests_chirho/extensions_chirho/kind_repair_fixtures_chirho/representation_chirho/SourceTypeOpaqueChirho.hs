-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, ExplicitForAll, TypeApplications #-}
module SourceTypeOpaqueChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
import qualified GHC.Exts as RChirho
data TYPE (a_chirho :: Type) = SourceChirho
valueChirho :: Proxy (TYPE Bool) -> Proxy (RChirho.TYPE RChirho.LiftedRep)
valueChirho = id
