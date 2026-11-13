-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ALibChirho (makeChirho, useChirho) where
data HiddenChirho = HiddenChirho Char
makeChirho = HiddenChirho 'a'
useChirho (HiddenChirho valueChirho) = valueChirho
