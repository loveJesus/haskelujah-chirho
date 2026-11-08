-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, ExplicitForAll, TypeApplications #-}
module LiftedVtaChirho where
import Data.Kind (Type, Constraint)
import Data.Proxy (Proxy(..))
import GHC.Exts (TYPE, CONSTRAINT, RuntimeRep(..), Levity(..), LiftedRep, UnliftedRep)
identityChirho :: forall r_chirho (a_chirho :: TYPE r_chirho). Proxy a_chirho -> Proxy a_chirho
identityChirho x_chirho = x_chirho
valueChirho = identityChirho @LiftedRep @Int Proxy
