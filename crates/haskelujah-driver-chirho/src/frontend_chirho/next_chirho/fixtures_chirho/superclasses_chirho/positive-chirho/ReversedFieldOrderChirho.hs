-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ReversedFieldOrderChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class SecondChirho aChirho where
  secondChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho ignoredChirho = 'a'
instance SecondChirho Bool where
  secondChirho ignoredChirho = 'b'
class (SecondChirho aChirho, FirstChirho aChirho) => BothChirho aChirho where
  thirdChirho :: aChirho -> Char
  fourthChirho :: aChirho -> Char
instance BothChirho Bool where
  fourthChirho ignoredChirho = 'd'
  thirdChirho ignoredChirho = 'c'
inspectChirho :: BothChirho aChirho => aChirho -> Char
inspectChirho valueChirho = case (firstChirho valueChirho, secondChirho valueChirho, thirdChirho valueChirho, fourthChirho valueChirho) of
  ('a', 'b', 'c', 'd') -> 'o'
  _ -> '!'
resultChirho = inspectChirho True
