-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module IntComparisonsChirho where
equalChirho = if (2 :: Int) == 2 then 't' else 'f'
unequalChirho = if (2 :: Int) /= 3 then 't' else 'f'
lessChirho = if (1 :: Int) < 2 then 't' else 'f'
lessEqualChirho = if (2 :: Int) <= 2 then 't' else 'f'
greaterChirho = if (3 :: Int) > 2 then 't' else 'f'
greaterEqualChirho = if (2 :: Int) >= 2 then 't' else 'f'
orderChirho = case compare (1 :: Int) 2 of LT -> 'l'; EQ -> 'e'; GT -> 'g'
maximumChirho :: Int
maximumChirho = max 2 3
minimumChirho :: Int
minimumChirho = min 2 3
notChirho = if not ((2 :: Int) == 2) then 't' else 'f'
