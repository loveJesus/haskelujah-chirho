-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, ScopedTypeVariables, ExplicitForAll, KindSignatures #-}
module Main where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
import Data.Typeable (Typeable)
keepChirho :: forall (kChirho :: Type) (aChirho :: kChirho). Typeable aChirho => Proxy aChirho -> String
keepChirho _ = "prefix"
main :: IO ()
main = putStrLn (keepChirho (Proxy :: Proxy Maybe))
