-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ActualInstanceChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  defaultedChirho :: aChirho -> Char
  defaultedChirho valueChirho = renderChirho valueChirho
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'b'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
resultChirho = case (defaultedChirho True, defaultedChirho 'x') of
  ('b', 'x') -> 'a'
  _ -> '!'
