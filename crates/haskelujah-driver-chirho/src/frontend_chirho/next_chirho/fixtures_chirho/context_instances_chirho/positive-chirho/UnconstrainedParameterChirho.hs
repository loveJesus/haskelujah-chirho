-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module UnconstrainedParameterChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
data BoxChirho aChirho = BoxChirho aChirho
data LeafChirho = LeafChirho
instance RenderChirho (BoxChirho aChirho) where
  renderChirho ignoredChirho = 'u'
resultChirho = renderChirho (BoxChirho LeafChirho)
