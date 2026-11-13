-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module PartialBoolInstanceChirho where
import GHC.Internal.Num (Num(..))

instance Num Bool where
    fromInteger _ = True
    xChirho + _ = xChirho
    _ * yChirho = yChirho
    abs xChirho = xChirho
    signum xChirho = xChirho
    negate False = True
    negate True = False
pickChirho False = 'a'
pickChirho True = 'b'
resultChirho = pickChirho (5 - 7)
