-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ContextSuperclassChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class FirstChirho aChirho => SecondChirho aChirho where
  secondChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'a'
instance SecondChirho Bool where
  secondChirho ignoredChirho = 'b'
data BoxChirho aChirho = BoxChirho aChirho
instance FirstChirho aChirho => FirstChirho (BoxChirho aChirho) where
  firstChirho (BoxChirho valueChirho) = firstChirho valueChirho
instance SecondChirho aChirho => SecondChirho (BoxChirho aChirho) where
  secondChirho (BoxChirho valueChirho) = secondChirho valueChirho
inspectChirho :: SecondChirho aChirho => aChirho -> Char
inspectChirho valueChirho = case (firstChirho valueChirho, secondChirho valueChirho) of
  ('a', 'b') -> 's'
  _ -> '!'
resultChirho = inspectChirho (BoxChirho (BoxChirho True))
