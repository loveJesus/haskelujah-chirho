-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, PolyKinds, TypeApplications, TypeFamilies #-}
module Main where
import Data.Kind (Type)
data KProxyChirho (aChirho :: Type) = KProxyChirho
type family KPChirho (pChirho :: KProxyChirho kChirho)
type instance KPChirho ('KProxyChirho :: KProxyChirho Bool) = Int
type instance KPChirho ('KProxyChirho :: KProxyChirho Type) = Char
intChirho :: KPChirho @Bool ('KProxyChirho @Bool)
intChirho = 11
charChirho :: KPChirho @Type ('KProxyChirho @Type)
charChirho = 'e'
main :: IO ()
main = do print intChirho; print charChirho
