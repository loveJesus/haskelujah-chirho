-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ExplicitForallChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho flagChirho = if flagChirho then 'b' else 'f'
instance RenderChirho Char where
  renderChirho letterChirho = letterChirho
applyChirho :: forall aChirho. RenderChirho aChirho => aChirho -> Char
applyChirho valueChirho = renderChirho valueChirho
resultChirho = applyChirho 'e'
