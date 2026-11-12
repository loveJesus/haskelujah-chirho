-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ExplicitContextsChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class FirstChirho aChirho => SecondChirho aChirho where
  secondChirho :: aChirho -> Char
class SecondChirho aChirho => ThirdChirho aChirho where
  thirdChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'a'
instance SecondChirho Bool where
  secondChirho ignoredChirho = 'b'
instance ThirdChirho Bool where
  thirdChirho ignoredChirho = 'c'
inspectChirho :: (FirstChirho aChirho, ThirdChirho aChirho) => aChirho -> Char
inspectChirho valueChirho = case (firstChirho valueChirho, thirdChirho valueChirho) of
  ('a', 'c') -> 'e'
  _ -> '!'
resultChirho = inspectChirho True
