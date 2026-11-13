-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module DifferentTypeChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  defaultedChirho :: aChirho -> Char
  defaultedChirho ignoredChirho = renderChirho True
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'd'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
resultChirho = defaultedChirho '!'
