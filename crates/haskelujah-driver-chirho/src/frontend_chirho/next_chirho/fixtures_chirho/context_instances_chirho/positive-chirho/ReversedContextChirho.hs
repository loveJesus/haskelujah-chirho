-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ReversedContextChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'b'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
data PairChirho aChirho bChirho = PairChirho aChirho bChirho
instance (RenderChirho bChirho, RenderChirho aChirho) => RenderChirho (PairChirho aChirho bChirho) where
  renderChirho (PairChirho firstChirho secondChirho) = case renderChirho firstChirho of
    'b' -> renderChirho secondChirho
    _ -> '!'
resultChirho = renderChirho (PairChirho True 'o')
