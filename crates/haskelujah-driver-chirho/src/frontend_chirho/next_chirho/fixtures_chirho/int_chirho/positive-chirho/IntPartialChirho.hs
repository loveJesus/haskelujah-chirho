-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module IntPartialChirho where
boomIntChirho :: Int
boomIntChirho = case True of False -> 0
ignoreChirho _ = 7 :: Int
partialChirho = (+) boomIntChirho
resultChirho = ignoreChirho partialChirho
