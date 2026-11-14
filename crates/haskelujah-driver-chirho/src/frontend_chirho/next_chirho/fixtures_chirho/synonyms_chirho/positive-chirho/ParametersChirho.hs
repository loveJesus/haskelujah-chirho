-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ParametersChirho where
type SwapChirho aChirho bChirho = (bChirho, aChirho)
type ForwardChirho aChirho = EndoChirho aChirho
type EndoChirho aChirho = aChirho -> aChirho
identityChirho :: ForwardChirho (SwapChirho Bool Char)
identityChirho valueChirho = valueChirho
resultChirho = case identityChirho ('p', True) of (characterChirho, _) -> characterChirho
