-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MethodOrderChirho where
class PairChirho aChirho where
  firstChirho :: aChirho -> Char
  secondChirho :: aChirho -> Char
instance PairChirho Bool where
  secondChirho flagChirho = if flagChirho then 's' else '!'
  firstChirho ignoredChirho = markerChirho
markerChirho = 'f'
resultChirho = case (firstChirho True, secondChirho True) of
  ('f', 's') -> 'o'
  _ -> '!'
