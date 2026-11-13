-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module TopPolymorphismChirho where
resultChirho = if boolChirho then selfChirho charChirho else 'n'
boolChirho = identityChirho True
charChirho = identityChirho 'q'
selfChirho = identityChirho identityChirho
identityChirho valueChirho = valueChirho
