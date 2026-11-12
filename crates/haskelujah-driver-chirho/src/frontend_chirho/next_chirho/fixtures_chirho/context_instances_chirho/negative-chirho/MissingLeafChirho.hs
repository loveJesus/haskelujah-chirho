-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MissingLeafChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
data BoxChirho aChirho = BoxChirho aChirho
instance RenderChirho aChirho => RenderChirho (BoxChirho aChirho) where
  renderChirho (BoxChirho valueChirho) = renderChirho valueChirho
resultChirho = renderChirho (BoxChirho True)
