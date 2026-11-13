-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ClassPatternChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  (leftChirho, rightChirho) = ('a', 'b')
  laterChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'p'
resultChirho = renderChirho True
