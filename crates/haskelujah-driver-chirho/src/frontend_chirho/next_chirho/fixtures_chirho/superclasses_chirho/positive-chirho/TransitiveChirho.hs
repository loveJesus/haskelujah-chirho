-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TransitiveChirho where
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
inspectChirho :: ThirdChirho aChirho => aChirho -> Char
inspectChirho valueChirho = case (firstChirho valueChirho, secondChirho valueChirho, thirdChirho valueChirho) of
  ('a', 'b', 'c') -> 't'
  _ -> '!'
resultChirho = inspectChirho True
