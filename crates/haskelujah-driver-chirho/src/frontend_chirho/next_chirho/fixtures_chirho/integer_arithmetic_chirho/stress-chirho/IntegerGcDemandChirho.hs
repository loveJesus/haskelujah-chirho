-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerGcDemandChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
data NatChirho = ZeroChirho | SuccChirho NatChirho
addNatChirho ZeroChirho rightChirho = rightChirho
addNatChirho (SuccChirho leftChirho) rightChirho = SuccChirho (addNatChirho leftChirho rightChirho)
mulNatChirho ZeroChirho _ = ZeroChirho
mulNatChirho (SuccChirho leftChirho) rightChirho = addNatChirho rightChirho (mulNatChirho leftChirho rightChirho)
tenChirho = (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho (SuccChirho ZeroChirho))))))))))
hundredChirho = mulNatChirho tenChirho tenChirho
bigChirho = mulNatChirho hundredChirho hundredChirho
lastChirho :: NatChirho -> Integer
lastChirho ZeroChirho = 7
lastChirho (SuccChirho restChirho) = lastChirho restChirho
resultChirho :: Integer
resultChirho = lastChirho bigChirho + (2 + 3)
