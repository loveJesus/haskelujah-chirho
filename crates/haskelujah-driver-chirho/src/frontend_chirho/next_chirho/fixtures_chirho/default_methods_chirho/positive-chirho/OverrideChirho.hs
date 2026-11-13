-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module OverrideChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  defaultedChirho :: aChirho -> Char
  defaultedChirho valueChirho = renderChirho valueChirho
instance RenderChirho Bool where
  defaultedChirho ignoredChirho = 'o'
  renderChirho ignoredChirho = '!'
resultChirho = defaultedChirho True
