-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

{-# LANGUAGE NoImplicitPrelude #-}
module HomeEqDefaultChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
class Eq a where
  (==) :: a -> a -> a
instance Eq Integer where
  xChirho == _ = xChirho
resultChirho = 1 == 2
