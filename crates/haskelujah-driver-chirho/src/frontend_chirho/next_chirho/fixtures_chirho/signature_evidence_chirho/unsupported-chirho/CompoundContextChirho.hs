-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module CompoundContextChirho where
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
applyChirho :: RenderChirho [aChirho] => [aChirho] -> Char
applyChirho valueChirho = renderChirho valueChirho
