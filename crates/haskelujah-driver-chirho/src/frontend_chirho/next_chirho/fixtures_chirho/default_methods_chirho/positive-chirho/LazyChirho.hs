-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module LazyChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  unusedChirho :: aChirho -> Char
  unusedChirho valueChirho = unusedChirho valueChirho
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'z'
resultChirho = renderChirho True
