-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module NominalDataChirho where
data CharChirho = CharChirho Bool
data BoxChirho aChirho = BoxChirho aChirho
resultChirho = case (BoxChirho (CharChirho True) :: BoxChirho CharChirho) of
  BoxChirho (CharChirho flagChirho) -> if flagChirho then 'i' else '!'
