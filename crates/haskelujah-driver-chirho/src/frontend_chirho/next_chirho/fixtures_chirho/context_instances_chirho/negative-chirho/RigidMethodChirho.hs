-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module RigidMethodChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'b'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
data BoxChirho aChirho = BoxChirho aChirho
instance RenderChirho aChirho => RenderChirho (BoxChirho aChirho) where
  renderChirho (BoxChirho valueChirho) = if valueChirho then 'x' else 'y'
