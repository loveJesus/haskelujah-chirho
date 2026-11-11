-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module LazyPatternsChirho where
data BoxChirho aChirho = BoxChirho aChirho
loopChirho = loopChirho
ignoreChirho _ = 'l'
variableChirho = case loopChirho of valueChirho -> 'v'
wildcardChirho = case loopChirho of _ -> 'w'
fieldChirho (BoxChirho _) = 'f'
resultChirho = case ignoreChirho loopChirho of
  'l' -> case variableChirho of
    'v' -> case wildcardChirho of
      'w' -> fieldChirho (BoxChirho loopChirho)
      _ -> '!'
    _ -> '!'
  _ -> '!'
