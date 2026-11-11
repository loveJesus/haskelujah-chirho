-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ListTupleAsChirho where
pickChirho [] = '!'
pickChirho [_, 'z'] = 'z'
pickChirho wholeChirho@(headChirho : _) = case wholeChirho of
  [_, _] -> (\(firstChirho, _) -> firstChirho) (headChirho, True)
  _ -> '!'
resultChirho = pickChirho "ep"
