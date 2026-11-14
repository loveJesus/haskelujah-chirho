-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

module ComparisonStrictChirho where
boomChirho :: Integer
boomChirho = partialChirho True
partialChirho valueChirho = case valueChirho of { False -> 0 }
leftChirho = (boomChirho == 0)
rightChirho = (0 == boomChirho)
compareLeftChirho = compare boomChirho 0
compareRightChirho = compare 0 boomChirho
