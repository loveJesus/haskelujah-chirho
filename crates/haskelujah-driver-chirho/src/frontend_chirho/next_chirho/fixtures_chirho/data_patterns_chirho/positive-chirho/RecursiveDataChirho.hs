-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module RecursiveDataChirho where
data TreeChirho aChirho = TipChirho aChirho | ForkChirho (TreeChirho aChirho) (TreeChirho aChirho)
data LeftChirho aChirho = LeftChirho (RightChirho aChirho) | LeafChirho aChirho
data RightChirho aChirho = RightChirho (LeftChirho aChirho)
firstChirho (TipChirho itemChirho) = itemChirho
firstChirho (ForkChirho leftChirho _) = firstChirho leftChirho
unwrapChirho (LeftChirho (RightChirho (LeafChirho itemChirho))) = itemChirho
unwrapChirho _ = '!'
resultChirho = firstChirho (ForkChirho (TipChirho (unwrapChirho (LeftChirho (RightChirho (LeafChirho 'r'))))) (TipChirho '!'))
