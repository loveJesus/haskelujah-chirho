-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE StandaloneKindSignatures, PolyKinds, DataKinds, GADTs, RankNTypes, UnliftedNewtypes, ExplicitForAll, KindSignatures #-}
module ForallNewtypeFieldChirho where
import Data.Kind (Type)
import GHC.Exts (TYPE, RuntimeRep(..))
newtype NChirho = MkNChirho (forall (aChirho :: Type). aChirho)
