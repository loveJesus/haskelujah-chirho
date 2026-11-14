-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module IntArithmeticChirho where
nestedChirho :: Int
nestedChirho = (1 + 2) * 3
wrappedAddChirho :: Int
wrappedAddChirho = 9223372036854775807 + 1
wrappedNegateChirho :: Int
wrappedNegateChirho = negate (-9223372036854775808)
wrappedMultiplyChirho :: Int
wrappedMultiplyChirho = 4611686018427387904 * 4
wrappedSubtractChirho :: Int
wrappedSubtractChirho = (-9223372036854775808) - 1
convertedLowChirho :: Int
convertedLowChirho = fromInteger 18446744073709551621
convertedNegativeChirho :: Int
convertedNegativeChirho = fromInteger (-9223372036854775809)
minimumAbsChirho :: Int
minimumAbsChirho = abs (-9223372036854775808)
negativeAbsChirho :: Int
negativeAbsChirho = abs (-7)
negativeSignumChirho :: Int
negativeSignumChirho = signum (-7)
zeroSignumChirho :: Int
zeroSignumChirho = signum 0
positiveSignumChirho :: Int
positiveSignumChirho = signum 7
integerChirho :: Integer
integerChirho = 18446744073709551621
convertedTypedChirho :: Int
convertedTypedChirho = fromInteger integerChirho
