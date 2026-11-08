-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, KindSignatures, PolyKinds, TypeApplications, TypeFamilies #-}
module Main where
import Data.Kind (Type)
data KProxyChirho (aChirho :: Type) = KProxyChirho
type family KPChirho (pChirho :: KProxyChirho kChirho)
type instance KPChirho ('KProxyChirho :: KProxyChirho Bool) = Int
type instance KPChirho ('KProxyChirho :: KProxyChirho Type) = Char
intChirho :: KPChirho ('KProxyChirho :: KProxyChirho Bool)
intChirho = 3
charChirho :: KPChirho ('KProxyChirho :: KProxyChirho Type)
charChirho = 'x'
main :: IO ()
main = do print intChirho; print charChirho
