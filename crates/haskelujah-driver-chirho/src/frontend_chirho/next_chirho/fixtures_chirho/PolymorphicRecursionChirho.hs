-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module PolymorphicRecursionChirho where
badChirho valueChirho = if True then 'r' else badChirho [valueChirho]
