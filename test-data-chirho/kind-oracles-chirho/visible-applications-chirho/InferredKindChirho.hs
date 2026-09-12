-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, GADTs, TypeApplications, DataKinds, RankNTypes #-}
module InferredKindChirho where
import Data.Kind (Type)
data BoxChirho :: forall {indexChirho} (valueChirho :: indexChirho). Type
kindChirho :: BoxChirho @True -> ()
kindChirho _ = ()
