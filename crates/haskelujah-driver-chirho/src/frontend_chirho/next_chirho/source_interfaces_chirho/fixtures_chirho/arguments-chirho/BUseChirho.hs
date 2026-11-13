-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module BUseChirho where
import ALibChirho
data TChirho = TChirho
data UChirho = UChirho
instance PChirho TChirho where pChirho _ = 't'
instance PChirho UChirho where pChirho _ = 'u'
resultChirho = (qChirho (BoxChirho TChirho), qChirho (BoxChirho UChirho))
