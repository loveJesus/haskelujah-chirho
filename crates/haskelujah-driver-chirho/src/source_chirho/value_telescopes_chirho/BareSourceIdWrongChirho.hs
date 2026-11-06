-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeApplications #-}
module BareSourceIdWrongChirho where
import Prelude hiding (id)
import Data.Function (id)
valueChirho = id @Int
