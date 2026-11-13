-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module SeparateEquationsChirho where
class RenderChirho aChirho where
  firstChirho :: aChirho -> Bool -> Char
  firstChirho ignoredChirho True = 'a'
  secondChirho :: aChirho -> Char
  secondChirho ignoredChirho = 'b'
  firstChirho ignoredChirho False = 'c'
