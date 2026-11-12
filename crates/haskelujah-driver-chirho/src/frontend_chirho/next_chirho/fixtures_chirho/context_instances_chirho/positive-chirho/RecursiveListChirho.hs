-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module RecursiveListChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance RenderChirho Bool where
  renderChirho ignoredChirho = 'b'
instance RenderChirho Char where
  renderChirho valueChirho = valueChirho
instance RenderChirho aChirho => RenderChirho [aChirho] where
  renderChirho valuesChirho = case valuesChirho of
    [] -> 'e'
    firstChirho : restChirho -> case restChirho of
      [] -> renderChirho firstChirho
      _ -> renderChirho restChirho
resultChirho = renderChirho [[True], [False]]
