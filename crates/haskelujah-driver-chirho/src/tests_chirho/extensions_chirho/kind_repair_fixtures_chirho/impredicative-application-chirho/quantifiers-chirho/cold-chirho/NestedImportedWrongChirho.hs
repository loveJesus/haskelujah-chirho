-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeApplications #-}
module NestedImportedWrongChirho where
import qualified QuantifierOriginChirho as QChirho
valueChirho :: Bool
valueChirho = (QChirho.exposeChirho ()) @Bool True (3 :: Int)
