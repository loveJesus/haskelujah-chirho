-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ScopedAnnotationChirho where
identityChirho :: forall aChirho. aChirho -> aChirho
identityChirho valueChirho = (valueChirho :: aChirho)
resultChirho = identityChirho 'a'
