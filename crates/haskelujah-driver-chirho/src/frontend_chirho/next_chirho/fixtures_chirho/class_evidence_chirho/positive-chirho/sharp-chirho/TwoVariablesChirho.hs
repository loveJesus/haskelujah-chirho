-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TwoVariablesChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho flagChirho = if flagChirho then 'b' else 'f'
instance RenderChirho Char where
  renderChirho letterChirho = letterChirho
pairChirho leftChirho rightChirho = (renderChirho leftChirho, renderChirho rightChirho)
resultChirho = case pairChirho True 'q' of
  ('b', valueChirho) -> valueChirho
  _ -> '!'
