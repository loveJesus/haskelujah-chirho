-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module SignedCycleChirho where
constantChirho :: Bool -> Bool
constantChirho flagChirho = checkChirho flagChirho
checkChirho flagChirho = if flagChirho then constantChirho False else False
resultChirho = if checkChirho True then 'n' else 's'
