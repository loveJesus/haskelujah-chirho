-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module NestedSignatureChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho flagChirho = if flagChirho then 'b' else 'f'
instance RenderChirho Char where
  renderChirho letterChirho = letterChirho
outerChirho :: RenderChirho aChirho => aChirho -> Char
outerChirho ignoredChirho = let
  innerChirho :: RenderChirho aChirho => aChirho -> Char
  innerChirho valueChirho = renderChirho valueChirho
  in innerChirho 'q'
resultChirho = outerChirho True
