-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

module ComparisonEdgesChirho where
smallChirho :: Integer
smallChirho = -9223372036854775808
largeChirho :: Integer
largeChirho = 340282366920938463463374607431768211457
negativeChirho :: Integer
negativeChirho = -340282366920938463463374607431768211457
resultChirho = if (smallChirho < 0) then (if largeChirho > 9223372036854775808 then 'e' else 'x') else 'x'
equalChirho = if largeChirho == (340282366920938463463374607431768211456 + 1) then 't' else 'x'
unequalChirho = if largeChirho /= negativeChirho then 'u' else 'x'
leChirho = if largeChirho <= largeChirho then 'l' else 'x'
geChirho = if negativeChirho >= negativeChirho then 'g' else 'x'
negativeOrderChirho = case compare negativeChirho smallChirho of { LT -> 'n'; EQ -> 'x'; GT -> 'x' }
zeroOrderChirho = case compare (largeChirho + negativeChirho) 0 of { EQ -> 'z'; _ -> 'x' }
maximumChirho = max largeChirho negativeChirho
minimumChirho = min largeChirho negativeChirho
