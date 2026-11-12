-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ScopedReuseGoodChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
outerChirho :: forall aChirho. RenderChirho aChirho => aChirho -> Char
outerChirho valueChirho = let
  innerChirho :: aChirho -> Char
  innerChirho innerValueChirho = renderChirho innerValueChirho
  in innerChirho valueChirho
resultChirho = outerChirho 's'
