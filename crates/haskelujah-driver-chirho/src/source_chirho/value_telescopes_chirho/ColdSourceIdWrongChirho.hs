-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeApplications #-}
module ColdSourceIdWrongChirho where
import Prelude hiding (id)
import qualified IdentityProducerChirho as FunctionChirho
valueChirho = FunctionChirho.id @Int
