-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DefaultSignatures #-}
module DefaultSignatureChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
  default renderChirho :: aChirho -> Char
  renderChirho ignoredChirho = 'd'
instance RenderChirho Bool
