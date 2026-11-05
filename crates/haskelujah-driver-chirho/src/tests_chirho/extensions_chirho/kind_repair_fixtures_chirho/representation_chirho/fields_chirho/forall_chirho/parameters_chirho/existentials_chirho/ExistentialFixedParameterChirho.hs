-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, ExplicitForAll, GADTs, KindSignatures, MagicHash, PolyKinds, ScopedTypeVariables #-}
module ExistentialFixedParameterChirho where
import GHC.Exts (TYPE, RuntimeRep(..))
data PackedChirho where
  PackedChirho :: forall (aChirho :: TYPE IntRep). (aChirho -> aChirho) -> PackedChirho
consumeChirho :: PackedChirho -> ()
consumeChirho (PackedChirho fChirho) = let gChirho xChirho = fChirho xChirho in ()
