-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ScopedReuseBadChirho where
outerChirho :: forall aChirho. aChirho -> Char
outerChirho ignoredChirho = let
  innerChirho :: aChirho -> aChirho
  innerChirho valueChirho = valueChirho
  in innerChirho 'q'
