-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module BUseChirho where
import ALibChirho
data TChirho = TChirho
instance PChirho TChirho where
  pChirho _ = 't'
resultChirho :: Char
resultChirho = qChirho (BoxChirho TChirho)
