-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ConstructorCaseChirho where
data MaybeChirho aChirho = NothingChirho | JustChirho aChirho
buildChirho = JustChirho
chooseChirho fallbackChirho valueChirho = case valueChirho of
  NothingChirho -> fallbackChirho
  JustChirho itemChirho -> itemChirho
boolChirho = chooseChirho False (buildChirho True)
resultChirho = if boolChirho then chooseChirho '!' (buildChirho 'a') else '!'
