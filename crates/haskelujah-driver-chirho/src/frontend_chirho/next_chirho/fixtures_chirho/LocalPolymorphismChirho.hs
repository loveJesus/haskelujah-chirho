-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module LocalPolymorphismChirho where
resultChirho = let
  identityChirho valueChirho = valueChirho
  boolChirho = identityChirho True
  charChirho = identityChirho 'l'
  in if boolChirho then charChirho else 'n'
