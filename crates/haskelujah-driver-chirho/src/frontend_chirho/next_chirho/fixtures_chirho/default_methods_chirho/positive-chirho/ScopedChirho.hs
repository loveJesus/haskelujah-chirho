-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ScopedChirho where
class PreserveChirho aChirho where
  preserveChirho :: aChirho -> aChirho
  preserveChirho valueChirho = let
    helperChirho :: aChirho -> aChirho
    helperChirho ignoredChirho = valueChirho
    in helperChirho valueChirho
instance PreserveChirho Char
resultChirho = preserveChirho 'v'
