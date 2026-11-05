-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, PolyKinds, KindSignatures, RankNTypes, ScopedTypeVariables, MagicHash #-}
module AnnotatedUnboxedLambdaChirho where
import GHC.Exts (Int#)
valueChirho :: Int# -> Int#
valueChirho = \(xChirho :: Int#) -> xChirho
