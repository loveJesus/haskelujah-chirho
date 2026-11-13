-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module MissingGivenChirho where
class OtherChirho aChirho where
  otherChirho :: aChirho -> Char
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  renderChirho valueChirho = otherChirho valueChirho
instance OtherChirho Bool where
  otherChirho ignoredChirho = 'x'
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'o'
