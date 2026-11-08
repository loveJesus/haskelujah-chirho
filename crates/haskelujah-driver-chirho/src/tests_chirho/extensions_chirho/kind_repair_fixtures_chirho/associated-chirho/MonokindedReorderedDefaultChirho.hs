-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoPolyKinds, KindSignatures, TypeFamilies, MultiParamTypeClasses, FlexibleInstances #-}
module Main where
import Data.Kind (Type)
class CChirho (a :: Type) (b :: Type) where
  type FChirho b x :: Type
  type FChirho z y = (z, y)
instance CChirho Bool Int
valueChirho :: FChirho Int Char
valueChirho = (1, 'x')
main :: IO ()
main = pure ()
