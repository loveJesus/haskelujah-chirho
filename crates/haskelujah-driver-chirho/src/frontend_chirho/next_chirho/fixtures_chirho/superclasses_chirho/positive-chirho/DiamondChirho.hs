-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module DiamondChirho where
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
class FirstChirho aChirho => OtherChirho aChirho where
  otherChirho :: aChirho -> Char
class (ThirdChirho aChirho, OtherChirho aChirho) => DiamondChirho aChirho where
  diamondChirho :: aChirho -> Char
instance OtherChirho Bool where
  otherChirho ignoredChirho = 'd'
instance DiamondChirho Bool where
  diamondChirho ignoredChirho = 'e'
inspectChirho :: DiamondChirho aChirho => aChirho -> Char
inspectChirho valueChirho = case (firstChirho valueChirho, secondChirho valueChirho, thirdChirho valueChirho, otherChirho valueChirho, diamondChirho valueChirho) of
  ('a', 'b', 'c', 'd', 'e') -> 'd'
  _ -> '!'
resultChirho = inspectChirho True
