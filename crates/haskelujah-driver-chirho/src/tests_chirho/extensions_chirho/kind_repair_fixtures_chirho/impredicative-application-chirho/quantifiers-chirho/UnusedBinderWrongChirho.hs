-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, RankNTypes, KindSignatures, DataKinds, TypeApplications #-}
module UnusedBinderWrongChirho where
import Data.Kind (Type)
acceptChirho :: (forall (aChirho :: Bool). ()) -> ()
acceptChirho _ = ()
candidateChirho :: forall (aChirho :: Type). ()
candidateChirho = ()
valueChirho :: ()
valueChirho = acceptChirho candidateChirho
