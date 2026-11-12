-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TupleDictionaryChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'b'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
instance (RenderChirho aChirho, RenderChirho bChirho) => RenderChirho (aChirho, bChirho) where
  renderChirho (firstChirho, secondChirho) = case renderChirho firstChirho of
    'b' -> renderChirho secondChirho
    _ -> '!'
resultChirho = renderChirho (True, 't')
