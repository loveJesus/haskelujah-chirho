-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module SourceShowDefaultsChirho where
data TChirho = TChirho
instance Show TChirho where
  show _ = "box"
resultChirho = showsPrec 9 TChirho "!"
listChirho = showList [TChirho,TChirho] "!"
