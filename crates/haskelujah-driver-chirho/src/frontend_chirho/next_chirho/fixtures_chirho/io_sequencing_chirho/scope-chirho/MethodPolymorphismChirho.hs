-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
firstIOChirho :: IO Integer
firstIOChirho = fmap (\value_chirho -> value_chirho + 1) (pure 2)
secondIOChirho :: IO Bool
secondIOChirho = fmap (\value_chirho -> value_chirho) (pure True)
main = firstIOChirho >>= print
