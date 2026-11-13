-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module LaterNamesChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
  firstChirho ignoredChirho = helperChirho True
class LaterChirho aChirho where
  laterChirho :: aChirho -> Char
instance FirstChirho Char
instance LaterChirho Bool where
  laterChirho ignoredChirho = 'l'
helperChirho valueChirho = laterChirho valueChirho
resultChirho = firstChirho '!'
