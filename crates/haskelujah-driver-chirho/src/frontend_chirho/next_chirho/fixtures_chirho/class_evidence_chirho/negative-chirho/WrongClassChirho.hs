-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module WrongClassChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class SecondChirho aChirho where
  secondChirho :: aChirho -> Char
instance FirstChirho Bool where
  firstChirho flagChirho = if flagChirho then 'b' else 'f'
resultChirho = secondChirho True
