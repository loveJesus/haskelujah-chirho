-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module DistinctClassesChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class SecondChirho aChirho where
  secondChirho :: aChirho -> Char
class RenderChirho aChirho where
  renderChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'f'
instance SecondChirho Char where
  secondChirho valueChirho = valueChirho
data PairChirho aChirho bChirho = PairChirho aChirho bChirho
instance (SecondChirho bChirho, FirstChirho aChirho) => RenderChirho (PairChirho aChirho bChirho) where
  renderChirho (PairChirho firstValueChirho secondValueChirho) = case firstChirho firstValueChirho of
    'f' -> secondChirho secondValueChirho
    _ -> '!'
resultChirho = renderChirho (PairChirho True 'd')
