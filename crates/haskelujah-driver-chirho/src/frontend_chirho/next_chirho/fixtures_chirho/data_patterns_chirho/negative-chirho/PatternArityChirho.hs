-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module PatternArityChirho where
data BoxChirho aChirho = BoxChirho aChirho
pickChirho (BoxChirho firstChirho secondChirho) = firstChirho
