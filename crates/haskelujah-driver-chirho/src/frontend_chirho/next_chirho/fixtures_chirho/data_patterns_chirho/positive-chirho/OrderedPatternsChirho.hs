-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module OrderedPatternsChirho where
data OptionChirho aChirho = NoneChirho | SomeChirho aChirho
loopChirho = loopChirho
chooseChirho True NoneChirho = 'a'
chooseChirho False _ = 'b'
chooseChirho True _ = 'c'
resultChirho = case chooseChirho True NoneChirho of
  'a' -> case chooseChirho False loopChirho of
    'b' -> chooseChirho True (SomeChirho 'x')
    _ -> '!'
  _ -> '!'
