-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module MixedPatternChirho where
data TreeChirho = LeafChirho Char | TreeChirho :++ TreeChirho
infixr 4 :++
fChirho (aChirho :++ bChirho :++ cChirho) = aChirho :++ bChirho :++ cChirho
