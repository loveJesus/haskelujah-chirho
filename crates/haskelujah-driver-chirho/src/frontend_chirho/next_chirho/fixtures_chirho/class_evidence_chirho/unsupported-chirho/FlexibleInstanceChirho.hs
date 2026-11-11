-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module FlexibleInstanceChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho aChirho where
  renderChirho ignoredChirho = 'x'
