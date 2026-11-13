-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MutualChirho where
class RenderChirho aChirho where
  firstChirho :: aChirho -> Char
  firstChirho valueChirho = secondChirho valueChirho
  secondChirho :: aChirho -> Char
  secondChirho valueChirho = firstChirho valueChirho
instance RenderChirho Bool where
  firstChirho ignoredChirho = 'b'
instance RenderChirho Char where
  secondChirho valueChirho = valueChirho
resultChirho = case (secondChirho True, firstChirho 'c') of
  ('b', 'c') -> 'm'
  _ -> '!'
