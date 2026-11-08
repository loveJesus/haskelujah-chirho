-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, PolyKinds, TypeApplications, TypeFamilies #-}
module Main where
import Data.Kind (Type)
data KProxyChirho (aChirho :: Type) = KProxyChirho
type family KPChirho (pChirho :: KProxyChirho kChirho)
type instance KPChirho ('KProxyChirho :: KProxyChirho Bool) = Int
type instance KPChirho ('KProxyChirho :: KProxyChirho Type) = Char
badChirho :: KPChirho ('KProxyChirho :: KProxyChirho Type)
badChirho = (3 :: Int)
main :: IO ()
main = pure ()
