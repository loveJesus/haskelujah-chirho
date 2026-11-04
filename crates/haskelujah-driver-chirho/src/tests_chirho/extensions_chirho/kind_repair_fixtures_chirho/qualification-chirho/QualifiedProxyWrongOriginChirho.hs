-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures #-}
module Main where
import qualified Data.Proxy as PChirho
import Data.Typeable (Typeable)
data Proxy (aChirho :: kChirho) = Proxy
actualChirho :: PChirho.Proxy Maybe
actualChirho = (Proxy :: Proxy Maybe)
localChirho :: Proxy Bool
localChirho = Proxy
main :: IO ()
main = pure ()
