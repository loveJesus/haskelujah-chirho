-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module RecursiveEvidenceChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho flagChirho = if flagChirho then 'b' else 'f'
instance RenderChirho Char where
  renderChirho letterChirho = letterChirho
firstChirho flagChirho valueChirho = if flagChirho then renderChirho valueChirho else secondChirho True valueChirho
secondChirho flagChirho valueChirho = if flagChirho then renderChirho valueChirho else firstChirho True valueChirho
resultChirho = case firstChirho False True of
  'b' -> secondChirho False 'q'
  _ -> '!'
