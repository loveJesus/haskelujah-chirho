-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# OPTIONS_GHC -fdefer-type-errors #-}
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures #-}
module DeferredBoxedLevityFieldChirho where
import GHC.Exts (TYPE, RuntimeRep(..), Levity)
data BoxChirho (lChirho :: Levity) (aChirho :: TYPE (BoxedRep lChirho)) = MkChirho aChirho
