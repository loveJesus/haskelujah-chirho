-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module WrongContextChirho where
class FirstChirho aChirho where
  firstChirho :: aChirho -> Char
class SecondChirho aChirho where
  secondChirho :: aChirho -> Char
data BoxChirho aChirho = BoxChirho aChirho
instance SecondChirho aChirho => FirstChirho (BoxChirho aChirho) where
  firstChirho (BoxChirho valueChirho) = firstChirho valueChirho
