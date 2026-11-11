-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ComplexHeadChirho where
firstChirho letterChirho = 'f'
secondChirho letterChirho = 's'
pickChirho flagChirho = (if flagChirho then firstChirho else secondChirho) 'x'
resultChirho = case pickChirho True of
  'f' -> pickChirho False
  _ -> '!'
