-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ALibChirho where
class PChirho aChirho where
  pChirho :: aChirho -> Char
  qChirho :: aChirho -> Char
  qChirho valueChirho = pChirho valueChirho
data BoxChirho aChirho = BoxChirho aChirho
instance PChirho aChirho => PChirho (BoxChirho aChirho) where
  pChirho (BoxChirho valueChirho) = qChirho valueChirho
