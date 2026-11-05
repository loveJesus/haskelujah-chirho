-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures, ExplicitForAll, TypeApplications #-}
module UnusedHigherKindWrongChirho where
import Data.Kind (Type)
fooChirho :: forall (fChirho :: Type -> Type). Int
fooChirho = 0
valueChirho :: Int
valueChirho = fooChirho @Int
