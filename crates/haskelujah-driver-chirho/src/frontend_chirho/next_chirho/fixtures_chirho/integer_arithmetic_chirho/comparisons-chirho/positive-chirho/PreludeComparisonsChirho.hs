-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

module PreludeComparisonsChirho where
lessChirho = case compare 1 2 of { LT -> 'l'; EQ -> 'e'; GT -> 'g' }
equalChirho = case 2 == 2 of { True -> 't'; False -> 'f' }
greaterChirho = case compare 3 2 of { LT -> 'l'; EQ -> 'e'; GT -> 'g' }
sameChirho = case compare 2 2 of { LT -> 'l'; EQ -> 'e'; GT -> 'g' }
notChirho = if not (1 == 2) then 'n' else 'x'
notEqualChirho = if not (2 == 2) then 'x' else 'n'
