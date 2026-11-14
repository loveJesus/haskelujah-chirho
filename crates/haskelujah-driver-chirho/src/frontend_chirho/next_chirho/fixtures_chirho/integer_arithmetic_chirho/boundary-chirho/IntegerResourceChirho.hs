-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerResourceChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
squareChirho :: Integer -> Integer
squareChirho valueChirho = valueChirho * valueChirho
startChirho :: Integer
startChirho = 340282366920938463463374607431768211457
power0Chirho = squareChirho startChirho
power1Chirho = squareChirho power0Chirho
power2Chirho = squareChirho power1Chirho
power3Chirho = squareChirho power2Chirho
power4Chirho = squareChirho power3Chirho
power5Chirho = squareChirho power4Chirho
power6Chirho = squareChirho power5Chirho
power7Chirho = squareChirho power6Chirho
power8Chirho = squareChirho power7Chirho
power9Chirho = squareChirho power8Chirho
power10Chirho = squareChirho power9Chirho
power11Chirho = squareChirho power10Chirho
resultChirho :: Integer
resultChirho = power11Chirho
