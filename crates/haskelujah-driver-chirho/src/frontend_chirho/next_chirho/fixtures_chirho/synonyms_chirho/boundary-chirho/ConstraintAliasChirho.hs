-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ConstraintAliasChirho where
class HomeChirho aChirho where tagChirho :: aChirho -> Char
type CtxChirho aChirho = HomeChirho aChirho
gChirho :: CtxChirho aChirho => aChirho -> aChirho
gChirho valueChirho = valueChirho
