-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ALibChirho where
class AChirho aChirho where aChirho :: aChirho -> Char
class AChirho aChirho => BChirho aChirho where bChirho :: aChirho -> Char
probeChirho :: BChirho aChirho => aChirho -> Char
probeChirho valueChirho = aChirho valueChirho
