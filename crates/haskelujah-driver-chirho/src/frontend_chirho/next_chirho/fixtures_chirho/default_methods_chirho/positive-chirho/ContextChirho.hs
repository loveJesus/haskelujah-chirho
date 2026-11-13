-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ContextChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  defaultedChirho :: aChirho -> Char
  defaultedChirho valueChirho = renderChirho valueChirho
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'c'
data BoxChirho aChirho = BoxChirho aChirho
instance RenderChirho aChirho => RenderChirho (BoxChirho aChirho) where
  renderChirho (BoxChirho valueChirho) = defaultedChirho valueChirho
resultChirho = defaultedChirho (BoxChirho (BoxChirho True))
