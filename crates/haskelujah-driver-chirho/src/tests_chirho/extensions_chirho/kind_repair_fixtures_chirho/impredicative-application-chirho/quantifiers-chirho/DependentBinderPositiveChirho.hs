-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, RankNTypes, KindSignatures, DataKinds, TypeApplications #-}
module DependentBinderPositiveChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
acceptChirho :: (forall (aChirho :: Bool). Proxy aChirho -> Proxy aChirho) -> ()
acceptChirho _ = ()
candidateChirho :: forall (aChirho :: Bool). Proxy aChirho -> Proxy aChirho
candidateChirho valueChirho = valueChirho
valueChirho :: ()
valueChirho = acceptChirho candidateChirho
