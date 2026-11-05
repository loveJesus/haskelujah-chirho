-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, EmptyDataDecls, DataKinds, KindSignatures, PolyKinds, MagicHash, TypeFamilies, UnliftedNewtypes #-}
module ScalarNewtypeResultChirho where
import GHC.Exts (TYPE, RuntimeRep(..), Levity(..), Int#)
newtype BoxChirho :: TYPE 'IntRep where
  MkChirho :: Int# -> BoxChirho
