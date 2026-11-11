-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ResultDirectedChirho where
class DefChirho aChirho where
  defChirho :: aChirho
instance DefChirho Bool where
  defChirho = True
instance DefChirho Char where
  defChirho = 'd'
pairChirho = (defChirho, defChirho) :: (Bool, Char)
resultChirho = case pairChirho of
  (True, valueChirho) -> valueChirho
  _ -> '!'
