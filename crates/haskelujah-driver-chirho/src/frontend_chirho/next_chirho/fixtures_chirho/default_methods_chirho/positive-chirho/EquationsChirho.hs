-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module EquationsChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  defaultedChirho :: aChirho -> Bool -> Char
  defaultedChirho valueChirho True = renderChirho valueChirho
  defaultedChirho ignoredChirho False = 'f'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
resultChirho = case (defaultedChirho 'e' True, defaultedChirho '!' False) of
  ('e', 'f') -> 'e'
  _ -> '!'
