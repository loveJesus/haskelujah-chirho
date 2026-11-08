-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ScopedTypeVariables, FlexibleContexts #-}
module ImprovementAnchoredChoiceChirho where
import Data.Proxy
type family FChirho aChirho
type family UnFChirho aChirho
fChirho :: forall bChirho cChirho. (UnFChirho (FChirho bChirho) ~ bChirho, UnFChirho (FChirho cChirho) ~ cChirho) => Proxy bChirho -> FChirho cChirho -> ()
fChirho _ _ = ()
