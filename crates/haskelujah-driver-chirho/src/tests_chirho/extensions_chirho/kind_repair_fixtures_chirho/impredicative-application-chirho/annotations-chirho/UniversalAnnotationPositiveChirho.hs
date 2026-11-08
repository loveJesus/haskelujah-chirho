-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ScopedTypeVariables, PolyKinds #-}
module UniversalAnnotationPositiveChirho where
import Data.Proxy
fooChirho = (undefined :: Proxy aChirho) :: forall aChirho. Proxy aChirho
