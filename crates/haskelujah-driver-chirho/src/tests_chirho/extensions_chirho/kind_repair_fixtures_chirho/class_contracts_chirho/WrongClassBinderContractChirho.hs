-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, ExplicitForAll, FlexibleContexts, KindSignatures, PolyKinds, StandaloneKindSignatures #-}
module WrongClassBinderContractChirho where
import Data.Kind (Type, Constraint)
import GHC.Exts (TYPE, RuntimeRep)
import Data.Proxy (Proxy(..))
type CChirho :: Type -> Constraint
class CChirho (aChirho :: Bool)
