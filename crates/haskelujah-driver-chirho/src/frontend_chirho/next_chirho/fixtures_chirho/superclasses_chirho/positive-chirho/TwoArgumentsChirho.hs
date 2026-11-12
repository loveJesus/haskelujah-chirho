-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TwoArgumentsChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class FirstChirho aChirho => SecondChirho aChirho where
  secondChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'b'
instance SecondChirho Bool where
  secondChirho ignoredChirho = 'u'
instance FirstChirho Char where
  firstChirho valueChirho = valueChirho
instance SecondChirho Char where
  secondChirho ignoredChirho = 'v'
inspectChirho firstValueChirho secondValueChirho = case (firstChirho firstValueChirho, firstChirho secondValueChirho, secondChirho firstValueChirho, secondChirho secondValueChirho) of
  ('b', 'z', 'u', 'v') -> 'p'
  _ -> '!'
resultChirho = inspectChirho True 'z'
