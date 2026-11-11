-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module MutualPolymorphismChirho where
leftChirho valueChirho = if True then valueChirho else rightChirho valueChirho
rightChirho valueChirho = leftChirho valueChirho
boolChirho = leftChirho True
resultChirho = if boolChirho then rightChirho 'm' else 'n'
