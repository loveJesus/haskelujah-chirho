-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ExtraArgumentsChirho where
data BoxChirho aChirho = BoxChirho aChirho
type FChirho = BoxChirho
readChirho :: FChirho Char -> Char
readChirho (BoxChirho valueChirho) = valueChirho
resultChirho = readChirho (BoxChirho 'e')
