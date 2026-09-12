-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, DataKinds, PolyKinds, TypeApplications, KindSignatures, RankNTypes #-}
module AnnotatedKindChirho where
import Data.Kind (Type)
data WitnessChirho :: forall kindChirho. kindChirho -> Type
probeChirho :: forall (indexChirho :: WitnessChirho @Type Int). WitnessChirho indexChirho -> ()
probeChirho _ = ()
