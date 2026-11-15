-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ShowNumbersChirho where
resultChirho = show 5
sumChirho = show (1 + 2)
negativeChirho = show (-5)
intPrecChirho = showsPrec 11 (-5 :: Int) ""
integerSixChirho = showsPrec 6 (-5 :: Integer) ""
integerSevenChirho = showsPrec 7 (-5 :: Integer) ""
bigChirho = showsPrec 7 (-12345678901234567890123 :: Integer) ""
positiveChirho = showsPrec 11 (18446744073709551616 :: Integer) ""
minimumChirho = show (fromInteger 9223372036854775808 :: Int)
minimumPrecChirho = showsPrec 11 (fromInteger 9223372036854775808 :: Int) ""
signedTailChirho = shows (12 :: Int) "!"
integerTailChirho = shows (12 :: Integer) "!"
hugeChirho = show (340282366920938463463374607431768211457 :: Integer)
