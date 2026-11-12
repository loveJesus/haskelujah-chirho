-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TwoClassesChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class SecondChirho aChirho where
  secondChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'f'
instance SecondChirho Char where
  secondChirho ignoredChirho = 's'
pairChirho :: (FirstChirho aChirho, SecondChirho bChirho) => aChirho -> bChirho -> (Char, Char)
pairChirho leftChirho rightChirho = (firstChirho leftChirho, secondChirho rightChirho)
resultChirho = case pairChirho True 'q' of
  ('f', 's') -> 'o'
  _ -> '!'
