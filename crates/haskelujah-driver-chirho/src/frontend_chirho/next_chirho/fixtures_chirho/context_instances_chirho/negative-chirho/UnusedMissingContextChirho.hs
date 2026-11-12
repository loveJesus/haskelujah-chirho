-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module UnusedMissingContextChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
class MissingChirho aChirho where
  missingChirho :: aChirho -> Char
data BoxChirho aChirho = BoxChirho aChirho
instance MissingChirho aChirho => RenderChirho (BoxChirho aChirho) where
  renderChirho ignoredChirho = 'x'
resultChirho = renderChirho (BoxChirho True)
